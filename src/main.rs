use std::error::Error;

use clap::AppSettings;
use clap::Parser;
use clap::Subcommand;

mod command;
mod file;
mod log;

mod package;
use package::Invoke as PackageInvoke;
mod release;
use release::Invoke as ReleaseInvoke;

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
    /// FS backup commands
    Backup,

    /// OS package commands
    #[clap(subcommand)]
    Package(package::Command),

    /// Self release commands
    #[clap(subcommand)]
    Release(release::Command),
}

impl Command {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        match self {
            Command::Backup => Ok(()),
            Command::Package(c) => c.run().map(|_| ()).map_err(|e| e.into()),
            Command::Release(c) => c.run(),
        }
    }
}

fn main() {
    openssl_probe::init_ssl_cert_env_vars();

    let args = Cli::parse();
    let command = args.command;
    let _ = command.run();
}
