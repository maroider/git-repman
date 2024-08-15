use std::env;

use clap::{Parser, CommandFactory};

mod commands;

fn main() -> anyhow::Result<()> {
    if env::args_os().count() == 1 {
        Opts::command().print_help().unwrap();
        return Ok(());
    }

    let opts = Opts::parse();

    match opts.command {
        Command::Clone(clone) => clone.run(),
    }
}

#[derive(Parser)]
struct Opts {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser)]
enum Command {
    Clone(commands::Clone),
}
