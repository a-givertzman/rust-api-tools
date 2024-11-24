//!
//! # Messages transmitted over socket.
//! 
//! - Data can be encoded using varius data `Kind`, `Size` and payload Data
//! 
//! - Message format
//!     Field name | Start | Kind |  Size  | Data |
//!     ---       |  ---  | ---  |  ---   | ---  |
//!     Data type |  u8   | u8   | u32    | [u8; Size] |
//!     Value     |  22   | StringValue | xxx    | [..., ...]  |
//!     
//!     - Start - Each message starts with SYN (22)
//!     - Kind - The `Kind` of the data stored in the `Data` field, refer to
//!     - Size - The length of the `Data` field in bytes
//!     - Data - Data structured depending on it `Kind`
//! 
//! - `Kind` of data
//!     - 00, Any
//!     - 01, Empty
//!     - 02, Bytes
//!     - 08, Bool
//!     - 16, UInt16
//!     - 17, UInt32
//!     - 18, UInt64
//!     - 24, Int16
//!     - 25, Int32
//!     - 26, Int64
//!     - 32, F32
//!     - 33, F64
//!     - 40, String
//!     - 48, Timestamp
//!     - 49, Duration
//!     - .., ...
//! 
use crate::{debug::dbg_id::DbgId, error::str_err::StrErr};
use super::fields::{FieldData, FieldId, FieldKind, FieldSize, FieldSyn};
///
/// 
pub type Bytes = Vec<u8>;
///
/// Parse Message structure from bytes Interface 
pub trait MessageParse<T> {
    ///
    /// Extracting some pattern from input `bytes`
    fn parse(&mut self, bytes: Bytes) -> Result<T, StrErr>;
    ///
    /// Resets state to the initial
    fn reset(&mut self);
}
/// 
/// 
#[derive(Debug, Clone, PartialEq)]
pub enum MessageField {
    Syn(FieldSyn),
    Id(FieldId),
    Kind(FieldKind),
    Size(FieldSize),
    Data(FieldData),
}
///
/// Socket Message
pub struct Message<T> {
    dbgid: DbgId,
    build: Vec<MessageField>, 
    parse: Box<dyn MessageParse<T>>,
}

//
//
impl<T> std::fmt::Debug for Message<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Message")
            .field("dbgid", &self.dbgid)
            .field("build", &self.build)
            .finish()
    }
}
//
//
impl<T> Message<T> {
    ///
    /// Returns `Message` new instance 
    pub fn new(
        dbgid: &DbgId,
        build: Vec<MessageField>,
        parse: impl MessageParse<T> + 'static
    ) -> Self {
        Self {
            dbgid: DbgId(format!("{}/Message", dbgid)),
            build,
            parse: Box::new(parse),
        }
    }
    ///
    /// Returns message built according to specified fields and passed `bytes`
    pub fn build(&mut self, bytes: &[u8], id: u32) -> Vec<u8> {
        let mut message = vec![];
        for field in &mut self.build {
            match field {
                MessageField::Syn(field_syn) => message.push(field_syn.0),
                MessageField::Id(_) => message.extend(FieldId(id).to_be_bytes()),
                MessageField::Kind(field_kind) => message.extend(field_kind.to_bytes()),
                MessageField::Size(field_size) => message.extend(field_size.to_be_bytes(bytes.len() as u32)),
                MessageField::Data(_) => {
                    message.extend_from_slice(bytes);
                }
            }
        }
        message
    }

}
//
//
impl<T> MessageParse<T> for Message<T> {
    ///
    /// Extracting [Message] fields from the input bytes
    /// - returns `Id`, `Kind`, `Size` & `Bytes` following by the `Size`
    /// - call this method multiple times, until the end of message
    fn parse(&mut self, bytes: Bytes) -> Result<T, StrErr> {
        self.parse.parse(bytes)
    }
    //
    //
    fn reset(&mut self) {
        self.parse.reset()
    }
}