use crate::commands;
use clap::Parser;

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
            Query(commands::Query),
            /// Add a location alias to the list
            #[clap(name = "add")]
            AddAlias(commands::AddAlias),
            /// Remove a location alias from the list
            #[clap(name = "rm")]
            RemoveAlias(commands::RemoveAlias),
            /// List all location aliases
            #[clap(name = "ls")]
            ListAliases(commands::ListAliases),
            /// Edit the location of an alias
            #[clap(name = "edit")]
            EditAlias(commands::EditAlias),
            /// Change the name of an alias
            #[clap(name = "rename")]
            RenameAlias(commands::RenameAlias),
            /// Initialize the database and the bash config to make wormhole work
            Init(commands::Init),
        },
        /// The path to the sqlite database file
        #[clap(long, default_value_t = format!("{}/.wormhole.db", std::env::var("HOME").unwrap()))]
        pub db_path: String,
    }
}
