#[cfg(test)]

mod api_reply {
    use std::{sync::Once, time::Duration};
    use serde_json::json;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::stuff::max_test_duration::TestDuration;
    use crate::
        api::reply::api_reply::ApiReply
    ;
    use crate::error::api_error::ApiError;
    ///    
    static INIT: Once = Once::new();
    ///
    /// once called initialisation
    fn init_once() {
        INIT.call_once(|| {
                // implement your initialisation code to be called only once for current test file
            }
        )
    }
    ///
    /// returns:
    ///  - ...
    fn init_each() -> () {}
    ///
    /// 
    #[test]
    fn serialize() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!("");
        let self_id = "test ApiRequest";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let service_keep_alive = false;
        let test_data = [
            (
                ApiReply {
                    auth_token: "123zxy456!@#".to_string(),
                    id: "1".to_string(),
                    query: "{\"database\":\"test_api_query\",\"sql\":\"select * from customer limit 3;\"}".to_string(),
                    data: vec![],
                    keep_alive: service_keep_alive,
                    error: ApiError::empty(),
                },
                r#"{"authToken":"123zxy456!@#","id":"1","keepAlive":false,"query":"{\"database\":\"test_api_query\",\"sql\":\"select * from customer limit 3;\"}","data":[],"error":{"message":""}}"#,
            ),
            // (
            //     ApiQuery::new(
            //         ApiQueryKind::Sql(ApiQuerySql::new(database, "select * from customer limit 3;")),
            //         service_keep_alive, 
            //     ),
            //     keep_alive,
            //     r#"{"authToken":"123zxy456!@#","id":"2","sql":{"database":"test_api_query","sql":"select * from customer limit 3;"},"keepAlive":true,"debug":false}"#,
            // ),
            // (
            //     ApiQuery::new(
            //         ApiQueryKind::Python(ApiQueryPython::new("test_script", json!(HashMap::<String, f64>::new()))),
            //         service_keep_alive,
            //     ),
            //     keep_alive,
            //     r#"{"authToken":"123zxy456!@#","id":"3","python":{"script":"test_script","params":{}},"keepAlive":true,"debug":false}"#,
            // ),
            // (
            //     ApiQuery::new(
            //         ApiQueryKind::Executable(ApiQueryExecutable::new("test_app", json!(HashMap::<String, f64>::new()))),
            //         service_keep_alive,
            //     ),
            //     close_connection,
            //     r#"{"authToken":"123zxy456!@#","id":"4","executable":{"name":"test_app","params":{}},"keepAlive":false,"debug":false}"#,
            // ),
        ];
        for (reply, target) in test_data {
            println!("\n reply: {:?}", reply);
            let result = json!(reply);
            let target: serde_json::Value = serde_json::from_str(target).unwrap();
            assert!(result == target, "\n result: {:?}\n target: {:?}", result, target);
            println!("\n result: {:?}\n target: {:?}", result, target);
        }
        test_duration.exit();
    }
    ///
    /// 
    #[test]
    fn deserialize() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!("");
        let self_id = "test ApiReply";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let test_data = [
            (
                r#"{"authToken":"authToken","id":"id","keepAlive":true,"query":"","data":[],"error":{"message":""}}"#,
                ApiReply { 
                    auth_token: "authToken".to_string(), 
                    id: "id".to_string(), 
                    keep_alive: true, 
                    query: String::new(), 
                    data: vec![], 
                    error: ApiError::empty(),
                },
            ),
            (
                "{\"authToken\":\"123!@#\",\"id\":\"1\",\"query\":\"\",\"data\":[],\"keepAlive\":true,\"error\":{\"message\":\"\"}}",
                ApiReply { 
                    auth_token: "123!@#".to_string(), 
                    id: "1".to_string(), 
                    keep_alive: true, 
                    query: String::new(), 
                    data: vec![], 
                    error: ApiError::empty(),
                },
            ),
        ];
        for (reply, target) in test_data {
            let result: serde_json::Value = serde_json::from_str(&reply).unwrap();
            println!("json: {}", result);
            let result: ApiReply = serde_json::from_str(&reply).unwrap();
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        test_duration.exit();
    }
}
