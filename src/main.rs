use log::LevelFilter;
use std::error::Error;

use clap::AppSettings;
use clap::Parser;
use clap::Subcommand;

use simple_logger::SimpleLogger;

mod command;
mod maintain;
use maintain::Invoke as MaintainInvoke;
mod package;
use package::Invoke as PackageInvoke;

#[derive(Parser)]
#[clap(about, version)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
#[clap(global_setting(AppSettings::DisableHelpSubcommand))]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Backup,
    #[clap(subcommand)]
    Maintain(maintain::Command),
    #[clap(subcommand)]
    Package(package::Command),
}

impl Command {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        match self {
            Command::Backup => Ok(()),
            Command::Package(c) => c.run().map(|_| ()).map_err(|e| e.into()),
            Command::Maintain(c) => c.run(),
        }
    }
}

fn main() {
    openssl_probe::init_ssl_cert_env_vars();

    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .with_colors(true)
        .with_utc_timestamps()
        .init()
        .unwrap();

    let args = Cli::parse();
    let command = args.command;
    let _ = command.run();
}
