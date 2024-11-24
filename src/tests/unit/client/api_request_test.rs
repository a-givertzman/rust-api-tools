#[cfg(test)]

mod api_request {
    use std::{collections::HashMap, process::Command, sync::{atomic::AtomicUsize, Once}, thread, time::{Duration, Instant}};
    use serde_json::json;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use testing::{session::teardown::Teardown, stuff::max_test_duration::TestDuration};
    use crate::{
        api::reply::api_reply::ApiReply, client::{api_query::{ApiQuery, ApiQueryExecutable, ApiQueryKind, ApiQueryPython, ApiQuerySql}, api_request::ApiRequest},
        debug::dbg_id::DbgId, tests::unit::client::prepare_postgres::TestDatabasePostgres,
    };
    ///    
    static INIT: Once = Once::new();
    static TEARDOWN_COUNT: AtomicUsize = AtomicUsize::new(0);
    ///
    /// once called initialisation
    /// returns ApiServer PID
    fn init_once(self_id: &DbgId, database: &str, tmp_path: &str, git_repo: &str) -> u32 {
        let mut child: u32 = 0;
        INIT.call_once(|| {
                // implement your initialisation code to be called only once for current test file
                let mut client = TestDatabasePostgres::connect_db(&self_id.0, "postgres", "postgres", "localhost:5432", "").unwrap();
                TestDatabasePostgres::create_db(&self_id.0, &mut client, database).unwrap();
                client.close().unwrap();
                let mut client = TestDatabasePostgres::connect_db(&self_id.0, database, database, "localhost:5432", database).unwrap();
                TestDatabasePostgres::create_db_table(&self_id.0, &mut client, database, "customer").unwrap();
                client.close().unwrap();
                let setup_sh = "./src/tests/unit/client/setup-build.sh";
                log::debug!("{}.init_once | Preparing new instance of api-server...
                    \t - Be sure you are have internet connection for git clone
                    \t - Be sure local address '0.0.0.0:8080' is not busy before test executed
                    \t - Check '{}'",self_id, setup_sh
                );
                let output = Command::new(setup_sh)
                    .arg(tmp_path)
                    .arg(git_repo)
                    .output()
                    .expect("Failed to exec setup-build.sh");
                log::debug!("{}.init_once | setup-build: {:?}",self_id, output);
                let run_sh = "./src/tests/unit/client/setup-run.sh";
                let p = Command::new(run_sh)
                    .arg(tmp_path)
                    .spawn()
                    .expect("Failed to exec setup-run.sh");
                log::debug!("{}.init_once | setup-run: {:?}",self_id, p);
                child = p.id()
            }
        );
        child
    }
    ///
    /// Once called after all tests
    fn teardown_once(self_id: &DbgId, database: &str, tmp_path: &str, child_id: u32) {
        let mut client = TestDatabasePostgres::connect_db(&self_id.0, "postgres", "postgres", "localhost:5432", "").unwrap();
        TestDatabasePostgres::drop_db(&self_id.0, &mut client, database).unwrap();
        let p = Command::new("rm")
            .arg("rf")
            .arg(tmp_path)
            .output()
            .expect("Failed to remove tmp dir");
        log::debug!("std: {:#?}", p);
        let p = Command::new("kill")
            .arg(format!("{}", child_id))
            .spawn()
            .expect("Failed to remove tmp dir");
        log::debug!("std: {:#?}", p.stdout);
    }
    ///
    /// returns:
    ///  - ...
    fn init_each() -> () {}
    ///
    /// 
    #[test]
    fn debug_false() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_each();
        println!("");
        let dbgid = DbgId("test ApiRequest".into());
        println!("{}", dbgid);
        let test_duration = TestDuration::new(&dbgid, Duration::from_secs(120));
        test_duration.run().unwrap();
        let database = "test_api_query";
        let tmp_path = "/tmp/api-tools-test/api-server/";
        let git_repo = "https://github.com/a-givertzman/api-server.git";
        let child_id = init_once(&dbgid, database, tmp_path, git_repo);
        let _teardown_once = || {
            teardown_once(&dbgid, &database, &tmp_path, child_id);
        };
        let _teardown = Teardown::new(&TEARDOWN_COUNT, &|| {}, &_teardown_once);
        let port = "8080";     //TestSession::free_tcp_port_str();
        let addtess = format!("0.0.0.0:{}", port);
        let token = "123zxy456!@#";
        let keep_alive = true;
        let close_connection = false;
        let service_keep_alive = false;
        let debug = false;
        let test_data = [
            (
                ApiQuery::new(
                    ApiQueryKind::Sql(ApiQuerySql::new(database, "select * from customer;")),
                    service_keep_alive, 
                ),
                keep_alive,
                r#"{"authToken":"123zxy456!@#","id":"1","sql":{"database":"test_api_query","sql":"select * from customer;"},"keepAlive":true,"debug":false}"#,
                
            ),
            (
                ApiQuery::new(
                    ApiQueryKind::Sql(ApiQuerySql::new(database, "select * from customer limit 3;")),
                    service_keep_alive, 
                ),
                keep_alive,
                r#"{"authToken":"123zxy456!@#","id":"2","sql":{"database":"test_api_query","sql":"select * from customer limit 3;"},"keepAlive":true,"debug":false}"#,
            ),
            (
                ApiQuery::new(
                    ApiQueryKind::Python(ApiQueryPython::new("test_script", json!(HashMap::<String, f64>::new()))),
                    service_keep_alive,
                ),
                keep_alive,
                r#"{"authToken":"123zxy456!@#","id":"3","python":{"script":"test_script","params":{}},"keepAlive":true,"debug":false}"#,
            ),
            (
                ApiQuery::new(
                    ApiQueryKind::Executable(ApiQueryExecutable::new("test_app", json!(HashMap::<String, f64>::new()))),
                    service_keep_alive,
                ),
                close_connection,
                r#"{"authToken":"123zxy456!@#","id":"4","executable":{"name":"test_app","params":{}},"keepAlive":false,"debug":false}"#,
            ),
        ];
        let mut request = ApiRequest::new(
            &dbgid,
            &addtess,
            token, 
            ApiQuery::new(ApiQueryKind::Sql(ApiQuerySql::new("", "")), false),
            true,
            debug,
        );
        thread::sleep(Duration::from_secs(1));
        for (query, keep_alive, target) in test_data {
            println!("\nrequest: {:?}", request);
            match request.fetch_with(&query, keep_alive) {
                Ok(bytes) => {
                    let reply = ApiReply::try_from(bytes);
                    println!("\nreply: {:?}", reply);
                },
                Err(err) => {
                    panic!("{} | Error: {:?}", dbgid, err);
                },
            };
            let result = json!(request);
            let target: serde_json::Value = serde_json::from_str(target).unwrap();
            assert!(result == target, "\n result: {:?}\n target: {:?}", result, target);
            println!("\n result: {:?}\n target: {:?}", result, target);
        }
        // let mut client = TestDatabasePostgres::connect_db(self_id, "postgres", "postgres", "localhost:5432", "").unwrap();
        // TestDatabasePostgres::drop_db(self_id, &mut client, database).unwrap();
        test_duration.exit();
    }
    ///
    /// 
    #[test]
    fn debug_true() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_each();
        println!("");
        let dbgid = DbgId("test ApiRequest".into());
        println!("{}", dbgid);
        let test_duration = TestDuration::new(&dbgid, Duration::from_secs(120));
        test_duration.run().unwrap();
        let database = "test_api_query";
        let tmp_path = "/tmp/api-tools-test/api-server/";
        let git_repo = "https://github.com/a-givertzman/api-server.git";
        let child_id = init_once(&dbgid, database, tmp_path, git_repo);
        let _teardown_once = || {
            teardown_once(&dbgid, &database, &tmp_path, child_id);
        };
        let _teardown = Teardown::new(&TEARDOWN_COUNT, &|| {}, &_teardown_once);
        let port = "8080";     //TestSession::free_tcp_port_str();
        let addtess = format!("0.0.0.0:{}", port);
        let token = "123zxy456!@#";
        let keep_alive = true;
        // let close_connection = false;
        let service_keep_alive = false;
        let debug = true;
        let test_data = [
            (
                ApiQuery::new(
                    ApiQueryKind::Sql(ApiQuerySql::new(database, "select * from customer;")),
                    service_keep_alive, 
                ),
                keep_alive,
                r#"{"authToken":"123zxy456!@#","id":"1","sql":{"database":"test_api_query","sql":"select * from customer;"},"keepAlive":true,"debug":true}"#,
                
            ),
            // (
            //     ApiQuery::new(
            //         ApiQueryKind::Sql(ApiQuerySql::new(database, "select * from customer limit 3;")),
            //         service_keep_alive, 
            //     ),
            //     keep_alive,
            //     r#"{"authToken":"123zxy456!@#","id":"2","sql":{"database":"test_api_query","sql":"select * from customer limit 3;"},"keepAlive":true,"debug":false}"#,
            // ),
            // (
            //     ApiQuery::new(
            //         ApiQueryKind::Python(ApiQueryPython::new("test_script", json!(HashMap::<String, f64>::new()))),
            //         service_keep_alive,
            //     ),
            //     keep_alive,
            //     r#"{"authToken":"123zxy456!@#","id":"3","python":{"script":"test_script","params":{}},"keepAlive":true,"debug":false}"#,
            // ),
            // (
            //     ApiQuery::new(
            //         ApiQueryKind::Executable(ApiQueryExecutable::new("test_app", json!(HashMap::<String, f64>::new()))),
            //         service_keep_alive,
            //     ),
            //     close_connection,
            //     r#"{"authToken":"123zxy456!@#","id":"4","executable":{"name":"test_app","params":{}},"keepAlive":false,"debug":false}"#,
            // ),
        ];
        let mut request = ApiRequest::new(
            &dbgid,
            &addtess,
            token, 
            ApiQuery::new(ApiQueryKind::Sql(ApiQuerySql::new("", "")), false),
            true,
            debug,
        );
        for (query, keep_alive, target) in test_data {
            println!("\nrequest: {:?}", request);
            match request.fetch_with(&query, keep_alive) {
                Ok(bytes) => {
                    let reply = ApiReply::try_from(bytes);
                    println!("\nreply: {:?}", reply);
                },
                Err(err) => {
                    panic!("{} | Error: {:?}", dbgid, err);
                },
            };
            let result = json!(request);
            let target: serde_json::Value = serde_json::from_str(target).unwrap();
            assert!(result == target, "\n result: {:?}\n target: {:?}", result, target);
            println!("\n result: {:?}\n target: {:?}", result, target);
        }
        test_duration.exit();
        test_duration.exit();
    }
    ///
    /// ApiRequest performance test
    #[test]
    fn performance() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_each();
        println!("");
        let dbgid = DbgId("test ApiRequest".into());
        println!("{}", dbgid);
        let test_duration = TestDuration::new(&dbgid, Duration::from_secs(120));
        test_duration.run().unwrap();
        let database = "test_api_query";
        let tmp_path = "/tmp/api-tools-test/api-server/";
        let git_repo = "https://github.com/a-givertzman/api-server.git";
        let child_id = init_once(&dbgid, database, tmp_path, git_repo);
        let _teardown_once = || {
            teardown_once(&dbgid, &database, &tmp_path, child_id);
        };
        let _teardown = Teardown::new(&TEARDOWN_COUNT, &|| {}, &_teardown_once);
        let port = "8080";     //TestSession::free_tcp_port_str();
        let addtess = format!("0.0.0.0:{}", port);
        let token = "123zxy456!@#";
        let keep_alive = true;
        let service_keep_alive = false;
        let debug = false;
        let test_data = [
            (
                ApiQuery::new(
                    ApiQueryKind::Sql(ApiQuerySql::new(database, "select * from customer limit 4;")),
                    service_keep_alive, 
                ),
                keep_alive,
                r#"{"authToken":"123zxy456!@#","id":"1","sql":{"database":"test_api_query","sql":"select * from customer limit 4;"},"keepAlive":true,"debug":false}"#,
                
            ),
            (
                ApiQuery::new(
                    ApiQueryKind::Sql(ApiQuerySql::new(database, "select * from customer limit 3;")),
                    service_keep_alive, 
                ),
                keep_alive,
                r#"{"authToken":"123zxy456!@#","id":"2","sql":{"database":"test_api_query","sql":"select * from customer limit 3;"},"keepAlive":true,"debug":false}"#,
            ),
        ];
        let mut request = ApiRequest::new(
            &dbgid,
            &addtess,
            token, 
            ApiQuery::new(ApiQueryKind::Sql(ApiQuerySql::new("", "")), false),
            true,
            debug,
        );
        thread::sleep(Duration::from_secs(1));
        let t_total = Instant::now();
        let mut t: Instant;
        let queries = 0..100;
        for _ in queries.clone() {
            for (query, keep_alive, target) in &test_data {
                println!("\nrequest: {:?}", request);
                t = Instant::now();
                match request.fetch_with(&query, *keep_alive) {
                    Ok(bytes) => {
                        let reply = ApiReply::try_from(bytes);
                        println!("\nreply: {:?}", reply);
                    },
                    Err(err) => {
                        panic!("{} | Error: {:?}", dbgid, err);
                    },
                };
                let result = json!(request);
                let target: serde_json::Value = serde_json::from_str(target).unwrap();
                // assert!(result == target, "\n result: {:?}\n target: {:?}", result, target);
                println!("\n result: {:?}\n target: {:?}", result, target);
                println!("Elapsed: {:?}", t.elapsed());
            }
        }
        let queries_total = queries.len() * test_data.len();
        println!("Total queries: {:?}", queries_total);
        println!("Average elapsed per query: {:?}", t_total.elapsed() / queries_total as u32);
        println!("Total elapsed: {:?}", t_total.elapsed());
        test_duration.exit();
    }
}
