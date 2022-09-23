use std::io::Error;

use crate::command::run_and_wait;

use super::cmd_args;
use super::cmd_flag_long;
use super::Manager;

pub(super) struct Winget;

static PROGRAM: &str = "winget";

impl Manager for Winget {
    fn add(&self, list: &Vec<String>) -> Result<(), Error> {
        run_and_wait(&format!("{PROGRAM} install {}", cmd_args(list)))
    }

    fn clean(&self) -> Result<(), Error> {
        Ok(())
    }

    fn list(&self, pattern: &Option<String>) -> Result<(), Error> {
        run_and_wait(&format!(
            "{PROGRAM} list {}",
            if pattern.as_deref().is_some() {
                format!("| sls {}", String::from(pattern.as_deref().unwrap()))
            } else {
                String::new()
            }
        ))
    }

    fn old(&self) -> Result<(), Error> {
        run_and_wait(&format!("{PROGRAM} upgrade"))
    }
    
    fn remove(&self, list: &Vec<String>) -> Result<(), Error> {
        run_and_wait(&format!("{PROGRAM} uninstall {}", cmd_args(list)))
    }
    
    fn search(&self, pattern: &String) -> Result<(), Error> {
        run_and_wait(&format!("{PROGRAM} search {}", pattern))
    }

    fn sync(&self, list: &Vec<String>) -> Result<(), Error> {
        run_and_wait(&format!(
            "{PROGRAM} upgrade {}",
            if list.len() > 0 {
                cmd_args(list)
            } else {
                cmd_flag_long("all")
            }
        ))
    }
}
