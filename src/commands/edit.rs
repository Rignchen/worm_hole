use clap::Parser;
use crate::{
    path::Path,
    db::Database,
    error::WHResult,
};

#[derive(Parser, Debug)]
pub struct EditAlias {
	/// The alias to edit
	pub alias: String,
	/// The new path to the location
	pub path: Path,
}

impl EditAlias {
    pub fn run(&self, database: &Database) -> WHResult<()> {
        database.edit_alias(self.alias.as_str(), self.path.str())
    }
}
