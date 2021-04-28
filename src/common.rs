use std::fs;
use std::io::{Error, ErrorKind};

use directories::ProjectDirs;
use log::debug;
use savefile::{load_file, save_file};
use savefile_derive::Savefile;
use std::path::{Path, PathBuf};

pub const ID_FILENAME: &str = ".rpilot";
pub const CONFIG_FILENAME: &str = "config";

#[derive(Savefile, Debug)]
pub struct Entry {
    pub name: String,
    pub hash: String,
    pub id: String,
}

#[derive(Savefile, Debug)]
pub struct Project {
    pub entries: Vec<Entry>,
    pub current_profile: Box<Option<String>>,
}

pub fn get_data_dir() -> std::io::Result<PathBuf> {
    match ProjectDirs::from("org", "rpilot", "rp") {
        Some(proj_dirs) => Ok(PathBuf::from(proj_dirs.data_dir())),
        _ => panic!("Failed at get_data_dir()"),
    }
}

pub fn get_project_id(current_dir: &Path) -> Option<String> {
    debug!("Reading .rpilot in the current directory");
    let config = current_dir.join(ID_FILENAME);
    match fs::read_to_string(config) {
        Ok(v) => Some(v),
        Err(_) => None,
    }
}

pub fn read_config(project_dir: &Path, id: &str) -> (PathBuf, Project) {
    debug!("Reading the config file for this project");
    let data_dir = project_dir.join(id.to_string());
    let config_path = data_dir.join(CONFIG_FILENAME);
    let config_path_name = config_path.to_str().unwrap();
    let content = match load_file(&config_path_name, 0) {
        Ok(v) => v,
        Err(_) => Project {
            entries: Vec::new(),
            current_profile: Box::new(None),
        },
    };

    (config_path, content)
}

pub fn read_env(
    project_dir: &Path,
    project_id: &str,
    profile_id: &str,
) -> (PathBuf, Option<String>) {
    let env_path = project_dir.join(project_id);
    let env_path = env_path.join(profile_id);
    let env = match fs::read_to_string(&env_path) {
        Ok(v) => Some(v),
        Err(_) => None,
    };
    (env_path, env)
}

pub fn save_config(project: Project, config_path: &mut PathBuf) -> Result<(), Error> {
    let config_path = config_path.to_str().unwrap();
    save_file(config_path, 0, &project)
        .map_err(|_| Error::new(ErrorKind::Other, "Failed at saving config"))
}

pub fn select_profile<'a>(project: &'a Project, name: &str) -> Result<&'a Entry, Error> {
    match project.entries.iter().position(|entry| entry.name == name) {
        Some(ind) => Ok(&project.entries[ind]),
        None => Err(Error::new(
            ErrorKind::Other,
            "Failed at getting the specified profile",
        )),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tempdir::TempDir;

    #[test]
    fn test_get_data_dir() {
        let proj = get_data_dir().unwrap();
        let target = proj.to_str().unwrap();
        // Linux only contains the last word of project dir
        assert_eq!(target.contains("rp"), true);
    }

    #[test]
    fn test_get_project_id() {
        let tmp_dir = TempDir::new("test_check_if_initialized").unwrap();
        let tmp_dir_path = tmp_dir.path().to_owned();
        let project_id = get_project_id(&tmp_dir_path);

        assert_eq!(project_id, None);

        let config = tmp_dir_path.join(ID_FILENAME);
        fs::write(config, "1234").unwrap();
        let project_id = get_project_id(&tmp_dir_path);
        assert_eq!(project_id.is_some(), true);
    }

    #[test]
    fn test_save_config() {
        let tmp_dir = TempDir::new("test_insert_new_entry").unwrap();
        let tmp_dir_path = tmp_dir.path().to_owned();
        let mut config = tmp_dir_path.join(CONFIG_FILENAME);

        let entry = Entry {
            name: "test".to_string(),
            hash: "test hash".to_string(),
            id: "test id".to_string(),
        };
        let project = Project {
            entries: vec![entry],
            current_profile: Box::new(Some("test".to_string())),
        };
        assert_eq!(save_config(project, &mut config).is_ok(), true);
    }

    #[test]
    fn test_save_config_and_read_config() {
        let tmp_dir = TempDir::new("test_insert_new_entry").unwrap();
        let tmp_dir_path = tmp_dir.path().to_owned();
        let mut config = tmp_dir_path.join(CONFIG_FILENAME);

        let entry = Entry {
            name: "test".to_string(),
            hash: "test hash".to_string(),
            id: "test id".to_string(),
        };
        let project = Project {
            entries: vec![entry],
            current_profile: Box::new(None),
        };
        assert_eq!(save_config(project, &mut config).is_ok(), true);

        let (read_config, project) = read_config(&tmp_dir_path, "");
        assert_eq!(project.entries.len(), 1);
        assert_eq!(project.entries[0].name, "test");
        assert_eq!(read_config.to_str(), config.to_str());
    }

    #[test]
    fn test_select_profile() {
        let project = Project {
            entries: vec![
                Entry {
                    name: "first".to_string(),
                    hash: "test".to_string(),
                    id: "first id".to_string(),
                },
                Entry {
                    name: "second".to_string(),
                    hash: "test".to_string(),
                    id: "second id".to_string(),
                },
            ],
            current_profile: Box::new(None),
        };

        let profile = select_profile(&project, "first").unwrap();
        assert_eq!(profile.id, "first id");
    }
}
