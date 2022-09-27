use std::io::Error;

use crate::command::run_and_wait;
use crate::file::exists_in_path;

use super::cmd_args;
use super::cmd_flag_long;
use super::Manager;

pub(super) struct Pacman;

fn get_program() -> String {
    if exists_in_path("paru") {
        "paru".to_string()
    } else if exists_in_path("yay") {
        "yay".to_string()
    } else if exists_in_path("pacman") {
        "sudo pacman".to_string()
    } else {
        panic!("no pacman compatible program found in path")
    }
}

fn repo_update() -> Result<(), Error> {
    run_and_wait(&format!(
        "{} {} {}",
        get_program(),
        cmd_flag_long("sync"),
        cmd_flag_long("refresh"),
    ))
}

impl Manager for Pacman {
    fn add(&self, list: &Vec<String>) -> Result<(), Error> {
        repo_update()?;
        run_and_wait(&format!(
            "{} {} {}",
            get_program(),
            cmd_flag_long("sync"),
            cmd_args(list),
        ))
    }

    fn clean(&self) -> Result<(), Error> {
        run_and_wait(&format!(
            "{} {} {}",
            get_program(),
            cmd_flag_long("sync"),
            cmd_flag_long("clean"),
        ))
    }

    fn list(&self) -> Result<(), Error> {
        run_and_wait(&format!(
            "{} {}",
            get_program(),
            cmd_flag_long("query"),
        ))
    }

    fn old(&self) -> Result<(), Error> {
        repo_update()?;
        run_and_wait(&format!(
            "{} {} {}",
            get_program(),
            cmd_flag_long("query"),
            cmd_flag_long("upgrades"),
        ))
    }
    
    fn remove(&self, list: &Vec<String>) -> Result<(), Error> {
        run_and_wait(&format!(
            "{} {} {} {} {}",
            get_program(),
            cmd_flag_long("remove"),
            cmd_flag_long("recursive"),
            cmd_flag_long("nosave"),
            cmd_args(list),
        ))
    }

    fn search(&self, pattern: &String) -> Result<(), Error> {
        repo_update()?;
        run_and_wait(&format!(
            "{} {} {} {}",
            get_program(),
            cmd_flag_long("query"),
            cmd_flag_long("search"),
            pattern,
        ))
    }

    fn sync(&self, list: &Vec<String>) -> Result<(), Error> {
        repo_update()?;
        run_and_wait(&format!(
            "{} {} {}",
            get_program(),
            cmd_flag_long("sync"),
            if list.len() > 0 {
                cmd_args(list)
            } else {
                cmd_flag_long("sysupgrade")
            }
        ))
    }
}
