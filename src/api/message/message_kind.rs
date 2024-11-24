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
use crate::error::str_err::StrErr;
use super::from_bytes::FromBytes;
///
/// Internal Kind of Message
/// - Used for build / parsing
#[derive(Debug, Clone, PartialEq)]
pub enum _MessageKind {
    Any = Self::ANY as isize,
    Empty = Self::EMPTY as isize,
    Bytes = Self::BYTES as isize,
    Bool = Self::BOOL as isize,
    U16 = Self::UINT16 as isize,
    U32 = Self::UINT32 as isize,
    U64 = Self::UINT64 as isize,
    I16 = Self::INT16 as isize,
    I32 = Self::INT32 as isize,
    I64 = Self::INT64 as isize,
    F32 = Self::FLOAT32 as isize,
    F64 = Self::FLOAT64 as isize,
    String = Self::STRING as isize,
    Timestamp = Self::TIMESTAMP as isize,
    Duration = Self::DURATION as isize,
}
//
//
impl _MessageKind {
    const ANY: u8 = 00;
    const EMPTY: u8 = 01;
    const BYTES: u8 = 02;
    const BOOL: u8 = 08;
    const UINT16: u8 = 16;
    const UINT32: u8 = 17;
    const UINT64: u8 = 18;
    const INT16: u8 = 24;
    const INT32: u8 = 25;
    const INT64: u8 = 26;
    const FLOAT32: u8 = 32;
    const FLOAT64: u8 = 33;
    const STRING: u8 = 40;
    const TIMESTAMP: u8 = 48;
    const DURATION: u8 = 49;
    ///
    /// Returns bytes of the `MessageKund` variant    
    pub fn to_bytes(&self) -> &[u8] {
        match self {
            _MessageKind::Any => &[Self::ANY],
            _MessageKind::Empty => &[Self::EMPTY],
            _MessageKind::Bytes => &[Self::BYTES],
            _MessageKind::Bool => &[Self::BOOL],
            _MessageKind::U16 => &[Self::UINT16],
            _MessageKind::U32 => &[Self::UINT32],
            _MessageKind::U64 => &[Self::UINT64],
            _MessageKind::I16 => &[Self::INT16],
            _MessageKind::I32 => &[Self::INT32],
            _MessageKind::I64 => &[Self::INT64],
            _MessageKind::F32 => &[Self::FLOAT32],
            _MessageKind::F64 => &[Self::FLOAT64],
            _MessageKind::String => &[Self::STRING],
            _MessageKind::Timestamp => &[Self::TIMESTAMP],
            _MessageKind::Duration => &[Self::DURATION],
        }
    }
}
impl FromBytes for _MessageKind {
    ///
    /// Returns [MessageKind] converted from `bytes`
    fn from_bytes(bytes: &[u8]) -> Result<Self, StrErr> {
        match bytes {
            [Self::ANY] => Ok(_MessageKind::Any),
            [Self::EMPTY] => Ok(_MessageKind::Empty),
            [Self::BYTES] => Ok(_MessageKind::Bytes),
            [Self::BOOL] => Ok(_MessageKind::Bool),
            [Self::UINT16] => Ok(_MessageKind::U16),
            [Self::UINT32] => Ok(_MessageKind::U32),
            [Self::UINT64] => Ok(_MessageKind::U64),
            [Self::INT16] => Ok(_MessageKind::I16),
            [Self::INT32] => Ok(_MessageKind::I32),
            [Self::INT64] => Ok(_MessageKind::I64),
            [Self::FLOAT32] => Ok(_MessageKind::F32),
            [Self::FLOAT64] => Ok(_MessageKind::F64),
            [Self::STRING] => Ok(_MessageKind::String),
            [Self::TIMESTAMP] => Ok(_MessageKind::Timestamp),
            [Self::DURATION] => Ok(_MessageKind::Duration),
            [..] => Err(StrErr(format!("MessageKind.from_bytes | Wrong or Empty input: {:?}", &bytes[..16]))),
        }
    }
}
