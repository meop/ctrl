use std::io::Error;

use crate::command::run_and_wait;

use super::cmd_args;
use super::cmd_flag_long;
use super::Manager;

pub(super) struct Aptget {
    pub program: String,
}

fn repo_update(program: &String) -> Result<(), Error> {
    run_and_wait(&format!("{} update", program))
}

impl Manager for Aptget {
    fn add(&self, list: &Vec<String>) -> Result<(), Error> {
        repo_update(&self.program)?;
        run_and_wait(&format!("{} install {}", &self.program, cmd_args(list)))
    }

    fn clean(&self) -> Result<(), Error> {
        let mut program = self.program.clone();
        if program.ends_with("pkg") {
            program = "apt".to_string();
        }
        run_and_wait(&format!("{} autoclean", &program))?;
        run_and_wait(&format!("{} autoremove", &program))
    }

    fn list(&self) -> Result<(), Error> {
        let mut program = self.program.clone();
        if program.ends_with("pkg") {
            program = "apt".to_string();
        }
        run_and_wait(&format!("{} list {}", &program, cmd_flag_long("installed"),))
    }

    fn probe(&self) -> Result<(), Error> {
        let mut program = self.program.clone();
        if program.ends_with("pkg") {
            program = "apt".to_string();
        }
        repo_update(&program)?;
        run_and_wait(&format!(
            "{} list {}",
            &program,
            cmd_flag_long("upgradeable"),
        ))
    }

    fn remove(&self, list: &Vec<String>) -> Result<(), Error> {
        let mut program = self.program.clone();
        if program.ends_with("pkg") {
            program = "apt".to_string();
        }
        run_and_wait(&format!("{} purge {}", &program, cmd_args(list)))
    }

    fn search(&self, pattern: &String) -> Result<(), Error> {
        let mut program = self.program.clone();
        if program.ends_with("pkg") {
            program = "apt".to_string();
        }
        repo_update(&program)?;
        run_and_wait(&format!("{} search {}", &program, pattern))
    }

    fn upgrade(&self, list: &Vec<String>) -> Result<(), Error> {
        repo_update(&self.program)?;
        run_and_wait(&format!(
            "{} {}",
            self.program,
            if list.len() > 0 {
                format!("install {}", cmd_args(list))
            } else {
                if self.program.ends_with("apt") {
                    "full-upgrade".to_string()
                } else if self.program.ends_with("apt-get") {
                    "dist-upgrade".to_string()
                } else {
                    "upgrade".to_string()
                }
            }
        ))
    }
}
