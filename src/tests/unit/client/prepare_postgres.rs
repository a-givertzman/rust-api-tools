use log::{debug, warn};
use postgres::{Client, NoTls};

///
/// Sreates postgres database for testing purposes
pub struct TestDatabasePostgres {}
///
/// 
impl TestDatabasePostgres {
    ///
    /// Create user with standard privileges
    pub fn create_db_user(parent: impl Into<String>, client: &mut Client, name: &str) -> Result<(), String> {
        let parent = parent.into();
        match client.query(&format!("SELECT 1 FROM pg_user WHERE usename = '{}';", name), &[]) {
            Ok(rows) => {
                println!("\n rows: {:?}", rows);
                if rows.is_empty() {
                    let db_user = name;
                    let db_pass = name;
                    let result = client.batch_execute(&format!("CREATE USER {} WITH PASSWORD '{}' CREATEDB CREATEROLE;", db_user, db_pass));
                    println!("\n result: {:?}", result);
                    if let Err(err) = result {
                        return Err(format!("{}.create_db_user | Error: {:?}", parent, err));
                    }
                }
                Ok(())
            },
            Err(err) => {
                panic!("{}.create_db | Error: {:?}", parent, err);
            },
        }
    }
    ///
    /// Creates database and user with the same name
    pub fn create_db(parent: impl Into<String>, client: &mut Client, name: &str) -> Result<(), String> {
        let parent = parent.into();
        Self::create_db_user(&parent, client, name)?;
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
                            return Err(format!("{}.create_db | Error: {:?}", parent, err));
                        }
                    }
                }
            },
            Err(err) => {
                panic!("{}.create_db | Error: {:?}", parent, err);
            },
        }
        Ok(())
    }
    ///
    /// Creates table
    pub fn create_db_table(parent: impl Into<String>, client: &mut Client, database: &str, table: &str) ->Result<(), String> {
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
                return Err(format!("{}.create_db_table | Error: {:?}", parent.into(), err));
            }
        }
        Ok(())
    }
    ///
    /// Drops database and try to delete user with the same name
    pub fn drop_db(parent: impl Into<String>, client: &mut Client, name: &str) -> Result<(), String> {
        let parent = parent.into();
        let sql = [
            format!("DROP DATABASE IF EXISTS {};", name),
            format!("DROP USER IF EXISTS {};", name),
        ];
        for sql in sql {
            let result = client.batch_execute(&sql);
            println!("\n result: {:?}", result);
            if let Err(err) = result {
                return Err(format!("{}.drop_db | Error: {:?}", parent, err));
            }
        }
        Ok(())
    }
    ///
    /// 
    pub fn connect_db(parent: impl Into<String>, user: &str, pass: &str, path: &str, name: &str) -> Result<Client, String> {
        let parent = parent.into();
        let path = if !user.is_empty() && !pass.is_empty() {
            format!("postgresql://{}:{}@{}/{}", user, pass, path, name)    // postgresql://user:secret@localhost
        } else {
            format!("postgresql://{}/{}", path, name)                                                  // postgresql://localhost
        };
        debug!("{}.connect_db | connecting with params: {:?}", parent, path);
        match Client::connect(&path, NoTls) {
            Ok(client) => {
                Ok(client)
            },
            Err(err) => {
                let message = format!("{}.connect_db | connection error: {:?}", parent, err);
                warn!("{:?}", message);
                Err(message)
            },
        }
    }    
}
