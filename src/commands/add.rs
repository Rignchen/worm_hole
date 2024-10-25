use clap::Parser;
use crate::{
    path::Path,
    db::Database,
    error::WHResult,
};

#[derive(Parser, Debug)]
pub struct AddAlias {
	/// The alias to use to go to the location
	pub alias: String,
	/// The real path to the location
	pub path: Path,
}

impl AddAlias {
    pub fn run(&self, database: &Database) -> WHResult<()> {
        database.add_alias(self.alias.as_str(), self.path.str())
    }
}
