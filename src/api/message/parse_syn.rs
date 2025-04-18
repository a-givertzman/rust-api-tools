use sal_core::{dbg::Dbg, error::Error};
use super::{fields::FieldSyn, message::{Bytes, MessageParse}};
///
/// Extracting `Syn` symbol from the input bytes
/// - Used to identify a start of the message
pub struct ParseSyn {
    dbg: Dbg,
    conf: FieldSyn,
    value: Option<()>,
}
//
//
impl ParseSyn {
    ///
    /// Returns [ParseSyn] new instance
    pub fn new(parent: impl Into<String>, conf: FieldSyn) -> Self {
        Self {
            dbg: Dbg::new(parent, "ParseSyn"),
            conf,
            value: None,
        }
    }
}
//
//
impl MessageParse<Vec<u8>> for ParseSyn {
    ///
    /// Extracting `Syn` symbol from the input bytes
    /// - returns bytes following by the `Syn`
    /// - call this method multiple times, until the end of message
    fn parse(&mut self, bytes: Bytes) -> Result<Vec<u8>, Error> {
        let error = Error::new(&self.dbg, "parse");
        match self.value {
            Some(_) => Ok(bytes),
            None => {
                match bytes.iter().position(|b| *b == self.conf.0) {
                    Some(pos) => {
                        // log::trace!("{} | bytes: {:?}", self.dbgid, bytes);
                        match bytes.get((pos + 1)..) {
                            Some(bytes) => {
                                self.value = Some(());
                                Ok(bytes.to_owned())
                            }
                            None => Ok(vec![]),
                        }
                    }
                    None => {
                        let dbg_bytes = if bytes.len() > 16 { format!("{:?}...", &bytes[..16]) } else { format!("{:?}", bytes) };
                        Err(error.err(format!("Syn not found in message: {:?}", dbg_bytes)))
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
