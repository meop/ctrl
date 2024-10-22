use std::io::Error;

use crate::command::run_cmd;
use crate::command::run_cmd_filtered;

use super::cmd_args;
use super::cmd_flag_long;
use super::Manager;

pub(super) struct Homebrew {
    pub program: String,
}

fn repo_update(program: &String) -> Result<(), Error> {
    run_cmd(&format!("{} update", program))
}

impl Manager for Homebrew {
    fn add(&self, list: &Vec<String>) -> Result<(), Error> {
        repo_update(&self.program)?;
        run_cmd(&format!("{} install {}", &self.program, cmd_args(list),))
    }

    fn clean(&self) -> Result<(), Error> {
        run_cmd(&format!("{} cleanup --prune=all", &self.program))
    }

    fn list(&self, list: &Vec<String>) -> Result<(), Error> {
        run_cmd_filtered(&format!("{} list", &self.program), list)
    }

    fn probe(&self) -> Result<(), Error> {
        repo_update(&self.program)?;
        run_cmd(&format!("{} outdated", &self.program))
    }

    fn remove(&self, list: &Vec<String>) -> Result<(), Error> {
        run_cmd(&format!("{} uninstall {}", &self.program, cmd_args(list)))
    }

    fn search(&self, pattern: &String) -> Result<(), Error> {
        repo_update(&self.program)?;
        run_cmd(&format!("{} search {}", &self.program, pattern))
    }

    fn upgrade(&self, list: &Vec<String>) -> Result<(), Error> {
        repo_update(&self.program)?;
        run_cmd(&format!(
            "{} upgrade {} {}",
            &self.program,
            cmd_flag_long("greedy"),
            cmd_args(list),
        ))
    }
}
