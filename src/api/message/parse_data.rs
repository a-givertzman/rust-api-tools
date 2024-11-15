use crate::{api::message::message_kind::MessageKind, debug::dbg_id::DbgId, error::str_err::StrErr};
use super::{fields::{FieldId, FieldSize}, message::{Bytes, MessageParse}};
///
/// Extracting `Data` field from the input bytes
pub struct ParseData {
    dbgid: DbgId,
    field: Box<dyn MessageParse<(FieldId, MessageKind, FieldSize, Bytes)>>,
    buffer: Bytes,
}
//
//
impl ParseData {
    ///
    /// Returns [ParseData] new instance
    pub fn new(dbgid: &DbgId, field: impl MessageParse<(FieldId, MessageKind, FieldSize, Bytes)> + 'static) -> Self {
        Self {
            dbgid: DbgId(format!("{}/ParseData", dbgid)),
            field: Box::new(field),
            buffer: vec![],
        }
    }
}
//
//
impl MessageParse<(FieldId, MessageKind, FieldSize, Bytes)> for ParseData {
    ///
    /// Extracting `Data` field from the input bytes
    /// - returns `Id`, `Kind`, `Size` & `Bytes` following by the `Size`
    /// - call this method multiple times, until the end of message
    fn parse(&mut self, bytes: Bytes) -> Result<(FieldId, MessageKind, FieldSize, Bytes), StrErr> {
        match self.field.parse(bytes) {
            Ok((id, kind, size, bytes)) => {
                let bytes = [std::mem::take(&mut self.buffer), bytes].concat();
                match bytes.get(..size.size()) {
                    Some(data_bytes) => {
                        let dbg_bytes = if data_bytes.len() > 16 {format!("{:?}...", &data_bytes[..16])} else {format!("{:?}", data_bytes)};
                        log::debug!("{}.parse | data_bytes: {:?}", self.dbgid, dbg_bytes);
                        if let Some(bytes) = bytes.get(size.size()..) {
                            self.buffer.extend_from_slice(bytes);
                        }
                        self.field.reset();
                        Ok((id, kind, size, data_bytes.to_vec()))
                    }
                    None => {
                        self.buffer.extend_from_slice(&bytes);
                        Err(format!("{}.parse | Take error", self.dbgid).into())
                    }
                }
            }
            Err(err) => Err(format!("{}.parse | Error: {:?}", self.dbgid, err).into())
        }
    }
    ///
    /// Resets state to the initial
    fn reset(&mut self) {
        self.field.reset();
        self.buffer.clear();
    }
}
