use std::io::Error;

use crate::command::run_and_wait;
use crate::file::exists_in_path;

use super::cmd_args;
use super::cmd_flag_long;
use super::Manager;

pub(super) struct Winget;

fn get_program() -> String {
    if exists_in_path("winget") {
        "winget".to_string()
    } else {
        panic!("no winget compatible program found in path")
    }
}

impl Manager for Winget {
    fn add(&self, list: &Vec<String>) -> Result<(), Error> {
        run_and_wait(&format!("{} install {}", get_program(), cmd_args(list)))
    }

    fn clean(&self) -> Result<(), Error> {
        Ok(())
    }

    fn list(&self) -> Result<(), Error> {
        run_and_wait(&format!(
            "{} list",
            get_program(),
        ))
    }

    fn old(&self) -> Result<(), Error> {
        run_and_wait(&format!("{} upgrade", get_program()))
    }

    fn remove(&self, list: &Vec<String>) -> Result<(), Error> {
        run_and_wait(&format!("{} uninstall {}", get_program(), cmd_args(list)))
    }

    fn search(&self, pattern: &String) -> Result<(), Error> {
        run_and_wait(&format!("{} search {}", get_program(), pattern))
    }

    fn sync(&self, list: &Vec<String>) -> Result<(), Error> {
        run_and_wait(&format!(
            "{} upgrade {}",
            get_program(),
            if list.len() > 0 {
                cmd_args(list)
            } else {
                cmd_flag_long("all")
            }
        ))
    }
}
