use log::{debug, error, info};
use std::env;
use std::fs;
use std::io::Result as SimpleResult;
use std::path::{Path, PathBuf};
use std::result::Result;
use thiserror::Error;
use uuid::Uuid;

use crate::common;

#[derive(Error, Debug)]
enum InitCommandError {
    #[error("this directory is already initialized for rpilot")]
    AlreadyInitialized,

    #[error("external library failed")]
    ExternalFail(#[from] std::io::Error),
}

pub fn execute() {
    match _execute() {
        Ok(_) => info!("Successfully initialized rpilot for this directory"),
        Err(e) => error!("{}", e),
    }
}

fn _execute() -> Result<(), InitCommandError> {
    let pwd = env::current_dir()?;
    let project_id = common::get_project_id(&pwd);

    if project_id.is_some() {
        return Err(InitCommandError::AlreadyInitialized);
    }

    debug!("Generating a new rpilot id");
    let (path, id) = create_env_dir()?;
    info!("{}", format!("Generated a new rpilot id<{}>", &id));

    if let Some(p) = pwd.to_str() {
        debug!("{}", format!("Creating .rpilot at {}", &p));
    }
    write_id(id, &pwd)?;
    write_config_file(&path)?;
    Ok(())
}

fn create_env_dir() -> Result<(PathBuf, String), InitCommandError> {
    let id = Uuid::new_v4();
    let data_dir = common::get_data_dir()?;
    let path = data_dir.join(id.to_string());
    fs::create_dir_all(&path)?;
    Ok((path, id.to_string()))
}

fn write_id(id: String, current_dir: &Path) -> SimpleResult<()> {
    let config = current_dir.join(common::ID_FILENAME);
    fs::write(config, id)
}

fn write_config_file(path: &Path) -> SimpleResult<fs::File> {
    let config = path.join(common::CONFIG_FILENAME);
    fs::File::create(config)
}

#[cfg(test)]
mod test {
    use super::*;
    use regex::Regex;
    use tempdir::TempDir;

    #[test]
    fn test_create_env_dir() {
        let (env_dir, id) = create_env_dir().unwrap();
        let target = env_dir.to_str().unwrap().to_owned();
        let re = Regex::new(r"rp/[0-9a-zA-Z-]*$").unwrap();
        assert_eq!(target.is_empty(), false);
        assert!(re.is_match(&target));
        assert!(target.contains(&id));
    }

    #[test]
    fn test_write_id() {
        let tmp_dir = TempDir::new("test_write_id").unwrap();
        let tmp_dir_path = tmp_dir.path().to_owned();
        assert_eq!(write_id("1234".to_string(), &tmp_dir_path).is_ok(), true);
    }

    #[test]
    fn test_write_config() {
        let tmp_dir = TempDir::new("test_write_config").unwrap();
        let tmp_dir_path = tmp_dir.path().to_owned();
        assert_eq!(write_config_file(&tmp_dir_path).is_ok(), true);
    }
}
