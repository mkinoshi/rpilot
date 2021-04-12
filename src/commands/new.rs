use crate::common::get_data_dir;
use directories::{BaseDirs, ProjectDirs, UserDirs};
use std::fs;
use std::io::Result;
use std::path::PathBuf;
use structopt::StructOpt;
use uuid::Uuid;

#[derive(Debug, PartialEq, StructOpt)]
pub struct NewCommmand {
    #[structopt(short, long)]
    debug: bool,

    #[structopt(short, long, parse(from_os_str))]
    source: PathBuf,

    #[structopt(short, long)]
    name: String,
}

pub fn execute(args: &NewCommmand) -> Result<()> {
    println!("Arg: {:?}", args);

    // We have to store the env file, unique identifier, and metadata(including name)
    if let Some(proj_dirs) = ProjectDirs::from("org", "rpilot", "rp") {
        let env_dir = create_env_dir()?;
        copy_env(args.source, &env_dir)
    }
    // 3. Create a data file and store the data
    Ok(())
}

fn create_env_dir() -> Result<String> {
    let id = Uuid::new_v4();
    let data_dir = get_data_dir()?;
    let path = data_dir.join(id.to_string());
    let path_str = path.to_str().unwrap().to_owned();
    fs::create_dir_all(path)?;
    Ok(path_str)
}

fn copy_env(source: PathBuf, env_dir: &PathBuf) -> Result<()> {
    fs::copy(source, env_dir)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use regex::Regex;
    use std::env;
    use tempfile::tempfile;

    #[test]
    fn test_create_env_dir() {
        let target = create_env_dir().unwrap();
        let re = Regex::new(r"org.rpilot.rp/[0-9a-zA-Z-]*$").unwrap();
        assert_eq!(target.is_empty(), false);
        assert!(re.is_match(&target));
    }

    #[test]
    fn test_copy_env() {
        let mut test_file = tempfile();
        let mut dir = env::temp_dir();
        assert!(copy_env(test_file, &dir).ok());
    }
}
