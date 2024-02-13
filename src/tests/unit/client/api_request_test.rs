#![allow(non_snake_case)]
#[cfg(test)]

mod tests {
    use log::{debug, warn};
    use std::{collections::HashMap, sync::Once};
    use serde_json::json;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use postgres::{Client, NoTls};
    use crate::client::{api_query::{ApiQuery, ApiQueryExecutable, ApiQueryKind, ApiQueryPython, ApiQuerySql}, api_reply::ApiReply, api_request::ApiRequest};
    
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    // use super::*;
    
    static INIT: Once = Once::new();
    
    ///
    /// once called initialisation
    fn initOnce() {
        INIT.call_once(|| {
                // implement your initialisation code to be called only once for current test file
            }
        )
    }
    
    
    ///
    /// returns:
    ///  - ...
    fn initEach() -> () {
    
    }
    
    #[test]
    fn test_api_query() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        initOnce();
        initEach();
        println!("");
        let self_id = "test ApiRequest";
        println!("{}", self_id);

        let database = "test_api_query";
        let mut client = connect_db(self_id, "postgres", "postgres", "localhost:5432", "").unwrap();
        create_db(self_id, &mut client, database).unwrap();
        client.close().unwrap();
        let mut client = connect_db(self_id, database, database, "localhost:5432", database).unwrap();
        create_db_table(self_id, &mut client, database, "customer").unwrap();
        client.close().unwrap();
        let port = "8080";     //TestSession::free_tcp_port_str();
        let addtess = format!("127.0.0.1:{}", port);
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
            self_id,
            &addtess,
            token, 
            ApiQuery::new(ApiQueryKind::Sql(ApiQuerySql::new("", "")), false),
            true,
            debug,
        );
        for (query, keep_alive, target) in test_data {
            println!("\nrequest: {:?}", request);
            match request.fetch(&query, keep_alive) {
                Ok(bytes) => {
                    let reply = ApiReply::try_from(bytes);
                    println!("\nreply: {:?}", reply);
                },
                Err(err) => {
                    panic!("{} | Error: {:?}", self_id, err);
                },
            };
            let result = json!(request);
            let target: serde_json::Value = serde_json::from_str(target).unwrap();
            assert!(result == target, "\n result: {:?}\n target: {:?}", result, target);
            println!("\n result: {:?}\n target: {:?}", result, target);
        }
        let mut client = connect_db(self_id, "postgres", "postgres", "localhost:5432", "").unwrap();
        drop_db(self_id, &mut client, database).unwrap();
    }
    ///
    /// 
    fn create_db_user(selfId: &str, client: &mut Client, name: &str) -> Result<(), String> {
        match client.query(&format!("SELECT 1 FROM pg_user WHERE usename = '{}';", name), &[]) {
            Ok(rows) => {
                println!("\n rows: {:?}", rows);
                if rows.is_empty() {
                    let db_user = name;
                    let db_pass = name;
                    let result = client.batch_execute(&format!("CREATE USER {} WITH PASSWORD '{}' CREATEDB CREATEROLE;", db_user, db_pass));
                    println!("\n result: {:?}", result);
                    if let Err(err) = result {
                        return Err(format!("{}.create_db_user | Error: {:?}", selfId, err));
                    }
                }
                Ok(())
            },
            Err(err) => {
                panic!("{}.create_db | Error: {:?}", selfId, err);
            },
        }
    }
    ///
    ///
    fn create_db(self_id: &str, client: &mut Client, name: &str) -> Result<(), String> {
        create_db_user(self_id, client, name)?;
        let db_name = name;
        let db_user = name;
        match client.query(&format!("SELECT 1 FROM pg_database WHERE datname = '{}';", name), &[]) {
            Ok(rows) => {
                println!("\n rows: {:?}", rows);
                if rows.is_empty() {
                    let sql = [
                        format!("CREATE DATABASE {};", name),
                        format!("GRANT ALL PRIVILEGES ON DATABASE {} TO {};", db_name, db_user),
                        format!("ALTER DATABASE {} OWNER TO {};", db_name, db_user),
                    ];
                    for sql in sql {
                        let result = client.batch_execute(&sql);
                        println!("\n result: {:?}", result);
                        if let Err(err) = result {
                            return Err(format!("{}.create_db | Error: {:?}", self_id, err));
                        }
                    }
                }
            },
            Err(err) => {
                panic!("{}.create_db | Error: {:?}", self_id, err);
            },
        }
        Ok(())
    }
    ///
    /// 
    fn create_db_table(self_id: &str, client: &mut Client, database: &str, table: &str) ->Result<(), String> {
        let sql = [
            format!(r#"
                CREATE TABLE IF NOT EXISTS {}.public.{} (
                    id bigserial,
                    name varchar(255),
                    account numeric(20,2),
                    created timestamp default current_timestamp
            );"#, database, table),
            format!(r#"
                insert into {}.public.{} (name, account) values
                ('Abigail Guerra', 0.0),
                ('Leland Kirk', 0.0),
                ('Ellis Quinn', 0.0),
                ('Rhys Friedman', 0.0),
                ('Aspyn Espinosa', 0.0),
                ('Khalid Glenn', 0.0),
                ('Blaire Watts', 0.0),
                ('Dakota Little', 0.0),
                ('Harley Reyes', 0.0),
                ('Eli Horton', 0.0),
                ('Aitana Kirby', 0.0),
                ('Tony McCarty', 0.0),
                ('Halo Warner', 0.0)
            ;"#, database, table),
        ];
        for sql in sql {
            let result = client.batch_execute(&sql);
            println!("\n result: {:?}", result);
            if let Err(err) = result {
                return Err(format!("{}.create_db_table | Error: {:?}", self_id, err));
            }
        }
        Ok(())
    }
    ///
    /// 
    fn drop_db(self_id: &str, client: &mut Client, name: &str) -> Result<(), String> {
        let sql = [
            format!("DROP DATABASE IF EXISTS {};", name),
        ];
        for sql in sql {
            let result = client.batch_execute(&sql);
            println!("\n result: {:?}", result);
            if let Err(err) = result {
                return Err(format!("{}.drop_db | Error: {:?}", self_id, err));
            }
        }
        Ok(())
    }
    fn connect_db(self_id: &str, user: &str, pass: &str, path: &str, name: &str) -> Result<Client, String> {
        let path = if !user.is_empty() && !pass.is_empty() {
            format!("postgresql://{}:{}@{}/{}", user, pass, path, name)    // postgresql://user:secret@localhost
        } else {
            format!("postgresql://{}/{}", path, name)                                                  // postgresql://localhost
        };
        debug!("{}.connect_db | connecting with params: {:?}", self_id, path);
        match Client::connect(&path, NoTls) {
            Ok(client) => {
                Ok(client)
            },
            Err(err) => {
                let message = format!("{}.connect_db | connection error: {:?}", self_id, err);
                warn!("{:?}", message);
                Err(message)
            },
        }
    }
}
