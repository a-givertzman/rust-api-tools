use crate::{debug::dbg_id::DbgId, error::str_err::StrErr};
use super::{fields::FieldSyn, message::{Bytes, MessageParse}};
///
/// Extracting `Syn` symbol from the input bytes
/// - Used to identify a start of the message
pub struct ParseSyn {
    dbgid: DbgId,
    conf: FieldSyn,
    value: Option<()>,
}
//
//
impl ParseSyn {
    ///
    /// Returns [ParseSyn] new instance
    pub fn new(dbgid: &DbgId, conf: FieldSyn) -> Self {
        Self {
            dbgid: DbgId(format!("{}/ParseSyn", dbgid)),
            conf,
            value: None,
        }
    }
}
impl MessageParse<Vec<u8>> for ParseSyn {
    ///
    /// Extracting `Syn` symbol from the input bytes
    /// - returns bytes following by the `Syn`
    /// - call this method multiple times, until the end of message
    fn parse(&mut self, bytes: Bytes) -> Result<Vec<u8>, StrErr> {
        match self.value {
            Some(_) => Ok(bytes),
            None => {
                match bytes.iter().position(|b| *b == self.conf.0) {
                    Some(pos) => {
                        match bytes.get((pos + 1)..) {
                            Some(bytes) => {
                                self.value = Some(());
                                Ok(bytes.to_owned())
                            }
                            None => Ok(vec![]),
                        }
                    }
                    None => {
                        let dbg_bytes = if bytes.len() > 16 { &bytes[..16] } else { &bytes };
                        Err(format!("{}.parse | Syn not found in the message: {:?}...", self.dbgid, dbg_bytes ).into())
                    }
                }
            }
        }
    }
    ///
    /// Resets state to the initial
    fn reset(&mut self) {
        self.value = None;        
    }
}
