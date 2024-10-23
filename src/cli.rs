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
            #[clap(name = "cd")]
            ChangeDir(
                /// The alias of the location to move to
                struct ChangeDir {
                    pub alias: String,
                    /// Display the location path instead of changing directory
                    #[clap(short, long)]
                    pub query: bool,
                }
            ),
            /// Add a location alias to the list
            #[clap(name = "add")]
            AddAlias(
                struct AddAlias {
                    /// The real path to the location
                    pub path: String,
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
                    pub path: String,
                }
            ),
        }
    }
}

