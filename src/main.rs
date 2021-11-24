mod entry;
mod project;

use clap::{Arg, App};

fn main() {
    let matches = App::new("Chronos")
        .version("0.1.0")
        .author("Lyuboslav Karev (@lyubolp)")
        .subcommand(
            App::new("project")
                .about("Controls Chronos project")
                .subcommand(
                    App::new("create")
                        .arg(
                            Arg::with_name("name")
                                .short("n")
                                .long("name")
                                .help("Name of the project")
                                .required(true)
                                .value_name("project_name")
                        )
                        .arg(
                            Arg::with_name("description")
                                .short("d")
                                .long("description")
                                .help("Description of the project")
                                .required(false)
                                .value_name("description")
                        )
                        .arg(
                            Arg::with_name("start")
                                .short("s")
                                .long("start")
                                .help("Start time of project in format \"DD-MM-YYYY-HH-MM\" or \"now\" for current time")
                                .required(false)
                                .value_name("start_time")
                                .default_value("now")
                        )
                )
                .subcommand(
                    App::new("rename")
                )
                .subcommand(
                    App::new("delete")
                )
        )
        .get_matches();

    /*
        .arg(Arg::with_name("language")
            .short("l")
            .long("language")
            .takes_value(true)
            .help("Language to run"))
     */

    println!("Hello, world!");
}
