use std::io::Error;
use std::process::ExitStatus;

use clap::Subcommand;
// use which::which;

use crate::command::Invoke;

mod homebrew;

#[derive(Subcommand)]
pub enum CliCommand {
    Add {
        #[clap(multiple_values = true, required = true, help = "list")]
        list: Vec<String>,
        #[clap(short, long)]
        force: bool,
    },
    List {
        #[clap(help = "pattern")]
        pattern: Option<String>,
    },
    Outdated {
    },
    Remove {
        #[clap(multiple_values = true, required = true, help = "list")]
        list: Vec<String>,
        #[clap(short, long)]
        force: bool,
    },
    Search {
        #[clap(help = "pattern")]
        pattern: String,
    },
    Upgrade {
        #[clap(multiple_values = true, help = "list")]
        list: Vec<String>,
    },
}

pub trait Manager {
    fn add(&self, list: &Vec<String>, force: &bool) -> Result<ExitStatus, Error>;
    fn list(&self, pattern: &Option<String>) -> Result<ExitStatus, Error>;
    fn outdated(&self) -> Result<ExitStatus, Error>;
    fn remove(&self, list: &Vec<String>, force: &bool) -> Result<ExitStatus, Error>;
    fn search(&self, pattern: &String) -> Result<ExitStatus, Error>;
    fn upgrade(&self, list: &Vec<String>) -> Result<ExitStatus, Error>;
}

pub fn get_package_manager() -> Box<dyn Manager> {
    if cfg!(target_os = "macos") {
        Box::new(homebrew::Homebrew::default())
    // else if cfg!(target_os = "windows") {
    //     Box::new(winget::Pm)
    // } else if cfg!(target_os = "linux") {
    //     if let Ok(_) = which("apt") {
    //         Box::new(apt::Pm)
    //     } else {
    //         if let Ok(_) = which("paru") {
    //             Box::new(paru::Pm)
    //         } else {
    //             panic!("not implemented!")        
    //         }
    //     }
    } else {
        panic!("not implemented!")
    }
}

impl Invoke for CliCommand {
    fn run(&self) -> Result<ExitStatus, Error> {
        let pm = get_package_manager();

        match self {
            CliCommand::Add { list, force } => pm.add(list, force),
            CliCommand::List { pattern } => pm.list(pattern),
            CliCommand::Outdated {} => pm.outdated(),
            CliCommand::Remove { list, force } => pm.remove(list, force),
            CliCommand::Search { pattern } => pm.search(pattern),
            CliCommand::Upgrade { list } => pm.upgrade(list),
        }
    }
}
