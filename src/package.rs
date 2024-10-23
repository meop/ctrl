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

#[derive(Subcommand)]
pub(super) enum Command {
    /// Add package(s)
    #[clap(alias("install"))]
    #[clap(alias("in"))]
    #[clap(visible_alias("a"))]
    Add {
        /// Package(s)
        #[clap(num_args = 1..)]
        #[clap(required = true)]
        #[arg(value_name = "PACKAGES")]
        list: Vec<String>,
    },

    /// Clean local cache
    #[clap(alias("cl"))]
    #[clap(visible_alias("c"))]
    Clean {},

    /// List present package(s)
    #[clap(alias("query"))]
    #[clap(alias("ls"))]
    #[clap(visible_alias("l"))]
    List {
        #[clap(num_args = 1..)]
        #[arg(value_name = "PACKAGES")]
        list: Vec<String>,
    },

    /// Probe for outdated package(s)
    #[clap(alias("outdated"))]
    #[clap(alias("out"))]
    #[clap(alias("obsolete"))]
    #[clap(alias("ob"))]
    #[clap(visible_alias("p"))]
    Probe {},

    /// Remove package(s)
    #[clap(alias("uninstall"))]
    #[clap(alias("unin"))]
    #[clap(alias("un"))]
    #[clap(alias("delete"))]
    #[clap(alias("del"))]
    #[clap(alias("rem"))]
    #[clap(alias("rm"))]
    #[clap(visible_alias("r"))]
    Remove {
        /// Package(s)
        #[clap(num_args = 1..)]
        #[clap(required = true)]
        #[arg(value_name = "PACKAGES")]
        list: Vec<String>,
    },

    /// Search for remote package(s)
    #[clap(alias("find"))]
    #[clap(alias("fi"))]
    #[clap(alias("sr"))]
    #[clap(visible_alias("s"))]
    Search {
        /// Pattern to match
        #[arg(value_name = "PATTERN")]
        pattern: String,
    },

    /// Upgrade package(s)
    #[clap(alias("update"))]
    #[clap(alias("up"))]
    #[clap(visible_alias("u"))]
    Upgrade {
        /// Package(s) [optional]
        #[clap(num_args = 1..)]
        #[arg(value_name = "PACKAGES")]
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
                Command::List { list } => manager.list(list),
                Command::Probe {} => manager.probe(),
                Command::Remove { list } => manager.remove(list),
                Command::Search { pattern } => manager.search(pattern),
                Command::Upgrade { list } => manager.upgrade(list),
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
    fn list(&self, list: &Vec<String>) -> Result<(), Error>;
    fn probe(&self) -> Result<(), Error>;
    fn remove(&self, list: &Vec<String>) -> Result<(), Error>;
    fn search(&self, pattern: &String) -> Result<(), Error>;
    fn upgrade(&self, list: &Vec<String>) -> Result<(), Error>;
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
        }
        // Android == Termux
        Type::Debian | Type::Ubuntu | Type::Mint | Type::Android => {
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
        }
        Type::Fedora => {
            let program = if exists_in_path("dnf") {
                "sudo dnf".to_string()
            } else {
                panic!("no system package manager found in path")
            };
            managers.push(Box::new(Dnf { program }));
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
