use log::error;
use std::env;
use std::result::Result;
use structopt::StructOpt;
use thiserror::Error;

use crate::common;

#[derive(Debug, PartialEq, StructOpt)]
pub struct ShowCommand {
    #[structopt(short, long)]
    name: String,
}

#[derive(Error, Debug)]
enum ShowCommandError {
    #[error("reading .rpilot file failed. Make sure this project is initialised properly.")]
    NotInitialized,

    #[error("the specified profile name does not exists for this project. Please make sure that you are passing the correvt name.")]
    NotExists,

    #[error("external library failed")]
    ExternalFail(#[from] std::io::Error),
}

pub fn execute(args: &ShowCommand) {
    match _execute(args) {
        Ok(_) => (),
        Err(e) => {
            error!("{}", e);
        }
    }
}

fn _execute(args: &ShowCommand) -> Result<(), ShowCommandError> {
    let pwd = env::current_dir()?;
    let project_dir = common::get_data_dir()?;
    let project_id = common::get_project_id(&pwd);

    if project_id.is_none() {
        return Err(ShowCommandError::NotInitialized);
    }

    let project_id = project_id.unwrap();
    let (_, project) = common::read_config(&project_dir, &project_id);

    let profile =
        common::select_profile(&project, &args.name).map_err(|_| ShowCommandError::NotExists)?;

    let (_, env) = common::read_env(&project_dir, &project_id, &profile.id);
    println!("The content of the env file");
    println!("---------------------------");
    println!("{}", env.unwrap_or_else(|| "".to_string()));

    Ok(())
}
