use std::io::Error;

use crate::command::run_and_wait;

use super::cmd_args;
use super::cmd_flag_long;
use super::Manager;

pub(super) struct Scoop {
    pub program: String,
}

fn repo_update(program: &String) -> Result<(), Error> {
    run_and_wait(&format!("{} update", program))
}

impl Manager for Scoop {
    fn add(&self, list: &Vec<String>) -> Result<(), Error> {
        repo_update(&self.program)?;
        run_and_wait(&format!("{} install {}", &self.program, cmd_args(list)))
    }

    fn clean(&self) -> Result<(), Error> {
        run_and_wait(&format!(
            "{} cleanup {} {}",
            &self.program,
            cmd_flag_long("all"),
            cmd_flag_long("cache")
        ))
    }

    fn list(&self) -> Result<(), Error> {
        run_and_wait(&format!("{} list", &self.program))
    }

    fn probe(&self) -> Result<(), Error> {
        repo_update(&self.program)?;
        run_and_wait(&format!("{} status", &self.program))
    }

    fn remove(&self, list: &Vec<String>) -> Result<(), Error> {
        run_and_wait(&format!("{} uninstall {}", &self.program, cmd_args(list)))
    }

    fn search(&self, pattern: &String) -> Result<(), Error> {
        repo_update(&self.program)?;
        run_and_wait(&format!("{} search {}", &self.program, pattern))
    }

    fn upgrade(&self, list: &Vec<String>) -> Result<(), Error> {
        repo_update(&self.program)?;
        run_and_wait(&format!(
            "{} update {}",
            &self.program,
            if list.len() > 0 {
                cmd_args(list)
            } else {
                cmd_flag_long("all")
            }
        ))
    }
}
