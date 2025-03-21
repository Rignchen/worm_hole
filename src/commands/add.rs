use crate::{db::Database, error::WHResult, path::DirPath};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct AddAlias {
	/// The alias to use to go to the location
	pub alias: String,
	/// The real path to the location
	#[clap(default_value = ".")]
	pub path: DirPath,
}

impl AddAlias {
	pub fn run(&self, database: &Database) -> WHResult<()> {
		database.add_alias(self.alias.as_str(), self.path.str())
	}
}
