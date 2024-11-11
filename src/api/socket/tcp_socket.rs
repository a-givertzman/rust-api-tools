use std::{io::Read, net::TcpStream, time::{Duration, Instant}};

use crate::{api::message::{fields::{FieldData, FieldSize}, message::{Message, MessageField}, message_kind::MessageKind}, debug::dbg_id::DbgId, error::str_err::StrErr};

use super::connection_status::IsConnected;

///
/// Basic Read / Write functional on the TCP Socket
pub struct TcpSocket {
    dbgid: DbgId,
    stream: TcpStream,
    message: Message,
    buf: [u8; Self::BUF_LEN],
    timeout: Duration,
}
//
//
impl TcpSocket {
    ///
    /// bytes to be read from socket at once
    const BUF_LEN: usize = 1024 * 1;
    ///
    /// Returns `TcpSocket` new instance
    pub fn new(dbid: DbgId, stream: TcpStream, message: Message) -> Self {
        Self {
            dbgid: DbgId::with_parent(&dbid, "TcpSocket"),
            stream,
            message,
            buf: [0; Self::BUF_LEN],
            timeout: Duration::from_secs(3),
        }
    }
    ///
    /// Reads all available data from the TspStream until `Message` parsed successfully
    /// - Returns payload bytes only (cuting header)
    fn read_message(&mut self) -> Result<Vec<u8>, StrErr> {
        // let mut buf = [0; Self::BUF_LEN];
        let time = Instant::now();
        loop {
            match self.stream.read(&mut self.buf) {
                Ok(len) => {
                    log::trace!("{}.read_message |     read len: {:?}", self.dbgid, len);
                    match self.message.parse(&self.buf[..len]) {
                        Ok(parsed) => match parsed.as_slice() {
                            [ MessageField::Kind(kind), MessageField::Size(FieldSize(size)), MessageField::Data(FieldData(data)) ] => {
                                log::debug!("{}.read_message | kind: {:?},  size: {},  data: {:?}", self.dbgid, kind, size, data);
                                match kind.0 {
                                    MessageKind::Any => log::warn!("{} | Message of kind '{:?}' - is not implemented yet", self.dbgid, kind),
                                    MessageKind::Empty => log::warn!("{} | Message of kind '{:?}' - is not implemented yet", self.dbgid, kind),
                                    MessageKind::Bytes => log::warn!("{} | Message of kind '{:?}' - is not implemented yet", self.dbgid, kind),
                                    MessageKind::Bool => log::warn!("{} | Message of kind '{:?}' - is not implemented yet", self.dbgid, kind),
                                    MessageKind::U16 => log::warn!("{} | Message of kind '{:?}' - is not implemented yet", self.dbgid, kind),
                                    MessageKind::U32 => log::warn!("{} | Message of kind '{:?}' - is not implemented yet", self.dbgid, kind),
                                    MessageKind::U64 => log::warn!("{} | Message of kind '{:?}' - is not implemented yet", self.dbgid, kind),
                                    MessageKind::I16 => log::warn!("{} | Message of kind '{:?}' - is not implemented yet", self.dbgid, kind),
                                    MessageKind::I32 => log::warn!("{} | Message of kind '{:?}' - is not implemented yet", self.dbgid, kind),
                                    MessageKind::I64 => log::warn!("{} | Message of kind '{:?}' - is not implemented yet", self.dbgid, kind),
                                    MessageKind::F32 => log::warn!("{} | Message of kind '{:?}' - is not implemented yet", self.dbgid, kind),
                                    MessageKind::F64 => log::warn!("{} | Message of kind '{:?}' - is not implemented yet", self.dbgid, kind),
                                    MessageKind::String => return Ok(data.to_owned()),
                                    MessageKind::Timestamp => log::warn!("{} | Message of kind '{:?}' - is not implemented yet", self.dbgid, kind),
                                    MessageKind::Duration => log::warn!("{} | Message of kind '{:?}' - is not implemented yet", self.dbgid, kind),
                                }
                            }
                            v if v.is_empty() => {}
                            [..] => {
                                let err = format!("{}.read_message | Unknown message kind {:?}", self.dbgid, parsed);
                                log::warn!("{}", err);
                                // return Err(err.into())
                            }
                        }
                        Err(err) => {
                            log::warn!("{}", err);
                            // return Err(err.into())
                        }
                    };
                    if len < Self::BUF_LEN {
                        if len == 0 {
                            return Err(format!("{}.read_message | tcp stream closed", self.dbgid).into());
                        }
                    }
                }
                Err(err) => {
                    _ = self.parse_err(err);
                }
            };
            if time.elapsed() > self.timeout {
                return Err(format!("{}.read_message | Valid message wasn`t received in specified timeout ({:?})", self.dbgid, self.timeout).into());
            }
        }
    }
    // ///
    // /// reads all avalible data from the TspStream
    // /// - returns Active: if read bytes non zero length without errors
    // /// - returns Closed:
    // ///    - if read 0 bytes
    // ///    - if on error
    // fn read_all(&mut self, mut stream: TcpStream, message: Message) -> Result<Vec<u8>, StrErr> {
    //     let mut buf = [0; Self::BUF_LEN];
    //     let mut result = vec![];
    //     loop {
    //         match stream.read(&mut buf) {
    //             Ok(len) => {
    //                 log::trace!("{}.read_all |     read len: {:?}", self.dbgid, len);
    //                 result.append(& mut buf[..len].into());
    //                 if len < Self::BUF_LEN {
    //                     if len == 0 {
    //                         return Err(format!("{}.read_all | tcp stream closed", self.dbgid).into());
    //                     } else {
    //                         if self.keep_alive {
    //                             self.connection.replace((stream, message));
    //                         }
    //                         return Ok(result)
    //                     }
    //                 }
    //             },
    //             Err(err) => {
    //                 _ = self.parse_err(err);
    //             }
    //         };
    //     }
    // }
    ///
    /// Returns Connection status dipending on IO Error
    fn parse_err(&self, err: std::io::Error) -> IsConnected {
        log::warn!("{}.read_all | error reading from socket: {:?}", self.dbgid, err);
        log::warn!("{}.read_all | error kind: {:?}", self.dbgid, err.kind());
        match err.kind() {
            // std::io::ErrorKind::NotFound => todo!(),
            std::io::ErrorKind::PermissionDenied => IsConnected::Closed,
            std::io::ErrorKind::ConnectionRefused => IsConnected::Closed,
            std::io::ErrorKind::ConnectionReset => IsConnected::Closed,
            // std::io::ErrorKind::HostUnreachable => ConnectionStatus::Closed,
            // std::io::ErrorKind::NetworkUnreachable => ConnectionStatus::Closed,
            std::io::ErrorKind::ConnectionAborted => IsConnected::Closed,
            std::io::ErrorKind::NotConnected => IsConnected::Closed,
            std::io::ErrorKind::AddrInUse => IsConnected::Closed,
            std::io::ErrorKind::AddrNotAvailable => IsConnected::Closed,
            // std::io::ErrorKind::NetworkDown => ConnectionStatus::Closed,
            std::io::ErrorKind::BrokenPipe => IsConnected::Closed,
            std::io::ErrorKind::AlreadyExists => todo!(),
            std::io::ErrorKind::WouldBlock => IsConnected::Closed,
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
            _ => IsConnected::Closed,
        }
    }
}