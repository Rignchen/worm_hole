use crate::error::{WHError, WHResult};
use sqlite::Connection;

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn new(path: &str) -> WHResult<Database> {
        let connection = Connection::open(path);
        match connection {
            Ok(connection) => {
                let db = Database { connection };
                db.init();
                Ok(db)
            }
            Err(_) => Err(WHError::DatabaseConnectionError(path.to_string())),
        }
    }

    pub fn init(&self) {
        self.connection
            .execute(
                "create table if not exists aliases (
                    id integer primary key,
                    alias text not null,
                    path text not null,
                    unique(alias)
                )",
            )
            .unwrap();
    }

    pub fn add_alias(&self, alias: &str, path: &str) -> WHResult<()> {
        let mut statement = self.connection
            .prepare("insert into aliases (alias, path) values (:alias, :path)")
            .unwrap();
        statement
            .bind::<&[(&str, &str)]>(&[(":alias", alias), (":path", path)])
            .unwrap();
        statement.next().unwrap();
        Ok(())
    }
}
