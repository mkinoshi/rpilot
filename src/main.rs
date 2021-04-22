use exitfailure::ExitFailure;
use structopt::StructOpt;

pub mod commands;
pub mod common;
use commands::new::{execute, NewCommmand};
use commands::init::{execute, InitCommand};

#[derive(Debug, PartialEq, StructOpt)]
enum Rpilot {
    New(NewCommmand),
    Init(InitCommand)
}

fn main() -> Result<(), ExitFailure> {
    let args = Rpilot::from_args();
    match args {
        Rpilot::New(v) => execute(&v),
        _ => {
            panic!("Unmatching command");
        }
    };
    Ok(())
}
