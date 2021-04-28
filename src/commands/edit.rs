use log::error;
use std::env;
use std::fs;
use std::io::Result as SimpleResult;
use std::path::Path;
use std::result::Result;
use structopt::StructOpt;
use thiserror::Error;

use crate::common;

#[derive(Debug, PartialEq, StructOpt)]
pub struct EditCommand {
    #[structopt(short, long)]
    name: String,
}

#[derive(Error, Debug)]
enum EditCommandError {
    #[error("reading .rpilot file failed. Make sure this project is initialised properly.")]
    NotInitialized,

    #[error("the specified profile name does not exists for this project. Please make sure that you are passing the correvt name.")]
    NotExists,

    #[error("external library failed")]
    ExternalFail(#[from] std::io::Error),
}

pub fn execute(args: &EditCommand) {
    match _execute(args) {
        Ok(_) => (),
        Err(e) => {
            error!("{}", e);
        }
    }
}

fn _execute(args: &EditCommand) -> Result<(), EditCommandError> {
    let pwd = env::current_dir()?;
    let project_dir = common::get_data_dir()?;
    let project_id = common::get_project_id(&pwd);

    if project_id.is_none() {
        return Err(EditCommandError::NotInitialized);
    }

    let project_id = project_id.unwrap();
    let (_, project) = common::read_config(&project_dir, &project_id);

    let profile =
        common::select_profile(&project, &args.name).map_err(|_| EditCommandError::NotExists)?;

    let (env_path, env) = common::read_env(&project_dir, &project_id, &profile.id);
    edit_and_save(&env_path, env.unwrap_or_else(|| "".to_string()))?;

    Ok(())
}

fn edit_and_save(env_path: &Path, env: String) -> SimpleResult<()> {
    let edited = edit::edit(env)?;
    fs::write(env_path, edited)?;
    Ok(())
}
