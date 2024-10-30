use crate::{db::Database, error::WHResult, path::Path};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct EditAlias {
    /// The alias to edit
    pub alias: String,
    /// The new path to the location
    #[clap(default_value = ".")]
    pub path: Path,
}

impl EditAlias {
    pub fn run(&self, database: &Database) -> WHResult<()> {
        database.edit_alias(self.alias.as_str(), self.path.str())
    }
}