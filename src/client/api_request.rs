use log::{debug, info, trace, warn};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use std::{io::{Read, Write}, net::{SocketAddr, TcpStream, ToSocketAddrs}};

use crate::{api::message::{fields::{FieldData, FieldKind, FieldSize, FieldSyn}, message::{Message, MessageFild}, message_kind::MessageKind}, client::api_query::ApiQuery, error::str_err::StrErr};

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
#[derive(Debug)]    // , Deserialize
pub struct ApiRequest {
    id: String,
    query_id: Id,
    address: SocketAddr,
    auth_token: String,
    query: ApiQuery,
    keep_alive: bool,
    debug: bool,
    connection: Option<(TcpStream, Message)>,
}
///
/// 
impl ApiRequest {
    ///
    /// Creates new instance of [ApiRequest]
    /// - [parent] - the ID if the parent entity
    pub fn new(parent: impl Into<String>, address: impl ToSocketAddrs + std::fmt::Debug, auth_token: impl Into<String>, query: ApiQuery, keep_alive: bool, debug: bool) -> Self {
        let address = match address.to_socket_addrs() {
            Ok(mut addr_iter) => {
                match addr_iter.next() {
                    Some(addr) => addr,
                    None => panic!("TcpClientConnect({}).connect | Empty address found: {:?}", parent.into(), address),
                }
            },
            Err(err) => panic!("TcpClientConnect({}).connect | Address parsing error: \n\t{:?}", parent.into(), err),
        };
        Self {
            id: format!("{}/ApiRequest", parent.into()),
            query_id: Id::new(),
            address,
            auth_token: auth_token.into(),
            query,
            keep_alive,
            debug,
            connection: None,
        }
    }
    ///
    /// 
    fn connect(&mut self) -> Result<(TcpStream, Message), String> {
        match self.connection.take() {
            Some(connection) => {
                Ok(connection)
            },
            None => {
                match TcpStream::connect(self.address) {
                    Ok(stream) => {
                        debug!("{}.connect | connected to: \n\t{:?}", self.id, stream);
                        let message = Message::new(&[
                            MessageFild::Syn(FieldSyn(Message::SYN)),
                            MessageFild::Kind(FieldKind(MessageKind::String)),
                            MessageFild::Size(FieldSize(4)),
                            MessageFild::Data(FieldData(vec![]))
                        ]);                        
                        Ok((stream, message))
                    },
                    Err(err) => {
                        let message = format!("{}.connect | Connection error: \n\t{:?}", self.id, err);
                        warn!("{}", message);
                        Err(message)
                    }
                }
            },
        }
    }
    ///
    /// Performs an API request with the parameters specified in the constructor
    pub fn fetch(&mut self, keep_alive: bool) -> Result<Vec<u8>, String>{
        match self.connect() {
            Ok((mut stream, mut message)) => {
                self.query_id.add();
                self.keep_alive = keep_alive;
                match serde_json::to_string(&self) {
                    Ok(query) => {
                        trace!("{}.fetch | query: \n\t{:?}", self.id, query);
                        let bytes = message.build(query.as_bytes());
                        match stream.write(&bytes) {
                            Ok(_) => {
                                self.read_all(stream, message)
                            },
                            Err(err) => {
                                let message = format!("{}.fetch | write to tcp stream error: {:?}", self.id, err);
                                warn!("{}", message);
                                Err(message)
                            },
                        }
                    },
                    Err(err) => {
                        let message = format!("{}.fetch | Serialize error: {:?}", self.id, err);
                        warn!("{}", message);
                        Err(message)
                    },
                }
            },
            Err(err) => Err(err)
        }
    }
    ///
    /// Performs an API request with passed query and parameters specified in the constructor
    pub fn fetch_with(&mut self, query: &ApiQuery, keep_alive: bool) -> Result<Vec<u8>, String>{
        match self.connect() {
            Ok(mut stream) => {
                self.query_id.add();
                self.query = query.clone();
                self.keep_alive = keep_alive;
                match serde_json::to_string(&self) {
                    Ok(query) => {
                        trace!("{}.fetch_with | query: \n\t{:?}", self.id, query);
                        match stream.write(query.as_bytes()) {
                            Ok(_) => {
                                self.read_all(stream)
                            },
                            Err(err) => {
                                let message = format!("{}.fetch_with | write to tcp stream error: {:?}", self.id, err);
                                warn!("{}", message);
                                Err(message)
                            },
                        }
                    },
                    Err(err) => {
                        let message = format!("{}.fetch_with | Serialize error: {:?}", self.id, err);
                        warn!("{}", message);
                        Err(message)
                    },
                }
            },
            Err(err) => Err(err)
        }
    }
    ///
    /// 
    fn read_message(&mut self, mut stream: TcpStream, mut message: Message) -> Result<Vec<u8>, StrErr> {
        let mut buf = [0; Self::BUF_LEN];
        loop {
            match stream.read(&mut buf) {
                Ok(len) => {
                    trace!("{}.read_all |     read len: {:?}", self.id, len);
                    match message.parse(&buf[..len]) {
                        Ok(parsed) => match parsed.as_slice() {
                            [ MessageFild::Kind(kind), MessageFild::Size(FieldSize(size)), MessageFild::Data(FieldData(data)) ] => {
                                log::debug!("{}.read_message | kind: {:?},  size: {},  data: {:?}", self.id, kind, size, data);
                                return Ok(data.to_owned())
                            }
                            [..] => {
                                let err = format!("{} | Unknown message kind {:?}", self.id, parsed);
                                log::warn!("{}", err);
                                return Err(err.into())
                            }
                        }
                        Err(_) => {},
                    };
                    if len < Self::BUF_LEN {
                        if len == 0 {
                            return Err(format!("{}.read_message | tcp stream closed", self.id).into());
                        }
                    }
                },
                Err(err) => {
                    self.parse_err(err);
                }
            };
        }
    }
    ///
    /// bytes to be read from socket at once
    const BUF_LEN: usize = 1024 * 1;
    // ///
    // /// reads all avalible data from the TspStream
    // /// - returns Active: if read bytes non zero length without errors
    // /// - returns Closed:
    // ///    - if read 0 bytes
    // ///    - if on error
    fn read_all(&mut self, mut stream: TcpStream, message: Message) -> Result<Vec<u8>, String> {
        let mut buf = [0; Self::BUF_LEN];
        let mut result = vec![];
        loop {
            match stream.read(&mut buf) {
                Ok(len) => {
                    trace!("{}.read_all |     read len: {:?}", self.id, len);
                    result.append(& mut buf[..len].into());
                    if len < Self::BUF_LEN {
                        if len == 0 {
                            return Err(format!("{}.read_all | tcp stream closed", self.id));
                        } else {
                            if self.keep_alive {
                                self.connection.replace((stream, message));
                            }
                            return Ok(result)
                        }
                    }
                },
                Err(err) => {
                    self.parse_err(err);
                }
            };
        }
    }
    ///
    /// 
    fn parse_err(&self, err: std::io::Error) -> Result<Vec<u8>, String>{
        warn!("{}.read_all | error reading from socket: {:?}", self.id, err);
        warn!("{}.read_all | error kind: {:?}", self.id, err.kind());
        let status = Err(format!("{}.read_all | tcp stream error: {:?}", self.id, err));
        match err.kind() {
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
    }
}
//
//
impl Serialize for ApiRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer {
        let mut state = serializer.serialize_struct("ApiRequest", 2)?;
        state.serialize_field("id", &self.query_id)?;
        state.serialize_field("authToken", &self.auth_token)?;
        state.serialize_field("keepAlive", &self.keep_alive)?;
        state.serialize_field("debug", &self.debug)?;
        match &self.query.query {
            super::api_query::ApiQueryKind::Sql(query) => {
                state.serialize_field("sql", query)?;
            },
            super::api_query::ApiQueryKind::Python(query) => {
                state.serialize_field("python", query)?;
            },
            super::api_query::ApiQueryKind::Executable(query) => {
                state.serialize_field("executable", query)?;
            },
        };
        state.end()
    }
}
///
/// 
#[derive(Debug)]
struct Id {
    value: usize,
}
impl Id {
    pub fn new() -> Self { Self { value: 0 } }
    pub fn add(&mut self) {
        self.value += 1;
    }
}
impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
    S: Serializer, {
        serializer.serialize_str(&self.value.to_string())
    }
}
// impl Into<usize> for Id {
//     fn into(self) -> usize {
//         self.value
//     }
// }
// impl From<usize> for Id {
//     fn from(value: usize) -> Self {
//         Id { value }
//     }
// }
