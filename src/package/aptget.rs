use std::io::Error;

use crate::command::run_and_wait;

use super::cmd_args;
use super::cmd_flag_long;
use super::Manager;

pub(super) struct Aptget;

static PROGRAM: &str = "apt";

fn repo_update() -> Result<(), Error> {
    run_and_wait(&format!("{PROGRAM} update"))
}

impl Manager for Aptget {
    fn add(&self, list: &Vec<String>) -> Result<(), Error> {
        repo_update()?;
        run_and_wait(&format!("{PROGRAM} install {}", cmd_args(list)))
    }

    fn clean(&self) -> Result<(), Error> {
        run_and_wait(&format!("{PROGRAM} autoclean"))?;
        run_and_wait(&format!("{PROGRAM} autoremove"))
    }

    fn list(&self, pattern: &Option<String>) -> Result<(), Error> {
        run_and_wait(&format!(
            "{PROGRAM} list {} {}",
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
            "{PROGRAM} list {}",
            cmd_flag_long("upgradeable"),
        ))
    }

    fn remove(&self, list: &Vec<String>) -> Result<(), Error> {
        run_and_wait(&format!(
            "{PROGRAM} purge {}",
            cmd_args(list)
        ))
    }

    fn search(&self, pattern: &String) -> Result<(), Error> {
        repo_update()?;
        run_and_wait(&format!("{PROGRAM} search {}", pattern))
    }

    fn sync(&self, list: &Vec<String>) -> Result<(), Error> {
        repo_update()?;
        run_and_wait(&format!(
            "{PROGRAM} {}",
            if list.len() > 0 {
                format!("install {}", cmd_args(list))
            } else {
                cmd_flag_long("dist-upgrade")
            }
        ))
    }
}
