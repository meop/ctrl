use std::io::Error;

use crate::command::run_cmd;
use crate::command::run_cmd_filtered;

use super::cmd_args;
use super::cmd_flag_long;
use super::Manager;

pub(super) struct Pacman {
    pub program: String,
}

fn repo_update(program: &String) -> Result<(), Error> {
    run_cmd(&format!(
        "{} {} {}",
        program,
        cmd_flag_long("sync"),
        cmd_flag_long("refresh"),
    ))
}

impl Manager for Pacman {
    fn add(&self, list: &Vec<String>) -> Result<(), Error> {
        repo_update(&self.program)?;
        run_cmd(&format!(
            "{} {} {}",
            &self.program,
            cmd_flag_long("sync"),
            cmd_args(list),
        ))
    }

    fn clean(&self) -> Result<(), Error> {
        run_cmd(&format!(
            "{} {} {}",
            &self.program,
            cmd_flag_long("sync"),
            cmd_flag_long("clean"),
        ))
    }

    fn list(&self, list: &Vec<String>) -> Result<(), Error> {
        run_cmd_filtered(
            &format!("{} {}", &self.program, cmd_flag_long("query"),),
            list,
        )
    }

    fn probe(&self) -> Result<(), Error> {
        repo_update(&self.program)?;
        run_cmd(&format!(
            "{} {} {}",
            &self.program,
            cmd_flag_long("query"),
            cmd_flag_long("upgrades"),
        ))
    }

    fn remove(&self, list: &Vec<String>) -> Result<(), Error> {
        run_cmd(&format!(
            "{} {} {} {} {}",
            &self.program,
            cmd_flag_long("remove"),
            cmd_flag_long("recursive"),
            cmd_flag_long("nosave"),
            cmd_args(list),
        ))
    }

    fn search(&self, pattern: &String) -> Result<(), Error> {
        repo_update(&self.program)?;
        run_cmd(&format!(
            "{} {} {} {}",
            &self.program,
            cmd_flag_long("query"),
            cmd_flag_long("search"),
            pattern,
        ))
    }

    fn upgrade(&self, list: &Vec<String>) -> Result<(), Error> {
        repo_update(&self.program)?;
        run_cmd(&format!(
            "{} {} {}",
            &self.program,
            cmd_flag_long("sync"),
            if list.len() > 0 {
                cmd_args(list)
            } else {
                cmd_flag_long("sysupgrade")
            }
        ))
    }
}
