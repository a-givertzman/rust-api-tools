#![allow(non_snake_case)]

use std::collections::HashMap;

use serde::Serialize;

use crate::server::api_query::api_query_type::ApiQueryType;

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
    pub id: String,
    pub query: ApiQueryType,
    pub keep_alive: bool,
}
///
/// 
impl ApiQuery {
    ///
    /// Creates new instance of ApiQuery
    pub fn new(
        // authToken: impl Into<String>,
        id: impl Into<String>,
        query: ApiQueryType,
        keep_alive: bool,
    ) -> Self {
        Self {
            id: id.into(),
            query,
            keep_alive,
        }
    }
    ///
    /// 
    pub fn update(&self, query: ApiQueryType, keep_alive: bool) -> Self {
        let mut selfSql = self.query.clone();
        selfSql.insert("sql".into(), sql.into());
        Self {
            id: self.id,
            database: self.database,
            query: selfSql,
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