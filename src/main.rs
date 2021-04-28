use env_logger::Env;
use structopt::StructOpt;

pub mod commands;
pub mod common;
use commands::apply::{execute as apply, ApplyCommand};
use commands::current::execute as current;
use commands::edit::{execute as edit, EditCommand};
use commands::init::execute as init;
use commands::list::execute as list;
use commands::new::{execute as new, NewCommand};
use commands::remove::{execute as remove, RemoveCommand};
use commands::show::{execute as show, ShowCommand};

#[derive(Debug, PartialEq, StructOpt)]
enum Rpilot {
    New(NewCommand),
    Init,
    List,
    Current,
    Remove(RemoveCommand),
    Show(ShowCommand),
    Edit(EditCommand),
    Apply(ApplyCommand),
}

fn main() {
    let env = Env::new().filter_or("LOG", "info");
    env_logger::init_from_env(env);

    match Rpilot::from_args() {
        Rpilot::New(v) => new(&v),
        Rpilot::Init => init(),
        Rpilot::List => list(),
        Rpilot::Current => current(),
        Rpilot::Remove(v) => remove(&v),
        Rpilot::Show(v) => show(&v),
        Rpilot::Edit(v) => edit(&v),
        Rpilot::Apply(v) => apply(&v),
    };
}
