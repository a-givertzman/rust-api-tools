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
//!     - 38, Json
//!     - 40, String
//!     - 48, Timestamp
//!     - 49, Duration
//!     - .., ...
//! 
use sal_core::error::Error;
use super::from_bytes::FromBytes;
///
/// Internal Kind of Message
/// - Used for build / parsing
#[derive(Debug, Clone, PartialEq)]
pub enum MessageKind {
    Any = Self::ANY as isize,
    Bool = Self::BOOL as isize,
    Bytes = Self::BYTES as isize,
    Duration = Self::DURATION as isize,
    Empty = Self::EMPTY as isize,
    F32 = Self::FLOAT32 as isize,
    F64 = Self::FLOAT64 as isize,
    I16 = Self::INT16 as isize,
    I32 = Self::INT32 as isize,
    I64 = Self::INT64 as isize,
    Json = Self::JSON as isize,
    String = Self::STRING as isize,
    Timestamp = Self::TIMESTAMP as isize,
    U16 = Self::UINT16 as isize,
    U32 = Self::UINT32 as isize,
    U64 = Self::UINT64 as isize,
}
//
//
impl MessageKind {
    const ANY: u8       = 00;
    const BOOL: u8      = 08;
    const BYTES: u8     = 02;
    const DURATION: u8  = 49;
    const EMPTY: u8     = 01;
    const FLOAT32: u8   = 32;
    const FLOAT64: u8   = 33;
    const INT16: u8     = 24;
    const INT32: u8     = 25;
    const INT64: u8     = 26;
    const JSON: u8      = 38;
    const STRING: u8    = 40;
    const TIMESTAMP: u8 = 48;
    const UINT16: u8    = 16;
    const UINT32: u8    = 17;
    const UINT64: u8    = 18;
    ///
    /// Returns bytes of the `MessageKund` variant    
    pub fn to_bytes(&self) -> &[u8] {
        match self {
            MessageKind::Any => &[Self::ANY],
            MessageKind::Bool => &[Self::BOOL],
            MessageKind::Bytes => &[Self::BYTES],
            MessageKind::Duration => &[Self::DURATION],
            MessageKind::Empty => &[Self::EMPTY],
            MessageKind::F32 => &[Self::FLOAT32],
            MessageKind::F64 => &[Self::FLOAT64],
            MessageKind::I16 => &[Self::INT16],
            MessageKind::I32 => &[Self::INT32],
            MessageKind::I64 => &[Self::INT64],
            MessageKind::Json => &[Self::JSON],
            MessageKind::String => &[Self::STRING],
            MessageKind::Timestamp => &[Self::TIMESTAMP],
            MessageKind::U16 => &[Self::UINT16],
            MessageKind::U32 => &[Self::UINT32],
            MessageKind::U64 => &[Self::UINT64],
        }
    }
}
impl FromBytes for MessageKind {
    ///
    /// Returns [MessageKind] converted from `bytes`
    fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        match bytes {
            [Self::ANY] => Ok(MessageKind::Any),
            [Self::BOOL] => Ok(MessageKind::Bool),
            [Self::BYTES] => Ok(MessageKind::Bytes),
            [Self::DURATION] => Ok(MessageKind::Duration),
            [Self::EMPTY] => Ok(MessageKind::Empty),
            [Self::FLOAT32] => Ok(MessageKind::F32),
            [Self::FLOAT64] => Ok(MessageKind::F64),
            [Self::INT16] => Ok(MessageKind::I16),
            [Self::INT32] => Ok(MessageKind::I32),
            [Self::INT64] => Ok(MessageKind::I64),
            [Self::JSON] => Ok(MessageKind::Json),
            [Self::STRING] => Ok(MessageKind::String),
            [Self::TIMESTAMP] => Ok(MessageKind::Timestamp),
            [Self::UINT16] => Ok(MessageKind::U16),
            [Self::UINT32] => Ok(MessageKind::U32),
            [Self::UINT64] => Ok(MessageKind::U64),
            [..] => Err(Error::new("MessageKind", "from_bytes").err(format!("Wrong or Empty input: {:?}", &bytes[..16]))),
        }
    }
}
