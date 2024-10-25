use clap::Parser;
use crate::{
    db::Database,
    error::WHResult,
};

#[derive(Parser, Debug)]
pub struct RemoveAlias {
	/// The alias to remove
	pub alias: String,
}

impl RemoveAlias {
    pub fn run(&self, database: &Database) -> WHResult<()> {
        database.remove_alias(self.alias.as_str())
    }
}
