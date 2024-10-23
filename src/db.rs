use crate::error::{WHError, WHResult};
use sqlite::Connection;

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn new(path: &str) -> WHResult<Database> {
        let connection = Connection::open(path);
        match connection {
            Ok(connection) => Ok(Database { connection }),
            Err(_) => Err(WHError::DatabaseConnectionError(path.to_string())),
        }
    }
}
