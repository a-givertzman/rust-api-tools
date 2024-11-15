#[cfg(test)]

mod message {
    use std::{sync::Once, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
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
    /// Testing `Vec concatinations`
    #[test]
    #[ignore = "Performance test ignored"]
    fn parse() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let dbgid = "test";
        log::debug!("\n{}", dbgid);
        let test_duration = TestDuration::new(dbgid, Duration::from_secs(30));
        test_duration.run().unwrap();
        let mut t: Instant = Instant::now();
        //           18446744073709551615
        // let mut src = (0..11111111111111111111).collect();
        let src: Vec<u64> = (0..1_111_111).collect();
        let mut dest1: Vec<u64> = vec![];
        let mut dest2: Vec<u64> = vec![];
        // append(&mut t, &mut src.clone(), &mut dest2);
        extend_from_slice(&mut t, &src.clone(), &mut dest1);
        extend(&mut t, src.clone(), &mut dest1);
        // append(&mut t, &mut src.clone(), &mut dest2);
        extend_from_slice(&mut t, &src.clone(), &mut dest1);
        extend(&mut t, src.clone(), &mut dest1);
        append(&mut t, &mut src.clone(), &mut dest2);
        extend_from_slice(&mut t, &src.clone(), &mut dest1);
        extend(&mut t, src.clone(), &mut dest1);

        test_duration.exit();
    }
    fn extend_from_slice(t: &mut Instant, src: &[u64], dest: &mut Vec<u64>) {
        *t = Instant::now();
        dest.extend_from_slice(src);
        println!("\t\textend_from_slice elapsed {:?}", t.elapsed());
    }
    fn extend(t: &mut Instant, src: Vec<u64>, dest: &mut Vec<u64>) {
        *t = Instant::now();
        dest.extend(src);
        println!("\textend elapsed {:?}", t.elapsed());
    }
    fn append(t: &mut Instant, src: &mut Vec<u64>, dest: &mut Vec<u64>) {
        *t = Instant::now();
        dest.append(src);
        println!("append elapsed {:?}", t.elapsed());
    }
}
