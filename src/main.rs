use log::LevelFilter;
use std::error::Error;

use clap::AppSettings;
use clap::Parser;
use clap::Subcommand;

use simple_logger::SimpleLogger;

mod command;
use command::Invoke;
mod package;
use package::CliCommand as PackageCliCommand;
mod self_update;

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
    Backup,
    #[clap(subcommand)]
    Package(PackageCliCommand),
    SelfUpdate,
}

impl CliCommand {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        match self {
            CliCommand::Backup => Ok(()),
            CliCommand::Package(x) => x.run().map(|_| ()).map_err(|e| e.into()),
            CliCommand::SelfUpdate => self_update::github(),
        }
    }
}

fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    let args = Cli::parse();
    let command = args.command;
    let _ = command.run();
}
