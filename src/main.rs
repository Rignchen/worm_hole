use clap::Parser;
use worm_hole::{
    cli::{AddAlias, Args, Command, EditAlias, ListAliases, Query, RemoveAlias},
    db::Database,
    error::{unwrap_worm_hole_error, WHError, WHResult},
};

fn main() {
    unwrap_worm_hole_error(run());
}

fn run() -> WHResult<()> {
    let args = Args::parse();
    let database = Database::new(args.db_path.as_str())?;

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
        Command::Init(init) => {
            database.init();
            let bash_init_code =
                "alias 'wh=worm_hole'
                __worm_hole_cd() {
                    if [ -z \"$1\" ]
                    then
                        cd $HOME
                    else
                        CD=$(wh query $1) && \\builtin cd $CD
                    fi
                }
                alias 'whcd=__worm_hole_cd'";
            println!("{}", bash_init_code);
        }
    }

    Ok(())
}
