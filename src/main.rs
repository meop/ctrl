use std::error::Error;

use clap::Parser;
use clap::Subcommand;

mod command;
mod file;
mod log;

mod package;
use package::Invoke as PackageInvoke;
mod me;
use me::Invoke as MeInvoke;

#[derive(Parser)]
#[clap(about)]
#[clap(disable_help_subcommand = true)]
#[clap(version)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// FS backup commands
    #[clap(visible_alias("b"))]
    Backup,

    /// OS package commands
    #[clap(subcommand)]
    #[clap(visible_alias("p"))]
    Package(package::Command),

    /// Self commands
    #[clap(subcommand)]
    #[clap(name = "self")]
    #[clap(visible_alias("s"))]
    Me(me::Command),
}

impl Command {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        match self {
            Command::Backup => Ok(()),
            Command::Package(c) => c.run().map(|_| ()).map_err(|e| e.into()),
            Command::Me(c) => c.run(),
        }
    }
}

fn main() {
    openssl_probe::init_ssl_cert_env_vars();

    let args = Cli::parse();
    let command = args.command;
    let _ = command.run();
}
