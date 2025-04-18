use std::{io::{BufReader, BufWriter, Read, Write}, net::{Shutdown, SocketAddr, TcpStream, ToSocketAddrs}, sync::Arc, time::{Duration, Instant}};
use sal_core::{dbg::Dbg, error::Error};
use crate::api::message::{fields::{FieldId, FieldSize}, message::{Bytes, Message, MessageParse}, message_kind::MessageKind, msg_kind::MsgKind};
///
/// 
pub type TcpMessage = Message<(FieldId, MessageKind, FieldSize, Bytes)>;
///
/// Connection status
pub enum IsConnected<T, E> {
    Active(T),
    Closed(E),
}
///
/// Basic Read / Write [Message]' via TCP Socket
pub struct TcpSocket {
    dbg: Dbg,
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
            .field("dbgid", &self.dbg)
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
    pub fn new(parent: impl Into<String>, address: impl ToSocketAddrs + std::fmt::Debug, message: TcpMessage, stream: Option<Arc<TcpStream>>) -> Self {
        let dbg = Dbg::new(parent, "TcpSocket");
        let address = match address.to_socket_addrs() {
            Ok(mut addrs) => match addrs.next() {
                Some(addr) => addr,
                None => panic!("{}.new | Empty address: {:?}", dbg, address),
            },
            Err(err) => panic!("{}.new | Address error: {:#?}", dbg, err),
        };
        Self {
            dbg,
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
    pub fn connect(&mut self) -> Result<Arc<TcpStream>, Error> {
        let error = Error::new(&self.dbg, "connect");
        let time = Instant::now();
        loop {
            match &self.connection {
                Some(stream) => {
                    return Ok(Arc::clone(stream))
                },
                None => {
                    match TcpStream::connect(self.address) {
                        Ok(stream) => {
                            log::debug!("{}.connect | connected to: \n\t{:?}", self.dbg, stream);
                            if let Err(err) = stream.set_read_timeout(Some(self.timeout)) {
                                let message = format!("{}.connect | set_read_timeout error: \n\t{:?}", self.dbg, err);
                                log::warn!("{}", message);
                            }
                            if let Err(err) = stream.set_write_timeout(Some(self.timeout)) {
                                let message = format!("{}.connect | set_write_timeout error: \n\t{:?}", self.dbg, err);
                                log::warn!("{}", message);
                            }
                            let stream = Arc::new(stream);
                            let stream_clone = Arc::clone(&stream);
                            self.connection = Some(stream);
                            return Ok(stream_clone)
                        },
                        Err(err) => {
                            let err = format!("{}.connect | Connection error: \n\t{:?}", self.dbg, err);
                            if log::max_level() >= log::LevelFilter::Trace {
                                log::warn!("{}", err);
                            }
                        }
                    }
                },
            }
            if time.elapsed() > self.timeout {
                let err = error.err(format!("Not connected in specified timeout {:?}", self.timeout));
                log::warn!("{}", err);
                return Err(err)
            }
        }
    }
    ///
    /// Closes a connection
    pub fn close(&mut self) -> Result<(), Error> {
        match &self.connection {
            Some(stream) => {
                stream
                    .shutdown(Shutdown::Both)
                    .map_err(|err| Error::new(&self.dbg, "close").pass(err.to_string()))
            },
            None => Ok(()),
        }
    }
    ///
    /// Sending a [Message] via TCP socket
    pub fn send(&mut self, bytes: &[u8], msg_id: Option<u32>) -> Result<FieldId, Error> {
        let error = Error::new(&self.dbg, "send");
        log::trace!("{}.send | bytes: {:?}", self.dbg, bytes);
        match self.connect() {
            Ok(stream) => {
                let msg_id = msg_id.unwrap_or_else(|| {
                    self.msg_id = (self.msg_id % u32::MAX) + 1;
                    self.msg_id
                });
                let bytes = self.message.build(bytes, msg_id);
                match BufWriter::new(stream.as_ref()).write_all(&bytes) {
                    Ok(_) => {
                        return Ok(FieldId(msg_id))
                    }
                    Err(err) => {
                        let err = error.pass_with("Write to tcp stream error", err.to_string());
                        log::warn!("{}", err);
                        if let Err(err) = self.close() {
                            log::warn!("{}.send | Close tcp stream error: {:?}", self.dbg, err);
                        }
                        return Err(err);
                    }
                }
            }
            Err(err) => {
                let err = error.pass_with("Connection error", err.to_string());
                log::warn!("{}", err);
                return Err(err);
            }
        };
    }
    ///
    /// Reads a [Message] parsed from TCP socket
    /// - Returns payload bytes only (cuting header)
    pub fn read(&mut self) -> Result<(FieldId, MsgKind), Error> {
        let error = Error::new(&self.dbg, "read");
        match self.connect() {
            Ok(stream) => {
                let time = Instant::now();
                let mut stream = BufReader::new(stream.as_ref());
                loop {
                    match stream.read(&mut self.buf) {
                        Ok(len) => {
                            log::trace!("{}.read |     read len: {:?}", self.dbg, len);
                            match self.message.parse(self.buf[..len].to_vec()) {
                                Ok((id, kind, size, bytes)) => {
                                    let dbg_bytes = if bytes.len() > 16 {format!("{:?} ...", &bytes[..16])} else {format!("{:?}", bytes)};
                                    log::trace!("{}.read | id: {:?},  kind: {:?},  size: {:?},  bytes: {:?}", self.dbg, id, kind, size, dbg_bytes);
                                    match kind {
                                        MessageKind::Any => return Ok((id.clone(), MsgKind::Bytes(bytes.to_owned()))),
                                        MessageKind::Empty => log::warn!("{} | Message of kind '{:?}' - is not implemented yet", self.dbg, kind),
                                        MessageKind::Bytes => return Ok((id.clone(), MsgKind::Bytes(bytes.to_owned()))),
                                        MessageKind::Bool => log::warn!("{} | Message of kind '{:?}' - is not implemented yet", self.dbg, kind),
                                        MessageKind::U16 => log::warn!("{} | Message of kind '{:?}' - is not implemented yet", self.dbg, kind),
                                        MessageKind::U32 => log::warn!("{} | Message of kind '{:?}' - is not implemented yet", self.dbg, kind),
                                        MessageKind::U64 => log::warn!("{} | Message of kind '{:?}' - is not implemented yet", self.dbg, kind),
                                        MessageKind::I16 => log::warn!("{} | Message of kind '{:?}' - is not implemented yet", self.dbg, kind),
                                        MessageKind::I32 => log::warn!("{} | Message of kind '{:?}' - is not implemented yet", self.dbg, kind),
                                        MessageKind::I64 => log::warn!("{} | Message of kind '{:?}' - is not implemented yet", self.dbg, kind),
                                        MessageKind::F32 => log::warn!("{} | Message of kind '{:?}' - is not implemented yet", self.dbg, kind),
                                        MessageKind::F64 => log::warn!("{} | Message of kind '{:?}' - is not implemented yet", self.dbg, kind),
                                        MessageKind::String => match String::from_utf8(bytes) {
                                            Ok(value) => return Ok((id.clone(), MsgKind::String(value))),
                                            Err(err) => return Err(format!("{}.read | Message::string parse error: {}", self.dbg, err).into()),
                                        },
                                        MessageKind::Timestamp => log::warn!("{}.read | Message of kind '{:?}' - is not implemented yet", self.dbg, kind),
                                        MessageKind::Duration => log::warn!("{}.read | Message of kind '{:?}' - is not implemented yet", self.dbg, kind),
                                    }
                                }
                                Err(err) => {
                                    log::warn!("{}", err);
                                }
                            };
                            if len == 0 {
                                if let Err(err) = self.close() {
                                    log::warn!("{}.read | Close tcp stream error: {:?}", self.dbg, err);
                                }
                                return Err(format!("{}.read | tcp stream closed", self.dbg).into());
                            }
                        }
                        Err(err) => {
                            let msg = error.pass_with("Close tcp stream error", err.to_string());
                            if let IsConnected::Closed(_) = self.parse_err(err) {
                                if let Err(err) = self.close() {
                                    log::warn!("{}.read | Close tcp stream error: {:?}", self.dbg, err);
                                }
                            };
                            return Err(msg);
                        }
                    };
                    if time.elapsed() > self.timeout {
                        let msg = error.err(format!("No valid message received in specified timeout {:?}", self.timeout));
                        log::warn!("{}", msg);
                        return Err(msg);
                    }
                }
            }
            Err(err) => {
                let err = error.pass_with("Connection error", err);
                log::warn!("{}", err);
                return Err(err);
            }
        };
    }
    ///
    /// Returns Connection status dipending on IO Error
    fn parse_err(&self, input: std::io::Error) -> IsConnected<(), Error> {
        log::warn!("{}.parse_err | error reading from socket: {:?}", self.dbg, input);
        log::warn!("{}.parse_err | error kind: {:?}", self.dbg, input.kind());
        let err = Error::new(&self.dbg, "parse_err").pass(&input.to_string());
        match input.kind() {
            // std::io::ErrorKind::NotFound => todo!(),
            std::io::ErrorKind::PermissionDenied => IsConnected::Closed(err),
            std::io::ErrorKind::ConnectionRefused => IsConnected::Closed(err),
            std::io::ErrorKind::ConnectionReset => IsConnected::Closed(err),
            std::io::ErrorKind::HostUnreachable => IsConnected::Closed(err),
            std::io::ErrorKind::NetworkUnreachable => IsConnected::Closed(err),
            std::io::ErrorKind::ConnectionAborted => IsConnected::Closed(err),
            std::io::ErrorKind::NotConnected => IsConnected::Closed(err),
            std::io::ErrorKind::AddrInUse => IsConnected::Closed(err),
            std::io::ErrorKind::AddrNotAvailable => IsConnected::Closed(err),
            std::io::ErrorKind::NetworkDown => IsConnected::Closed(err),
            std::io::ErrorKind::BrokenPipe => IsConnected::Closed(err),
            std::io::ErrorKind::AlreadyExists => IsConnected::Closed(err),
            std::io::ErrorKind::WouldBlock => IsConnected::Closed(err),
            // std::io::ErrorKind::NotADirectory => todo!(),
            // std::io::ErrorKind::IsADirectory => todo!(),
            // std::io::ErrorKind::DirectoryNotEmpty => todo!(),
            // std::io::ErrorKind::ReadOnlyFilesystem => todo!(),
            // std::io::ErrorKind::FilesystemLoop => todo!(),
            // std::io::ErrorKind::StaleNetworkFileHandle => todo!(),
            // std::io::ErrorKind::InvalidInput => todo!(),
            // std::io::ErrorKind::InvalidData => todo!(),
            std::io::ErrorKind::TimedOut => IsConnected::Closed(err),
            // std::io::ErrorKind::WriteZero => todo!(),
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
            // std::io::ErrorKind::Interrupted => todo!(),
            // std::io::ErrorKind::Unsupported => todo!(),
            // std::io::ErrorKind::UnexpectedEof => todo!(),
            // std::io::ErrorKind::OutOfMemory => todo!(),
            // std::io::ErrorKind::Other => todo!(),
            _ => IsConnected::Closed(err),
        }
    }
}
