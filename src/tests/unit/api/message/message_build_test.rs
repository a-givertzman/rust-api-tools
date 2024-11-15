#[cfg(test)]

mod message {
    use std::{sync::Once, time::Duration};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{api::message::{fields::{FieldData, FieldId, FieldKind, FieldSize, FieldSyn}, message::{Bytes, Message, MessageField, MessageParse}, message_kind::MessageKind}, debug::dbg_id::DbgId, error::str_err::StrErr};
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
    /// Testing such `Message.build`
    #[test]
    fn build() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let dbgid = DbgId("test".to_owned());
        log::debug!("\n{}", dbgid);
        let test_duration = TestDuration::new(&dbgid, Duration::from_secs(1));
        test_duration.run().unwrap();
        let test_data = [
            (
                00, "01234", 4294967291u32,
                vec![22, 0xff, 0xff, 0xff, 0xfb, MessageKind::String as u8, 00, 00, 00, 05, 48, 49, 50, 51, 52],
            ),
            (
                01, "1234 5", 4294967292,
                vec![22, 0xff, 0xff, 0xff, 0xfc, MessageKind::String as u8, 00, 00, 00, 06, 49, 50, 51, 52, 32, 53],
            ),
            (
                02, "!@#$%^&*()_+", 4294967293,
                vec![22, 0xff, 0xff, 0xff, 0xfd, MessageKind::String as u8, 00, 00, 00, 12, 33, 64, 35, 36, 37, 94, 38, 42, 40, 41, 95, 43],
            ),
            (
                03, r#"QWERTYUIOP{}ASDFGHJKL:"ZXCVBNM<>?""#, 4294967294,
                vec![22, 0xff, 0xff, 0xff, 0xfe, MessageKind::String as u8, 00, 00, 00, 34, 81, 87, 69, 82, 84, 89, 85, 73, 79, 80, 123, 125, 65, 83, 68, 70, 71, 72, 74, 75, 76, 58, 34, 90, 88, 67, 86, 66, 78, 77, 60, 62, 63, 34],
            ),
        ];
        let mut message = Message::new(
            &dbgid, 
            vec![
                MessageField::Syn(FieldSyn::default()),
                MessageField::Id(FieldId(4)),
                MessageField::Kind(FieldKind(MessageKind::String)),
                MessageField::Size(FieldSize(4)),
                MessageField::Data(FieldData(vec![]))
            ],
            FakeParse {},
        );
        for (step, data, id, target) in test_data {
            log::debug!("{} | step: {},  id: {},  kind: {:?},  size: {},  data: {:?}", dbgid, step, id, target[1], target[6..].len(), data);
            let result = message.build(data.as_bytes().to_owned().as_mut(), id);
            assert!(result == target, "step: {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
    struct FakeParse {}
    impl MessageParse<()> for FakeParse {
        fn parse(&mut self, _: Bytes) -> Result<(), StrErr> {
            todo!()
        }
        fn reset(&mut self) {
            todo!()
        }
    }
}
