#[cfg(test)]

mod api_reply {
    use std::sync::Once;
    use serde_json::json;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::client::{
        api_query::{ApiQuery, ApiQueryKind, ApiQuerySql},
        api_reply::ApiReply,
    };
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
    #[test]
    fn serialize() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!("");
        let self_id = "test ApiRequest";
        println!("{}", self_id);

        let database = "test_api_query";
        // let mut client = TestDatabasePostgres::connect_db(self_id, "postgres", "postgres", "localhost:5432", "").unwrap();
        // TestDatabasePostgres::create_db(self_id, &mut client, database).unwrap();
        // client.close().unwrap();
        // let mut client = TestDatabasePostgres::connect_db(self_id, database, database, "localhost:5432", database).unwrap();
        // TestDatabasePostgres::create_db_table(self_id, &mut client, database, "customer").unwrap();
        // client.close().unwrap();
        // let port = "8080";     //TestSession::free_tcp_port_str();
        // let addtess = format!("127.0.0.1:{}", port);
        // let token = "123zxy456!@#";
        // let keep_alive = true;
        // let close_connection = false;
        let service_keep_alive = false;
        let test_data = [
            (
                ApiReply {
                    authToken: "123zxy456!@#".to_string(),
                    id: "1".to_string(),
                    query: json!(ApiQuery::new(
                        ApiQueryKind::Sql(ApiQuerySql::new(database, "select * from customer limit 3;")),
                        service_keep_alive, 
                    )).to_string(),
                    data: json!(Vec::<usize>::new()),
                    keepAlive: service_keep_alive,
                    error: ApiError::empty(),
                },
                r#"{"authToken":"123zxy456!@#","id":"1","keepAlive":false,"query":{"database":"test_api_query","sql":"select * from customer limit 3;"},"data":[],"error":{"message":""}}"#,
                // r#"{"authToken":"123zxy456!@#","id":"1","query":{"database":"test_api_query","sql":"select * from customer;"},"keepAlive":true, "data": []}"#,
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
        // let mut client = TestDatabasePostgres::connect_db(self_id, "postgres", "postgres", "localhost:5432", "").unwrap();
        // TestDatabasePostgres::drop_db(self_id, &mut client, database).unwrap();
    }
}
