use serde::{Serialize, Deserialize};
use crate::error::api_error::ApiError;

///
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiQuerySql {
    pub database: String,
    pub sql: String,
}
impl ApiQuerySql {
    ///
    pub fn from_json(json_map: serde_json::Value) -> Result<Self, ApiError> {
        log::trace!("[ApiQuerySql.fromJson] json: {:?}", json_map);
        let key = "database";
        if let serde_json::Value::String(database) = &json_map[key] {
            log::trace!("[ApiQuerySql.fromJson] field '{}': {:?}", &key, &database);
            let key = "sql";
            if let serde_json::Value::String(sql) = &json_map[key] {
                log::trace!("[ApiQuerySql.fromJson] field '{}': {:?}", &key, &sql);
                return Ok(ApiQuerySql {
                    database: database.to_owned(),
                    sql: sql.to_owned(),
                });
            } else {
                let details = format!("[ApiQuerySql.fromJson] field '{}' of type String not found or invalid content", key);
                log::warn!("{}", details);
                return Err(ApiError::new(
                    format!("API SQL Service - invalid query (near field \"{}\")", key), 
                    details,
                ));
            }
        } else {
            let details = format!("[ApiQuerySql.fromJson] field '{}' of type String not found or invalid content", key);
            log::warn!("{}", details);
            return Err(ApiError::new(
                format!("API SQL Service - invalid query (near field \"{}\")", key), 
                details,
            ));
        }
    }
}