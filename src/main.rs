use clap::Parser;
use worm_hole::{
	cli::{Args, Command},
	db::Database,
	error::{unwrap_worm_hole_error, WHResult},
};

fn main() {
	unwrap_worm_hole_error(run());
}

fn run() -> WHResult<()> {
	let args = Args::parse();
	let database = Database::new(args.db_path.as_str())?;

	match args.cmd {
		Command::AddAlias(add) => {
			add.run(&database)?;
		}
		Command::RemoveAlias(remove) => {
			remove.run(&database)?;
		}
		Command::ListAliases(list) => {
			list.run(&database)?;
		}
		Command::SearchAliases(search) => {
			search.run(&database)?;
		}
		Command::Query(query) => {
			query.run(&database)?;
		}
		Command::EditAlias(edit) => {
			edit.run(&database)?;
		}
		Command::RenameAlias(rename) => {
			rename.run(&database)?;
		}
		Command::Init(init) => {
			init.run(&database, args.db_path.as_str())?;
		}
	}

	Ok(())
}
