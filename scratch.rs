// this code is cool, and took me some time, so want to keep it somewhere

// fn run_commands<F>(cmds: &Vec<String>, launch_func: F) -> Result<ExitStatus, std::io::Error>
// where
//     F: Fn(&String) -> Result<ExitStatus, std::io::Error>,
// {
//     let mut result = std::os::unix::process::ExitStatusExt::from_raw(0);
//     for cmd in cmds {
//         result = launch_func(cmd)?;
//     }
//     Ok(result)
// }

// &vec![
//     "sudo -S chown -R $(whoami) /usr/local/bin /usr/local/lib /usr/local/sbin".to_string(),
// ],
// |x| spawn_respond_and_wait(x, &"pword\n".to_string()),