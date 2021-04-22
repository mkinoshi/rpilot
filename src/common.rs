use directories::ProjectDirs;
use savefile_derive::Savefile;
use std::path::PathBuf;

#[derive(Savefile)]
pub struct Entry {
    pub name: String,
    pub hash: String,
    pub env_path: PathBuf,
}

pub struct Project {
    pub name: String,
    pub project_dir: PathBuf,
    pub entries: Vec<Entry>,
}

pub struct Rpilot {
    pub entries: Vec<Project>,
}

pub fn get_data_dir() -> std::io::Result<PathBuf> {
    match ProjectDirs::from("org", "rpilot", "rp") {
        Some(proj_dirs) => Ok(PathBuf::from(proj_dirs.data_dir())),
        _ => panic!("Failed at get_data_dir()"),
    }
}

#[test]
fn test_get_data_dir() {
    let proj = get_data_dir().unwrap();
    let target = proj.to_str().unwrap();
    // Linux only contains the last word of project dir
    assert_eq!(target.contains("rp"), true);
}
