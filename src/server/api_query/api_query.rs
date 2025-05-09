use serde::{Serialize, Deserialize};
use crate::{
    error::api_error::ApiError, 
    server::api_query::{api_query_error::ApiQueryError, api_query_executable::ApiQueryExecutable, api_query_python::ApiQueryPython, api_query_sql::ApiQuerySql, api_query_type::{ApiQueryType, ApiQueryTypeName}}, 
};

///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiQuery {
    auth_token: String,
    id: String,
    query: ApiQueryType,
    src_query: String, 
    pub keep_alive: bool,
    pub debug: bool,
}
impl ApiQuery {
    ///
    /// 
    pub fn auth_token(&self) -> String {
        self.auth_token.clone()
    }
    ///
    /// 
    pub fn id(&self) -> String {
        self.id.clone()
    }
    ///
    /// 
    pub fn query(&self) -> ApiQueryType {
        self.query.clone()
    }
    ///
    /// Returns original query string
    pub fn src_query(&self, debug: bool) -> String {
        if debug {
            self.src_query.clone()
        } else {
            "".to_string()
        }
    }
    ///
    /// 
    pub fn new(auth_token: String, id: String, query: ApiQueryType, src_query: impl Into<String>, keep_alive: bool, debug: bool) -> Self {
        Self { 
            auth_token, 
            id, 
            query, 
            src_query: src_query.into(), 
            keep_alive, 
            debug,
        }
    }
    ///
    /// Returns `ApiQuery` parsing query of type `ApiQueryType::Sql`
    fn parse_api_query_sql(src_query: &str, json: serde_json::Value, auth_token: String, id: String, keep_alive: bool, debug: bool) -> ApiQuery {
        log::debug!("[ApiQuery.parseApiQuerySql] detected: {}", ApiQueryTypeName::Sql.value());
        match ApiQuerySql::from_json(json[ApiQueryTypeName::Sql.value()].clone()) {
            Ok(api_query_sql) => {
                ApiQuery::new(
                    auth_token,
                    id,
                    ApiQueryType::Sql( api_query_sql ),
                    src_query,
                    keep_alive,
                    debug,
                )
            },
            Err(err) => {
                ApiQuery::new(
                    auth_token,
                    id,
                    ApiQueryType::Error( ApiQueryError::new(err) ),
                    src_query,
                    keep_alive,
                    debug,
                )
            },
        }
    }
    ///
    /// Returns `ApiQuery` parsing query of type `ApiQueryType::Python`
    fn parse_api_query_python(src_query: &str, json: serde_json::Value, auth_token: String, id: String, keep_alive: bool, debug: bool) -> ApiQuery {
        log::debug!("ApiQuery.fromBytes | detected: {}", ApiQueryTypeName::Python.value());
        match ApiQueryPython::from_json(json[ApiQueryTypeName::Python.value()].clone()) {
            Ok(api_query_python) => {
                ApiQuery::new(
                    auth_token,
                    id,
                    ApiQueryType::Python( api_query_python ),
                    src_query,
                    keep_alive,
                    debug,
                )
            },
            Err(err) => {
                ApiQuery::new(
                    auth_token,
                    id,
                    ApiQueryType::Error( ApiQueryError::new(err) ),
                    src_query,
                    keep_alive,
                    debug,
                )
            },
        }
    }
    ///
    /// Returns `ApiQuery` parsing query of type `ApiQueryType::Executable`
    fn parse_api_query_executable(src_query: &str, json: serde_json::Value, auth_token: String, id: String, keep_alive: bool, debug: bool) -> ApiQuery {
        log::debug!("ApiQuery.fromBytes | detected: {}", ApiQueryTypeName::Executable.value());
        match ApiQueryExecutable::from_json(json[ApiQueryTypeName::Executable.value()].clone()) {
            Ok(api_query_executable) => {
                ApiQuery::new(
                    auth_token,
                    id,
                    ApiQueryType::Executable( api_query_executable ),
                    src_query,
                    keep_alive,
                    debug,
                )
            },
            Err(err) => {
                ApiQuery::new(
                    auth_token,
                    id,
                    ApiQueryType::Error( ApiQueryError::new(err) ),
                    src_query,
                    keep_alive,
                    debug,
                )
            },
        }
    }
    ///
    /// Returns `ApiQueryTypeName` if parsed
    fn parse_query_type_name(query: &serde_json::map::Map<String, serde_json::Value>) -> Result<ApiQueryTypeName, ApiError> {
        let mut queries = 0;
        let mut query_type = ApiQueryTypeName::Unknown;
        if query.contains_key(ApiQueryTypeName::Sql.value()) {
            queries += 1;
            query_type = ApiQueryTypeName::Sql
        }
        if query.contains_key(ApiQueryTypeName::Python.value()) {
            queries += 1;
            query_type = ApiQueryTypeName::Python;
        }
        if query.contains_key(ApiQueryTypeName::Executable.value()) {
            queries += 1;
            query_type = ApiQueryTypeName::Executable
        }
        match queries.cmp(&1) {
            std::cmp::Ordering::Less => Ok(query_type),
            std::cmp::Ordering::Equal => Ok(query_type),
            std::cmp::Ordering::Greater => {
                let details = format!("ApiQuery.fromBytes | Unable to perform multiservice request: {:?}", query);
                log::warn!("{}", details);
                Err(
                    ApiError::new(
                        format!("API Service - Unable to perform multiservice request: {:?}", query), 
                        details,
                    )
                )
            },
        }
    }
    ///
    /// Builds ApiQuery from bytes
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut auth_token = "Unknown".to_string();
        let mut id = "Unknown".to_string();
        let mut keep_alive = false;
        let mut debug = true;
        match serde_json::from_slice::<serde_json::Value>(bytes) {
            Ok(json) => {
                match &json.as_object() {
                    Some(query_map) => {
                        let mut errors = vec![];
                        match query_map.get_value("authToken") {
                            Ok(value) => auth_token = value,
                            Err(err) => errors.push(err),
                        };
                        match query_map.get_value("id") {
                            Ok(value) => id = value,
                            Err(err) => errors.push(err),
                        };
                        match query_map.get_value("keepAlive") {
                            Ok(value) => {
                                log::debug!("ApiQuery.fromBytes | keep-alive detected");
                                keep_alive = value;
                            },
                            Err(_) => {},
                        };
                        match query_map.get_value("debug") {
                            Ok(value) => {
                                log::debug!("ApiQuery.fromBytes | debug detected");
                                debug = value;
                            },
                            Err(_) => debug = false,
                        };
                        match errors.get(0) {
                            Some(details) => {
                                ApiQuery {
                                    auth_token,
                                    id,
                                    query: ApiQueryType::Error( ApiQueryError::new(
                                        // json.clone(),
                                        ApiError::new(
                                            format!("API Service - invalid query: {:?}", json), 
                                            format!("ApiQuery.fromBytes | errors: {:?}", details), 
                                        ),
                                    )),
                                    src_query: json.to_string(),
                                    keep_alive,
                                    debug,
                                }
                            },
                            None => {
                                log::trace!("ApiQuery.fromBytes | obj: {:?}", query_map);
                                match Self::parse_query_type_name(&query_map) {
                                    Ok(query_type) => match query_type {
                                        ApiQueryTypeName::Sql => Self::parse_api_query_sql(&json.to_string(), json, auth_token, id, keep_alive, debug),
                                        ApiQueryTypeName::Python => Self::parse_api_query_python(&json.to_string(), json, auth_token, id, keep_alive, debug),
                                        ApiQueryTypeName::Executable => Self::parse_api_query_executable(&json.to_string(), json, auth_token, id, keep_alive, debug),
                                        ApiQueryTypeName::Unknown => ApiQuery {
                                            auth_token,
                                            id,
                                            query: ApiQueryType::Unknown,
                                            src_query: json.to_string(), 
                                            keep_alive,
                                            debug,
                                        },
                                    },
                                    Err(err) => {
                                        ApiQuery {
                                            auth_token,
                                            id,
                                            query: ApiQueryType::Error( ApiQueryError::new(err)),
                                            src_query: json.to_string(),
                                            keep_alive,
                                            debug,
                                        }
                                    },
                                }
                            },
                        }
                    },
                    None => {
                        let details = format!("ApiQuery.fromBytes | json parsing error: type Map not found in json: {:?}", json.to_string());
                        log::warn!("{}", details);
                        ApiQuery {
                            auth_token,
                            id,
                            query: ApiQueryType::Error( ApiQueryError::new(
                                // json.clone(), 
                                ApiError::new(
                                    format!("API Service - invalid query: {:?}", json), 
                                    details,
                                ),
                            )),
                            src_query: json.to_string(),
                            keep_alive,
                            debug,
                        }
                    },
                }
            },
            Err(err) => {
                let details = format!("ApiQuery.fromBytes | json parsing error: {:?}", err);
                let default = bytes.iter().map(|v| v.to_string()).reduce(|i, v| i + "," + &v).unwrap_or(String::new());
                let query_string = String::from_utf8(bytes.to_owned()).unwrap_or(default);
                log::warn!("{} \n\tin query: {}", details, query_string);
                ApiQuery {
                    auth_token,
                    id,
                    query: ApiQueryType::Error( ApiQueryError::new(
                        // json!(queryString), 
                        ApiError::new(
                            format!("API Service - invalid query: {:?}", query_string), 
                            details,
                        )
                    )),
                    src_query: query_string,
                    keep_alive,
                    debug,
                }
            },
        }
    }
}
//
//
trait GetJsonObjValue<T> {
    fn get_value(&self, key: &str) -> Result<T, String>;
}
//
//
impl GetJsonObjValue<String> for serde_json::map::Map<String, serde_json::Value> {
    fn get_value(&self, key: &str) -> Result<String, String> {
        let msg = format!("ApiQuery.fromBytes | field '{}' of type {:?} not found or invalid content", &key, "String");
        match self.get(key) {
            Some(json_value) => {
                if let serde_json::Value::String(value) = json_value {
                    Ok(value.to_string())
                } else {
                    Err(msg)
                }
            },
            None => {
                Err(msg)
            },
        }
    }
}
//
//
impl GetJsonObjValue<bool> for serde_json::map::Map<String, serde_json::Value> {
    fn get_value(&self, key: &str) -> Result<bool, String> {
        let msg = format!("ApiQuery.fromBytes | field '{}' of type {:?} not found or invalid content", &key, "String");
        match self.get(key) {
            Some(json_value) => {
                if let serde_json::Value::Bool(value) = json_value {
                    Ok(*value)
                } else {
                    Err(msg)
                }
            },
            None => {
                Err(msg)
            },
        }
    }
}

