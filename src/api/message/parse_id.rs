use crate::{debug::dbg_id::DbgId, error::str_err::StrErr};
use super::{fields::FieldId, message::{Bytes, MessageParse}};
///
/// Extracting `Id` field from the input bytes
pub struct ParseId {
    dbgid: DbgId,
    conf: FieldId,
    field: Box<dyn MessageParse<Bytes>>,
    value: Option<FieldId>,
    buffer: Bytes,
}
//
//
impl ParseId {
    ///
    /// Returns [ParseId] new instance
    pub fn new(dbgid: &DbgId, conf: FieldId, field: impl MessageParse<Bytes> + 'static) -> Self {
        Self {
            dbgid: DbgId(format!("{}/ParseId", dbgid)),
            conf,
            field: Box::new(field),
            value: None,
            buffer: vec![],
        }
    }
}
//
//
impl MessageParse<(FieldId, Bytes)> for ParseId {
    ///
    /// Extracting `Id` field from the input bytes
    /// - returns Id & bytes following by the `Id`
    /// - call this method multiple times, until the end of message
    fn parse(&mut self, bytes: Bytes) -> Result<(FieldId, Bytes), StrErr> {
        match self.field.parse(bytes) {
            Ok(bytes) => {
                let bytes = [std::mem::take(&mut self.buffer), bytes].concat();
                match &self.value {
                    Some(id) => Ok((id.to_owned(), bytes)),
                    None => {
                        match bytes.get(..self.conf.len()) {
                            Some(id_bytes) => {
                                let dbg_bytes = if id_bytes.len() > 16 {format!("{:?}...", &id_bytes[..16])} else {format!("{:?}", id_bytes)};
                                log::debug!("{}.parse | id_bytes: {:?}", self.dbgid, dbg_bytes);
                                match id_bytes.try_into() {
                                    Ok(id_bytes) => {
                                        let id= u32::from_be_bytes(id_bytes);
                                        self.value = Some(FieldId(id));
                                        Ok((FieldId(id), bytes[self.conf.len()..].to_vec()))
                                    },
                                    Err(err) => {
                                        self.buffer = id_bytes.into();
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
