use crate::{db::Database, error::WHResult};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct ListAliases {}

impl ListAliases {
    pub fn run(&self, database: &Database) -> WHResult<()> {
        let aliases = database.get_all_aliases()?;
        for alias in aliases {
            println!("{} -> {}", alias.0, alias.1);
        }
        Ok(())
    }
}
