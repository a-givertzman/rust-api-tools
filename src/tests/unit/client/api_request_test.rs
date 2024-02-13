#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::info;
    use testing::session::test_session::TestSession;
    use std::{collections::HashMap, sync::Once};
    use serde_json::json;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::client::{api_query::{ApiQuery, ApiQueryExecutable, ApiQueryKind, ApiQueryPython, ApiQuerySql}, api_reply::ApiReply, api_request::ApiRequest};
    
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    // use super::*;
    
    static INIT: Once = Once::new();
    
    ///
    /// once called initialisation
    fn initOnce() {
        INIT.call_once(|| {
                // implement your initialisation code to be called only once for current test file
            }
        )
    }
    
    
    ///
    /// returns:
    ///  - ...
    fn initEach() -> () {
    
    }
    
    #[test]
    fn test_api_query() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        let selfId = "test ApiRequest";
        println!("{}", selfId);
        let port = "8080";     //TestSession::free_tcp_port_str();
        let addtess = format!("127.0.0.1:{}", port);
        let token = "123zxy456!@#";
        let keep_alive = false;
        let service_keep_alive = false;
        let debug = false;
        let database = "flowers_app_server";
        let test_data = [
            (
                ApiQuery::new(
                    ApiQueryKind::Sql(ApiQuerySql::new(database, "select * from customer;")),
                    service_keep_alive, 
                ),
                r#"{"authToken":"123zxy456!@#","id":"001","sql":{"database":"flowers_app_server","sql":"select * from customer;"},"keepAlive":false,"debug":false}"#,
                
            ),
            (
                ApiQuery::new(
                    ApiQueryKind::Sql(ApiQuerySql::new(database, "select * from customer limit 3;")),
                    service_keep_alive, 
                ),
                r#"{"authToken":"123zxy456!@#","id":"001","sql":{"database":"flowers_app_server","sql":"select * from customer limit 3;"},"keepAlive":false,"debug":false}"#,
            ),
            (
                ApiQuery::new(
                    ApiQueryKind::Python(ApiQueryPython::new("test_script", json!(HashMap::<String, f64>::new()))),
                    service_keep_alive,
                ),
                r#"{"authToken":"123zxy456!@#","id":"001","python":{"script":"test_script","params":{}},"keepAlive":false,"debug":false}"#,
            ),
            (
                ApiQuery::new(
                    ApiQueryKind::Executable(ApiQueryExecutable::new("test_app", json!(HashMap::<String, f64>::new()))),
                    service_keep_alive,
                ),
                r#"{"authToken":"123zxy456!@#","id":"001","executable":{"name":"test_app","params":{}},"keepAlive":false,"debug":false}"#,
            ),
        ];
        for (query, target) in test_data {
            let mut request = ApiRequest::new(
                selfId,
                &addtess,
                token, 
                query.clone(),
                keep_alive,
                debug,
            );
            println!("\nrequest: {:?}", request);
            let result = json!(request);
            // let result: serde_json::Value = serde_json::from_str(&request).unwrap();
            let target: serde_json::Value = serde_json::from_str(target).unwrap();
            assert!(result == target, "\n result: {:?}\n target: {:?}", result, target);
            println!("\n result: {:?}\n target: {:?}", result, target);
            match request.fetch(&query, keep_alive) {
                Ok(bytes) => {
                    let reply = ApiReply::try_from(bytes);
                    println!("\nreply: {:?}", reply);
                },
                Err(err) => {
                    panic!("{} | Error: {:?}", selfId, err);
                },
            };
        }
    }
}
