use std::process::ExitStatus;

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Command {
    Add {
        #[clap(multiple_values = true, required = true, help = "Package(s) to add")]
        list: Vec<String>,
        #[clap(short, long)]
        force: bool,
    },
    Find {
        #[clap(help = "Pattern to find external Package(s)")]
        pattern: String,
    },
    List {
        #[clap(help = "Pattern to list installed Package(s)")]
        pattern: Option<String>,
    },
    Out {},
    Rem {
        #[clap(multiple_values = true, required = true, help = "Package(s) to remove")]
        list: Vec<String>,
        #[clap(short, long)]
        force: bool,
    },
    Up {
        #[clap(multiple_values = true, help = "Package(s) to upgrade")]
        list: Vec<String>,
    },
}

trait PackageManager {
    fn add(&self, list: &Vec<String>, force: &bool) -> Result<ExitStatus, std::io::Error>;
    fn find(&self, pattern: &String) -> Result<ExitStatus, std::io::Error>;
    fn list(&self, pattern: &Option<String>) -> Result<ExitStatus, std::io::Error>;
    fn out(&self) -> Result<ExitStatus, std::io::Error>;
    fn rem(&self, list: &Vec<String>, force: &bool) -> Result<ExitStatus, std::io::Error>;
    fn up(&self, list: &Vec<String>) -> Result<ExitStatus, std::io::Error>;
}

mod apt;
mod homebrew;
mod paru;
mod winget;

fn get_package_manager() -> Box<dyn PackageManager> {
    if cfg!(target_os = "macos") {
        Box::new(homebrew::Pm)
    } else {
        Box::new(winget::Pm)
    }
}

impl super::InvokeCommand for Command {
    fn run(&self) -> Result<ExitStatus, std::io::Error> {
        let pm = get_package_manager();

        match self {
            Command::Add { list, force } => pm.add(list, force),
            Command::Find { pattern } => pm.find(pattern),
            Command::List { pattern } => pm.list(pattern),
            Command::Out {} => pm.out(),
            Command::Rem { list, force } => pm.rem(list, force),
            Command::Up { list } => pm.up(list),
        }
    }
}
