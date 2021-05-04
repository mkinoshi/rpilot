use log::debug;
use log::error;
use log::info;
use std::env;
use std::fs;
use std::io::{stdin, stdout, Result as SimpleResult, Write};
use std::os::unix;
use std::path::Path;
use std::result::Result;
use structopt::StructOpt;
use thiserror::Error;

use crate::common;

#[derive(Debug, PartialEq, StructOpt)]
pub struct Args {
    #[structopt(short, long)]
    name: String,
}

#[derive(Error, Debug)]
enum ApplyCommandError {
    #[error("reading .rpilot file failed. Make sure this project is initialised properly.")]
    NotInitialized,

    #[error("the specified profile name does not exists for this project. Please make sure that you are passing the correvt name.")]
    NotExists,

    #[error("the command was aborted")]
    Aborted,

    #[error("Failed at saving the selected profile")]
    SaveFileError,

    #[error("Failed at reading the config")]
    ConfigReadError,

    #[error("external library failed")]
    ExternalFail(#[from] std::io::Error),
}

pub fn execute(args: &Args) {
    match _execute(args) {
        Ok(_) => info!("successfully updated the current .env file"),
        Err(e) => {
            error!("{}", e);
        }
    }
}

fn _execute(args: &Args) -> Result<(), ApplyCommandError> {
    let pwd = env::current_dir()?;
    let project_dir = common::get_data_dir()?;
    let project_id = common::get_project_id(&pwd);

    if project_id.is_none() {
        return Err(ApplyCommandError::NotInitialized);
    }

    let project_id = project_id.unwrap();
    let (mut config_path, mut project) = common::read_config(&project_dir, &project_id)
        .map_err(|_| ApplyCommandError::ConfigReadError)?;

    let profile =
        common::select_profile(&project, &args.name).map_err(|_| ApplyCommandError::NotExists)?;

    let (env_path, _) = common::read_env(&project_dir, &project_id, &profile.id);

    let should_apply = should_apply_env()?;

    if should_apply {
        set_symlink(&pwd, &env_path)?;
        project.current_profile = Box::new(Some(String::from(&args.name)));

        return common::save_config(&project, &mut config_path)
            .map_err(|_| ApplyCommandError::SaveFileError);
    }
    Err(ApplyCommandError::Aborted)
}

fn should_apply_env() -> Result<bool, ApplyCommandError> {
    let mut buffer = String::new();
    print!("This will create a symlink to .env file proceed if it is ok: [Y/N]");
    stdout().flush()?;
    let input = match stdin().read_line(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(_) => Err(ApplyCommandError::Aborted),
    }?;

    match input.as_str() {
        "Y\n" | "y\n" => Ok(true),
        v => {
            print!("{}", v);
            Ok(false)
        }
    }
}

fn set_symlink(pwd: &Path, env_path: &Path) -> SimpleResult<()> {
    let current_env_path = pwd.join(".env");

    if fs::remove_file(&current_env_path).is_err() {
        debug!(".env file does not exist in the current directory")
    }
    unix::fs::symlink(env_path, &current_env_path)?;
    let mut perms = fs::metadata(&current_env_path)?.permissions();
    perms.set_readonly(true);
    fs::set_permissions(&current_env_path, perms)?;
    Ok(())
}
