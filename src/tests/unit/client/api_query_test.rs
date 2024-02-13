#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::{debug, info, warn};
    use std::sync::Once;
    use serde_json::json;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::client::api_query::{ApiQuery, ApiQueryKind, ApiQuerySql};
    
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
    fn test_api_query() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!("");
        info!("test_api_query");
        let test_data = vec![
            (
                ApiQuery::new(
                    "111".to_string(), 
                    ApiQueryKind::Sql(ApiQuerySql { 
                        database: "database name".to_string(), 
                        sql: "Some valid sql query".to_string(), 
                    }),
                    true,
                ),
                r#"{"authToken":"123zxy456!@#","id":"111","sql":{"database":"database name","sql":"Some valid sql query"},"keepAlive":true,"debug":false}"#
            ),
            (
                ApiQuery::new(
                    "112".to_string(), 
                    ApiQueryKind::Sql(ApiQuerySql { 
                        database: "database name".to_string(), 
                        sql: "Some valid sql query".to_string(), 
                    }),
                    true,
                ),
                r#"{"authToken":"123zxy456!@#","id":"112","sql":{"database":"database name","sql":"Some valid sql query"},"keepAlive":false,"debug":true}"#
            ),
        ];
        for (query, target) in test_data {
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
