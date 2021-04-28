use log::{error, info};
use std::env;
use std::result::Result;
use structopt::StructOpt;
use thiserror::Error;

use crate::common;

#[derive(Debug, PartialEq, StructOpt)]
pub struct RemoveCommand {
    #[structopt(short, long)]
    name: String,
}

#[derive(Error, Debug)]
enum RemoveCommandError {
    #[error("reading .rpilot file failed. Make sure this project is initialised properly.")]
    NotInitialized,

    #[error("the specified profile name does not exists for this project. Please make sure that you are passing the correvt name.")]
    NotExists,

    #[error("failed at updating config")]
    SaveFileError,

    #[error("external library failed")]
    ExternalFail(#[from] std::io::Error),
}

pub fn execute(args: &RemoveCommand) {
    match _execute(args) {
        Ok(_) => {
            info!("successfully removed a profile");
        }
        Err(e) => {
            error!("{}", e);
        }
    }
}

fn _execute(args: &RemoveCommand) -> Result<(), RemoveCommandError> {
    let pwd = env::current_dir()?;
    let project_dir = common::get_data_dir()?;
    let project_id = common::get_project_id(&pwd);

    if project_id.is_none() {
        return Err(RemoveCommandError::NotInitialized);
    }

    let project_id = project_id.unwrap();

    let (mut config_path, mut project) = common::read_config(&project_dir, &project_id);

    remove_profile(&mut project, &args.name)?;

    common::save_config(project, &mut config_path)
        .map_err(|_| RemoveCommandError::SaveFileError)?;

    Ok(())
}

fn remove_profile(project: &mut common::Project, name: &str) -> Result<(), RemoveCommandError> {
    info!("removing the profile {}", name);
    match project.entries.iter().position(|entry| entry.name == name) {
        Some(ind) => {
            project.entries.remove(ind);

            if project
                .current_profile
                .as_ref()
                .as_ref()
                .unwrap_or(&"".to_string())
                == name
            {
                project.current_profile = Box::new(None)
            }
            Ok(())
        }
        None => Err(RemoveCommandError::NotExists),
    }
}

#[cfg(test)]
mod test {
    use super::common;
    use super::*;

    #[test]
    fn test_remove_profile() {
        let mut project = common::Project {
            entries: vec![
                common::Entry {
                    name: "first".to_string(),
                    hash: "test".to_string(),
                    id: "test".to_string(),
                },
                common::Entry {
                    name: "second".to_string(),
                    hash: "test".to_string(),
                    id: "test".to_string(),
                },
            ],
            current_profile: Box::new(Some("first".to_string())),
        };

        remove_profile(&mut project, "first").unwrap();

        assert_eq!(project.entries.len(), 1);
        assert_eq!(project.entries[0].name, "second");
        assert_eq!(project.current_profile.is_none(), true);
    }

    #[test]
    fn test_remove_profile_throws_error() {
        let mut project = common::Project {
            entries: vec![
                common::Entry {
                    name: "first".to_string(),
                    hash: "test".to_string(),
                    id: "test".to_string(),
                },
                common::Entry {
                    name: "second".to_string(),
                    hash: "test".to_string(),
                    id: "test".to_string(),
                },
            ],
            current_profile: Box::new(None),
        };

        assert_eq!(remove_profile(&mut project, "test").is_err(), true);
    }
}
