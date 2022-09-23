use std::io::Error;

use crate::command::run_and_wait;

use super::cmd_args;
use super::cmd_flag_long;
use super::Manager;

pub(super) struct Pacman;

static PROGRAM: &str = "paru";

fn repo_update() -> Result<(), Error> {
    run_and_wait(&format!(
        "{PROGRAM} {} {}",
        cmd_flag_long("sync"),
        cmd_flag_long("refresh"),
    ))
}

impl Manager for Pacman {
    fn add(&self, list: &Vec<String>) -> Result<(), Error> {
        repo_update()?;
        run_and_wait(&format!(
            "{PROGRAM} {} {}",
            cmd_flag_long("sync"),
            cmd_args(list),
        ))
    }

    fn clean(&self) -> Result<(), Error> {
        run_and_wait(&format!(
            "{PROGRAM} {} {}",
            cmd_flag_long("sync"),
            cmd_flag_long("clean"),
        ))
    }

    fn list(&self, pattern: &Option<String>) -> Result<(), Error> {
        run_and_wait(&format!(
            "{PROGRAM} {} {}",
            cmd_flag_long("query"),
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
            "{PROGRAM} {} {}",
            cmd_flag_long("query"),
            cmd_flag_long("upgrades"),
        ))
    }
    
    fn remove(&self, list: &Vec<String>) -> Result<(), Error> {
        run_and_wait(&format!(
            "{PROGRAM} {} {} {} {}",
            cmd_flag_long("remove"),
            cmd_flag_long("recursive"),
            cmd_flag_long("nosave"),
            cmd_args(list),
        ))
    }

    fn search(&self, pattern: &String) -> Result<(), Error> {
        repo_update()?;
        run_and_wait(&format!(
            "{PROGRAM} {} {} {}",
            cmd_flag_long("query"),
            cmd_flag_long("search"),
            pattern,
        ))
    }

    fn sync(&self, list: &Vec<String>) -> Result<(), Error> {
        repo_update()?;
        run_and_wait(&format!(
            "{PROGRAM} {} {}",
            cmd_flag_long("sync"),
            if list.len() > 0 {
                cmd_args(list)
            } else {
                cmd_flag_long("sysupgrade")
            }
        ))
    }
}
