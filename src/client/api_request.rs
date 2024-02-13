#![allow(non_snake_case)]

use log::{info, debug, warn};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use std::{io::{Read, Write}, net::{SocketAddr, TcpStream, ToSocketAddrs}, ops::Add};

use crate::client::api_query::ApiQuery;

///
/// - Holding single input queue
/// - Received string messages pops from the queue into the end of local buffer
/// - Sending messages (wrapped into ApiQuery) from the beginning of the buffer
/// - Sent messages immediately removed from the buffer
/// ```
/// ApiRequest::new(
///    address,
///    auth_token,
///    ApiQuerySql::new(
///       database,
///       sql,
///       keep_alive,
///    ),
///    debug,
/// )
/// ```
#[derive(Serialize)]    // , Deserialize
pub struct ApiRequest {
    #[serde(skip_serializing)]
    id: String,
    #[serde(rename(serialize = "id"))]
    _id: Id,
    address: SocketAddr,
    #[serde(rename(serialize = "authToken"))]
    auth_token: String,
    query: ApiQuery,
    #[serde(rename(serialize = "keepAlive"))]
    keep_alive: bool,
    debug: bool,
}
///
/// 
impl ApiRequest {
    ///
    /// Creates new instance of [ApiRequest]
    /// - [parent] - the ID if the parent entity
    pub fn new(parent: impl Into<String>, address: impl ToSocketAddrs + std::fmt::Debug, auth_token: impl Into<String>, query: ApiQuery, keep_alive: bool, debug: bool) -> Self {
        let address = match address.to_socket_addrs() {
            Ok(mut addrIter) => {
                match addrIter.next() {
                    Some(addr) => addr,
                    None => panic!("TcpClientConnect({}).connect | Empty address found: {:?}", parent.into(), address),
                }
            },
            Err(err) => panic!("TcpClientConnect({}).connect | Address parsing error: \n\t{:?}", parent.into(), err),
        };
        Self {
            id: format!("{:03}/ApiRequest", 0),
            _id: Id { value: 0 },
            address,
            auth_token: auth_token.into(),
            query,
            keep_alive,
            debug,
        }
    }
    ///
    /// Writing sql string to the TcpStream
    pub fn fetch(&mut self, query: &ApiQuery, keep_alive: bool) -> Result<Vec<u8>, String>{
        match TcpStream::connect(self.address) {
            Ok(mut stream) => {
                info!("{}.send | connected to: \n\t{:?}", self.id, stream);
                self._id.add();
                self.query = query.clone();
                self.keep_alive = keep_alive;
                match serde_json::to_string(&self) {
                    Ok(query) => {
                        match stream.write(query.as_bytes()) {
                            Ok(_) => {
                                Self::readAll(&self.id, &mut stream)
                            },
                            Err(err) => {
                                let message = format!("{}.send | write to tcp stream error: {:?}", self.id, err);
                                warn!("{}", message);
                                Err(message)
                            },
                        }
                    },
                    Err(err) => {
                        let message = format!("{}.send | Error: {:?}", self.id, err);
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
    // ///
    // /// reads all avalible data from the TspStream
    // /// - returns Active: if read bytes non zero length without errors
    // /// - returns Closed:
    // ///    - if read 0 bytes
    // ///    - if on error
    fn readAll(selfId: &str, stream: &mut TcpStream) -> Result<Vec<u8>, String> {
        let mut buf = [0; Self::BUF_LEN];
        let mut result = vec![];
        loop {
            match stream.read(&mut buf) {
                Ok(len) => {
                    debug!("{}.readAll |     read len: {:?}", selfId, len);
                    result.append(& mut buf[..len].into());
                    if len < Self::BUF_LEN {
                        if len == 0 {
                            return Err(format!("{}.readAll | tcp stream closed", selfId));
                        } else {
                            return Ok(result)
                        }
                    }
                },
                Err(err) => {
                    warn!("{}.readAll | error reading from socket: {:?}", selfId, err);
                    warn!("{}.readAll | error kind: {:?}", selfId, err.kind());
                    let status = Err(format!("{}.readAll | tcp stream error: {:?}", selfId, err));
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


#[derive(Clone, Serialize)]    // , Deserialize
struct Id {
    value: usize,
}
impl Id {
    pub fn add(&mut self) {
        self.value += 1;
    }
}
impl Into<String> for Id {
    fn into(self) -> String {
        format!("{:03}", self.value)
    }
}
impl Into<usize> for Id {
    fn into(self) -> usize {
        self.value
    }
}
impl From<usize> for Id {
    fn from(value: usize) -> Self {
        Id { value }
    }
}
// impl Serialize for Id {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
//     S: Serializer, {
//         state.serialize_field("message", &self.message)?;
//         if self.debug {
//             let mut state = serializer.serialize_struct("ApiError", 2)?;
//             state.serialize_field("details", &self.details)?;
//             state.end()
//         } else {
//             let mut state = serializer.serialize_struct("ApiError", 1)?;
//             state.serialize_field("message", &self.message)?;
//             state.end()
//         }
//         // 3 is the number of fields in the struct.
//     }
// }
