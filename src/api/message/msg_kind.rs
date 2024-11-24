use std::time::Duration;
use chrono::DateTime;
use super::message::Bytes;
///
/// Kind of a [Message]
/// - The `Kind` of the data stored in the `Data` field
/// - Currently supported kinds of data:
///     - 00, Any
///     - 01, Empty
///     - 02, Bytes
///     - 08, Bool
///     - 16, UInt16
///     - 17, UInt32
///     - 18, UInt64
///     - 24, Int16
///     - 25, Int32
///     - 26, Int64
///     - 32, F32
///     - 33, F64
///     - 40, String
///     - 48, Timestamp
///     - 49, Duration
///     - .., ...
#[derive(Debug, Clone, PartialEq)]
pub enum MsgKind {
    Any(Bytes),
    Empty,
    Bytes(Bytes),
    Bool(bool),
    U16(u16),
    U32(u32),
    U64(u64),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    String(String),
    Timestamp(DateTime<chrono::Utc>),
    Duration(Duration),
}
//
//
impl MsgKind {
    ///
    /// Returns be bytes of the `MsgKind` variant
    /// - `Any` - returns bytes as is
    /// - `Empty` - returns empty vec
    /// - `Bytes` - returns bytes as is  
    /// ...
    /// - `String` - returns utf8 bytes  
    /// - `Timestemp` - returns be bytes of the number of non-leap-microseconds since January 1, 1970 UTC.
    /// - `Duration` - returns be bytes of f64 seconds of duration value
    pub fn to_be_bytes<'a>(&'a self) -> Vec<u8> {
        match self {
            MsgKind::Any(value) => value.to_vec(),
            MsgKind::Empty => Vec::new(),
            MsgKind::Bytes(value) => value.to_vec(),
            MsgKind::Bool(value) => if *value {vec![1]} else {vec![0]},
            MsgKind::U16(value) => value.to_be_bytes().to_vec(),
            MsgKind::U32(value) => value.to_be_bytes().to_vec(),
            MsgKind::U64(value) => value.to_be_bytes().to_vec(),
            MsgKind::I16(value) => value.to_be_bytes().to_vec(),
            MsgKind::I32(value) => value.to_be_bytes().to_vec(),
            MsgKind::I64(value) => value.to_be_bytes().to_vec(),
            MsgKind::F32(value) => value.to_be_bytes().to_vec(),
            MsgKind::F64(value) => value.to_be_bytes().to_vec(),
            MsgKind::String(value) => value.as_bytes().to_vec(),
            MsgKind::Timestamp(value) => value.timestamp_micros().to_be_bytes().to_vec(),
            MsgKind::Duration(value) => value.as_secs_f64().to_be_bytes().to_vec(),
        }
    }
}
