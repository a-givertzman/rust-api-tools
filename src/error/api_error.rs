#![allow(non_snake_case)]

use std::collections::HashMap;

// use log::{debug, warn};
use serde::{Serialize, Deserialize, Serializer, ser::SerializeStruct};
use serde_json::json;

///
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ApiError {
    message: String,
    #[serde(default)]
    details: String,
    #[serde(default="bool::default")]
    debug: bool,
}
impl ApiError {
    ///
    /// 
    pub fn new(message: impl Into<String>, details: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            details: details.into(),
            debug: false,
        }        
    }
    ///
    /// 
    pub fn empty() -> Self {
        Self {
            message: String::new(),
            details: String::new(),
            debug: false,
        }
    }
    ///
    /// 
    pub fn debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }
    // ///
    // /// appends error details
    // pub fn append(&mut self, err: impl Serialize) {
    //     self.details = Some(            
    //         match &self.details.is_empty() {
    //             false => {
                    
    //                     ("curr", json!(err)),
    //                     ("prev", details.to_owned()),
    //                 ]))
    //             },
    //             true => json!(err),
    //         }
    //     )
    // }
    ///
    /// returns json representation of the accomulated errors
    /// - deppending on "debug" option:
    ///     - false - only short error message will be included
    ///     - true - short error message, original query, detiled errors info will bi included
    pub fn asJason(&self, debug: bool) -> serde_json::Value {
        match debug {
            false => {
                let result = serde_json::to_value(
                    HashMap::from([
                        ("message", self.message.clone()),
                    ])
                );
                match result {
                    Ok(value) => value,
                    Err(err) => {
                        json!(
                            HashMap::from([
                                ("message", err.to_string()),
                            ])
                        )
                    },
                }
            },
            true => {
                let result = serde_json::to_value(
                    HashMap::from([
                        ("message", self.message.clone()),
                        ("details", self.details.clone()),
                    ])
                );
                match result {
                    Ok(value) => value,
                    Err(err) => {
                        json!(
                            HashMap::from([
                                ("message", err.to_string()),
                            ])
                        )
                    },
                }

            }
        }
    }
    ///
    /// Returns true if no error information contains
    pub fn is_empty(&self) -> bool {
        self.message.is_empty() & self.details.is_empty()
    }
    ///
    /// 
    pub fn toString(self) -> String {
        format!("ApiError: {:?}\nerror: {:?}", self.message, self.details)
    }
    // ///
    // /// Returns originally received query
    // pub fn srcQuery(self) -> serde_json::Value {
    //     self.query()
    // }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
enum MsgType {
    Value(serde_json::Value),
    String(String),
}

impl Serialize for ApiError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
    S: Serializer, {
        if self.debug {
            let mut state = serializer.serialize_struct("ApiError", 2)?;
            state.serialize_field("message", &self.message)?;
            state.serialize_field("details", &self.details)?;
            state.end()
        } else {
            let mut state = serializer.serialize_struct("ApiError", 1)?;
            state.serialize_field("message", &self.message)?;
            state.end()
        }
        // 3 is the number of fields in the struct.
    }
}
