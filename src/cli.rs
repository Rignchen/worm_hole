use clap::Parser;

structstruck::strike! {
    #[strikethrough[derive(Parser, Debug)]]
    #[clap(verbatim_doc_comment)]
    #[command(version)]
    pub struct Args {
        #[clap(subcommand)]
        pub cmd: pub enum Command {
            #[clap(name = "cd")]
            ChangeDir(
                struct ChangeDir {
                    pub alias: String,
                    #[clap(short, long)]
                    pub query: bool,
                }
            ),
            #[clap(name = "add")]
            AddAlias(
                struct AddAlias {
                    pub path: String,
                    pub alias: String,
                }
            ),
            #[clap(name = "rm")]
            RemoveAlias(
                struct RemoveAlias {
                    pub alias: String,
                }
            ),
            #[clap(name = "ls")]
            ListAliases(
                struct ListAliases {}
            ),
            #[clap(name = "edit")]
            EditAlias(
                struct EditAlias {
                    pub alias: String,
                    pub path: String,
                }
            ),
        }
    }
}

