#[cfg(test)]

mod vec_concat {
    use std::{sync::{Arc, Once, mpsc::channel}, time::{Duration, Instant}};
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel};
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
    // #[ignore = "Performance test ignored"]
    fn parse() {
        DebugSession::new().filter(LogLevel::Debug).init();
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
        let src1: Vec<u64> = (0..1_111).collect();
        let src2: Vec<u64> = (0..1_111).collect();
        let src3: Vec<u64> = (0..1_111).collect();
        let src4: Vec<u64> = (0..1_111).collect();
        let mut dest1: Vec<u64> = (0..1111).collect();
        let mut dest2: Vec<u64> = (0..1111).collect();
        let mut dest3: Vec<u64> = (0..1111).collect();
        let mut dest4: Vec<u64> = (0..1111).collect();
        let iterations = 1000u32;
        let wait1 = Arc::new(std::sync::Barrier::new(4));
        let wait2 = wait1.clone();
        let wait3 = wait1.clone();
        let wait4 = wait1.clone();
        let h1 = std::thread::spawn(move || {
            let mut elapsed = Duration::ZERO;
            let mut e;
            wait1.wait();
            for _ in 0..iterations {
                (dest1, e) = append(&mut t, &mut src1.clone(), dest1);
                elapsed += e;
            }
            (elapsed / iterations, format!("\tappend elapsed {:?}", elapsed / iterations))
        });
        let h2 = std::thread::spawn(move || {
            let mut elapsed = Duration::ZERO;
            let mut e;
            wait2.wait();
            for _ in 0..iterations {
                (dest2, e) = iter(&mut t, src2.clone(), dest2);
                elapsed += e;
            }
            (elapsed / iterations, format!("\tfor in elapsed {:?}", elapsed / iterations))
        });
        let h3 = std::thread::spawn(move || {
            let mut elapsed = Duration::ZERO;
            let mut e;
            wait3.wait();
            for _ in 0..iterations {
                (dest3, e) = extend_from_slice(&mut t, &src3.clone(), dest3);
                elapsed += e;
            }
            (elapsed / iterations, format!("\textend_from_slice elapsed {:?}", elapsed / iterations))
        });
        let h4 = std::thread::spawn(move || {
            let mut elapsed = Duration::ZERO;
            let mut e;
            wait4.wait();
            for _ in 0..iterations {
                (dest4, e) = extend(&mut t, &src4.clone(), dest4);
                elapsed += e;
            }
            (elapsed / iterations, format!("\textend elapsed {:?}", elapsed / iterations))
        });
        let mut results: Vec<(Duration, String)> = [h1, h2, h3, h4].into_iter().map(|h| {
            let (e, m) = h.join().unwrap();
            (e, m)
        }).collect();
        results.sort_by(|(e1, _), (e2, _)| e1.cmp(e2));
        for (_, msg) in results {
            println!("{msg}")
        }
        test_duration.exit();
    }
    fn extend_from_slice(t: &mut Instant, src: &[u64], mut dest: Vec<u64>) -> (Vec<u64>, Duration) {
        *t = Instant::now();
        dest.extend_from_slice(src);
        (dest, t.elapsed())
    }
    fn extend(t: &mut Instant, src: &Vec<u64>, mut dest: Vec<u64>) -> (Vec<u64>, Duration) {
        *t = Instant::now();
        dest.extend(src);
        (dest, t.elapsed())
    }
    fn append(t: &mut Instant, src: &mut Vec<u64>, mut dest: Vec<u64>) -> (Vec<u64>, Duration) {
        *t = Instant::now();
        dest.append(src);
        (dest, t.elapsed())
    }
    fn iter(t: &mut Instant, src: Vec<u64>, mut dest: Vec<u64>) -> (Vec<u64>, Duration) {
        *t = Instant::now();
        for v in src {
            dest.push(v);
        }
        (dest, t.elapsed())
        // println!("\tappend elapsed {:?}", t.elapsed());
    }
}
