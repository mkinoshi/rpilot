use directories::ProjectDirs;
use std::path::{Path, PathBuf};

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
    assert_eq!(target.contains("org.rpilot.rp"), true);
}
