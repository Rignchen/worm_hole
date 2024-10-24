use clap::Parser;
use worm_hole::{
    cli::{
        Args,
        Command,
        AddAlias,
        RemoveAlias,
        ListAliases,
        EditAlias,
        Query,
    },
    error::{unwrap_worm_hole_error, WHResult, WHError},
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
        Command::RemoveAlias(RemoveAlias { alias }) => {
            database.remove_alias(alias.as_str())?;
        }
        Command::ListAliases(ListAliases {}) => {
            let aliases = database.get_all_aliases()?;
            for alias in aliases {
                println!("{} -> {}", alias.0, alias.1);
            }
        }
        Command::Query(Query { alias }) => {
            let path: String = database.get_alias(alias.as_str())?;
            if !std::path::Path::new(&path).exists() {
                return Err(WHError::PathOfAliasNotExist(alias, path));
            }
            println!("cd {}", path);
        }
        Command::EditAlias(EditAlias { alias, path }) => {
            database.edit_alias(alias.as_str(), path.str())?;
        }
    }

    Ok(())
}
