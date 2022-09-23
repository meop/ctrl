use std::io::Error;

use crate::command::run_and_wait;
use crate::file::exists_in_path;

use super::cmd_args;
use super::cmd_flag_long;
use super::Manager;

pub(super) struct Aptget;

fn get_program() -> String {
    if exists_in_path("apt") {
        "sudo apt".to_string()
    } else if exists_in_path("apt-get") {
        "sudo apt-get".to_string()
    } else {
        panic!("no apt-get compatible program found in path")
    }
}

fn repo_update() -> Result<(), Error> {
    run_and_wait(&format!("{} update", get_program()))
}

impl Manager for Aptget {
    fn add(&self, list: &Vec<String>) -> Result<(), Error> {
        repo_update()?;
        run_and_wait(&format!("{} install {}", get_program(), cmd_args(list)))
    }

    fn clean(&self) -> Result<(), Error> {
        run_and_wait(&format!("{} autoclean", get_program()))?;
        run_and_wait(&format!("{} autoremove", get_program()))
    }

    fn list(&self, pattern: &Option<String>) -> Result<(), Error> {
        run_and_wait(&format!(
            "{} list {} {}",
            get_program(),
            cmd_flag_long("installed"),
            if pattern.as_deref().is_some() {
                format!("| grep {}", String::from(pattern.as_deref().unwrap()))
            } else {
                String::new()
            }
        ))
    }

    fn old(&self) -> Result<(), Error> {
        repo_update()?;
        run_and_wait(&format!(
            "{} list {}",
            get_program(),
            cmd_flag_long("upgradeable"),
        ))
    }

    fn remove(&self, list: &Vec<String>) -> Result<(), Error> {
        run_and_wait(&format!("{} purge {}", get_program(), cmd_args(list)))
    }

    fn search(&self, pattern: &String) -> Result<(), Error> {
        repo_update()?;
        run_and_wait(&format!("{} search {}", get_program(), pattern))
    }

    fn sync(&self, list: &Vec<String>) -> Result<(), Error> {
        repo_update()?;
        let program = get_program();
        run_and_wait(&format!(
            "{} {}",
            program,
            if list.len() > 0 {
                format!("install {}", cmd_args(list))
            } else {
                cmd_flag_long(if program == "apt" {
                    "full-upgrade"
                } else {
                    "dist-upgrade"
                })
            }
        ))
    }
}
