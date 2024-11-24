use crate::{api::message::message_kind::_MessageKind, debug::dbg_id::DbgId, error::str_err::StrErr};
use super::{fields::{FieldId, FieldSize}, message::{Bytes, MessageParse}};
///
/// Extracting `Size` field from the input bytes
pub struct ParseSize {
    dbgid: DbgId,
    conf: FieldSize,
    field: Box<dyn MessageParse<(FieldId, _MessageKind, Bytes)>>,
    value: Option<FieldSize>,
    buffer: Bytes,
}
//
//
impl ParseSize {
    ///
    /// Returns [ParseSize] new instance
    pub fn new(dbgid: &DbgId, conf: FieldSize, field: impl MessageParse<(FieldId, _MessageKind, Bytes)> + 'static) -> Self {
        Self {
            dbgid: DbgId(format!("{}/ParseSize", dbgid)),
            conf,
            field: Box::new(field),
            value: None,
            buffer: vec![],
        }
    }
}
//
//
impl MessageParse<(FieldId, _MessageKind, FieldSize, Bytes)> for ParseSize {
    ///
    /// Extracting `Size` field from the input bytes
    /// - returns `Id`, `Kind`, `Size` & `Bytes` following by the `Size`
    /// - call this method multiple times, until the end of message
    fn parse(&mut self, bytes: Bytes) -> Result<(FieldId, _MessageKind, FieldSize, Bytes), StrErr> {
        match self.field.parse(bytes) {
            Ok((id, kind, bytes)) => {
                let bytes = [std::mem::take(&mut self.buffer), bytes].concat();
                match &self.value {
                    Some(size) => Ok((id.clone(), kind.clone(), size.clone(), bytes)),
                    None => {
                        match bytes.get(..self.conf.len()) {
                            Some(size_bytes) => {
                                let dbg_bytes = if size_bytes.len() > 16 {format!("{:?}...", &size_bytes[..16])} else {format!("{:?}", size_bytes)};
                                log::trace!("{}.parse | size_bytes: {:?}", self.dbgid, dbg_bytes);
                                match size_bytes.try_into() {
                                    Ok(size_bytes) => {
                                        let size= u32::from_be_bytes(size_bytes);
                                        self.value = Some(FieldSize(size));
                                        Ok((id, kind, FieldSize(size), bytes[self.conf.len()..].to_vec()))
                                    },
                                    Err(err) => {
                                        self.buffer = size_bytes.into();
                                        Err(format!("{}.parse | Parse error: {:#?}", self.dbgid, err).into())
                                    }
                                }
                            }
                            None => {
                                self.buffer.extend_from_slice(&bytes);
                                Err(format!("{}.parse | Take error", self.dbgid).into())
                            }
                        }
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
        self.value = None;
        self.buffer.clear();
    }
}
