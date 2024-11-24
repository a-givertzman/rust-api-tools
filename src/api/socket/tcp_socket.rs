use std::{io::{BufReader, BufWriter, Read, Write}, net::{Shutdown, SocketAddr, TcpStream, ToSocketAddrs}, sync::Arc, time::{Duration, Instant}};
use crate::{
    api::message::{fields::{FieldId, FieldSize}, message::{Bytes, Message, MessageParse}, message_kind::_MessageKind, msg_kind::MsgKind},
    debug::dbg_id::DbgId, error::str_err::StrErr,
};
///
/// 
pub type TcpMessage = Message<(FieldId, _MessageKind, FieldSize, Bytes)>;
///
/// Connection status
pub enum IsConnected<T, E> {
    Active(T),
    Closed(E),
}
///
/// Basic Read / Write [Message]' via TCP Socket
pub struct TcpSocket {
    dbgid: DbgId,
    address: SocketAddr,
    message: TcpMessage,
    msg_id: u32,
    connection: Option<Arc<TcpStream>>,
    buf: [u8; Self::BUF_LEN],
    timeout: Duration,
}
//
//
impl std::fmt::Debug for TcpSocket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TcpSocket")
            .field("dbgid", &self.dbgid)
            .field("address", &self.address)
            .field("message", &self.message)
            .field("connection", &self.connection)
            // .field("stream", &self.stream)
            // .field("buf", &self.buf)
            .field("timeout", &self.timeout).finish()
    }
}
//
//
impl TcpSocket {
    ///
    /// bytes to be read from socket at once
    const BUF_LEN: usize = 1024 * 4;
    ///
    /// Returns [TcpSocket] new instance
    /// - `address` - TCP address of the remote host to be connected
    /// - `message` - [TcpMessage] provides `build` and `parse`
    /// - `stream` - TcpStream if already connected,
    ///    - If None specified, connection will be opened internally only when required 
    pub fn new(dbid: &DbgId, address: impl ToSocketAddrs + std::fmt::Debug, message: TcpMessage, stream: Option<Arc<TcpStream>>) -> Self {
        let dbgid = DbgId::with_parent(&dbid, "TcpSocket");
        let address = match address.to_socket_addrs() {
            Ok(mut addrs) => match addrs.next() {
                Some(addr) => addr,
                None => panic!("{}.new | Empty address: {:?}", dbgid, address),
            },
            Err(err) => panic!("{}.new | Address error: {:#?}", dbgid, err),
        };
        Self {
            dbgid,
            address: address.into(),
            message,
            msg_id: 0,
            connection: stream,
            buf: [0; Self::BUF_LEN],
            timeout: Duration::from_secs(10),
        }
    }
    ///
    /// Opens a connection to the TCP Socket and preparing the `Message`
    pub fn connect(&mut self) -> Result<Arc<TcpStream>, StrErr> {
        let time = Instant::now();
        loop {
            match &self.connection {
                Some(stream) => {
                    return Ok(Arc::clone(stream))
                },
                None => {
                    match TcpStream::connect(self.address) {
                        Ok(stream) => {
                            log::debug!("{}.connect | connected to: \n\t{:?}", self.dbgid, stream);
                            if let Err(err) = stream.set_read_timeout(Some(self.timeout)) {
                                let message = format!("{}.connect | set_read_timeout error: \n\t{:?}", self.dbgid, err);
                                log::warn!("{}", message);
                            }
                            if let Err(err) = stream.set_write_timeout(Some(self.timeout)) {
                                let message = format!("{}.connect | set_write_timeout error: \n\t{:?}", self.dbgid, err);
                                log::warn!("{}", message);
                            }
                            let stream = Arc::new(stream);
                            let stream_clone = Arc::clone(&stream);
                            self.connection = Some(stream);
                            return Ok(stream_clone)
                        },
                        Err(err) => {
                            let err = format!("{}.connect | Connection error: \n\t{:?}", self.dbgid, err);
                            if log::max_level() >= log::LevelFilter::Trace {
                                log::warn!("{}", err);
                            }
                        }
                    }
                },
            }
            if time.elapsed() > self.timeout {
                let err = format!("{}.connect | Not connected in specified timeout {:?}", self.dbgid, self.timeout);
                log::warn!("{}", err);
                return Err(StrErr(err))
            }
        }
    }
    ///
    /// Closes a connection
    pub fn close(&mut self) -> Result<(), StrErr> {
        match &self.connection {
            Some(stream) => {
                stream
                    .shutdown(Shutdown::Both)
                    .map_err(|err| StrErr(format!("{}.close | Error: {:#?}", self.dbgid, err)))
            },
            None => Ok(()),
        }
    }
    ///
    /// Sending a [Message] via TCP socket
    pub fn send(&mut self, bytes: &[u8]) -> Result<FieldId, StrErr> {
        log::trace!("{}.send | bytes: {:#?}", self.dbgid, bytes);
        match self.connect() {
            Ok(stream) => {
                self.msg_id = (self.msg_id % u32::MAX) + 1;
                let msg_id = self.msg_id;
                let bytes = self.message.build(bytes, msg_id);
                match BufWriter::new(stream.as_ref()).write_all(&bytes) {
                    Ok(_) => {
                        return Ok(FieldId(msg_id))
                    }
                    Err(err) => {
                        let err = format!("{}.send | write to tcp stream error: {:?}", self.dbgid, err);
                        log::warn!("{}", err);
                        if let Err(err) = self.close() {
                            log::warn!("{}.send | Close tcp stream error: {:?}", self.dbgid, err);
                        }
                        return Err(StrErr(err));
                    }
                }
            }
            Err(err) => {
                let err = format!("{}.send | Connection error: {:?}", self.dbgid, err);
                log::warn!("{}", err);
                return Err(StrErr(err));
            }
        };
    }
    ///
    /// Reads a [Message] parsed from TCP socket
    /// - Returns payload bytes only (cuting header)
    pub fn read(&mut self) -> Result<(FieldId, MsgKind), StrErr> {
        match self.connect() {
            Ok(stream) => {
                let time = Instant::now();
                let mut stream = BufReader::new(stream.as_ref());
                loop {
                    match stream.read(&mut self.buf) {
                        Ok(len) => {
                            log::trace!("{}.read |     read len: {:?}", self.dbgid, len);
                            match self.message.parse(self.buf[..len].to_vec()) {
                                Ok((id, kind, size, bytes)) => {
                                    let dbg_bytes = if bytes.len() > 16 {format!("{:?} ...", &bytes[..16])} else {format!("{:?}", bytes)};
                                    log::trace!("{}.read | id: {:?},  kind: {:?},  size: {:?},  bytes: {:?}", self.dbgid, id, kind, size, dbg_bytes);
                                    match kind {
                                        _MessageKind::Any => return Ok((id.clone(), MsgKind::Bytes(bytes.to_owned()))),
                                        _MessageKind::Empty => log::warn!("{} | Message of kind '{:?}' - is not implemented yet", self.dbgid, kind),
                                        _MessageKind::Bytes => return Ok((id.clone(), MsgKind::Bytes(bytes.to_owned()))),
                                        _MessageKind::Bool => log::warn!("{} | Message of kind '{:?}' - is not implemented yet", self.dbgid, kind),
                                        _MessageKind::U16 => log::warn!("{} | Message of kind '{:?}' - is not implemented yet", self.dbgid, kind),
                                        _MessageKind::U32 => log::warn!("{} | Message of kind '{:?}' - is not implemented yet", self.dbgid, kind),
                                        _MessageKind::U64 => log::warn!("{} | Message of kind '{:?}' - is not implemented yet", self.dbgid, kind),
                                        _MessageKind::I16 => log::warn!("{} | Message of kind '{:?}' - is not implemented yet", self.dbgid, kind),
                                        _MessageKind::I32 => log::warn!("{} | Message of kind '{:?}' - is not implemented yet", self.dbgid, kind),
                                        _MessageKind::I64 => log::warn!("{} | Message of kind '{:?}' - is not implemented yet", self.dbgid, kind),
                                        _MessageKind::F32 => log::warn!("{} | Message of kind '{:?}' - is not implemented yet", self.dbgid, kind),
                                        _MessageKind::F64 => log::warn!("{} | Message of kind '{:?}' - is not implemented yet", self.dbgid, kind),
                                        _MessageKind::String => match String::from_utf8(bytes) {
                                            Ok(value) => return Ok((id.clone(), MsgKind::String(value))),
                                            Err(err) => return Err(format!("{}.read | Message::string parse error: {}", self.dbgid, err).into()),
                                        },
                                        _MessageKind::Timestamp => log::warn!("{}.read | Message of kind '{:?}' - is not implemented yet", self.dbgid, kind),
                                        _MessageKind::Duration => log::warn!("{}.read | Message of kind '{:?}' - is not implemented yet", self.dbgid, kind),
                                    }
                                }
                                Err(err) => {
                                    log::warn!("{}", err);
                                }
                            };
                            if len < Self::BUF_LEN {
                                if len == 0 {
                                    if let Err(err) = self.close() {
                                        log::warn!("{}.read | Close tcp stream error: {:?}", self.dbgid, err);
                                    }
                                    return Err(format!("{}.read | tcp stream closed", self.dbgid).into());
                                }
                            }
                        }
                        Err(err) => {
                            let msg = format!("{}.read | Close tcp stream error: {:#?}", self.dbgid, err);
                            if let IsConnected::Closed(_) = self.parse_err(err) {
                                if let Err(err) = self.close() {
                                    log::warn!("{}.read | Close tcp stream error: {:?}", self.dbgid, err);
                                }
                            };
                            return Err(StrErr(msg));
                        }
                    };
                    if time.elapsed() > self.timeout {
                        let err = format!("{}.read | No valid message received in specified timeout {:?}", self.dbgid, self.timeout);
                        log::warn!("{}", err);
                        return Err(StrErr(err));
                    }
                }
            }
            Err(err) => {
                let err = format!("{}.read | Connection error: {:?}", self.dbgid, err);
                log::warn!("{}", err);
                return Err(StrErr(err));
            }
        };
    }
    ///
    /// Returns Connection status dipending on IO Error
    fn parse_err(&self, err: std::io::Error) -> IsConnected<(), StrErr> {
        log::warn!("{}.parse_err | error reading from socket: {:?}", self.dbgid, err);
        log::warn!("{}.parse_err | error kind: {:?}", self.dbgid, err.kind());
        let str_err = format!("{}.parse_err | error: {:#?}", self.dbgid, err).into();
        match err.kind() {
            // std::io::ErrorKind::NotFound => todo!(),
            std::io::ErrorKind::PermissionDenied => IsConnected::Closed(str_err),
            std::io::ErrorKind::ConnectionRefused => IsConnected::Closed(str_err),
            std::io::ErrorKind::ConnectionReset => IsConnected::Closed(str_err),
            // std::io::ErrorKind::HostUnreachable => ConnectionStatus::Closed(str_err),
            // std::io::ErrorKind::NetworkUnreachable => ConnectionStatus::Closed(str_err),
            std::io::ErrorKind::ConnectionAborted => IsConnected::Closed(str_err),
            std::io::ErrorKind::NotConnected => IsConnected::Closed(str_err),
            std::io::ErrorKind::AddrInUse => IsConnected::Closed(str_err),
            std::io::ErrorKind::AddrNotAvailable => IsConnected::Closed(str_err),
            // std::io::ErrorKind::NetworkDown => ConnectionStatus::Closed(str_err),
            std::io::ErrorKind::BrokenPipe => IsConnected::Closed(str_err),
            std::io::ErrorKind::AlreadyExists => todo!(),
            std::io::ErrorKind::WouldBlock => IsConnected::Closed(str_err),
            // std::io::ErrorKind::NotADirectory => todo!(),
            // std::io::ErrorKind::IsADirectory => todo!(),
            // std::io::ErrorKind::DirectoryNotEmpty => todo!(),
            // std::io::ErrorKind::ReadOnlyFilesystem => todo!(),
            // std::io::ErrorKind::FilesystemLoop => todo!(),
            // std::io::ErrorKind::StaleNetworkFileHandle => todo!(),
            std::io::ErrorKind::InvalidInput => todo!(),
            std::io::ErrorKind::InvalidData => todo!(),
            std::io::ErrorKind::TimedOut => todo!(),
            std::io::ErrorKind::WriteZero => todo!(),
            // std::io::ErrorKind::StorageFull => todo!(),
            // std::io::ErrorKind::NotSeekable => todo!(),
            // std::io::ErrorKind::FilesystemQuotaExceeded => todo!(),
            // std::io::ErrorKind::FileTooLarge => todo!(),
            // std::io::ErrorKind::ResourceBusy => todo!(),
            // std::io::ErrorKind::ExecutableFileBusy => todo!(),
            // std::io::ErrorKind::Deadlock => todo!(),
            // std::io::ErrorKind::CrossesDevices => todo!(),
            // std::io::ErrorKind::TooManyLinks => todo!(),
            // std::io::ErrorKind::InvalidFilename => todo!(),
            // std::io::ErrorKind::ArgumentListTooLong => todo!(),
            std::io::ErrorKind::Interrupted => todo!(),
            std::io::ErrorKind::Unsupported => todo!(),
            std::io::ErrorKind::UnexpectedEof => todo!(),
            std::io::ErrorKind::OutOfMemory => todo!(),
            std::io::ErrorKind::Other => todo!(),
            _ => IsConnected::Closed(str_err),
        }
    }
}
