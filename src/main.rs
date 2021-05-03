use env_logger::Env;
use structopt::StructOpt;

pub mod commands;
pub mod common;
use commands::apply;
use commands::current;
use commands::edit;
use commands::init;
use commands::list;
use commands::new;
use commands::remove;
use commands::show;

#[derive(Debug, PartialEq, StructOpt)]
enum Rpilot {
    New(new::Args),
    Init,
    List,
    Current,
    Remove(remove::Args),
    Show(show::Args),
    Edit(edit::Args),
    Apply(apply::Args),
}

fn main() {
    let env = Env::new().filter_or("LOG", "info");
    env_logger::init_from_env(env);

    match Rpilot::from_args() {
        Rpilot::New(v) => new::execute(&v),
        Rpilot::Init => init::execute(),
        Rpilot::List => list::execute(),
        Rpilot::Current => current::execute(),
        Rpilot::Remove(v) => remove::execute(&v),
        Rpilot::Show(v) => show::execute(&v),
        Rpilot::Edit(v) => edit::execute(&v),
        Rpilot::Apply(v) => apply::execute(&v),
    };
}
