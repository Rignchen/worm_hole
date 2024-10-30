use crate::{db::Database, error::WHResult};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct ListAliases {}

impl ListAliases {
    pub fn run(&self, database: &Database) -> WHResult<()> {
        let alias_list = database.get_all_aliases()?;
        alias_list.print();
        Ok(())
    }
}

