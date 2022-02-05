use std::io::Write;
use std::process::Command;
use std::process::ExitStatus;
use std::process::Stdio;

pub struct Pm;

static SHELL: &str = "bash";
static SHELL_ARG: &str = "-c";
static PACKAGE_MANAGER: &str = "brew";

fn get_pkg_cmd(cmd: &String) -> String {
    format!("{} {}", PACKAGE_MANAGER, cmd)
}

fn get_pkg_cmd_arg(cmd: &String, pkg: &String) -> String {
    get_pkg_cmd(&format!("{} {}", cmd, pkg))
}

fn get_pkg_cmd_arg_list(cmd: &String, args: &Vec<String>) -> String {
    get_pkg_cmd_arg(cmd, &args.join(" "))
}

fn run_and_wait(command: &String, stdin: &Option<String>) -> Result<ExitStatus, std::io::Error> {
    let mut proc = Command::new(SHELL)
        .arg(SHELL_ARG)
        .arg(command)
        .stdin(Stdio::piped())
        .spawn()?;
    if let Some(mut input) = proc.stdin.take() {
        input.write_all(if let Some(r) = stdin {
            r.as_bytes()
        } else {
            &[]
        })?;
    }
    proc.wait()
}

fn cmd_force(cmd: &mut String, force: &bool) {
    if *force {
        cmd.push_str(" --force");
    }
}

impl super::PackageManager for Pm {
    fn add(&self, list: &Vec<String>, force: &bool) -> Result<ExitStatus, std::io::Error> {
        fix_fs_perm()?;
        let mut cmd = "install".to_string();
        cmd_force(&mut cmd, force);
        run_and_wait(&get_pkg_cmd_arg_list(&cmd, list), &None)
    }

    fn find(&self, pattern: &String) -> Result<ExitStatus, std::io::Error> {
        run_and_wait(&get_pkg_cmd_arg(&"search".to_string(), pattern), &None)
    }

    fn list(&self, pattern: &Option<String>) -> Result<ExitStatus, std::io::Error> {
        let full_cmd = if let Some(x) = pattern {
            get_pkg_cmd_arg(&"list".to_string(), x)
        } else {
            get_pkg_cmd(&"list".to_string())
        };
        run_and_wait(&full_cmd, &None)
    }

    fn out(&self) -> Result<ExitStatus, std::io::Error> {
        run_and_wait(&get_pkg_cmd(&"outdated".to_string()), &None)
    }

    fn rem(&self, list: &Vec<String>, force: &bool) -> Result<ExitStatus, std::io::Error> {
        fix_fs_perm()?;
        let mut cmd = "uninstall".to_string();
        cmd_force(&mut cmd, force);
        run_and_wait(&get_pkg_cmd_arg_list(&cmd.to_string(), list), &None)
    }

    fn up(&self, list: &Vec<String>) -> Result<ExitStatus, std::io::Error> {
        fix_fs_perm()?;
        run_and_wait(
            &get_pkg_cmd_arg_list(&"upgrade --greedy".to_string(), list),
            &None,
        )
    }
}

fn fix_fs_perm() -> Result<ExitStatus, std::io::Error> {
    log::info!("Password: <- will be auto filled");
    let mut response = run_and_wait(
        &"sudo -S chown -R $(whoami) /usr/local/bin /usr/local/lib /usr/local/sbin".to_string(),
        &Some("tbd\n".to_string()),
    )?;
    if response.success() {
        response = run_and_wait(
            &"chmod u+w /usr/local/bin /usr/local/lib /usr/local/sbin".to_string(),
            &None,
        )?;
    }
    Ok(response)
}
