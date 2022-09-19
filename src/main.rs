use std::io::Error;
use std::process::ExitStatus;

use clap::AppSettings;
use clap::Parser;
use clap::Subcommand;

use simple_logger::SimpleLogger;

mod command;
use command::Invoke;
mod package;
use package::CliCommand as PackageCliCommand;

#[derive(Parser)]
#[clap(about, version)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
#[clap(global_setting(AppSettings::DisableHelpSubcommand))]
struct Cli {
    #[clap(subcommand)]
    command: CliCommand,
}

#[derive(Subcommand)]
enum CliCommand {
    #[clap(subcommand)]
    Package(PackageCliCommand),
}

impl Invoke for CliCommand {
    fn run(&self) -> Result<ExitStatus, Error> {
        match self {
            CliCommand::Package(x) => x.run(),
        }
    }
}

fn main() {
    SimpleLogger::new().init().unwrap();

    let args = Cli::parse();
    let command = args.command;
    let _ = command.run();
}
