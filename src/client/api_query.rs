use serde::Serialize;

///
/// Client side API query structure
#[derive(Serialize)]    // , Deserialize
pub struct ApiQuery {
    pub id: String,
    pub query: ApiQueryKind,
    pub keep_alive: bool,
}
///
/// 
impl ApiQuery {
    ///
    /// 
    /// Creates new instance of ApiQuery
    pub fn new(id: impl Into<String>, query: ApiQueryKind, keep_alive: bool) -> Self {
        Self {
            id: id.into(),
            query,
            keep_alive,
        }
    }
}
///
/// Contains properties specific to quety kind 
///  - ApiQuerySql
///  - ApiQueryPython
///  - ApiQueryQxecutable
#[derive(Serialize)]    // , Deserialize
pub enum ApiQueryKind {
    #[serde(rename(serialize = "sql"))]
    Sql(ApiQuerySql),
    #[serde(rename(serialize = "python"))]
    Python(ApiQueryPython),
    #[serde(rename(serialize = "executable"))]
    Executable(ApiQueryExecutable),
}

///
/// Wrap a structure of an API query
/// {
///     "id": "123",
///     "sql": {
///         "database": "database name",
///         "sql": "Some valid sql query"
///     },
///     "keep-alive": true,
/// }
#[derive(Serialize)]    // , Deserialize
pub struct ApiQuerySql {
    pub database: String,
    pub sql: String,
}
///
/// 
impl ApiQuerySql {
    ///
    /// Creates new instance of ApiQuery
    pub fn new(
        // authToken: impl Into<String>,
        database: impl Into<String>,
        sql: impl Into<String>,
    ) -> Self {
        Self {
            database: database.into(),
            sql: sql.into(),
        }
    }
}
///
/// 
#[derive(Serialize)]    // , Deserialize
pub struct ApiQueryPython {
    pub script: String,
    pub params: String,
}
///
/// 
impl ApiQueryPython {
    ///
    /// Creates new instance of ApiQuery
    pub fn new(
        // authToken: impl Into<String>,
        script: impl Into<String>,
        params: impl Into<String>,
    ) -> Self {
        Self {
            script: script.into(),
            params: params.into(),
        }
    }
}

#[derive(Serialize)]    // , Deserialize
pub struct ApiQueryExecutable {
    pub script: String,
    pub params: String,
}
///
/// 
impl ApiQueryExecutable {
    ///
    /// Creates new instance of ApiQuery
    pub fn new(
        // authToken: impl Into<String>,
        script: impl Into<String>,
        params: impl Into<String>,
    ) -> Self {
        Self {
            script: script.into(),
            params: params.into(),
        }
    }
}


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
