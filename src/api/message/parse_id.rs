use sal_core::{dbg::Dbg, error::Error};
use super::{fields::FieldId, message::{Bytes, MessageParse}};
///
/// Extracting `Id` field from the input bytes
pub struct ParseId {
    dbg: Dbg,
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
    pub fn new(parent: impl Into<String>, conf: FieldId, field: impl MessageParse<Bytes> + 'static) -> Self {
        Self {
            dbg: Dbg::new(parent, "ParseId"),
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
    fn parse(&mut self, bytes: Bytes) -> Result<(FieldId, Bytes), Error> {
        let error = Error::new(&self.dbg, "parse");
        match self.field.parse(bytes) {
            Ok(bytes) => {
                let bytes = [std::mem::take(&mut self.buffer), bytes].concat();
                match &self.value {
                    Some(id) => Ok((id.to_owned(), bytes)),
                    None => {
                        match bytes.get(..self.conf.len()) {
                            Some(id_bytes) => {
                                let dbg_bytes = if id_bytes.len() > 16 {format!("{:?}...", &id_bytes[..16])} else {format!("{:?}", id_bytes)};
                                log::trace!("{}.parse | id_bytes: {:?}", self.dbg, dbg_bytes);
                                match id_bytes.try_into() {
                                    Ok(id_bytes) => {
                                        let id= u32::from_be_bytes(id_bytes);
                                        self.value = Some(FieldId(id));
                                        Ok((FieldId(id), bytes[self.conf.len()..].to_vec()))
                                    },
                                    Err(err) => {
                                        self.buffer = id_bytes.into();
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
