use std::io::Error;

use crate::command::run_and_wait;

use super::cmd_args;
use super::cmd_flag_long;
use super::Manager;

pub(super) struct Homebrew {
    pub program: String,
}

fn repo_update(program: &String) -> Result<(), Error> {
    run_and_wait(&format!("{} update", program))
}

fn fix_fs_perm() -> Result<(), Error> {
    run_and_wait(
        &"sudo -S chown -R $(whoami) /usr/local/bin /usr/local/lib /usr/local/sbin".to_string(),
    )?;
    run_and_wait(&"chmod u+w /usr/local/bin /usr/local/lib /usr/local/sbin".to_string())
}

impl Manager for Homebrew {
    fn add(&self, list: &Vec<String>) -> Result<(), Error> {
        fix_fs_perm()?;
        repo_update(&self.program)?;
        run_and_wait(&format!("{} install {}", &self.program, cmd_args(list),))
    }

    fn clean(&self) -> Result<(), Error> {
        run_and_wait(&format!("{} cleanup", &self.program))
    }

    fn list(&self) -> Result<(), Error> {
        run_and_wait(&format!("{} list", &self.program))
    }

    fn outdated(&self) -> Result<(), Error> {
        repo_update(&self.program)?;
        run_and_wait(&format!("{} outdated", &self.program))
    }

    fn remove(&self, list: &Vec<String>) -> Result<(), Error> {
        fix_fs_perm()?;
        run_and_wait(&format!("{} uninstall {}", &self.program, cmd_args(list)))
    }

    fn search(&self, pattern: &String) -> Result<(), Error> {
        repo_update(&self.program)?;
        run_and_wait(&format!("{} search {}", &self.program, pattern))
    }

    fn sync(&self, list: &Vec<String>) -> Result<(), Error> {
        fix_fs_perm()?;
        repo_update(&self.program)?;
        run_and_wait(&format!(
            "{} upgrade {} {}",
            &self.program,
            cmd_flag_long("greedy"),
            cmd_args(list),
        ))
    }
}
