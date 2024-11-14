#[cfg(test)]

mod parse_syn {
    use std::{sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{api::message::{fields::{FieldData, FieldId, FieldKind, FieldSize, FieldSyn}, message::{Message, MessageField, MessageParse}, message_kind::MessageKind, parse_syn::ParseSyn}, debug::dbg_id::DbgId};
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
    /// Testing [ParseSyn.parse]
    #[test]
    fn parse() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let dbgid = DbgId("test".to_owned());
        log::debug!("\n{}", dbgid);
        let test_duration = TestDuration::new(&dbgid, Duration::from_secs(1));
        test_duration.run().unwrap();
        fn to_bytes(data: &str, id: u32) -> Vec<u8> {
            let data = data.as_bytes();
            let size = data.len() as u32;
            [
                Message::SYN.to_be_bytes().as_slice(),
                FieldId(id).to_be_bytes().as_slice(),
                MessageKind::String.to_bytes(),
                size.to_be_bytes().as_slice(),
                data,
            ].concat()
        }
        let test_data = [
            (
                00, vec![
                    to_bytes("123", 4294967291),
                ],
                vec![255, 255, 255, 251, 40, 0, 0, 0, 3, 49, 50, 51],
            ),
            (
                01, vec![
                    vec![000, 001, 002, 002],
                    to_bytes("23456", 4294967292)[..4].to_vec(),
                    to_bytes("23456", 4294967292)[4..].to_vec(),
                ],
                vec![255, 255, 255, 252, 40, 0, 0, 0, 5, 50, 51, 52, 53, 54],
            ),
            (
                02, vec![
                    vec![000, 001, 002, 002],
                    [&[003, 004], &to_bytes("23456", 4294967292)[..4]].concat(),
                    to_bytes("23456", 4294967292)[4..].to_vec(),
                ],
                vec![255, 255, 255, 252, 40, 0, 0, 0, 5, 50, 51, 52, 53, 54],
            ),
        ];
        let mut parse_syn = ParseSyn::new(
            &dbgid,
            FieldSyn(Message::SYN),
        );
        for (step, messages, target) in test_data {
            let mut result = vec![];
            for bytes in messages {
                match parse_syn.parse(&bytes) {
                    Ok(bytes) => {
                        log::debug!("{} | step: {},  bytes: {:?}", dbgid, step, bytes);
                        // assert!(result == target, "step: {} \nresult: {:?}\ntarget: {:?}", step, result, target);
                        result.extend_from_slice(&bytes);
                    }
                    Err(err) => {
                        log::warn!("{} | {}",dbgid, err);
                    }
                }
            }
            parse_syn.reset();
            assert!(result == target, "step: {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
}
