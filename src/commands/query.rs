use crate::{
	db::Database,
	error::{WHError, WHResult},
};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Query {
	/// The alias of the location to move to
	pub alias: String,
}

impl Query {
	pub fn run(&self, database: &Database) -> WHResult<()> {
		let path: String = database.get_alias(self.alias.as_str())?;
		if !std::path::Path::new(&path).exists() {
			return Err(WHError::PathOfAliasNotExist(self.alias.clone(), path));
		}
		println!("{}", path);
		Ok(())
	}
}
