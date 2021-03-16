use std::env;

use clap::{Clap, IntoApp};

mod commands;

fn main() -> anyhow::Result<()> {
    if env::args_os().count() == 1 {
        Opts::into_app().print_help().unwrap();
        return Ok(());
    }

    let opts = Opts::parse();

    match opts.command {
        Command::Clone(clone) => clone.run(),
    }
}

#[derive(Clap)]
struct Opts {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Clap)]
enum Command {
    Clone(commands::Clone),
}
