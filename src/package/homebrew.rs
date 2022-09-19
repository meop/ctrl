use std::io::Error;
use std::process::ExitStatus;

use crate::command::run_and_wait;

pub struct Homebrew {
    manager: String,
}

impl Default for Homebrew {
    fn default() -> Homebrew {
        Homebrew {
            manager: "brew".to_string(),
        }
    }
}

fn cmd_force(cmd: &mut String, force: &bool) {
    if *force {
        cmd.push_str(" --force");
    }
}

fn fix_fs_perm() -> Result<ExitStatus, Error> {
    let mut response = run_and_wait(
        &"sudo -S chown -R $(whoami) /usr/local/bin /usr/local/lib /usr/local/sbin".to_string(),
    )?;
    if response.success() {
        response = run_and_wait(
            &"chmod u+w /usr/local/bin /usr/local/lib /usr/local/sbin".to_string(),
        )?;
    }
    Ok(response)
}

impl Homebrew {
    fn get_package_command(&self, cmd: &String) -> String {
        format!("{} {}", self.manager, cmd)
    }
    
    fn get_package_command_arg(&self, cmd: &String, pkg: &String) -> String {
        self.get_package_command(&format!("{} {}", cmd, pkg))
    }
    
    fn get_package_command_arg_list(&self, cmd: &String, args: &Vec<String>) -> String {
        self.get_package_command_arg(cmd, &args.join(" "))
    }    
}

impl super::Manager for Homebrew {
    fn add(&self, list: &Vec<String>, force: &bool) -> Result<ExitStatus, Error> {
        fix_fs_perm()?;
        let mut cmd = "install".to_string();
        cmd_force(&mut cmd, force);
        run_and_wait(&self.get_package_command_arg_list(&cmd, list))
    }
    
    fn list(&self, pattern: &Option<String>) -> Result<ExitStatus, Error> {
        let full_cmd = if let Some(x) = pattern {
            self.get_package_command_arg(&"list".to_string(), x)
        } else {
            self.get_package_command(&"list".to_string())
        };
        run_and_wait(&full_cmd)
    }

    fn outdated(&self) -> Result<ExitStatus, Error> {
        run_and_wait(&self.get_package_command(&"outdated".to_string()))
    }

    fn remove(&self, list: &Vec<String>, force: &bool) -> Result<ExitStatus, Error> {
        fix_fs_perm()?;
        let mut cmd = "uninstall".to_string();
        cmd_force(&mut cmd, force);
        run_and_wait(&self.get_package_command_arg_list(&cmd.to_string(), list))
    }

    fn search(&self, pattern: &String) -> Result<ExitStatus, Error> {
        run_and_wait(&self.get_package_command_arg(&"search".to_string(), pattern))
    }
    
    fn upgrade(&self, list: &Vec<String>) -> Result<ExitStatus, Error> {
        fix_fs_perm()?;
        run_and_wait(
            &self.get_package_command_arg_list(&"upgrade --greedy".to_string(), list),
        )
    }
}
