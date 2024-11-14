//!
//! # Messages transmitted over socket.
//! 
//! - Data can be encoded using varius data `Kind`, `Size` and payload Data
//! 
//! - Message format
//!     Field name | Start | Kind |  Size  | Data |
//!     ---       |  ---  | ---  |  ---   | ---  |
//!     Data type |  u8   | u8   | u32    | [u8; Size] |
//!     Value     |  22   | StringValue | xxx    | [..., ...]  |
//!     
//!     - Start - Each message starts with SYN (22)
//!     - Kind - The `Kind` of the data stored in the `Data` field, refer to
//!     - Size - The length of the `Data` field in bytes
//!     - Data - Data structured depending on it `Kind`
//! 
//! - `Kind` of data
//!     - 00, Any
//!     - 01, Empty
//!     - 02, Bytes
//!     - 08, Bool
//!     - 16, UInt16
//!     - 17, UInt32
//!     - 18, UInt64
//!     - 24, Int16
//!     - 25, Int32
//!     - 26, Int64
//!     - 32, F32
//!     - 33, F64
//!     - 40, String
//!     - 48, Timestamp
//!     - 49, Duration
//!     - .., ...
//! 
use std::iter::Peekable;
use crate::{debug::dbg_id::DbgId, error::str_err::StrErr};
use super::{fields::{FieldData, FieldId, FieldKind, FieldSize, FieldSyn}, from_bytes::FromBytes, message_kind::MessageKind};
/// 
/// 
#[derive(Debug, Clone, PartialEq)]
pub enum MessageField {
    Syn(FieldSyn),
    Id(FieldId),
    Kind(FieldKind),
    Size(FieldSize),
    Data(FieldData),
}
///
/// Socket Message
pub struct Message {
    dbgid: DbgId,
    fields: Vec<MessageField>, 
    state: Peekable<Box<dyn Iterator<Item = MessageField>>>,
    result: Vec<MessageField>, 
    start: usize,
    end: usize,
    id: Option<u32>,
    size: Option<u32>,
    buffer: Vec<u8>,
}
//
//
impl Clone for Message {
    fn clone(&self) -> Self {
        Self { 
            dbgid: self.dbgid.clone(),
            fields: self.fields.clone(),
            state: (Box::new(self.fields.to_owned().into_iter().cycle()) as Box<dyn Iterator<Item = MessageField>>).peekable(),
            result: self.result.clone(),
            start: self.start.clone(),
            end: self.end.clone(),
            id: self.id.clone(),
            size: self.size.clone(),
            buffer: self.buffer.clone(),
        }
    }
}
//
//
impl std::fmt::Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Message")
            .field("fields", &self.fields)
            .field("result", &self.result)
            .field("start", &self.start)
            .field("end", &self.end)
            .field("size", &self.size)
            .field("buffer", &self.buffer)
            .finish()
    }
}
//
//
impl Message {
    /// Each Message starts with
    pub const SYN: u8 = 22;
    ///
    /// Returns `Message` new instance 
    pub fn new(
        dbgid: &DbgId,
        fields: &[MessageField]
    ) -> Self {
        Self {
            dbgid: DbgId(format!("{}/Message", dbgid)),
            fields: fields.to_owned(),
            state: (Box::new(fields.to_owned().into_iter().cycle()) as Box<dyn Iterator<Item = MessageField>>).peekable(),
            result: vec![],
            start: 0,
            end: 0,
            id: None,
            size: None,
            buffer: vec![],
        }
    }
    ///
    /// 
    pub fn restart(&mut self) {
        self.state = (Box::new(self.fields.to_owned().into_iter().cycle()) as Box<dyn Iterator<Item = MessageField>>).peekable();
    }
    ///
    /// Returns message (by fields) read and parsed from socket 
    /// - Parse done by fields specified in the constructor, 
    pub fn parse(&mut self, bytes: &[u8]) -> Result<Vec<MessageField>, StrErr> {
        let bytes = [&std::mem::take(&mut self.buffer), bytes].concat();
        let dbg_bytes = if bytes.len() > 16 {format!("{:?} ...", &bytes[..16])} else {format!("{:?}", bytes)};
        log::debug!("{}.parse | Input bytes ({}): {:?}", self.dbgid, bytes.len(), dbg_bytes);
        loop {
            match self.state.peek() {
                Some(state) => {
                    match state {
                        MessageField::Syn(field) => {
                            log::debug!("{}.parse | Fild::Syn", self.dbgid);
                            self.start = match bytes.iter().position(|b| *b == field.0) {
                                Some(pos) => pos,
                                None => {
                                    return Err(format!("{}.parse | Syn not found in the message: {:?}...", self.dbgid, if bytes.len() > 16 { &bytes[..16] } else { &bytes } ).into());
                                }
                            };
                            self.end = self.start + field.len();
                            self.state.next().unwrap();
                            self.start = self.end;
                            log::debug!("{}.parse | Fild::Syn pos: {}..{}", self.dbgid, self.start, self.end);
                            // log::debug!("{}.parse | Fild::Syn bytes: {:?}", &bytes[self.start..self.end]);
                        }
                        MessageField::Id(field) => {
                            self.end = self.start + field.len();
                            log::debug!("{}.parse | Fild::Id pos: {}..{}", self.dbgid, self.start, self.end);
                            // log::debug!("{}.parse | Fild::Size bytes: {:?}", &bytes[self.start..self.end]);
                            match bytes.get(self.start..self.end) {
                                Some(bytes) => {
                                    let dbg_bytes = if bytes.len() > 16 {format!("{:?} ...", &bytes[..16])} else {format!("{:?}", bytes)};
                                    log::debug!("{}.parse | Fild::Id bytes: {:?}", self.dbgid, dbg_bytes);
                                    match bytes.try_into() {
                                        Ok(id_bytes) => {
                                            log::debug!("{}.parse | Fild::Id bytes: {:?}", self.dbgid, id_bytes);
                                            let id= u32::from_be_bytes(id_bytes);
                                            self.id = Some(id);
                                            self.result.push(MessageField::Id(FieldId(id)));
                                            self.state.next().unwrap();
                                            self.start = self.end;
                                        },
                                        Err(err) => {
                                            self.buffer = bytes.into();
                                            return Err(format!("{}.parse | Filed 'Id' take error: {:#?}", self.dbgid, err).into());
                                        }
                                    }
                                }
                                None => {
                                    self.buffer = bytes.into();
                                    return Err(format!("{}.parse | Filed 'Id' take error", self.dbgid).into());
                                }
                            }
                        }
                        MessageField::Kind(field) => {
                            self.end = self.start + field.len();
                            log::debug!("{}.parse | Fild::Kind pos: {}..{}", self.dbgid, self.start, self.end);
                            // log::debug!("{}.parse | Fild::Kind bytes: {:?}", &bytes[self.start..self.end]);
                            match bytes.get(self.start..self.end) {
                                Some(bytes) => match bytes.try_into() {
                                    Ok(bytes) => match MessageKind::from_bytes(bytes) {
                                        Ok(kind) => {
                                            self.result.push(MessageField::Kind(FieldKind(kind)));
                                            self.state.next().unwrap();
                                            self.start = self.end;
                                        },
                                        Err(err) => {
                                            self.restart();
                                            return Err(format!("{}.parse | Filed 'Kind' parse error: {:#?}", self.dbgid, err).into())
                                        }
                                    }
                                    Err(err) => {
                                        self.buffer = bytes.into();
                                        return Err(format!("{}.parse | Filed 'Kind' take error: {:#?}", self.dbgid, err).into())
                                    }
                                }
                                None => {
                                    self.buffer = bytes.into();
                                    return Err(format!("{}.parse | Filed 'Kind' take error", self.dbgid).into())
                                }
                            }
                        }
                        MessageField::Size(field) => {
                            self.end = self.start + field.len();
                            log::debug!("{}.parse | Fild::Size pos: {}..{}", self.dbgid, self.start, self.end);
                            // log::debug!("{}.parse | Fild::Size bytes: {:?}", &bytes[self.start..self.end]);
                            match bytes.get(self.start..self.end) {
                                Some(bytes) => {
                                    log::debug!("{}.parse | Fild::Size bytes: {:?}", self.dbgid, bytes);
                                    match bytes.try_into() {
                                        Ok(size_bytes) => {
                                            log::debug!("{}.parse | Fild::Size bytes: {:?}", self.dbgid, size_bytes);
                                            let s= u32::from_be_bytes(size_bytes);
                                            self.size = Some(s);
                                            self.result.push(MessageField::Size(FieldSize(s)));
                                            self.state.next().unwrap();
                                            self.start = self.end;
                                        },
                                        Err(err) => {
                                            self.buffer = bytes.into();
                                            return Err(format!("{}.parse | Filed 'Size' take error: {:#?}", self.dbgid, err).into());
                                        }
                                    }
                                }
                                None => {
                                    self.buffer = bytes.into();
                                    return Err(format!("{}.parse | Filed 'Size' take error", self.dbgid).into());
                                }
                            }
                        }
                        MessageField::Data(_) => {
                            // log::debug!("{}.parse | Fild::Data");
                            match self.size {
                                Some(size) => {
                                    self.end = self.start + (size as usize);
                                    log::debug!("{}.parse | Fild::Data pos: {}..{}", self.dbgid, self.start, self.end);
                                    // log::debug!("{}.parse | Fild::Data bytes: {:?}", &bytes[self.start..self.end]);
                                    match bytes.get(self.start..self.end) {
                                        Some(bytes) => match bytes.try_into() {
                                            Ok(data) => {
                                                self.result.push(MessageField::Data(FieldData(data)));
                                                self.state.next().unwrap();
                                                return Ok(std::mem::take(&mut self.result));
                                            },
                                            Err(err) => {
                                                self.buffer = bytes.into();
                                                return Err(format!("{}.parse | Filed 'Data' take error: {:#?}", self.dbgid, err).into());
                                            }
                                        }
                                        None => {
                                            self.buffer = bytes.into();
                                            return Ok(vec![]);
                                            // return Err(format!("{}.parse | Filed 'Data' take error").into());
                                        }
                                    }
                                }
                                None => {
                                    self.restart();
                                    return Err(format!("{}.parse | Field 'Data' can't be read because Filed 'Size' is not ready", self.dbgid).into());
                                }
                            }
                        }
                    }
                }
                None => {
                    self.restart();
                    return Err(format!("{}.parse | State error", self.dbgid).into());
                }
            }
        }
    }
    ///
    /// Returns message built according to specified fields and passed `bytes`
    pub fn build(&mut self, bytes: &[u8], id: u32) -> Vec<u8> {
        let mut message = vec![];
        for field in &mut self.fields {
            match field {
                MessageField::Syn(field_syn) => message.push(field_syn.0),
                MessageField::Id(_) => message.extend(FieldId(id).to_be_bytes()),
                MessageField::Kind(field_kind) => message.extend(field_kind.to_bytes()),
                MessageField::Size(field_size) => message.extend(field_size.to_be_bytes(bytes.len() as u32)),
                MessageField::Data(_) => {
                    message.extend_from_slice(bytes);
                }
            }
        }
        message
    }

}
///
/// 
pub type Bytes = Vec<u8>;
///
/// Parse Message structure from bytes Interface 
pub trait MessageParse<T> {
    ///
    /// Extracting some pattern from input `bytes`
    fn parse(&mut self, bytes: Bytes) -> Result<T, StrErr>;
    ///
    /// Resets state to the initial
    fn reset(&mut self);
}