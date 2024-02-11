#![allow(non_snake_case)]

use log::{info, debug, trace, warn};
use std::{collections::HashMap, io::{Read, Write}, net::{SocketAddr, TcpStream, ToSocketAddrs}, sync::{atomic::{AtomicBool, Ordering}, mpsc::{self, Receiver, Sender}, Arc}, thread::{self, JoinHandle}, time::Duration};

use crate::client::api_query::ApiQuery;

///
/// - Holding single input queue
/// - Received string messages pops from the queue into the end of local buffer
/// - Sending messages (wrapped into ApiQuery) from the beginning of the buffer
/// - Sent messages immediately removed from the buffer
pub struct ApiRequest {
    id: String,
    addr: SocketAddr,
    query: ApiQuery,
}
///
/// 
impl ApiRequest {
    ///
    /// Creates new instance of [ApiRequest]
    /// - [parent] - the ID if the parent entity
    pub fn new(parent: impl Into<String>, query: ApiQuery, addr: impl ToSocketAddrs + std::fmt::Debug) -> Self {
        let addr = match addr.to_socket_addrs() {
            Ok(mut addrIter) => {
                match addrIter.next() {
                    Some(addr) => addr,
                    None => panic!("TcpClientConnect({}).connect | Empty address found: {:?}", parent.into(), addr),
                }
            },
            Err(err) => panic!("TcpClientConnect({}).connect | Address parsing error: \n\t{:?}", parent.into(), err),
        };

        Self {
            id: format!("{}/ApiRequest", parent.into()),
            addr,
            query,
        }
    }
    ///
    /// Writing sql string to the TcpStream
    fn send(&self, sql: String) -> Result<(), String>{
        match TcpStream::connect(self.addr) {
            Ok(mut stream) => {
                info!("{}.send | connected to: \n\t{:?}", self.id, stream);
                let query = ApiQuery::new("authToken", "id", "database", sql, true, true);
                match stream.write(query.toJson().as_bytes()) {
                    Ok(_) => Ok(()),
                    Err(err) => {
                        let message = format!("{}.send | write to tcp stream error: {:?}", self.id, err);
                        warn!("{}", message);
                        Err(message)
                    },
                }
            },
            Err(err) => {
                let message = format!("{}.send | Connection error: \n\t{:?}", self.id, err);
                warn!("{}", message);
                Err(message)
            }
        }
    }
    ///
    /// bytes to be read from socket at once
    const BUF_LEN: usize = 1024 * 4;
    ///
    /// reads all avalible data from the TspStream
    /// - returns Active: if read bytes non zero length without errors
    /// - returns Closed:
    ///    - if read 0 bytes
    ///    - if on error
    fn readAll(selfId: &str, stream: &mut TcpStream) -> ConnectionStatus<Vec<u8>, String> {
        let mut buf = [0; Self::BUF_LEN];
        let mut result = vec![];
        loop {
            match stream.read(&mut buf) {
                Ok(len) => {
                    debug!("{}.readAll |     read len: {:?}", selfId, len);
                    result.append(& mut buf[..len].into());
                    if len < Self::BUF_LEN {
                        if len == 0 {
                            return ConnectionStatus::Closed(format!("{}.readAll | tcp stream closed", selfId));
                        } else {
                            return ConnectionStatus::Active(result)
                        }
                    }
                },
                Err(err) => {
                    warn!("{}.readAll | error reading from socket: {:?}", selfId, err);
                    warn!("{}.readAll | error kind: {:?}", selfId, err.kind());
                    let status = ConnectionStatus::Closed(format!("{}.readAll | tcp stream error: {:?}", selfId, err));
                    return match err.kind() {
                        std::io::ErrorKind::NotFound => status,
                        std::io::ErrorKind::PermissionDenied => status,
                        std::io::ErrorKind::ConnectionRefused => status,
                        std::io::ErrorKind::ConnectionReset => status,
                        // std::io::ErrorKind::HostUnreachable => status,
                        // std::io::ErrorKind::NetworkUnreachable => status,
                        std::io::ErrorKind::ConnectionAborted => status,
                        std::io::ErrorKind::NotConnected => status,
                        std::io::ErrorKind::AddrInUse => status,
                        std::io::ErrorKind::AddrNotAvailable => status,
                        // std::io::ErrorKind::NetworkDown => status,
                        std::io::ErrorKind::BrokenPipe => status,
                        std::io::ErrorKind::AlreadyExists => status,
                        std::io::ErrorKind::WouldBlock => status,
                        // std::io::ErrorKind::NotADirectory => todo!(),
                        // std::io::ErrorKind::IsADirectory => todo!(),
                        // std::io::ErrorKind::DirectoryNotEmpty => todo!(),
                        // std::io::ErrorKind::ReadOnlyFilesystem => todo!(),
                        // std::io::ErrorKind::FilesystemLoop => todo!(),
                        // std::io::ErrorKind::StaleNetworkFileHandle => todo!(),
                        std::io::ErrorKind::InvalidInput => status,
                        std::io::ErrorKind::InvalidData => status,
                        std::io::ErrorKind::TimedOut => status,
                        std::io::ErrorKind::WriteZero => status,
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
                        std::io::ErrorKind::Interrupted => status,
                        std::io::ErrorKind::Unsupported => status,
                        std::io::ErrorKind::UnexpectedEof => status,
                        std::io::ErrorKind::OutOfMemory => status,
                        std::io::ErrorKind::Other => status,
                        _ => status,
                    }
                },
            };
        }
    }    
}
