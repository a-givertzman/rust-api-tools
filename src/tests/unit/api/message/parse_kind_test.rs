#[cfg(test)]

mod parse_kind {
    use std::{sync::Once, time::Duration};
    use sal_core::dbg::Dbg;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::api::message::{fields::{FieldId, FieldKind, FieldSyn}, message::MessageParse, message_kind::MessageKind, parse_id::ParseId, parse_kind::ParseKind, parse_syn::ParseSyn};
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
        let dbg = Dbg::own("parse_kind");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(1));
        test_duration.run().unwrap();
        fn to_bytes(data: &str, id: u32) -> Vec<u8> {
            let data = data.as_bytes();
            let size = data.len() as u32;
            [
                FieldSyn::default().0.to_be_bytes().as_slice(),
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
                FieldId(4294967291), MessageKind::String, vec![0, 0, 0, 3, 49, 50, 51],
            ),
            (
                01, vec![
                    vec![000, 001, 002, 002],
                    to_bytes("23456", 4294967292)[..3].to_vec(),
                    to_bytes("23456", 4294967292)[3..].to_vec(),
                ],
                FieldId(4294967292), MessageKind::String, vec![0, 0, 0, 5, 50, 51, 52, 53, 54],
            ),
            (
                02, vec![
                    vec![000, 001, 002, 002],
                    [&[003, 004], &to_bytes("23456", 4294967293)[..4]].concat(),
                    to_bytes("23456", 4294967293)[4..].to_vec(),
                ],
                FieldId(4294967293), MessageKind::String, vec![0, 0, 0, 5, 50, 51, 52, 53, 54],
            ),
        ];
        let mut message = ParseKind::new(
            &dbg,
            FieldKind(MessageKind::Any),
            ParseId::new(
                &dbg,
                FieldId(4),
                ParseSyn::new(
                    &dbg,
                    FieldSyn::default(),
                ),
            ),
        );
        for (step, messages, target_id, target_kind, target_bytes) in test_data {
            let mut result_bytes = vec![];
            for bytes in messages {
                match message.parse(bytes) {
                    Ok((id, kind, bytes)) => {
                        log::debug!("{} | step: {},  id: {:?},  kind: {:?},  bytes: {:?}", dbg, step, id, kind, bytes);
                        let result = id;
                        assert!(result == target_id, "step: {} \nresult: {:?}\ntarget: {:?}", step, result, target_id);
                        let result = kind;
                        assert!(result == target_kind, "step: {} \nresult: {:?}\ntarget: {:?}", step, result, target_kind);
                        result_bytes.extend(bytes);
                    }
                    Err(err) => {
                        log::warn!("{} | {}",dbg, err);
                    }
                }
            }
            message.reset();
            assert!(result_bytes == target_bytes, "step: {} \nresult: {:?}\ntarget: {:?}", step, result_bytes, target_bytes);
        }
        test_duration.exit();
    }
}
