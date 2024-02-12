#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::info;
    use testing::session::test_session::TestSession;
    use std::sync::Once;
    use serde_json::json;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::client::{api_query::ApiQuery, api_reply::ApiReply, api_request::ApiRequest};
    
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
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        let selfId = "test ApiRequest";
        println!("{}", selfId);
        // let testData = vec![
        //     (
        //         ApiQueryStruct { 
        //             authToken: "123zxy456!@#".to_string(), 
        //             id: "111".to_string(), 
        //             database: "database name".to_string(), 
        //             sql: "Some valid sql query".to_string(), 
        //             keepAlive: true, 
        //             debug: false},
        //         r#"{"authToken":"123zxy456!@#","id":"111","sql":{"database":"database name","sql":"Some valid sql query"},"keepAlive":true,"debug":false}"#
        //     ),
        //     (
        //         ApiQueryStruct { 
        //             authToken: "123zxy456!@#".to_string(), 
        //             id: "112".to_string(), 
        //             database: "database name".to_string(), 
        //             sql: "Some valid sql query".to_string(), 
        //             keepAlive: false, 
        //             debug: true},
        //         r#"{"authToken":"123zxy456!@#","id":"112","sql":{"database":"database name","sql":"Some valid sql query"},"keepAlive":false,"debug":true}"#
        //     ),
        // ];
        // for (value, target) in testData {
        //     let query = ApiQuery::new(
        //         value.authToken,
        //         value.id,
        //         value.database,
        //         value.sql,
        //         value.keepAlive,
        //         value.debug,
        //     );
        //     let json = query.toJson().to_string();
        //     let json = json!(json);
        //     let target = json!(target);
        //     assert!(json.as_object() == target.as_object(), "\n  json: {:?}\ntarget: {:?}", json, target);
        // }
        let port = "8080";     //TestSession::free_tcp_port_str();
        let addtess = format!("127.0.0.1:{}", port);
        let token = "auth-token";
        let keep_alive = false;
        let debug = true;
        let database = "flowers_app_server";
        let sql = "select * from customer;";
        let request = ApiRequest::new(
            selfId,
            addtess,
            token, 
            ApiQuery::new(
                "001", 
                database, 
                sql, 
                keep_alive, 
            ),
            debug,
        );
        match request.fetch(sql, keep_alive) {
            Ok(bytes) => {
                let reply = ApiReply::try_from(bytes);
                println!("reply: {:?}", reply);
            },
            Err(err) => {
                panic!("{} | Error: {:?}", selfId, err);
            },
        };

    }
    
    // struct ApiQueryStruct {
    //     authToken: String,
    //     id: String,
    //     database: String,
    //     sql: String,
    //     keepAlive: bool,
    //     debug: bool,
    // }
}
