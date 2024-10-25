use crate::error::{WHError, WHResult};
use sqlite::Connection;
use sqlite::State;

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
        let mut statement = self
            .connection
            .prepare("insert into aliases (alias, path) values (:alias, :path)")
            .unwrap();
        statement
            .bind::<&[(&str, &str)]>(&[(":alias", alias), (":path", path)])
            .unwrap();
        match statement.next() {
            Ok(_) => Ok(()),
            Err(_) => Err(WHError::AliasAlreadyExists(alias.to_string())),
        }
    }

    pub fn edit_alias(&self, alias: &str, paths: &str) -> WHResult<()> {
        self.get_alias(alias)?; // Check if alias exists
        let mut statement = self
            .connection
            .prepare("update aliases set path = :path where alias = :alias")
            .unwrap();
        statement
            .bind::<&[(&str, &str)]>(&[(":alias", alias), (":path", paths)])
            .unwrap();
        statement.next().unwrap();
        Ok(())
    }

    pub fn remove_alias(&self, alias: &str) -> WHResult<()> {
        let mut statement = self
            .connection
            .prepare("delete from aliases where alias = :alias")
            .unwrap();
        statement.bind::<(&str, &str)>((":alias", alias)).unwrap();
        statement.next().unwrap();
        Ok(())
    }

    pub fn get_all_aliases(&self) -> WHResult<Vec<(String, String)>> {
        let mut statement = self
            .connection
            .prepare("select alias, path from aliases")
            .unwrap();
        let mut aliases = Vec::new();
        while let Ok(State::Row) = statement.next() {
            aliases.push((
                statement.read::<String, _>("alias").unwrap(),
                statement.read::<String, _>("path").unwrap(),
            ));
        }
        Ok(aliases)
    }

    pub fn get_alias(&self, alias: &str) -> WHResult<String> {
        let mut statement = self
            .connection
            .prepare("select path from aliases where alias = :alias")
            .unwrap();
        statement.bind::<(&str, &str)>((":alias", alias)).unwrap();
        if let Ok(State::Row) = statement.next() {
            Ok(statement.read::<String, _>("path").unwrap())
        } else {
            Err(WHError::AliasNotFound(alias.to_string()))
        }
    }
}
