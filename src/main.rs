mod entry;
mod project;
mod cli;

use clap::{Arg, App};
use cli::cli::build_cli;

fn main() {
    let matches = build_cli().get_matches();

    match matches.subcommand().0 {
        "project" => unimplemented!("Not yet implemented"),
        "time" => unimplemented!("Not yet implemented"),
        "start" => unimplemented!("Not yet implemented"),
        "stop" => unimplemented!("Not yet implemented"),
        "summary" => unimplemented!("Not yet implemented"),
        "export" => unimplemented!("Not yet implemented"),
        _ => unimplemented!("Not yet implemented")
    };

    println!("{:?}", matches.subcommand().0);
}
