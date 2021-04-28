use log::{debug, error, info};
use ring::digest::{Context, SHA256};
use std::env;
use std::fs;
use std::io::{BufReader, Read, Result as SimpleResult};
use std::path::{Path, PathBuf};
use std::result::Result;
use structopt::StructOpt;
use thiserror::Error;
use uuid::Uuid;

use crate::common;

#[derive(Debug, PartialEq, StructOpt)]
pub struct NewCommand {
    #[structopt(short, long, parse(from_os_str))]
    source: Option<PathBuf>,

    #[structopt(short, long)]
    name: String,
}

#[derive(Error, Debug)]
enum NewCommandError {
    #[error("reading .rpilot file failed. Make sure this project is initialised properly")]
    NotInitialized,

    #[error("The specified profile name already exists for this project. Please specify different name. If you want to modify the content of the existing env, then please use edit command.")]
    AlreadyExists,

    #[error("There was no valid env variables in the file")]
    NoValidEnv,

    #[error("Failed at saving a new rpilot entry ")]
    SaveFileError,

    #[error("external library failed")]
    ExternalFail(#[from] std::io::Error),
}

pub fn execute(args: &NewCommand) {
    match _execute(args) {
        Ok(_) => {
            info!("Successfully created a new rpilot entry");
        }
        Err(e) => {
            error!("{}", e);
        }
    }
}

fn _execute(args: &NewCommand) -> Result<(), NewCommandError> {
    let pwd = env::current_dir()?;
    let project_dir = common::get_data_dir()?;

    let project_id = common::get_project_id(&pwd);

    if project_id.is_none() {
        return Err(NewCommandError::NotInitialized);
    }

    let project_id = project_id.unwrap();
    info!("Retrieved proejct id is {}", project_id);

    let (mut config, mut project) = common::read_config(&project_dir, &project_id);

    let exists = check_if_profile_exists(&project.entries, &args.name);
    if exists {
        return Err(NewCommandError::AlreadyExists);
    }

    let (env_path, env_id) = get_env_internal_path(&project_dir, &project_id);

    match &args.source {
        Some(v) => {
            copy_env(&v, &env_path)?;
        }
        None => {
            generate_new_env_file(&env_path)?;
        }
    };

    info!(
        "{}",
        format!("Copied the env file to {}", env_path.to_str().unwrap())
    );

    let hash = generate_file_hash(&env_path)?;
    let entry = common::Entry {
        hash,
        name: String::from(&args.name),
        id: env_id,
    };

    project.entries.push(entry);
    common::save_config(project, &mut config).map_err(|_| NewCommandError::SaveFileError)?;
    Ok(())
}

fn check_if_profile_exists(entries: &[common::Entry], name: &str) -> bool {
    debug!("Checking whether the same profile name already exists in the entries");
    return entries.iter().any(|entry| entry.name == name);
}

fn get_env_internal_path(project_dir: &Path, id: &str) -> (PathBuf, String) {
    debug!("Retrieving the new path to copy this env file");

    let env_id = Uuid::new_v4().to_string();
    let data_dir = project_dir.join(id.to_string());
    let env_path = data_dir.join(&env_id);
    (env_path, env_id)
}

fn copy_env(source: &Path, env_path: &Path) -> SimpleResult<u64> {
    fs::copy(source, env_path)
}

fn generate_new_env_file(env_path: &Path) -> Result<(), NewCommandError> {
    let template = "# Please add new env values";
    let edited = edit::edit(template)?;

    debug!("Edited values: {}", edited);
    if edited == template {
        return Err(NewCommandError::NoValidEnv);
    }
    fs::write(env_path, edited)?;
    Ok(())
}

fn generate_file_hash(path: &Path) -> Result<String, NewCommandError> {
    let input = fs::File::open(path)?;
    let mut reader = BufReader::new(input);

    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    let digest = context.finish();
    let hash_bytes = digest.as_ref();
    let encoded_hash = base64::encode(hash_bytes);
    Ok(encoded_hash)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    use std::io::prelude::*;
    use tempdir::TempDir;

    #[test]
    fn test_copy_env() {
        let tmp_dir = TempDir::new("test_copy_env").unwrap();
        let mut tmp_dir_path = tmp_dir.path().to_owned();
        tmp_dir_path.push(".env");
        fs::File::create(&tmp_dir_path).unwrap();

        let tmp_dest_dir = TempDir::new("test_copy_env_dest").unwrap();
        let mut tmp_dest_path = tmp_dest_dir.path().to_owned();
        tmp_dest_path.push("test");
        assert_eq!(copy_env(&tmp_dir_path, &tmp_dest_path).is_ok(), true);
    }

    #[test]
    fn test_generate_file_hash() {
        let tmp_dir = TempDir::new("test_generate_file_hash").unwrap();
        let mut tmp_dir_path = tmp_dir.path().to_owned();
        tmp_dir_path.push(".env");
        let mut tmp_file = fs::File::create(&tmp_dir_path).unwrap();
        tmp_file.write_all(b"ENV=test").unwrap();
        assert_eq!(generate_file_hash(&tmp_dir_path).is_ok(), true);
    }
}
