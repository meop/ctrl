use std::io::Error;

use crate::command::run_cmd;
use crate::command::run_cmd_filtered;

use super::cmd_args;
use super::cmd_flag_long;
use super::Manager;

pub(super) struct Aptget {
    pub program: String,
}

fn repo_update(program: &String) -> Result<(), Error> {
    run_cmd(&format!("{} update", program))
}

impl Manager for Aptget {
    fn add(&self, list: &Vec<String>) -> Result<(), Error> {
        repo_update(&self.program)?;
        run_cmd(&format!("{} install {}", &self.program, cmd_args(list)))
    }

    fn clean(&self) -> Result<(), Error> {
        run_cmd(&format!("{} autoclean", &self.program))
    }

    fn list(&self, list: &Vec<String>) -> Result<(), Error> {
        run_cmd_filtered(
            &format!("{} list {}", &self.program, cmd_flag_long("installed"),),
            list,
        )
    }

    fn probe(&self) -> Result<(), Error> {
        repo_update(&self.program)?;
        run_cmd(&format!(
            "{} list {}",
            &self.program,
            cmd_flag_long("upgradeable"),
        ))
    }

    fn remove(&self, list: &Vec<String>) -> Result<(), Error> {
        run_cmd(&format!("{} purge {}", &self.program, cmd_args(list)))?;
        run_cmd(&format!("{} autoremove", &self.program))
    }

    fn search(&self, pattern: &String) -> Result<(), Error> {
        repo_update(&self.program)?;
        run_cmd(&format!("{} search {}", &self.program, pattern))
    }

    fn upgrade(&self, list: &Vec<String>) -> Result<(), Error> {
        repo_update(&self.program)?;
        run_cmd(&format!(
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
