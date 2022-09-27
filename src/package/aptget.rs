use std::io::Error;

use crate::command::run_and_wait;
use crate::file::exists_in_path;

use super::cmd_args;
use super::cmd_flag_long;
use super::Manager;

pub(super) struct Aptget;

fn get_program() -> String {
    if exists_in_path("pkg") {
        "pkg".to_string()
    } else if exists_in_path("apt") {
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
        let mut program = get_program();
        if program.ends_with("pkg") {
            program = "apt".to_string();
        }
        run_and_wait(&format!("{} autoclean", &program))?;
        run_and_wait(&format!("{} autoremove", &program))
    }

    fn list(&self) -> Result<(), Error> {
        let mut program = get_program();
        if program.ends_with("pkg") {
            program = "apt".to_string();
        }
        run_and_wait(&format!(
            "{} list {}",
            &program,
            cmd_flag_long("installed"),
        ))
    }

    fn old(&self) -> Result<(), Error> {
        let mut program = get_program();
        if program.ends_with("pkg") {
            program = "apt".to_string();
        }
        repo_update()?;
        run_and_wait(&format!(
            "{} list {}",
            &program,
            cmd_flag_long("upgradeable"),
        ))
    }

    fn remove(&self, list: &Vec<String>) -> Result<(), Error> {
        let mut program = get_program();
        if program.ends_with("pkg") {
            program = "apt".to_string();
        }
        run_and_wait(&format!("{} purge {}", &program, cmd_args(list)))
    }

    fn search(&self, pattern: &String) -> Result<(), Error> {
        let mut program = get_program();
        if program.ends_with("pkg") {
            program = "apt".to_string();
        }
        repo_update()?;
        run_and_wait(&format!("{} search {}", &program, pattern))
    }

    fn sync(&self, list: &Vec<String>) -> Result<(), Error> {
        let program = get_program();
        repo_update()?;
        run_and_wait(&format!(
            "{} {}",
            &program,
            if list.len() > 0 {
                format!("install {}", cmd_args(list))
            } else {
                if program.ends_with("apt") {
                    "full-upgrade".to_string()
                } else if program.ends_with("apt-get") {
                    "dist-upgrade".to_string()
                } else {
                    "upgrade".to_string()
                }
            }
        ))
    }
}
