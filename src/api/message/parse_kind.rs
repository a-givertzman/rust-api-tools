use sal_core::{dbg::Dbg, error::Error};
use crate::api::message::{from_bytes::FromBytes, message_kind::MessageKind};
use super::{fields::{FieldId, FieldKind}, message::{Bytes, MessageParse}};
///
/// Extracting `Kind` field from the input bytes
pub struct ParseKind {
    dbg: Dbg,
    conf: FieldKind,
    field: Box<dyn MessageParse<(FieldId, Bytes)>>,
    value: Option<MessageKind>,
    buffer: Bytes,
}
//
//
impl ParseKind {
    ///
    /// Returns [ParseKind] new instance
    pub fn new(parent: impl Into<String>, conf: FieldKind, field: impl MessageParse<(FieldId, Bytes)> + 'static) -> Self {
        Self {
            dbg: Dbg::new(parent, "ParseKind"),
            conf,
            field: Box::new(field),
            value: None,
            buffer: vec![],
        }
    }
}
//
//
impl MessageParse<(FieldId, MessageKind, Bytes)> for ParseKind {
    ///
    /// Extracting `Kind` field from the input bytes
    /// - returns `Id`, `Kind` & `Bytes` following by the `Kind`
    /// - call this method multiple times, until the end of message
    fn parse(&mut self, bytes: Bytes) -> Result<(FieldId, MessageKind, Bytes), Error> {
        let error = Error::new(&self.dbg, "parse");
        match self.field.parse(bytes) {
            Ok((id, bytes)) => {
                let bytes = [std::mem::take(&mut self.buffer), bytes].concat();
                match &self.value {
                    Some(kind) => Ok((id.clone(), kind.clone(), bytes)),
                    None => {
                        match bytes.get(..self.conf.len()) {
                            Some(kind_bytes) => {
                                let dbg_bytes = if kind_bytes.len() > 16 {format!("{:?}...", &kind_bytes[..16])} else {format!("{:?}", kind_bytes)};
                                log::trace!("{}.parse | bytes: {:?}", self.dbg, dbg_bytes);
                                match MessageKind::from_bytes(kind_bytes) {
                                    Ok(kind) => {
                                        log::trace!("{}.parse | kind: {:?}", self.dbg, kind);
                                        self.value = Some(kind.clone());
                                        Ok((id, kind, bytes[self.conf.len()..].to_vec()))
                                    },
                                    Err(err) => {
                                        self.buffer = kind_bytes.into();
                                        Err(error.pass_with("Parse error", err.to_string()))
                                    }
                                }
                            }
                            None => {
                                self.buffer.extend_from_slice(&bytes);
                                Err(error.err("Take error"))
                            }
                        }
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
        self.value = None;
        self.buffer.clear();
    }
}
