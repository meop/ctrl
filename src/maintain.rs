use std::env::current_exe;
use std::error::Error;
use std::fs::copy;
use std::fs::File;
use std::path::Path;

use clap::Subcommand;
use dialoguer::Confirm;
use reqwest;
use version_compare::Cmp;

#[derive(Subcommand)]
pub enum Command {
    Upgrade {
    },
}

pub trait Invoke {
    fn run(&self) -> Result<(), Box<dyn Error>>;
}

impl Invoke for Command {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        match self {
            Command::Upgrade {} => upgrade(),
        }
    }
}

pub fn upgrade() -> Result<(), Box<dyn Error>> {
    let releases = self_update::backends::github::ReleaseList::configure()
        .repo_owner("meop")
        .repo_name("ctrl")
        .build()?
        .fetch()?;

    if releases.len() == 0 {
        log::info!("no releases found");
        return Ok(());
    }

    let running_version = self_update::cargo_crate_version!();
    let latest_version = &releases[0].version;
    
    log::info!("running version: {running_version}");
    log::info!("latest version: {latest_version}");

    if version_compare::compare_to(running_version, latest_version, Cmp::Ge).unwrap() {
        log::info!("running version is already at or above latest version");
        return Ok(());
    }

    if !Confirm::new().with_prompt("upgrade to latest version?").interact()? {
        return Ok(());
    }

    let target: Vec<&str> = self_update::get_target().split('-').collect();
    let arch = target[0];
    let sys = target[2];

    let binary = format!("ctrl-{sys}-{arch}");
    if let Some(asset) = releases[0].asset_for(&binary) {
        let cur_path = current_exe()?;
        let mut tmp_path_str = cur_path.to_str().unwrap().to_string();
        tmp_path_str.push_str(".tmp");

        let tmp_path = Path::new(&tmp_path_str);

        // copying preserves fs permissions
        copy(&cur_path, &tmp_path)?;

        // create will then truncate the existing file
        let tmp_path_file = File::create(tmp_path)?;

        self_update::Download::from_url(&asset.download_url)
            .set_header(reqwest::header::ACCEPT, "application/octet-stream".parse()?)
            .download_to(&tmp_path_file)?;

        let mut tmp_tmp_path_str = tmp_path_str.clone();
        tmp_tmp_path_str.push_str(".tmp");

        self_update::Move::from_source(tmp_path)
            .to_dest(&cur_path)?;
    } else {
        log::info!("latest version does not contain asset: {binary}");
        return Ok(());
    }

    Ok(())
}
