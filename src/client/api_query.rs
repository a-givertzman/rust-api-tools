use serde::Serialize;

///
/// Client side API query structure
#[derive(Debug, Clone, Serialize)]    // , Deserialize
pub struct ApiQuery {
    pub query: ApiQueryKind,
    #[serde(skip_serializing)]
    pub keep_alive: bool,
}
///
/// 
impl ApiQuery {
    ///
    /// 
    /// Creates new instance of ApiQuery
    pub fn new(query: ApiQueryKind, keep_alive: bool) -> Self {
        Self {
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
#[derive(Debug, Clone, Serialize)]    // , Deserialize
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
#[derive(Debug, Clone, Serialize)]    // , Deserialize
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
#[derive(Debug, Clone, Serialize)]    // , Deserialize
pub struct ApiQueryPython {
    pub script: String,
    pub params: serde_json::Value,
}
///
/// 
impl ApiQueryPython {
    ///
    /// Creates new instance of ApiQuery
    pub fn new(
        script: impl Into<String>,
        params: serde_json::Value,
    ) -> Self {
        Self {
            script: script.into(),
            params: params,
        }
    }
}

#[derive(Debug, Clone, Serialize)]    // , Deserialize
pub struct ApiQueryExecutable {
    pub name: String,
    pub params: serde_json::Value,
}
///
/// 
impl ApiQueryExecutable {
    ///
    /// Creates new instance of ApiQuery
    pub fn new(
        name: impl Into<String>,
        params: serde_json::Value,
    ) -> Self {
        Self {
            name: name.into(),
            params: params,
        }
    }
}
