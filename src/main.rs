mod entry;
mod project;
mod cli;
mod data;
mod persistent;

use std::string::ParseError;
use chrono::{DateTime, Local};
use clap::{Arg, App, ArgMatches};
use cli::cli::build_cli;
use crate::project::project::Project;

fn project(matches: &ArgMatches, projects: &mut Vec<Project>) -> Result<String, String> {
    let command = matches.subcommand();

    match command.0 {
        "create" => {
            match create_project(command.1.unwrap()) {
                Ok(new_project) => {
                    projects.push(new_project);
                    Ok(String::from("New project created"))
                },
                Err(error) => error
            }
        },
        "rename" => unimplemented!(),
        "delete" => unimplemented!(),
        _ => unimplemented!()
    }
}
fn match_date_string(value: &str) -> Result<DateTime<Local>, chrono::ParseError> {
    match value {
        "now" => Ok(Local::now()),
        value => DateTime::parse_from_str(value, "DD-MM-YYYY-HH-MM")
    }
}
fn create_project(matches: &ArgMatches) -> Result<Project, String> {
    let name: &str = matches.value_of("name").unwrap();
    let description: Option<&str> = matches.value_of("description");
    let start = match_date_string(matches.value_of("start").unwrap());
    let end_value = matches.value_of("end");

    if start.is_err() {
        Err(String::from("Error when parsing start time"))
    }
    else if end_value.is_none() {
        Ok(Project::new(name, description, start.unwrap(), None, true))
    }
    else {
        match match_date_string(end_value.unwrap()) {
            Ok(end_value) => Ok(Project::new(name, description, start.unwrap(), Some(end_value)))
            Err(_) => Err(String::from("Error when parsing end time"))
        }
    }
}
fn main() {
    let matches = build_cli().get_matches();
    let subcommand = matches.subcommand();

    let mut projects: Vec<Project> = vec!();

    let result = match subcommand.0 {
        "project" => project(subcommand.1.unwrap(), &mut projects),
        "time" => unimplemented!("Not yet implemented"),
        "start" => unimplemented!("Not yet implemented"),
        "stop" => unimplemented!("Not yet implemented"),
        "summary" => unimplemented!("Not yet implemented"),
        "export" => unimplemented!("Not yet implemented"),
        _ => unimplemented!("Not yet implemented")
    };
}
