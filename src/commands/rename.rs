use crate::{db::Database, error::WHResult};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct RenameAlias {
	/// The alias to rename
	pub old_alias: String,
	/// The new alias to use
	pub new_alias: String,
}

impl RenameAlias {
	pub fn run(&self, database: &Database) -> WHResult<()> {
		database.rename_alias(&self.old_alias, &self.new_alias)
	}
}
