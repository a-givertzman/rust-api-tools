use sal_core::{dbg::Dbg, error::Error};
use crate::api::message::message_kind::MessageKind;
use super::{fields::{FieldId, FieldSize}, message::{Bytes, MessageParse}};
///
/// Extracting `Data` field from the input bytes
pub struct ParseData {
    dbg: Dbg,
    field: Box<dyn MessageParse<(FieldId, MessageKind, FieldSize, Bytes)>>,
    buffer: Bytes,
    remains: Bytes,
}
//
//
impl ParseData {
    ///
    /// Returns [ParseData] new instance
    pub fn new(parent: impl Into<String>, field: impl MessageParse<(FieldId, MessageKind, FieldSize, Bytes)> + 'static) -> Self {
        Self {
            dbg: Dbg::new(parent, "ParseData"),
            field: Box::new(field),
            buffer: vec![],
            remains: vec![],
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
    fn parse(&mut self, bytes: Bytes) -> Result<(FieldId, MessageKind, FieldSize, Bytes), Error> {
        let error = Error::new(&self.dbg, "parse");
        let bytes = [std::mem::take(&mut self.remains), bytes].concat();
        match self.field.parse(bytes) {
            Ok((id, kind, size, bytes)) => {
                let bytes = [std::mem::take(&mut self.buffer), bytes].concat();
                match bytes.get(..size.size()) {
                    Some(data_bytes) => {
                        let dbg_bytes = if data_bytes.len() > 16 {format!("{:?}...", &data_bytes[..16])} else {format!("{:?}", data_bytes)};
                        log::trace!("{}.parse | data_bytes: {:?}", self.dbg, dbg_bytes);
                        if let Some(bytes) = bytes.get(size.size()..) {
                            self.remains.extend_from_slice(bytes);
                        }
                        self.field.reset();
                        Ok((id, kind, size, data_bytes.to_vec()))
                    }
                    None => {
                        self.buffer.extend_from_slice(&bytes);
                        Err(error.err("Take error"))
                    }
                }
            }
            Err(err) => Err(error.pass(err))
        }
    }
    ///
    /// Resets state to the initial
    fn reset(&mut self) {
        self.field.reset();
        self.buffer.clear();
    }
}
