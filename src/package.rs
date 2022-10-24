use std::io::Error;

use clap::Subcommand;
use os_info::Type;

mod aptget;
use aptget::Aptget;
mod homebrew;
use homebrew::Homebrew;
mod pacman;
use pacman::Pacman;

mod winget;
use winget::Winget;

#[derive(Subcommand)]
pub(super) enum Command {
    /// Add new package(s)
    Add {
        /// List of packages [required]
        #[clap(num_args = 1..)]
        #[clap(required = true)]
        list: Vec<String>,
    },

    /// Clean local cache
    Clean {},

    /// List local installed package(s)
    List {},

    /// List out-of-date installed packages
    Old {},

    /// Remove local installed package(s)
    Remove {
        /// List of packages [required]
        #[clap(num_args = 1..)]
        #[clap(required = true)]
        list: Vec<String>,
    },

    /// Search for remote package(s)
    Search {
        /// Optional pattern to match
        pattern: String,
    },

    /// Sync to latest installed package(s)
    Sync {
        /// List of packages [optional]
        #[clap(num_args = 1..)]
        list: Vec<String>,
    },
}

pub(super) trait Invoke {
    fn run(&self) -> Result<(), Error>;
}

impl Invoke for Command {
    fn run(&self) -> Result<(), Error> {
        let pm = get_manager();

        match self {
            Command::Add { list } => pm.add(list),
            Command::Clean {} => pm.clean(),
            Command::List {} => pm.list(),
            Command::Old {} => pm.old(),
            Command::Remove { list } => pm.remove(list),
            Command::Search { pattern } => pm.search(pattern),
            Command::Sync { list } => pm.sync(list),
        }
    }
}

pub(super) trait Manager {
    fn add(&self, list: &Vec<String>) -> Result<(), Error>;
    fn clean(&self) -> Result<(), Error>;
    fn list(&self) -> Result<(), Error>;
    fn old(&self) -> Result<(), Error>;
    fn remove(&self, list: &Vec<String>) -> Result<(), Error>;
    fn search(&self, pattern: &String) -> Result<(), Error>;
    fn sync(&self, list: &Vec<String>) -> Result<(), Error>;
}

fn get_manager() -> Box<dyn Manager> {
    let os_type = os_info::get().os_type();
    match os_type {
        Type::Arch | Type::Manjaro => Box::new(Pacman),
        Type::Android | Type::Debian | Type::Mint | Type::Raspbian | Type::Ubuntu => {
            Box::new(Aptget)
        }
        Type::Macos => Box::new(Homebrew),
        Type::Windows => Box::new(Winget),
        _ => panic!("not yet supported: {os_type}"),
    }
}

fn cmd_args(list: &Vec<String>) -> String {
    list.join(" ")
}

fn cmd_flag_long(value: &str) -> String {
    format!("--{}", value.to_lowercase())
}

fn cmd_flag_short(value: &str) -> String {
    format!("-{}", value.to_uppercase())
}
