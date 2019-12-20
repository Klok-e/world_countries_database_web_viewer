use r2d2_oracle::{oracle, r2d2, OracleConnectionManager};
use rocket::config::Value;
use rocket_contrib::databases::{DatabaseConfig, Poolable};

pub struct DbConnectionManager(OracleConnectionManager);

impl DbConnectionManager {
    pub fn new(username: &str, password: &str, connect_string: &str) -> DbConnectionManager {
        DbConnectionManager(OracleConnectionManager::new(
            username,
            password,
            connect_string,
        ))
    }
}

impl r2d2::ManageConnection for DbConnectionManager {
    type Connection = DbConnection;
    type Error = oracle::Error;

    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        self.0.connect().map(DbConnection)
    }

    fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        self.0.is_valid(&mut conn.0)
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        self.0.has_broken(&mut conn.0)
    }
}

pub struct DbConnection(oracle::Connection);

impl DbConnection {
    pub fn oracle_connection(&self) -> &oracle::Connection {
        &self.0
    }
}

impl Poolable for DbConnection {
    type Manager = DbConnectionManager;
    type Error = oracle::Error;

    fn pool(config: DatabaseConfig) -> Result<r2d2::Pool<Self::Manager>, Self::Error> {
        fn extract_string<'a>(
            config: &'a DatabaseConfig,
            extra_name: &'static str,
        ) -> Option<&'a String> {
            config.extras.get(extra_name).and_then(|v| {
                if let Value::String(sv) = v {
                    Some(sv)
                } else {
                    None
                }
            })
        }
        let uname = extract_string(&config, "username").unwrap();
        let pass = extract_string(&config, "password").unwrap();
        let connect_str = extract_string(&config, "connect_string").unwrap();
        let manager = DbConnectionManager::new(uname, pass, connect_str);
        Ok(r2d2::Pool::builder().max_size(20).build(manager).unwrap())
    }
}

#[database("oracle_db")]
pub struct OracleConnection(DbConnection);
