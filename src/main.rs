use clap::Parser;
use worm_hole::{
    cli::Args,
    error::{
        WHResult, 
        unwrap_worm_hole_error
    },
};

fn main() {
    unwrap_worm_hole_error(run());
}

fn run() -> WHResult<()> {
    let args = Args::parse();
    println!("{:?}", args);
    Ok(())
}
