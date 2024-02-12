#![allow(non_snake_case)]

use std::collections::HashMap;

use serde::Serialize;

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
pub struct ApiQuery {
    id: String,
    sql:  HashMap<String, String>,
    keep_alive: bool,
}
///
/// 
impl ApiQuery {
    ///
    /// Creates new instance of ApiQuery
    pub fn new(
        // authToken: impl Into<String>,
        id: impl Into<String>,
        database: impl Into<String>,
        sql: impl Into<String>,
        keep_alive: bool,
    ) -> Self {
        Self {
            id: id.into(),
            sql: HashMap::from([
                ("database".to_string(), database.into()),
                ("sql".to_string(), sql.into()),
            ]),
            keep_alive,
        }
    }
    ///
    /// 
    pub fn with_sql(&self, sql: String, keep_alive: bool) -> Self {
        let mut selfSql = self.sql.clone();
        selfSql.insert("sql".into(), sql);
        Self {
            id: self.id.clone(),
            sql: selfSql,
            keep_alive,
        }
    }
    ///
    /// Returns a JSON representation of the ApiQuery
    pub fn toJson(&self) -> String {
        match serde_json::to_string(self) {
            Ok(json) => json,
            Err(err) => panic!("ApiQuery.toJson | convertion error: {:?}", err),
        }
    }
    ///
    /// Returns ApiQuery in bytes, ready to write to socket
    pub fn asBytes(&self) -> Vec<u8> {
        self.toJson().as_bytes().to_vec()
    }
}