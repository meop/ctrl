use std::io::Error;

use crate::command::run_and_wait;

use super::cmd_args;
use super::cmd_flag_long;
use super::Manager;

pub(super) struct Homebrew;

static PROGRAM: &str = "brew";

fn fix_fs_perm() -> Result<(), Error> {
    run_and_wait(
        &"sudo -S chown -R $(whoami) /usr/local/bin /usr/local/lib /usr/local/sbin".to_string(),
    )?;
    run_and_wait(&"chmod u+w /usr/local/bin /usr/local/lib /usr/local/sbin".to_string())
}

impl Manager for Homebrew {
    fn add(&self, list: &Vec<String>) -> Result<(), Error> {
        fix_fs_perm()?;
        run_and_wait(&format!(
            "{PROGRAM} install {}",
            cmd_args(list),
        ))
    }

    fn clean(&self) -> Result<(), Error> {
        run_and_wait(&format!("{PROGRAM} cleanup"))
    }

    fn list(&self, pattern: &Option<String>) -> Result<(), Error> {
        run_and_wait(&format!(
            "{PROGRAM} list {}",
            if pattern.as_deref().is_some() {
                String::from(pattern.as_deref().unwrap())
            } else {
                String::new()
            }
        ))
    }

    fn old(&self) -> Result<(), Error> {
        run_and_wait(&format!("{PROGRAM} outdated"))
    }

    fn remove(&self, list: &Vec<String>) -> Result<(), Error> {
        fix_fs_perm()?;
        run_and_wait(&format!(
            "{PROGRAM} uninstall {}",
            cmd_args(list),
        ))
    }
    
    fn search(&self, pattern: &String) -> Result<(), Error> {
        run_and_wait(&format!("{PROGRAM} search {}", pattern))
    }
    
    fn sync(&self, list: &Vec<String>) -> Result<(), Error> {
        fix_fs_perm()?;
        run_and_wait(&format!(
            "{PROGRAM} upgrade {} {}",
            cmd_flag_long("greedy"),
            cmd_args(list),
        ))
    }
}
