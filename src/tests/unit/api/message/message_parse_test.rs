#[cfg(test)]

mod message {
    use std::{sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{api::message::{fields::{FieldData, FieldId, FieldKind, FieldSize, FieldSyn}, message::{Message, MessageField}, message_kind::MessageKind}, debug::dbg_id::DbgId};
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
    /// Testing such `Message.parse`
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
        let field_kind = MessageField::Kind(FieldKind(MessageKind::String));
        fn field_size(size: u32) -> MessageField { MessageField::Size(FieldSize(size)) }
        let test_data = [
            (
                00, vec![
                    to_bytes("123", 4294967291),
                ],
                (FieldId(4294967291), field_kind.clone(), field_size(3), MessageField::Data(FieldData("123".as_bytes().to_vec()))),
            ),
            (
                01, vec![
                    [10, 11, 12, 13].to_vec(),
                    to_bytes("12345", 4294967292)[..1].to_vec(),
                    to_bytes("12345", 4294967292)[1..].to_vec(),
                ],
                (FieldId(4294967292), field_kind.clone(), field_size(5), MessageField::Data(FieldData("12345".as_bytes().to_vec()))),
            ),
            (
                02, vec![
                    to_bytes("2345", 4294967293)[..2].to_vec(),
                    to_bytes("2345", 4294967293)[2..].to_vec(),
                    [20, 21, 23, 24].to_vec(),
                ],
                (FieldId(4294967293), field_kind.clone(), field_size(4), MessageField::Data(FieldData("2345".as_bytes().to_vec()))),
            ),
            (
                03, vec![
                    [31, 32, 33, 34].to_vec(),
                    to_bytes("3456", 4294967294)[..3].to_vec(),
                    to_bytes("3456", 4294967294)[3..].to_vec(),
                    [35, 36, 37, 38].to_vec(),
                ],
                (FieldId(4294967294), field_kind.clone(), field_size(4), MessageField::Data(FieldData("3456".as_bytes().to_vec()))),
            ),
            (
                04, vec![
                    to_bytes("456", 4294967295)[..4].to_vec(),
                    to_bytes("456", 4294967295)[4..].to_vec(),
                ],
                (FieldId(4294967295), field_kind.clone(), field_size(3), MessageField::Data(FieldData("456".as_bytes().to_vec()))),
            ),
            (
                05, vec![
                    to_bytes("56789", 4294967281)[..5].to_vec(),
                    to_bytes("56789", 4294967281)[5..].to_vec(),
                ],
                (FieldId(4294967281), field_kind.clone(), field_size(5), MessageField::Data(FieldData("56789".as_bytes().to_vec()))),
            ),
            (
                06, vec![
                    to_bytes("67890", 4294967282)[..6].to_vec(),
                    to_bytes("67890", 4294967282)[6..].to_vec(),
                ],
                (FieldId(4294967282), field_kind.clone(), field_size(5), MessageField::Data(FieldData("67890".as_bytes().to_vec()))),
            ),
            (
                07, vec![
                    to_bytes("78901", 4294967283)[..7].to_vec(),
                    to_bytes("78901", 4294967283)[7..].to_vec(),
                ],
                (FieldId(4294967283), field_kind.clone(), field_size(5), MessageField::Data(FieldData("78901".as_bytes().to_vec()))),
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
                (FieldId(4294967284), field_kind.clone(), field_size(10), MessageField::Data(FieldData("1234567890".as_bytes().to_vec()))),
            ),
            (
                09, vec![
                    [[34, 36].into(), to_bytes("123N", 4294967285), [78, 22].into()].concat(),
                ],
                (FieldId(4294967285), field_kind.clone(), field_size(4), MessageField::Data(FieldData("123N".as_bytes().to_vec()))),
            ),
        ];
        let mut message = Message::new(&dbgid, &[
            MessageField::Syn(FieldSyn(Message::SYN)),
            MessageField::Id(FieldId(4)),
            MessageField::Kind(FieldKind(MessageKind::String)),
            MessageField::Size(FieldSize(4)),
            MessageField::Data(FieldData(vec![]))
        ]);
        for (step, messages, (target_id, target_kind, target_size, target_bytes)) in test_data {
            let mut result_data = None;
            for bytes in messages {
                match message.parse(&bytes) {
                    Ok(parsed) => match parsed.as_slice() {
                        [ MessageField::Id(FieldId(id)), MessageField::Kind(kind), MessageField::Size(FieldSize(size)), MessageField::Data(FieldData(data)) ] => {
                            log::debug!("{} | step: {},  id: {},  kind: {:?},  size: {},  data: {:?}", dbgid, step, id, kind, size, data);
                            let result = FieldId(*id);
                            assert!(result == target_id, "step: {} \nresult: {:?}\ntarget: {:?}", step, result, target_id);
                            let result = MessageField::Kind(kind.to_owned());
                            assert!(result == target_kind, "step: {} \nresult: {:?}\ntarget: {:?}", step, result, target_kind);
                            let result = MessageField::Size(FieldSize(*size));
                            assert!(result == target_size, "step: {} \nresult: {:?}\ntarget: {:?}", step, result, target_size);
                            let result = MessageField::Data(FieldData(data.clone()));
                            assert!(result == target_bytes, "step: {} \nresult: {:?}\ntarget: {:?}", step, result, target_bytes);
                            result_data = Some(result);
                        }
                        v if v.is_empty() => {}
                        [..] => {
                            panic!("{} | Unknown message kind {:?}", dbgid, parsed);
                        }
                    }
                    Err(err) => {
                        log::debug!("{} | {}",dbgid, err);
                    }
                }
            }
            assert!(result_data == Some(target_bytes.clone()), "step: {} \nresult: {:?}\ntarget: {:?}", step, result_data, Some(target_bytes));
        }
        test_duration.exit();
    }
}
