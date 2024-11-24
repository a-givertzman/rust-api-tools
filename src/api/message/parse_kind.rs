use crate::{api::message::{from_bytes::FromBytes, message_kind::_MessageKind}, debug::dbg_id::DbgId, error::str_err::StrErr};
use super::{fields::{FieldId, FieldKind}, message::{Bytes, MessageParse}};
///
/// Extracting `Kind` field from the input bytes
pub struct ParseKind {
    dbgid: DbgId,
    conf: FieldKind,
    field: Box<dyn MessageParse<(FieldId, Bytes)>>,
    value: Option<_MessageKind>,
    buffer: Bytes,
}
//
//
impl ParseKind {
    ///
    /// Returns [ParseKind] new instance
    pub fn new(dbgid: &DbgId, conf: FieldKind, field: impl MessageParse<(FieldId, Bytes)> + 'static) -> Self {
        Self {
            dbgid: DbgId(format!("{}/ParseKind", dbgid)),
            conf,
            field: Box::new(field),
            value: None,
            buffer: vec![],
        }
    }
}
//
//
impl MessageParse<(FieldId, _MessageKind, Bytes)> for ParseKind {
    ///
    /// Extracting `Kind` field from the input bytes
    /// - returns `Id`, `Kind` & `Bytes` following by the `Kind`
    /// - call this method multiple times, until the end of message
    fn parse(&mut self, bytes: Bytes) -> Result<(FieldId, _MessageKind, Bytes), StrErr> {
        match self.field.parse(bytes) {
            Ok((id, bytes)) => {
                let bytes = [std::mem::take(&mut self.buffer), bytes].concat();
                match &self.value {
                    Some(kind) => Ok((id.clone(), kind.clone(), bytes)),
                    None => {
                        match bytes.get(..self.conf.len()) {
                            Some(kind_bytes) => {
                                let dbg_bytes = if kind_bytes.len() > 16 {format!("{:?}...", &kind_bytes[..16])} else {format!("{:?}", kind_bytes)};
                                log::trace!("{}.parse | bytes: {:?}", self.dbgid, dbg_bytes);
                                match _MessageKind::from_bytes(kind_bytes) {
                                    Ok(kind) => {
                                        log::trace!("{}.parse | kind: {:?}", self.dbgid, kind);
                                        self.value = Some(kind.clone());
                                        Ok((id, kind, bytes[self.conf.len()..].to_vec()))
                                    },
                                    Err(err) => {
                                        self.buffer = kind_bytes.into();
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
