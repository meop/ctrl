use std::io::Error;

use crate::command::run_cmd;
use crate::command::run_cmd_filtered;

use super::cmd_args;
use super::cmd_flag_long;
use super::Manager;

pub(super) struct Dnf {
    pub program: String,
}

fn repo_update(program: &String) -> Result<(), Error> {
    run_cmd(&format!("{} check-update", program))
}

impl Manager for Dnf {
    fn add(&self, list: &Vec<String>) -> Result<(), Error> {
        repo_update(&self.program)?;
        run_cmd(&format!("{} install {}", &self.program, cmd_args(list)))
    }

    fn clean(&self) -> Result<(), Error> {
        run_cmd(&format!("{} clean dbcache", &self.program))?;
        run_cmd(&format!("{} autoremove", &self.program))
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
            cmd_flag_long("upgrades"),
        ))
    }

    fn remove(&self, list: &Vec<String>) -> Result<(), Error> {
        run_cmd(&format!("{} remove {}", &self.program, cmd_args(list)))
    }

    fn search(&self, pattern: &String) -> Result<(), Error> {
        repo_update(&self.program)?;
        run_cmd(&format!("{} search {}", &self.program, pattern))
    }

    fn upgrade(&self, list: &Vec<String>) -> Result<(), Error> {
        repo_update(&self.program)?;
        run_cmd(&format!(
            "{} {}",
            &self.program,
            if list.len() > 0 {
                format!("upgrade {}", cmd_args(list))
            } else {
                "distro-sync".to_string()
            }
        ))
    }
}
