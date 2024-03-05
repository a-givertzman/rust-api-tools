#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::{warn, info, debug};
    use std::{sync::Once, time::{Duration, Instant}};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::stuff::max_test_duration::TestDuration;
    use crate::error::api_error::ApiError; 
    
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    // use super::*;
    
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
    fn init_each() -> () {
    
    }
    
    #[test]
    fn test_api_error() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!("");
        let self_id = "test ApiReply";
        println!("{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(10));
        test_duration.run().unwrap();
        let errs = [
            (r#"{"message":""}"#, ApiError::new(String::new(), String::new())),
            (r#"{"message":"mmm"}"#, ApiError::new(String::from("mmm"), String::new())),
            (r#"{"message":"", "details":""}"#, ApiError::new(String::new(), String::new())),
            (r#"{"message":"mmm", "details":"ddd"}"#, ApiError::new(String::from("mmm"), String::from("ddd"))),
        ];
        for (err, target) in errs {

            let result: serde_json::Value = serde_json::from_str(&err).unwrap();
            println!("json: {}", result);
            let result: ApiError = serde_json::from_str(&err).unwrap();
            assert!(result == target, "\nresult: {:?}\ntarget: {:?}", result, target);
        }
        test_duration.exit();
    }
}
