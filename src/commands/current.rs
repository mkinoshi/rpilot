use log::error;
use std::env;
use std::result::Result;
use structopt::StructOpt;
use thiserror::Error;

use crate::common;

#[derive(Debug, PartialEq, StructOpt)]
pub struct CurrentCommand {
    #[structopt(short, long)]
    name: String,
}

#[derive(Error, Debug)]
enum CurrentCommandError {
    #[error("reading .rpilot file failed. Make sure this project is initialised properly.")]
    NotInitialized,

    #[error("the specified profile name does not exists for this project. Please make sure that you are passing the correvt name.")]
    NotExists,

    #[error(
        "no profile has been applied yet. Please use apply command to use the certain profile"
    )]
    NoProfileIsApplied,

    #[error("external library failed")]
    ExternalFail(#[from] std::io::Error),
}

pub fn execute() {
    match _execute() {
        Ok(_) => (),
        Err(e) => {
            error!("{}", e);
        }
    }
}

fn _execute() -> Result<(), CurrentCommandError> {
    let pwd = env::current_dir()?;
    let project_dir = common::get_data_dir()?;
    let project_id = common::get_project_id(&pwd);

    if project_id.is_none() {
        return Err(CurrentCommandError::NotInitialized);
    }

    let project_id = project_id.unwrap();
    let (_, project) = common::read_config(&project_dir, &project_id);

    if project.current_profile.as_ref().is_none() {
        return Err(CurrentCommandError::NoProfileIsApplied);
    }

    let empty_name = "".to_string();
    let current_profile = project
        .current_profile
        .as_ref()
        .as_ref()
        .unwrap_or(&empty_name);

    let profile = common::select_profile(&project, current_profile)
        .map_err(|_| CurrentCommandError::NotExists)?;

    let (_, env) = common::read_env(&project_dir, &project_id, &profile.id);
    println!("The current profile is {}", current_profile);
    println!("---------------------------");
    println!("{}", env.unwrap_or_else(|| "".to_string()));

    Ok(())
}
