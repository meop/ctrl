use std::io::Error;
use std::process::Command;

use crate::file::exists_in_path;
use crate::log::Category;
use crate::log::logc;
use crate::log::logcln;
use crate::log::logln;

fn get_shell_cmd_arg() -> (String, String) {
    let mut shell = "sh";
    let mut cmd_arg = "-c";

    if cfg!(target_os = "windows") {
        if exists_in_path("powershell") {
            shell = "powershell";
        } else if exists_in_path("pwsh") {
            shell = "pwsh";
        } else if exists_in_path("cmd") {
            shell = "cmd";
            cmd_arg = "/C"
        }
    } else {
        if exists_in_path("zsh") {
            shell = "zsh"
        } else if exists_in_path("bash") {
            shell = "bash"
        }
    }
    logc(&format!("{shell} {cmd_arg} "), Category::Cmd);
    (shell.to_string(), cmd_arg.to_string())
}

pub(crate) fn run_cmd(command: &String) -> Result<(), Error> {
    let (shell, cmd_arg) = get_shell_cmd_arg();
    logcln(command, Category::Arg);
    Command::new(shell).arg(cmd_arg).arg(command).status()?;
    Ok(())
}

pub(crate) fn run_cmd_filtered(command: &String, filters: &Vec<String>) -> Result<(), Error> {
    if filters.is_empty() {
        return run_cmd(command);
    }

    let (shell, cmd_arg) = get_shell_cmd_arg();
    logcln(command, Category::Arg);
    let output = Command::new(shell).arg(cmd_arg).arg(command).output()?;

    logcln("matches:", Category::Info);
    for line in String::from_utf8(output.stdout).unwrap().lines() {
        for filter in filters {
            if line.to_lowercase().contains(&filter.to_lowercase()) {
                logln(line);
            }
        }
    }
    Ok(())
}
