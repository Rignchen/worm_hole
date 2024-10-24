use clap::Parser;
use worm_hole::{
    cli::{
        Args,
        Command,
        AddAlias,
        ListAliases,
    },
    error::{unwrap_worm_hole_error, WHResult},
    db::Database,
};

fn main() {
    unwrap_worm_hole_error(run());
}

fn run() -> WHResult<()> {
    let args = Args::parse();
    let database = Database::new(args.db_path.to_str().unwrap())?;

    match args.cmd {
        Command::AddAlias(AddAlias { alias, path }) => {
            database.add_alias(alias.as_str(), path.str())?;
        }
        Command::ListAliases(ListAliases {}) => {
            let aliases = database.get_all_aliases()?;
            for alias in aliases {
                println!("{} -> {}", alias.0, alias.1);
            }
        }
        _ => {
            eprintln!("Command not implemented");
        }
    }

    Ok(())
}
