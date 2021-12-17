mod entry;
mod project;
mod cli;

use clap::{Arg, App};
use cli::cli::build_cli;

fn main() {
    let matches = build_cli().get_matches();

    println!("{:?}", matches);

    println!("Hello, world!");
}
