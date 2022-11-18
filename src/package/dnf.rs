use std::io::Error;

use crate::command::run_and_wait;

use super::cmd_args;
use super::cmd_flag_long;
use super::Manager;

pub(super) struct Dnf {
    pub program: String,
}

fn repo_update(program: &String) -> Result<(), Error> {
    run_and_wait(&format!("{} check-update", program))
}

impl Manager for Dnf {
    fn add(&self, list: &Vec<String>) -> Result<(), Error> {
        repo_update(&self.program)?;
        run_and_wait(&format!("{} install {}", &self.program, cmd_args(list)))
    }

    fn clean(&self) -> Result<(), Error> {
        run_and_wait(&format!("{} clean dbcache", &self.program))?;
        run_and_wait(&format!("{} autoremove", &self.program))
    }

    fn list(&self) -> Result<(), Error> {
        run_and_wait(&format!(
            "{} list {}",
            &self.program,
            cmd_flag_long("installed"),
        ))
    }

    fn outdated(&self) -> Result<(), Error> {
        repo_update(&self.program)?;
        run_and_wait(&format!(
            "{} list {}",
            &self.program,
            cmd_flag_long("upgrades"),
        ))
    }

    fn remove(&self, list: &Vec<String>) -> Result<(), Error> {
        run_and_wait(&format!("{} remove {}", &self.program, cmd_args(list)))
    }

    fn search(&self, pattern: &String) -> Result<(), Error> {
        repo_update(&self.program)?;
        run_and_wait(&format!("{} search {}", &self.program, pattern))
    }

    fn sync(&self, list: &Vec<String>) -> Result<(), Error> {
        repo_update(&self.program)?;
        run_and_wait(&format!(
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
