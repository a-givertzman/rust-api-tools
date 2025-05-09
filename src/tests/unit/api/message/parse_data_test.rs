#[cfg(test)]

mod parse_data {
    use std::{sync::Once, time::Duration};
    use sal_core::dbg::Dbg;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::api::message::{fields::{FieldId, FieldKind, FieldSize, FieldSyn}, message::MessageParse, message_kind::MessageKind, parse_data::ParseData, parse_id::ParseId, parse_kind::ParseKind, parse_size::ParseSize, parse_syn::ParseSyn};
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
    fn parse1() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let dbg = Dbg::own("parse_data");
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
                    to_bytes("234567890", 4294967295)[7..16].to_vec(),
                    to_bytes("234567890", 4294967295)[16..].to_vec(),
                ],
                FieldId(4294967295), MessageKind::String, FieldSize(9), vec![50, 51, 52, 53, 54, 55, 56, 57, 48],
            ),
            (
                05, vec![
                    vec![221, 222, 223, 224],
                    [to_bytes("567890123455", 5), to_bytes("567890123456", 6)].concat(),
                ],
                FieldId(5), MessageKind::String, FieldSize(12), vec![53, 54, 55, 56, 57, 48, 49, 50, 51, 52, 53, 53],
            ),
            (
                06, vec![
                    vec![],
                ],
                FieldId(6), MessageKind::String, FieldSize(12), vec![53, 54, 55, 56, 57, 48, 49, 50, 51, 52, 53, 54],
            ),
        ];
        let mut message = ParseData::new(
            &dbg,
            ParseSize::new(
                &dbg,
                FieldSize(4),
                ParseKind::new(
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
                ),
            ),
        );
        for (step, messages, target_id, target_kind, target_size, target_bytes) in test_data {
            let mut result_bytes = vec![];
            for bytes in messages {
                match message.parse(bytes) {
                    Ok((id, kind, size, bytes)) => {
                        log::debug!("{} | step: {},  id: {:?},  kind: {:?},  size: {:?},  bytes: {:?}", dbg, step, id, kind, size, bytes);
                        let result = id;
                        assert!(result == target_id, "step: {} \nresult: {:?}\ntarget: {:?}", step, result, target_id);
                        let result = kind;
                        assert!(result == target_kind, "step: {} \nresult: {:?}\ntarget: {:?}", step, result, target_kind);
                        let result = size;
                        assert!(result == target_size, "step: {} \nresult: {:?}\ntarget: {:?}", step, result, target_size);
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
    ///
    /// Testing such `Message.parse`
    #[test]
    fn parse_advanced() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let dbg = Dbg::own("parse_advanced");
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
        let field_kind = MessageKind::String;
        let test_data = [
            (
                00, vec![
                    to_bytes("123", 4294967291),
                ],
                (FieldId(4294967291), field_kind.clone(), FieldSize(3), "123".as_bytes().to_vec()),
            ),
            (
                01, vec![
                    [10, 11, 12, 13].to_vec(),
                    to_bytes("12345", 4294967292)[..1].to_vec(),
                    to_bytes("12345", 4294967292)[1..].to_vec(),
                ],
                (FieldId(4294967292), field_kind.clone(), FieldSize(5), "12345".as_bytes().to_vec()),
            ),
            (
                02, vec![
                    to_bytes("2345", 4294967293)[..2].to_vec(),
                    to_bytes("2345", 4294967293)[2..].to_vec(),
                    [20, 21, 23, 24].to_vec(),
                ],
                (FieldId(4294967293), field_kind.clone(), FieldSize(4), "2345".as_bytes().to_vec()),
            ),
            (
                03, vec![
                    [31, 32, 33, 34].to_vec(),
                    to_bytes("3456", 4294967294)[..3].to_vec(),
                    to_bytes("3456", 4294967294)[3..].to_vec(),
                    [35, 36, 37, 38].to_vec(),
                ],
                (FieldId(4294967294), field_kind.clone(), FieldSize(4), "3456".as_bytes().to_vec()),
            ),
            (
                04, vec![
                    to_bytes("456", 4294967295)[..4].to_vec(),
                    to_bytes("456", 4294967295)[4..].to_vec(),
                ],
                (FieldId(4294967295), field_kind.clone(), FieldSize(3), "456".as_bytes().to_vec()),
            ),
            (
                05, vec![
                    to_bytes("56789", 4294967281)[..5].to_vec(),
                    to_bytes("56789", 4294967281)[5..].to_vec(),
                ],
                (FieldId(4294967281), field_kind.clone(), FieldSize(5), "56789".as_bytes().to_vec()),
            ),
            (
                06, vec![
                    to_bytes("67890", 4294967282)[..6].to_vec(),
                    to_bytes("67890", 4294967282)[6..].to_vec(),
                ],
                (FieldId(4294967282), field_kind.clone(), FieldSize(5), "67890".as_bytes().to_vec()),
            ),
            (
                07, vec![
                    to_bytes("78901", 4294967283)[..7].to_vec(),
                    to_bytes("78901", 4294967283)[7..].to_vec(),
                ],
                (FieldId(4294967283), field_kind.clone(), FieldSize(5), "78901".as_bytes().to_vec()),
            ),
            (
                08, vec![
                    [80, 81, 82, 83].to_vec(),
                    to_bytes("1234567890", 4294967284)[ ..2].to_vec(),
                    to_bytes("1234567890", 4294967284)[2..5].to_vec(),
                    to_bytes("1234567890", 4294967284)[5..7].to_vec(),
                    to_bytes("1234567890", 4294967284)[7..9].to_vec(),
                    to_bytes("1234567890", 4294967284)[9.. ].to_vec(),
                ],
                (FieldId(4294967284), field_kind.clone(), FieldSize(10), "1234567890".as_bytes().to_vec()),
            ),
            (
                09, vec![
                    [[34, 36].into(), to_bytes("123N", 4294967285), [78, 22].into()].concat(),
                ],
                (FieldId(4294967285), field_kind.clone(), FieldSize(4), "123N".as_bytes().to_vec()),
            ),
        ];
        // let mut message = Message::new(&dbgid, &[
        //     MessageField::Syn(FieldSyn(Message::SYN)),
        //     MessageField::Id(FieldId(4)),
        //     MessageField::Kind(FieldKind(MessageKind::String)),
        //     MessageField::Size(FieldSize(4)),
        //     MessageField::Data(FieldData(vec![]))
        // ]);
        let mut message = ParseData::new(
            &dbg,
            ParseSize::new(
                &dbg,
                FieldSize(4),
                ParseKind::new(
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
                ),
            ),
        );
        for (step, messages, (target_id, target_kind, target_size, target_bytes)) in test_data {
            let mut result_data = None;
            for bytes in messages {
                match message.parse(bytes) {
                    Ok((id, kind, size, bytes)) => {
                        log::debug!("{} | step: {},  id: {:?},  kind: {:?},  size: {:?},  data: {:?}", dbg, step, id, kind, size, bytes);
                        let result = id;
                        assert!(result == target_id, "step: {} \nresult: {:?}\ntarget: {:?}", step, result, target_id);
                        let result = kind;
                        assert!(result == target_kind, "step: {} \nresult: {:?}\ntarget: {:?}", step, result, target_kind);
                        let result = size;
                        assert!(result == target_size, "step: {} \nresult: {:?}\ntarget: {:?}", step, result, target_size);
                        let result = bytes;
                        assert!(result == target_bytes, "step: {} \nresult: {:?}\ntarget: {:?}", step, result, target_bytes);
                        result_data = Some(result);
                    }
                    Err(err) => {
                        log::debug!("{} | {}",dbg, err);
                    }
                }
            }
            assert!(result_data == Some(target_bytes.clone()), "step: {} \nresult: {:?}\ntarget: {:?}", step, result_data, Some(target_bytes));
        }
        test_duration.exit();
    }
}
