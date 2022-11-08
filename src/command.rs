use std::io::Error;
use std::process::Command;

use crate::log::logc;
use crate::log::logcln;
use crate::log::Category;

pub(crate) fn run_and_wait(command: &String) -> Result<(), Error> {
    let mut shell = "sh";
    let mut shell_arg = "-c";

    if cfg!(target_os = "windows") {
        shell = "cmd";
        shell_arg = "/C";
    }

    logc(&format!("{shell} {shell_arg} "), Category::Cmd);
    logcln(command, Category::Arg);

    Command::new(shell).arg(shell_arg).arg(command).status()?;
    Ok(())
}
