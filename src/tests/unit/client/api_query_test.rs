#[cfg(test)]

mod tests {
    use log::{debug, info};
    use std::{collections::HashMap, sync::Once};
    use serde_json::json;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::client::api_query::{ApiQuery, ApiQueryExecutable, ApiQueryKind, ApiQueryPython, ApiQuerySql};
    
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
                    ApiQueryKind::Sql(ApiQuerySql { 
                        database: "database1".to_string(), 
                        sql: "Some valid sql query1".to_string(), 
                    }),
                    true,
                ),
                r#"{"query":{"sql":{"database":"database1","sql":"Some valid sql query1"}}}"#
            ),
            (
                ApiQuery::new(
                    ApiQueryKind::Sql(ApiQuerySql { 
                        database: "database2".to_string(), 
                        sql: "Some valid sql query2".to_string(), 
                    }),
                    true,
                ),
                r#"{"query":{"sql":{"database":"database2","sql":"Some valid sql query2"}}}"#
            ),
            (
                ApiQuery::new(
                    ApiQueryKind::Python(ApiQueryPython { 
                        script: "python_script".to_string(), 
                        params: json!(HashMap::<String, f64>::new()), 
                    }),
                    true,
                ),
                r#"{"query":{"python":{"script":"python_script","params":{}}}}"#
            ),
            (
                ApiQuery::new(
                    ApiQueryKind::Executable(ApiQueryExecutable { 
                        name: "executable_name".to_string(), 
                        params: json!(HashMap::<String, f64>::new()), 
                    }),
                    true,
                ),
                r#"{"query":{"executable":{"name":"executable_name","params":{}}}}"#
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
            let result: serde_json::Value = serde_json::from_str(&result).unwrap();
            let target: serde_json::Value = serde_json::from_str(target).unwrap();
            println!("\n result: {:?}\n target: {:?}", result, target);
            assert!(result == target, "\n result: {:?}\n target: {:?}", result, target);
        }
    }
}
