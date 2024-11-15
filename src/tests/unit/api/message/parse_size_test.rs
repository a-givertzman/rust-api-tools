#[cfg(test)]

mod parse_size {
    use std::{sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{api::message::{fields::{FieldId, FieldKind, FieldSize, FieldSyn}, message::MessageParse, message_kind::MessageKind, parse_id::ParseId, parse_kind::ParseKind, parse_size::ParseSize, parse_syn::ParseSyn}, debug::dbg_id::DbgId};
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
                FieldId(4294967291), MessageKind::String, FieldSize(3), vec![49, 50, 51],
            ),
            (
                01, vec![
                    vec![000, 001, 002, 002],
                    to_bytes("23456", 4294967292)[..3].to_vec(),
                    to_bytes("23456", 4294967292)[3..].to_vec(),
                ],
                FieldId(4294967292), MessageKind::String, FieldSize(5), vec![50, 51, 52, 53, 54],
            ),
            (
                02, vec![
                    vec![000, 001, 002, 002],
                    [&[003, 004], &to_bytes("23456", 4294967293)[..4]].concat(),
                    to_bytes("23456", 4294967293)[4..].to_vec(),
                ],
                FieldId(4294967293), MessageKind::String, FieldSize(5), vec![50, 51, 52, 53, 54],
            ),
            (
                03, vec![
                    vec![011, 012, 013, 014],
                    to_bytes("2345678", 4294967294)[..3].to_vec(),
                    to_bytes("2345678", 4294967294)[3..4].to_vec(),
                    to_bytes("2345678", 4294967294)[4..6].to_vec(),
                    to_bytes("2345678", 4294967294)[6..].to_vec(),
                ],
                FieldId(4294967294), MessageKind::String, FieldSize(7), vec![50, 51, 52, 53, 54, 55, 56],
            ),
            (
                04, vec![
                    vec![111, 112, 113, 114],
                    to_bytes("234567890", 4294967295)[..3].to_vec(),
                    to_bytes("234567890", 4294967295)[3..4].to_vec(),
                    to_bytes("234567890", 4294967295)[4..7].to_vec(),
                    to_bytes("234567890", 4294967295)[7..].to_vec(),
                ],
                FieldId(4294967295), MessageKind::String, FieldSize(9), vec![50, 51, 52, 53, 54, 55, 56, 57, 48],
            ),
        ];
        let mut message = ParseSize::new(
            &dbgid,
            FieldSize(4),
            ParseKind::new(
                &dbgid,
                FieldKind(MessageKind::Any),
                ParseId::new(
                    &dbgid,
                    FieldId(4),
                    ParseSyn::new(
                        &dbgid,
                        FieldSyn::default(),
                    ),
                ),
            ),
        );
        for (step, messages, target_id, target_kind, target_size, target_bytes) in test_data {
            let mut result_bytes = vec![];
            for bytes in messages {
                match message.parse(bytes) {
                    Ok((id, kind, size, bytes)) => {
                        log::debug!("{} | step: {},  id: {:?},  kind: {:?},  size: {:?},  bytes: {:?}", dbgid, step, id, kind, size, bytes);
                        let result = id;
                        assert!(result == target_id, "step: {} \nresult: {:?}\ntarget: {:?}", step, result, target_id);
                        let result = kind;
                        assert!(result == target_kind, "step: {} \nresult: {:?}\ntarget: {:?}", step, result, target_kind);
                        let result = size;
                        assert!(result == target_size, "step: {} \nresult: {:?}\ntarget: {:?}", step, result, target_size);
                        result_bytes.extend(bytes);
                    }
                    Err(err) => {
                        log::warn!("{} | {}",dbgid, err);
                    }
                }
            }
            message.reset();
            assert!(result_bytes == target_bytes, "step: {} \nresult: {:?}\ntarget: {:?}", step, result_bytes, target_bytes);
        }
        test_duration.exit();
    }
}
