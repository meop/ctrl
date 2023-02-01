use std::io::Error;

use clap::Subcommand;
use os_info::Type;

mod aptget;
use aptget::Aptget;
mod dnf;
use dnf::Dnf;
mod pacman;
use pacman::Pacman;

mod homebrew;
use homebrew::Homebrew;

mod winget;
use winget::Winget;
mod scoop;
use scoop::Scoop;

use crate::file::exists_in_path;

pub enum ThirdPartyManager {
    #[cfg(unix)]
    Homebrew,
    #[cfg(windows)]
    Scoop
}

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
    Outdated {},

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
        let managers = get_managers();
        let mut result: Result<(), Error> = Ok(());

        for manager in managers.iter() {
            result = match self {
                Command::Add { list } => manager.add(list),
                Command::Clean {} => manager.clean(),
                Command::List {} => manager.list(),
                Command::Outdated {} => manager.outdated(),
                Command::Remove { list } => manager.remove(list),
                Command::Search { pattern } => manager.search(pattern),
                Command::Sync { list } => manager.sync(list),
            };

            if let Err(_) = result {
                break;
            }
        }

        return result;
    }
}

pub(super) trait Manager {
    fn add(&self, list: &Vec<String>) -> Result<(), Error>;
    fn clean(&self) -> Result<(), Error>;
    fn list(&self) -> Result<(), Error>;
    fn outdated(&self) -> Result<(), Error>;
    fn remove(&self, list: &Vec<String>) -> Result<(), Error>;
    fn search(&self, pattern: &String) -> Result<(), Error>;
    fn sync(&self, list: &Vec<String>) -> Result<(), Error>;
}

fn check_for_brew(mut Vec<Box<dyn Manager) {

}

fn get_managers() -> Vec<Box<dyn Manager>> {
    let os_type = os_info::get().os_type();
    let mut managers: Vec<Box<dyn Manager>> = Vec::new();

    match os_type {
        Type::Arch | Type::Manjaro => {
            let program = if exists_in_path("paru") {
                "paru".to_string()
            } else if exists_in_path("yay") {
                "yay".to_string()
            } else if exists_in_path("pacman") {
                "sudo pacman".to_string()
            } else {
                panic!("no system package manager found in path")
            };
            managers.push(Box::new(Pacman { program }));
            if exists_in_path("brew") {
                managers.push(Box::new(Homebrew {
                    program: "brew".to_string(),
                }));
            }
        }
        // Android == Termux
        Type::Android | Type::Debian | Type::Mint | Type::Raspbian | Type::Ubuntu => {
            let program = if exists_in_path("pkg") {
                "pkg".to_string()
            } else if exists_in_path("apt") {
                "sudo apt".to_string()
            } else if exists_in_path("apt-get") {
                "sudo apt-get".to_string()
            } else {
                panic!("no system package manager found in path")
            };
            managers.push(Box::new(Aptget { program }));
            if exists_in_path("brew") {
                managers.push(Box::new(Homebrew {
                    program: "brew".to_string(),
                }));
            }
        }
        Type::CentOS | Type::Fedora => {
            let program = if exists_in_path("dnf") {
                "sudo dnf".to_string()
            } else {
                panic!("no system package manager found in path")
            };
            managers.push(Box::new(Dnf { program }));
            if exists_in_path("brew") {
                managers.push(Box::new(Homebrew {
                    program: "brew".to_string(),
                }));
            }
        }
        Type::Macos => {
            if exists_in_path("brew") {
                managers.push(Box::new(Homebrew {
                    program: "brew".to_string(),
                }));
            }
        }
        Type::Windows => {
            if exists_in_path("winget") {
                managers.push(Box::new(Winget {
                    program: "winget".to_string(),
                }));
            }
            if exists_in_path("scoop") {
                managers.push(Box::new(Scoop {
                    program: "scoop".to_string(),
                }));
            }
        }
        _ => panic!("not yet supported: {os_type}"),
    }

    return managers;
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
