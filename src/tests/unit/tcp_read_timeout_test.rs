#[cfg(test)]

mod tests {
    use std::{io::{BufReader, Read}, net::{TcpListener, TcpStream}, sync::Once, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{debug::dbg_id::DbgId, error::str_err::StrErr};
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
    /// Testing such functionality / behavior
    #[test]
    fn test_task_cycle() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        let dbgid = DbgId("test".to_owned());
        log::debug!("\n{}", dbgid);
        let test_duration = TestDuration::new(&dbgid, Duration::from_secs(10));
        test_duration.run().unwrap();
        let time = Instant::now();
        let timeout = Duration::from_secs(3);
        let addr = "0.0.0.0:7676";
        let mut step: usize = 0;
        let result = match TcpListener::bind(addr) {
            Ok(_) => {
                let stream = TcpStream::connect(addr).unwrap();
                if let Err(err) = stream.set_read_timeout(Some(timeout)) {
                    let message = format!("{}.connect | set_read_timeout error: \n\t{:?}", dbgid, err);
                    log::warn!("{}", message);
                }
                if let Err(err) = stream.set_write_timeout(Some(timeout)) {
                    let message = format!("{}.connect | set_write_timeout error: \n\t{:?}", dbgid, err);
                    log::warn!("{}", message);
                }
                let mut stream = BufReader::new(stream);
                let mut buf = vec![0; 1024];
                let result = stream.read(&mut buf);
                let elapsed = time.elapsed();
                log::debug!("{} | result: {:?}", dbgid, result);
                assert!(result.is_err(), "step {} \nresult: {:?}\ntarget: {:?}", step.inc(), result.is_err(), true);
                let target = timeout + Duration::from_millis(500);
                assert!(elapsed < target, "step {} \nresult: {:?}\ntarget: {:?}", step.inc(), elapsed, target);
                let target = timeout - Duration::from_millis(1);
                assert!(elapsed > target, "step {} \nresult: {:?}\ntarget: {:?}", step.inc(), elapsed, target);
                Ok(())
            }
            Err(err) => {
                let err = format!("{} | Error: {:?}", dbgid, err);
                log::error!("{}", err);
                Err(StrErr(err))
            }
        };
        assert!(result.is_ok(), "step {} \nresult: {:?}\ntarget: {:?}", step.inc(), result.is_ok(), true);
        test_duration.exit();
    }
}
