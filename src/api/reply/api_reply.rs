use indexmap::IndexMap;
use serde::{Serialize, Deserialize};
use crate::error::api_error::ApiError;

///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiReply {
    #[serde(rename = "authToken")]
    pub auth_token: String,
    pub id: String,
    pub query: String,
    pub data: Vec<IndexMap<String, serde_json::Value>>,
    #[serde(rename = "keepAlive")]
    pub keep_alive: bool,
    pub error: ApiError,
}
impl ApiReply {
    ///
    /// Returns bytes of the Self serialized with json_serde 
    pub fn as_bytes(&self) -> Vec<u8> {
        let result = serde_json::to_string(&self);
        match result {
            Ok(json_string) => {
                json_string.clone().as_bytes().to_owned()
            },
            Err(_) => todo!(),
        }
    }
    ///
    /// Creates ApiReply without error information
    pub fn new(
        auth_token: impl Into<String>,
        id: impl Into<String>,
        keep_alive: bool,
        query: impl Into<String>, 
        data: Vec<IndexMap<String, serde_json::Value>>,
    ) -> Self {
        ApiReply {
            auth_token: auth_token.into(),
            id: id.into(),
            keep_alive,
            query: query.into(),
            data,
            error: ApiError::empty(),
        }        
    }
    ///
    /// Creates ApiReply with error information only
    pub fn error(
        auth_token: impl Into<String>,
        id: impl Into<String>,
        keep_alive: bool,
        query: impl Into<String>, 
        error: ApiError,
    ) -> Self {
        ApiReply {
            auth_token: auth_token.into(),
            id: id.into(),
            keep_alive,
            query: query.into(),
            data: vec![],
            error,
        }        
    }
    ///
    /// Returns true if self.error is empty
    pub fn has_error(&self) -> bool {
        !self.error.is_empty()
    }
}
///
/// 
impl TryFrom<Vec<u8>> for ApiReply {
    type Error = String;
    fn try_from(bytes: Vec<u8>) -> Result<Self, String> {
        match serde_json::from_slice(&bytes) {
            Ok(value) => {
                Ok(value)
            },
            Err(err) => Err(format!("ApiReply.try_from | Error: {:?}", err)),
        }
    }
}
