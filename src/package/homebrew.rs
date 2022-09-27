use std::io::Error;

use crate::command::run_and_wait;
use crate::file::exists_in_path;

use super::cmd_args;
use super::cmd_flag_long;
use super::Manager;

pub(super) struct Homebrew;

fn get_program() -> String {
    if exists_in_path("brew") {
        "brew".to_string()
    } else {
        panic!("no homebrew compatible program found in path")
    }
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
        run_and_wait(&format!(
            "{} install {}",
            get_program(),
            cmd_args(list),
        ))
    }

    fn clean(&self) -> Result<(), Error> {
        run_and_wait(&format!("{} cleanup", get_program()))
    }

    fn list(&self) -> Result<(), Error> {
        run_and_wait(&format!(
            "{} list",
            get_program(),
        ))
    }

    fn old(&self) -> Result<(), Error> {
        run_and_wait(&format!("{} outdated", get_program()))
    }

    fn remove(&self, list: &Vec<String>) -> Result<(), Error> {
        fix_fs_perm()?;
        run_and_wait(&format!(
            "{} uninstall {}",
            get_program(),
            cmd_args(list),
        ))
    }
    
    fn search(&self, pattern: &String) -> Result<(), Error> {
        run_and_wait(&format!("{} search {}", get_program(), pattern))
    }
    
    fn sync(&self, list: &Vec<String>) -> Result<(), Error> {
        fix_fs_perm()?;
        run_and_wait(&format!(
            "{} upgrade {} {}",
            get_program(),
            cmd_flag_long("greedy"),
            cmd_args(list),
        ))
    }
}
