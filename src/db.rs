use crate::{
    error::{WHError, WHResult},
    alias::AliasList,
};
use sqlite::{
    Connection,
    State,
};

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

    fn run_statement<T: sqlite::Bindable>(&self, query: &str, bind: T) -> sqlite::Statement {
        let mut statement = self
            .connection
            .prepare(query)
            .unwrap();
        statement
            .bind::<T>(bind)
            .unwrap();
        statement
    }

    fn run_querry(&self, query: &str) -> sqlite::Statement {
        self.connection
            .prepare(query)
            .unwrap()
    }

    pub fn add_alias(&self, alias: &str, path: &str) -> WHResult<()> {
        let mut statement = self.run_statement::<&[(&str, &str)]>("insert into aliases (alias, path) values (:alias, :path)", &[(":alias", alias), (":path", path)]);
        match statement.next() {
            Ok(_) => Ok(()),
            Err(_) => Err(WHError::AliasAlreadyExists(alias.to_string())),
        }
    }

    pub fn edit_alias(&self, alias: &str, paths: &str) -> WHResult<()> {
        self.get_alias(alias)?; // Check if alias exists
        let mut statement = self.run_statement::<&[(&str, &str)]>("update aliases set path = :path where alias = :alias", &[(":alias", alias), (":path", paths)]);
        statement.next().unwrap();
        Ok(())
    }

    pub fn rename_alias(&self, old_alias: &str, new_alias: &str) -> WHResult<()> {
        self.get_alias(old_alias)?; // Check if alias exists
        let mut statement = self.run_statement::<&[(&str, &str)]>("update aliases set alias = :new_alias where alias = :old_alias", &[(":old_alias", old_alias), (":new_alias", new_alias)]);
        match statement.next() {
            Ok(_) => Ok(()),
            Err(_) => Err(WHError::AliasAlreadyExists(new_alias.to_string())),
        }
    }

    pub fn remove_alias(&self, alias: &str) -> WHResult<()> {
        let mut statement = self.run_statement::<(&str, &str)>("delete from aliases where alias = :alias", (":alias", alias));
        statement.next().unwrap();
        Ok(())
    }

    pub fn get_all_aliases(&self) -> WHResult<AliasList> {
        let mut statement = self.run_querry("select alias, path from aliases order by alias");
        let mut aliases = AliasList::new();
        while let Ok(State::Row) = statement.next() {
            aliases.add((
                statement.read::<String, _>("alias").unwrap(),
                statement.read::<String, _>("path").unwrap(),
            ).into());
        }
        Ok(aliases)
    }

    pub fn get_alias(&self, alias: &str) -> WHResult<String> {
        let mut statement = self.run_statement::<(&str, &str)>("select path from aliases where alias = :alias", (":alias", alias));
        if let Ok(State::Row) = statement.next() {
            Ok(statement.read::<String, _>("path").unwrap())
        } else {
            Err(WHError::AliasNotFound(alias.to_string()))
        }
    }
}
