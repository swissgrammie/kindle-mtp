mod cli;
mod commands;
mod device;
mod error;

use clap::Parser;
use cli::{Args, Command, Output};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = Args::parse();
    let output = Output::new(args.json, args.quiet);

    let result = match args.command {
        Command::Status => commands::run_status(&output),
        Command::Info => commands::run_info(&output),
        Command::Ls { path, long } => commands::run_ls(&output, &path, long),
        Command::Pull {
            remote,
            local,
            recursive,
        } => commands::run_pull(&output, &remote, &local, recursive),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            if !args.quiet {
                eprintln!("Error: {}", e);
            }
            e.exit_code()
        }
    }
}
