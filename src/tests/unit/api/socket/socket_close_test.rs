#[cfg(test)]

mod socket_close {
    use std::{io::{BufReader, Read}, net::{TcpListener, TcpStream}, sync::{atomic::{AtomicBool, Ordering}, Arc, Once}, thread, time::{Duration, Instant}};
    use testing::{session::test_session::TestSession, stuff::max_test_duration::TestDuration};
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{api::{message::{fields::{FieldData, FieldId, FieldKind, FieldSize, FieldSyn}, message::MessageField, message_kind::MessageKind, parse_data::ParseData, parse_id::ParseId, parse_kind::ParseKind, parse_size::ParseSize, parse_syn::ParseSyn}, socket::tcp_socket::{TcpMessage, TcpSocket}}, debug::dbg_id::DbgId, error::str_err::StrErr};
    ///
    /// inline increment
    trait Inc<T> {
        fn inc(&mut self) -> T where Self: std::ops::AddAssign<T>;
    }
    impl Inc<usize> for usize {
        fn inc(&mut self) -> usize where Self: std::ops::AddAssign<usize> {
            *self += 1;
            *self
        }
    }
    ///
    ///
    static INIT: Once = Once::new();
    ///
    /// once called initialisation
    fn init_once() {
        INIT.call_once(|| {
            // implement your initialisation code to be called only once for current test file
        })
    }
    ///
    /// returns:
    ///  - ...
    fn init_each() -> () {}
    ///
    /// Testing Socket read timeout
    /// - research test
    #[test]
    fn tcp_stream_close() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbgid = DbgId("test".to_owned());
        log::debug!("\n{}", dbgid);
        let test_duration = TestDuration::new(&dbgid, Duration::from_secs(24));
        test_duration.run().unwrap();
        let timeout = Duration::from_secs(1);
        let addr = &format!("0.0.0.0:{}", TestSession::free_tcp_port_int());
        let mut step: usize = 0;
        let exit = Arc::new(AtomicBool::new(false));
        let result = match TcpListener::bind(addr) {
            Ok(tcp_listener) => {
                let dbgid = dbgid.clone();
                // let exit = exit.clone();
                let _ = thread::spawn(move || {
                    match tcp_listener.accept() {
                        Ok((_, addr)) => {
                            log::debug!("{}.server | Connection accepted from: {:?}", dbgid, addr);
                            thread::sleep(Duration::from_secs(3));
                            // while !exit.load(Ordering::SeqCst) {
                            //     thread::sleep(Duration::from_millis(10));
                            // }
                        }
                        Err(err) => {
                            panic!("{}.server | Accept Error: {:?}", dbgid, err);
                        },
                    };
                });
                Ok(())
            }
            Err(err) => {
                let err = format!("{} | Error: {:?}", dbgid, err);
                log::error!("{}", err);
                Err(StrErr(err))
            }
        };
        log::debug!("{}.server | Bind result: {:?}", dbgid, result);
        assert!(result.is_ok(), "step {} \nresult: {:?}\ntarget: {:?}", step.inc(), result.is_ok(), true);
        match TcpStream::connect(addr) {
            Ok(socket) => {
                log::debug!("{} | Connected on: {:?}", dbgid, socket.peer_addr());
                if let Err(err) = socket.set_read_timeout(Some(timeout)) {
                    let message = format!("{}.connect | set_read_timeout error: \n\t{:?}", dbgid, err);
                    log::warn!("{}", message);
                }
                if let Err(err) = socket.set_write_timeout(Some(timeout)) {
                    let message = format!("{}.connect | set_write_timeout error: \n\t{:?}", dbgid, err);
                    log::warn!("{}", message);
                }
                let socket = Arc::new(socket);
                let mut stream = BufReader::new(socket.as_ref());
                let mut buf = vec![0; 1024];
                let time = Instant::now();
                while !exit.load(Ordering::SeqCst) {
                    let result = stream.read(&mut buf);
                    log::debug!("{} | Read done in: {:?}", dbgid, time.elapsed());
                    log::debug!("{} | Read result: {:?}", dbgid, result);
                    match result {
                        Ok(len) => {
                            if len == 0 {
                                log::debug!("{} | Connection closed", dbgid);
                                exit.store(true, Ordering::SeqCst);
                            }
                        }
                        Err(err) => match err.kind() {
                            std::io::ErrorKind::NotFound => todo!(),
                            std::io::ErrorKind::PermissionDenied => todo!(),
                            std::io::ErrorKind::ConnectionRefused => todo!(),
                            std::io::ErrorKind::ConnectionReset => todo!(),
                            // std::io::ErrorKind::HostUnreachable => todo!(),
                            // std::io::ErrorKind::NetworkUnreachable => todo!(),
                            std::io::ErrorKind::ConnectionAborted => todo!(),
                            std::io::ErrorKind::NotConnected => todo!(),
                            std::io::ErrorKind::AddrInUse => todo!(),
                            std::io::ErrorKind::AddrNotAvailable => todo!(),
                            // std::io::ErrorKind::NetworkDown => todo!(),
                            std::io::ErrorKind::BrokenPipe => todo!(),
                            std::io::ErrorKind::AlreadyExists => todo!(),
                            std::io::ErrorKind::WouldBlock => {
                                log::debug!("{} | Read timeout", dbgid);
                            },
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
                            _ => todo!(),
                        }
                    }
                }
            }
            Err(err) => {
                exit.store(true, Ordering::SeqCst);
                panic!("{} | Connect Error: {:?}", dbgid, err);
            }
        }
        exit.store(true, Ordering::SeqCst);
        test_duration.exit();
    }
    ///
    /// Testing Socket read timeout
    /// - research test
    #[test]
    fn tcp_socket_close() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbgid = DbgId("test".to_owned());
        log::debug!("\n{}", dbgid);
        let test_duration = TestDuration::new(&dbgid, Duration::from_secs(24));
        test_duration.run().unwrap();
        let addr = &format!("0.0.0.0:{}", TestSession::free_tcp_port_int());
        let mut step: usize = 0;
        let exit = Arc::new(AtomicBool::new(false));
        let result = match TcpListener::bind(addr) {
            Ok(tcp_listener) => {
                let dbgid = dbgid.clone();
                // let exit = exit.clone();
                let _ = thread::spawn(move || {
                    match tcp_listener.accept() {
                        Ok((_, addr)) => {
                            log::debug!("{}.server | Connection accepted from: {:?}", dbgid, addr);
                            thread::sleep(Duration::from_secs(3));
                            // while !exit.load(Ordering::SeqCst) {
                            //     thread::sleep(Duration::from_millis(10));
                            // }
                        }
                        Err(err) => {
                            panic!("{}.server | Accept Error: {:?}", dbgid, err);
                        },
                    };
                });
                Ok(())
            }
            Err(err) => {
                let err = format!("{} | Error: {:?}", dbgid, err);
                log::error!("{}", err);
                Err(StrErr(err))
            }
        };
        log::debug!("{}.server | Bind result: {:?}", dbgid, result);
        assert!(result.is_ok(), "step {} \nresult: {:?}\ntarget: {:?}", step.inc(), result.is_ok(), true);
        let mut socket = TcpSocket::new(
            &dbgid,
            addr,
            TcpMessage::new(
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
            ),
            None,
        );
        match socket.connect() {
            Ok(_) => {
                // log::debug!("{} | Connected on: {:?}", dbgid, socket.peer_addr());
                let time = Instant::now();
                while !exit.load(Ordering::SeqCst) {
                    let result = socket.read();
                    log::debug!("{} | Read done in: {:?}", dbgid, time.elapsed());
                    log::debug!("{} | Read result: {:?}", dbgid, result);
                    match result {
                        Ok(_) => {
                        }
                        Err(err) => {
                            log::debug!("{} | Connection closed, error: {}", dbgid, err);
                            exit.store(true, Ordering::SeqCst);
                        }
                    }
                }
            }
            Err(err) => {
                exit.store(true, Ordering::SeqCst);
                panic!("{} | Connect Error: {:?}", dbgid, err);
            }
        }
        exit.store(true, Ordering::SeqCst);
        test_duration.exit();
    }
}
