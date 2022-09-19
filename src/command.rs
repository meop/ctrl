use std::io::Error;
use std::process::Command;
use std::process::ExitStatus;

pub trait Invoke {
    fn run(&self) -> Result<ExitStatus, Error>;
}

pub fn run_and_wait(command: &String) -> Result<ExitStatus, Error> {
    let mut shell = "sh";
    let mut shell_arg = "-c";

    if cfg!(target_os = "windows") {
        shell = "cmd";
        shell_arg = "/C";
    }

    log::info!("{shell} {shell_arg} {command}");

    Command::new(shell)
        .arg(shell_arg)
        .arg(command)
        .status()
}
