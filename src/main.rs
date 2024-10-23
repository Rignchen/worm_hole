use clap::Parser;
use worm_hole::{
    cli::Args,
    error::{unwrap_worm_hole_error, WHResult},
};

fn main() {
    unwrap_worm_hole_error(run());
}

fn run() -> WHResult<()> {
    let args = Args::parse();
    println!("{:?}", args);
    Ok(())
}
