use serde::{
    Serialize, 
    Deserialize, 
};

use crate::{
    server::api_query::api_query_sql::ApiQuerySql,
    server::api_query::api_query_python::ApiQueryPython, 
    server::api_query::api_query_executable::ApiQueryExecutable, 
    server::api_query::api_query_error::ApiQueryError, 
};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// #[serde(rename_all = "lowercase")]
pub enum ApiQueryType {
    Sql(ApiQuerySql),
    Python(ApiQueryPython),
    Executable(ApiQueryExecutable),
    Unknown,
    Error(ApiQueryError),
}

pub enum ApiQueryTypeName {
    Sql,
    Python,
    Executable,
    Unknown,
}
impl ApiQueryTypeName {
    pub fn value(&self) -> &str {
        match *self {
            ApiQueryTypeName::Sql => "sql",
            ApiQueryTypeName::Python => "python",
            ApiQueryTypeName::Executable => "executable",
            ApiQueryTypeName::Unknown => "unknown",
        }
    }
}
