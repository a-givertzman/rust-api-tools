use serde::{Serialize, Deserialize};

///
/// 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiQueryUnknown {
    pub query: serde_json::Value,
}
///
/// 
impl ApiQueryUnknown {
    ///
    /// 
    pub fn src_query(self) -> serde_json::Value {
        self.query
    }
}
