use crate::{db::Database, error::WHResult};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct ListAliases {}

impl ListAliases {
    pub fn run(&self, database: &Database) -> WHResult<()> {
        let alias_list = AliasList::from(database.get_all_aliases()?);
        alias_list.print();
        Ok(())
    }
}

struct Alias {
    alias: String,
    path: String,
}

impl From<(String, String)> for Alias {
    fn from((alias, path): (String, String)) -> Self {
        Alias { alias, path }
    }
}

struct AliasList {
    aliases: Vec<Alias>,
    alias_max_len: usize,
}

impl From<Vec<(String, String)>> for AliasList {
    fn from(aliases: Vec<(String, String)>) -> Self {
        AliasList {
            aliases: aliases.clone().into_iter().map(Alias::from).collect(),
            alias_max_len: aliases.iter().map(|(alias, _)| alias.len()).max().unwrap_or(0),
        }
    }
}

impl AliasList {
    fn print(&self) {
        for alias in &self.aliases {
            println!("{:width$} -> {}", alias.alias, alias.path, width = self.alias_max_len);
        }
    }
}
