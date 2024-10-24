use clap::Parser;
use worm_hole::{
    cli::Args,
    error::{unwrap_worm_hole_error, WHResult},
    db::Database,
};

fn main() {
    unwrap_worm_hole_error(run());
}

fn run() -> WHResult<()> {
    let args = Args::parse();
    let database = Database::new(args.db_path.to_str().unwrap())?;
    println!("{:?}", args);

    Ok(())
}
