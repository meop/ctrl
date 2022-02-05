use std::process::ExitStatus;

use clap::AppSettings;
use clap::Parser;
use clap::Subcommand;

use simple_logger::SimpleLogger;

mod cfg;
mod pkg;

#[derive(Debug, Parser)]
#[clap(about, version)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
#[clap(global_setting(AppSettings::DisableHelpSubcommand))]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[clap(subcommand)]
    Pkg(pkg::Command),
}

trait InvokeCommand {
    fn run(&self) -> Result<ExitStatus, std::io::Error>;
}

impl InvokeCommand for Command {
    fn run(&self) -> Result<ExitStatus, std::io::Error> {
        match self {
            Command::Pkg(x) => x.run(),
        }
    }
}

fn main() {
    SimpleLogger::new().init().unwrap();

    let args = Cli::parse();
    let command = args.command;
    let _ = command.run();
}
