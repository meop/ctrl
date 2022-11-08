use std::io::Error;

use crate::command::run_and_wait;

use super::cmd_args;
use super::cmd_flag_long;
use super::Manager;

pub(super) struct Winget {
    pub program: String,
}

impl Manager for Winget {
    fn add(&self, list: &Vec<String>) -> Result<(), Error> {
        run_and_wait(&format!("{} install {}", &self.program, cmd_args(list)))
    }

    fn clean(&self) -> Result<(), Error> {
        Ok(())
    }

    fn list(&self) -> Result<(), Error> {
        run_and_wait(&format!("{} list", &self.program,))
    }

    fn outdated(&self) -> Result<(), Error> {
        run_and_wait(&format!("{} upgrade", &self.program))
    }

    fn remove(&self, list: &Vec<String>) -> Result<(), Error> {
        run_and_wait(&format!("{} uninstall {}", &self.program, cmd_args(list)))
    }

    fn search(&self, pattern: &String) -> Result<(), Error> {
        run_and_wait(&format!("{} search {}", &self.program, pattern))
    }

    fn sync(&self, list: &Vec<String>) -> Result<(), Error> {
        run_and_wait(&format!(
            "{} upgrade {}",
            &self.program,
            if list.len() > 0 {
                cmd_args(list)
            } else {
                cmd_flag_long("all")
            }
        ))
    }
}
