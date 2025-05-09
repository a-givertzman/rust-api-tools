use sal_core::{dbg::Dbg, error::Error};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use std::{net::ToSocketAddrs, time::Duration};
use crate::{
    api::{
        message::{
            fields::{FieldData, FieldId, FieldKind, FieldSize, FieldSyn},
            message::MessageField, message_kind::MessageKind, msg_kind,
            parse_data::ParseData, parse_id::ParseId, parse_kind::ParseKind,
            parse_size::ParseSize, parse_syn::ParseSyn
        },
        socket::tcp_socket::{TcpMessage, TcpSocket},
    },
    client::api_query::ApiQuery,
};
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
#[derive(Debug)]
pub struct ApiRequest {
    dbg: Dbg,
    query_id: Id,
    auth_token: String,
    query: ApiQuery,
    keep_alive: bool,
    debug: bool,
    timeout: Duration,
    socket: TcpSocket,
}
//
//
impl ApiRequest {
    ///
    /// Creates new instance of [ApiRequest]
    /// - [parent] - the ID if the parent entity
    pub fn new(parent: impl Into<String>, address: impl ToSocketAddrs + std::fmt::Debug, auth_token: impl Into<String>, query: ApiQuery, keep_alive: bool, debug: bool) -> Self {
        let dbgid = Dbg::new(parent, "ApiRequest");
        let address = match address.to_socket_addrs() {
            Ok(mut addr_iter) => match addr_iter.next() {
                Some(addr) => addr,
                None => panic!("TcpClientConnect({}).connect | Empty address: {:?}", dbgid, address),
            },
            Err(err) => panic!("TcpClientConnect({}).connect | Address error: {:#?}", dbgid, err),
        };
        let message = TcpMessage::new(
            &dbgid,
            vec![
                MessageField::Syn(FieldSyn::default()),
                MessageField::Id(FieldId(4)),
                MessageField::Kind(FieldKind(MessageKind::Bytes)),
                MessageField::Size(FieldSize(4)),
                MessageField::Data(FieldData(vec![]))
            ],
            ParseData::new(
                &dbgid,
                ParseSize::new(
                    &dbgid,
                    FieldSize(4),
                    ParseKind::new(
                        &dbgid,
                        FieldKind(MessageKind::Bytes),
                        ParseId::new(
                            &dbgid,
                            FieldId(4),
                            ParseSyn::new(
                                &dbgid,
                                FieldSyn::default(),
                            ),
                        ),
                    ),
                ),
            ),
        );
        Self {
            socket: TcpSocket::new(&dbgid, address, message, None),
            dbg: dbgid,
            query_id: Id::new(),
            auth_token: auth_token.into(),
            query,
            keep_alive,
            debug,
            timeout: Duration::from_secs(10),
        }
    }
    ///
    /// Returns [ApiRequest] with specified socket read/write timeout (default 10 sec)
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
    ///
    /// Performs an API request with the parameters specified in the constructor
    pub fn fetch(&mut self, keep_alive: bool) -> Result<Vec<u8>, Error> {
        self.fetch_with(&self.query.clone(), keep_alive)
            .map_err(|err| Error::new(&self.dbg, "fetch").pass(err))
    }
    ///
    /// Performs an API request with passed query and parameters specified in the constructor
    pub fn fetch_with(&mut self, query: &ApiQuery, keep_alive: bool) -> Result<Vec<u8>, Error>{
        let error = Error::new(&self.dbg, "fetch_with");
        self.query_id.add();
        self.query = query.clone();
        self.keep_alive = keep_alive;
        match serde_json::to_vec(&self) {
            Ok(query) => {
                log::trace!("{}.fetch | query: {:#?}", self.dbg, query);
                match self.socket.send(&query, None) {
                    Ok(_id) => {
                        match self.socket.read() {
                            Ok((_id, msg)) => match msg {
                                msg_kind::MsgKind::Bytes(bytes) =>  Ok(bytes),
                                _ => {
                                    let err = error.err(format!("Wrong Message kind error, expected Bytes, but found: {:?}", msg));
                                    log::warn!("{}", err);
                                    Err(err)
                                }
                            }
                            Err(err) => Err(error.pass(err)),
                        }
                    }
                    Err(err) => {
                        let err = error.pass_with("Send error", err);
                        log::warn!("{}", err);
                        Err(err)
                    }
                }
            }
            Err(err) => {
                let err = error.pass_with("Serialize  error", err.to_string());
                log::warn!("{}", err);
                Err(err)
            }
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
        self.value = (self.value % usize::MAX) + 1;
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
