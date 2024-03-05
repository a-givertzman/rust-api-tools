#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use std::{sync::Once, time::Duration};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::stuff::max_test_duration::TestDuration;
    use crate::{
        api::reply::api_reply::ApiReply,
        error::api_error::ApiError,
    }; 
    
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
    fn test_api_reply() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        let selfId = "test ApiReply";
        println!("{}", selfId);
        let testDuration = TestDuration::new(selfId, Duration::from_secs(10));
        testDuration.run().unwrap();
        let reply = r#"{"authToken":"authToken","id":"id","keepAlive":true,"query":"","data":[],"error":{"message":""}}"#;
        let target = ApiReply { 
            authToken: "authToken".to_string(), 
            id: "id".to_string(), 
            keepAlive: true, 
            query: String::new(), 
            data: vec![], 
            error: ApiError::new(String::new(), String::new()),
        };
        let result: serde_json::Value = serde_json::from_str(&reply).unwrap();
        println!("json: {}", result);
        let result: ApiReply = serde_json::from_str(&reply).unwrap();
        assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        testDuration.exit();
    }
}
