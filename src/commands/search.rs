use crate::{db::Database, error::WHResult};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct SearchAliases {
	/// The pattern to search for in the alias paths
	pub pattern: String,
}

impl SearchAliases {
	pub fn run(&self, database: &Database) -> WHResult<()> {
		let alias_list = database.get_aliases_matching(format!("%{}%", self.pattern).as_str());
		if alias_list.is_ok() {
			alias_list.unwrap().print();
			Ok(())
		} else {
			Err(alias_list.err().unwrap())
		}
	}
}
