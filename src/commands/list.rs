use log::error;
use std::env;
use std::result::Result;
use thiserror::Error;

use crate::common;

#[derive(Error, Debug)]
enum ListCommandError {
    #[error("reading .rpilot file failed. Make sure this project is initialised properly")]
    NotInitialized,

    #[error("Failed at reading the config")]
    ConfigReadError,

    #[error("external library failed")]
    ExternalFail(#[from] std::io::Error),
}

pub fn execute() {
    match _execute() {
        Ok(_) => (),
        Err(e) => error!("{}", e),
    }
}

fn _execute() -> Result<(), ListCommandError> {
    let pwd = env::current_dir()?;
    let project_dir = common::get_data_dir()?;
    let project_id = common::get_project_id(&pwd);

    if project_id.is_none() {
        return Err(ListCommandError::NotInitialized);
    }

    let project_id = project_id.unwrap();

    let (_, project) = common::read_config(&project_dir, &project_id)
        .map_err(|_| ListCommandError::ConfigReadError)?;

    print_entries(&project.entries);
    Ok(())
}

fn print_entries(entries: &[common::Entry]) {
    println!("Here are the list of the available profiles for this project");
    for entry in entries {
        println!("* {}", entry.name);
    }
}
