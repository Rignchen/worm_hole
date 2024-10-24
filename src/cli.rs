use clap::Parser;
use std::path::PathBuf;
use std::fs::canonicalize;
use std::str::FromStr;

structstruck::strike! {
    /// Worm hole is a simple CLI tool to easily navigate between directories.
    ///
    /// With Worm Hole you can set a name for a directory and then use that name to move to that
    /// directory from anywhere.
    #[strikethrough[derive(Parser, Debug)]]
    #[clap(verbatim_doc_comment)]
    #[command(version)]
    pub struct Args {
        #[clap(subcommand)]
        pub cmd: pub enum Command {
            /// Move to the given location alias
            Query(
                /// The alias of the location to move to
                struct Query {
                    pub alias: String,
                }
            ),
            /// Add a location alias to the list
            #[clap(name = "add")]
            AddAlias(
                struct AddAlias {
                    /// The real path to the location
                    pub path: Path,
                    /// The alias to use to go to the location
                    pub alias: String,
                }
            ),
            /// Remove a location alias from the list
            #[clap(name = "rm")]
            RemoveAlias(
                struct RemoveAlias {
                    /// The alias to remove
                    pub alias: String,
                }
            ),
            /// List all location aliases
            #[clap(name = "ls")]
            ListAliases(
                struct ListAliases {}
            ),
            /// Edit the location of an alias
            #[clap(name = "edit")]
            EditAlias(
                struct EditAlias {
                    /// The alias to edit
                    pub alias: String,
                    /// The new path to the location
                    pub path: Path,
                }
            ),
        },
        // We use a PathBuf here because we here instead of a Path because we don't need to
        // canonicalize and don't want to fail if the path doesn't exist
        /// The path to the sqlite database file
        #[clap(long, default_value = "~/.wormhole.db")]
        pub db_path: PathBuf,
    }
}

#[derive(Debug, Clone)]
pub struct Path(String);
impl FromStr for Path {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = canonicalize(s).map_err(|e| e.to_string())?;
        Ok(Self(path.to_string_lossy().to_string()))
    }
}
impl Path {
    pub fn str(&self) -> &str {
        &self.0
    }
}

