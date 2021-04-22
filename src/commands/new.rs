use ring::digest::{Context, SHA256};
use savefile::save_file;
use std::fs;
use std::io::{BufReader, Read, Result, Error, ErrorKind};
use std::path::PathBuf;
use structopt::StructOpt;
use uuid::Uuid;

use crate::common;

#[derive(Debug, PartialEq, StructOpt)]
pub struct NewCommand {
    #[structopt(short, long)]
    debug: bool,

    #[structopt(short, long, parse(from_os_str))]
    source: PathBuf,

    #[structopt(short, long)]
    name: String,
}



pub fn execute(args: &NewCommmand) -> Result<()> {
    // We have to store the env file, unique identifier, and metadata(including name)
    // Linux: rp Windows/OSX: org.rpilot.rp
    let mut env_dir = create_env_dir()?;
    let mut config = env_dir.join("config");

    copy_env(&args.source, &mut env_dir)?;
    let hash = generate_file_hash(&args.source)?;
    let entry = common::Entry {
        hash: hash,
        name: String::from(&args.name),
        env_path: args.source.to_owned(),
    };
    insert_new_entry(entry, &mut config);
    Ok(())
}

fn create_env_dir() -> Result<PathBuf> {
    let id = Uuid::new_v4();
    let data_dir = common::get_data_dir()?;
    let path = data_dir.join(id.to_string());
    fs::create_dir_all(&path)?;
    Ok(path)
}

fn copy_env(source: &PathBuf, env_dir: &mut PathBuf) -> Result<()> {
    let source_file_name = source.file_name().unwrap();
    env_dir.push(source_file_name);
    fs::copy(source, env_dir)?;
    Ok(())
}

fn generate_file_hash(path: &PathBuf) -> Result<String> {
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

fn insert_new_entry(entry: common::Entry, config: &mut PathBuf) -> Result<()> {
    let config_path = config.to_str().unwrap();
    save_file(config_path, 0, &entry).map_err(|e| Error::new(ErrorKind::Other, "oh no!"))
}

#[cfg(test)]
mod test {
    use super::*;
    use regex::Regex;
    use std::env;
    use std::fs;
    use std::io::prelude::*;
    use tempdir::TempDir;

    #[test]
    fn test_create_env_dir() {
        let target = create_env_dir().unwrap().to_str().unwrap().to_owned();
        let re = Regex::new(r"rp/[0-9a-zA-Z-]*$").unwrap();
        assert_eq!(target.is_empty(), false);
        assert!(re.is_match(&target));
    }

    #[test]
    fn test_copy_env() {
        let tmp_dir = TempDir::new("test_copy_env").unwrap();
        let mut tmp_dir_path = tmp_dir.path().to_owned();
        tmp_dir_path.push(".env");
        fs::File::create(&tmp_dir_path).unwrap();

        let mut dir = env::temp_dir();
        assert_eq!(copy_env(&tmp_dir_path, &mut dir).is_ok(), true);
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
