#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::{debug, info, warn};
    use std::sync::Once;
    use serde_json::json;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::client::api_query::{ApiQuery, ApiQuerySql};
    
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
        info!("test_api_query");
        let testData = vec![
            (
                ApiQuery::Sql(ApiQuerySql { 
                    id: "111".to_string(), 
                    database: "database name".to_string(), 
                    sql: "Some valid sql query".to_string(), 
                    keep_alive: true}),
                r#"{"authToken":"123zxy456!@#","id":"111","sql":{"database":"database name","sql":"Some valid sql query"},"keepAlive":true,"debug":false}"#
            ),
            (
                ApiQuery::Sql(ApiQuerySql { 
                    id: "112".to_string(), 
                    database: "database name".to_string(), 
                    sql: "Some valid sql query".to_string(), 
                    keep_alive: true}),
                r#"{"authToken":"123zxy456!@#","id":"112","sql":{"database":"database name","sql":"Some valid sql query"},"keepAlive":false,"debug":true}"#
            ),
        ];
        for (query, target) in testData {
            let result = match serde_json::to_string(&query) {
                Ok(query) => {
                    debug!("query json: {:?}", query);
                    query
                },
                Err(err) => {
                    let message = format!("Error: {:?}", err);
                    panic!("{}", message);
                },
            };
            let result = json!(result);
            let target = json!(target);
            assert!(result.as_object() == target.as_object(), "\n  result: {:?}\ntarget: {:?}", result, target);
        }
    }
}
