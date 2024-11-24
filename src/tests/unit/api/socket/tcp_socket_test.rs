#[cfg(test)]

mod tcp_socket {
    use std::{io::{Read, Write}, net::TcpListener, sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc, Once}, thread, time::Duration};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::{session::{teardown::Teardown, test_session::TestSession}, stuff::max_test_duration::TestDuration};
    use crate::{
        api::{
            message::{
                fields::{FieldData, FieldId, FieldKind, FieldSize, FieldSyn}, message::MessageField, message_kind::_MessageKind, msg_kind::MsgKind, parse_data::ParseData, parse_id::ParseId, parse_kind::ParseKind, parse_size::ParseSize, parse_syn::ParseSyn
            },
            reply::api_reply::ApiReply, socket::tcp_socket::{TcpMessage, TcpSocket},
        },
        debug::dbg_id::DbgId, error::str_err::StrErr,
    };
    ///    
    static INIT: Once = Once::new();
    static TEARDOWN_COUNT: AtomicUsize = AtomicUsize::new(0);
    ///
    /// once called initialisation
    fn init_once() {
        INIT.call_once(|| {
                // implement your initialisation code to be called only once for current test file
        });
    }
    ///
    /// Once called after all tests
    fn teardown_once() {
    }
    ///
    /// returns:
    ///  - ...
    fn init_each() -> () {}
    ///
    /// Builds a valid socket message
    fn to_bytes(data: &str, id: u32) -> Vec<u8> {
        let data = data.as_bytes();
        let size = data.len() as u32;
        [
            FieldSyn::default().0.to_be_bytes().as_slice(),
            FieldId(id).to_be_bytes().as_slice(),
            _MessageKind::String.to_bytes(),
            size.to_be_bytes().as_slice(),
            data,
        ].concat()
    }
    ///
    /// Testing TcpSocket messaging
    #[test]
    fn read_write() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        println!("");
        let dbgid = DbgId("test TcpSocket".into());
        println!("{}", dbgid);
        let test_duration = TestDuration::new(&dbgid, Duration::from_secs(120));
        test_duration.run().unwrap();
        let _teardown_once = || {
            teardown_once();
        };
        let _teardown = Teardown::new(&TEARDOWN_COUNT, &|| {}, &_teardown_once);
        let port = TestSession::free_tcp_port_str();
        let addr = format!("0.0.0.0:{}", port);
        let test_data = [
            (
                00, "123".as_bytes().to_vec(),
                MsgKind::Bytes(vec![49, 50, 51]),
            ),
            (
                01, "23456".as_bytes().to_vec(),
                MsgKind::Bytes(vec![50, 51, 52, 53, 54]),
            ),
            (
                02, "2345678".as_bytes().to_vec(),
                MsgKind::Bytes(vec![50, 51, 52, 53, 54, 55, 56]),
            ),
            (
                04, "2345678901234567890".as_bytes().to_vec(),
                MsgKind::Bytes("2345678901234567890".as_bytes().to_vec()),
            ),
        ];
        let mut socket = TcpSocket::new(
            &dbgid,
            &addr,
            TcpMessage::new(
                &dbgid,
                vec![
                    MessageField::Syn(FieldSyn::default()),
                    MessageField::Id(FieldId(4)),
                    MessageField::Kind(FieldKind(_MessageKind::Bytes)),
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
                            FieldKind(_MessageKind::Bytes),
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
            ),
            None,
        );
        let exit = Arc::new(AtomicBool::new(false));
        server(&addr, exit);
        thread::sleep(Duration::from_secs(1));
        for (step, message, target) in test_data {
            match socket.send(&message) {
                Ok(target_id) => {
                    log::debug!("{} | step {}  Sent | id: {:?}", dbgid, step, target_id);
                    match socket.read() {
                        Ok((id, kind)) => {
                            match &kind {
                                MsgKind::Bytes(data) => {
                                    log::debug!("{} | step {} Recv | id: {:?} kind: {:?}", dbgid, step, id, &kind);
                                    let result = id;
                                    assert!(result == target_id, "step: {} \nresult: {:?}\ntarget: {:?}", step, result, target_id);
                                    let result = kind.clone();
                                    assert!(result == target, "step: {} \nresult: {:?}\ntarget: {:?}", step, result, target);
                                    let result = data.to_vec();
                                    let target = message;
                                    assert!(result == target, "step: {} \nresult: {:?}\ntarget: {:?}", step, result, target);
                                }
                                _ => panic!("{} | step {},  Unexpected kind: {:?}", dbgid, step, kind),
                            }
                        }
                        Err(err) => {
                            panic!("{} | step {},  Error: {:?}", dbgid, step, err);
                        }
                    }
                },
                Err(err) => {
                    panic!("{} | step {},  Error: {:?}", dbgid, step, err);
                },
            };
        }
        test_duration.exit();
    }
    ///
    /// Server side
    fn server(addr: &str, exit: Arc<AtomicBool>) {
        let dbgid = DbgId("Server".to_owned());
        let addr = addr.to_owned();
        let handle = thread::Builder::new().name(format!("{}.run", &dbgid)).spawn(move || {
            let result = match TcpListener::bind(addr) {
                Ok(socket) => {
                    match socket.accept() {
                        Ok((mut stream, addr)) => {
                            log::debug!("{}.run | connection: {:?}", dbgid, addr);
                            stream.set_read_timeout(Some(Duration::from_secs(3))).unwrap();
                            let mut buf = vec![0; 4096];
                            loop {
                                let len = stream.read(&mut buf).unwrap();
                                log::debug!("{}.run | Received: {:?}", dbgid, &buf[..len]);
                                stream.write_all(&mut buf[..len]).unwrap();
                                if exit.load(Ordering::SeqCst) {
                                    break;
                                }
                            }
                            log::debug!("{}.run | Exit", dbgid);
                            Ok(())
                        }
                        Err(err) => Err(err),
                    }
                }
                Err(err) => Err(err),
            };
            assert!(result.is_ok(), "\n result: {:?}\n target: {:?}", result, Ok::<(), StrErr>(()));
        });

    }

}