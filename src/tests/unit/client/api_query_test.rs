#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::info;
    use std::sync::Once;
    use serde_json::json;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{client::api_query::ApiQuery, server::api_query::{api_query_sql::ApiQuerySql, api_query_type::ApiQueryType}};
    
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
                ApiQueryStruct { 
                    id: "111".to_string(), 
                    database: "database name".to_string(), 
                    sql: "Some valid sql query".to_string(), 
                    keepAlive: true},
                r#"{"authToken":"123zxy456!@#","id":"111","sql":{"database":"database name","sql":"Some valid sql query"},"keepAlive":true,"debug":false}"#
            ),
            (
                ApiQueryStruct { 
                    id: "112".to_string(), 
                    database: "database name".to_string(), 
                    sql: "Some valid sql query".to_string(), 
                    keepAlive: true},
                r#"{"authToken":"123zxy456!@#","id":"112","sql":{"database":"database name","sql":"Some valid sql query"},"keepAlive":false,"debug":true}"#
            ),
        ];
        for (value, target) in testData {
            let query = ApiQuery::new(
                value.id,
                ApiQueryType::Sql(ApiQuerySql {
                    database: value.database,
                    sql: value.sql,
                }),
                value.keepAlive,
            );
            let json = query.toJson().to_string();
            let json = json!(json);
            let target = json!(target);
            assert!(json.as_object() == target.as_object(), "\n  json: {:?}\ntarget: {:?}", json, target);
        }
    }
    
    struct ApiQueryStruct {
        id: String,
        database: String,
        sql: String,
        keepAlive: bool,
    }
}
