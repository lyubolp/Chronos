pub mod cli {
    use clap::{Arg, App};

    pub fn build_cli() -> App<'static, 'static> {
        App::new("Chronos")
            .version("0.1.0")
            .author("Lyuboslav Karev (@lyubolp)")
            .subcommand(project())
            .subcommand(time())
            .subcommand(start())
            .subcommand(stop())
            .subcommand(summary())
            .subcommand(export())
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
            .subcommand(time_add())
            .subcommand(time_remove())
            .subcommand(time_edit())
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
    fn time_remove() -> App<'static, 'static> {
        App::new("remove")
            .arg(
                Arg::with_name("project")
                    .short("p")
                    .long("project")
                    .help("Name of the project")
                    .required(true)
                    .value_name("project_name")
            )
            .arg(
                Arg::with_name("id")
                    .short("id")
                    .long("id")
                    .help("Id of the entry to remove")
                    .required(true)
                    .value_name("id")
            )
    }
    fn time_edit() -> App<'static, 'static> {
        App::new("edit")
            .arg(
                Arg::with_name("project")
                    .short("p")
                    .long("project")
                    .help("Name of the project")
                    .required(true)
                    .value_name("project_name")
            )
            .arg(
                Arg::with_name("id")
                    .short("id")
                    .long("id")
                    .help("Id of the entry to remove")
                    .required(true)
                    .value_name("id")
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

    fn start() -> App<'static, 'static> {
        App::new("start")
            .about("Starts tracking time for a given project")
            .arg(
                Arg::with_name("project")
                    .short("p")
                    .long("project")
                    .help("Name of the project")
                    .required(true)
                    .value_name("project_name")
            )
            .arg(
                Arg::with_name("comment")
                    .short("c")
                    .long("comment")
                    .help("New comment for entry")
                    .required(false)
                    .value_name("comment")
            )
    }

    fn stop() -> App<'static, 'static> {
        App::new("stop")
            .about("Stops tracking time for a given project")
            .arg(
                Arg::with_name("project")
                    .short("p")
                    .long("project")
                    .help("Name of the project")
                    .required(true)
                    .value_name("project_name")
            )
            .arg(
                Arg::with_name("comment")
                    .short("c")
                    .long("comment")
                    .help("New comment for entry")
                    .required(false)
                    .value_name("comment")
            )
    }

    fn summary() -> App<'static, 'static> {
        App::new("summary")
            .about("Shows summary for time spent on projects")
            .arg(
                Arg::with_name("project")
                    .short("p")
                    .long("project")
                    .help("Name of the project")
                    .required(false)
                    .value_name("project_name")
            )
    }

    fn export() -> App<'static, 'static> {
        App::new("export")
            .about("Export entries in JSON")
            .arg(
                Arg::with_name("project")
                    .short("p")
                    .long("project")
                    .help("Name of the project")
                    .required(false)
                    .value_name("project_name")
            )
    }
}