pub mod cli {
    use clap::{Arg, App};

    pub fn build_cli() -> App<'static, 'static> {
        App::new("Chronos")
            .version("0.1.0")
            .author("Lyuboslav Karev (@lyubolp)")
            .subcommand(project())
    }
    fn project() -> App<'static, 'static> {
        App::new("project")
            .about("Controls Chronos project")
            .subcommand(project_create())
            .subcommand(project_rename())
            .subcommand(project_delete())
    }

    fn project_create() -> App<'static, 'static> {
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
            .arg(
                Arg::with_name("end")
                    .short("e")
                    .long("end")
                    .help("End time of project in format \"DD-MM-YYYY-HH-MM\" or \"now\" for current time")
                    .required(false)
                    .value_name("end_time")
            )
    }
    fn project_rename() -> App<'static, 'static> {
        App::new("rename")
            .arg(
                Arg::with_name("old-name")
                    .short("o")
                    .long("old-name")
                    .help("Old name of the project")
                    .required(true)
                    .value_name("old_project_name")
            )
            .arg(
                Arg::with_name("new-name")
                    .short("n")
                    .long("new-name")
                    .help("New name of the project")
                    .required(true)
                    .value_name("new_project_name")
            )
    }
    fn project_delete() -> App<'static, 'static> {
        App::new("delete")
            .arg(
                Arg::with_name("name")
                    .short("n")
                    .long("name")
                    .help("Name of the project")
                    .required(true)
                    .value_name("project_name")
            )
    }

    fn time() -> App<'static, 'static> {
        App::new("time")
            .about("Controls time entries")
    }

    fn time_add() -> App<'static, 'static> {
        App::new("add")
            .arg(
                Arg::with_name("project")
                    .short("p")
                    .long("project")
                    .help("Name of the project")
                    .required(true)
                    .value_name("project_name")
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
            .arg(
                Arg::with_name("end")
                    .short("e")
                    .long("end")
                    .help("End time of project in format \"DD-MM-YYYY-HH-MM\" or \"now\" for current time")
                    .required(false)
                    .value_name("end_time")
            )
            .arg(
                Arg::with_name("amount")
                    .short("a")
                    .long("amount")
                    .help("Amount of minutes spent on the project")
                    .required(false)
                    .value_name("amount")
            )
            .arg(
                Arg::with_name("comment")
                    .short("c")
                    .long("comment")
                    .help("Comment for the current entry")
                    .required(false)
                    .value_name("comment")
            )
    }
}