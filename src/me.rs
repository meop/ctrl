use std::error::Error;
use std::fs::copy;
use std::fs::File;
use std::path::Path;

use clap::Subcommand;
use dialoguer::Confirm;
use reqwest;
use version_compare::Cmp;

use crate::file::get_cur_path_str;
use crate::log::logcln;
use crate::log::logln;
use crate::log::Category;

#[derive(Subcommand)]
pub(super) enum Command {
    /// Revert to local backup release
    #[clap(visible_alias("r"))]
    Revert {},

    /// Upgrade to latest release
    #[clap(alias("update"))]
    #[clap(alias("up"))]
    #[clap(visible_alias("u"))]
    Upgrade {},
}

pub(super) trait Invoke {
    fn run(&self) -> Result<(), Box<dyn Error>>;
}

impl Invoke for Command {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        match self {
            Command::Revert {} => revert(),
            Command::Upgrade {} => upgrade(),
        }
    }
}

fn get_bak_path_str(prefix: &str) -> String {
    format!("{}.bak", prefix)
}

fn get_tmp_path_str(prefix: &str) -> String {
    format!("{}.tmp", prefix)
}

fn revert() -> Result<(), Box<dyn Error>> {
    let cur_path_str = get_cur_path_str()?;

    let bak_path_str = get_bak_path_str(&cur_path_str);
    let tmp_path_str = get_tmp_path_str(&cur_path_str);

    if !Path::new(&bak_path_str).exists() {
        logcln("no backup to revert", Category::Info);
        return Ok(());
    }

    if !Confirm::new()
        .with_prompt("revert to backup version?")
        .default(true)
        .interact()?
    {
        return Ok(());
    }

    self_update::Move::from_source(Path::new(&bak_path_str))
        // windows requires this; unix is optional
        .replace_using_temp(Path::new(&tmp_path_str))
        .to_dest(Path::new(&cur_path_str))?;

    Ok(())
}

fn upgrade() -> Result<(), Box<dyn Error>> {
    let releases = self_update::backends::github::ReleaseList::configure()
        .repo_owner("meop")
        .repo_name("ctrl")
        .build()?
        .fetch()?;

    if releases.len() == 0 {
        logcln("releases empty", Category::Info);
        return Ok(());
    }

    let current_version = self_update::cargo_crate_version!();
    let mut latest_version = releases[0].version.as_str();

    let binary = format!("{}-ctrl", self_update::get_target());
    let asset = releases[0].asset_for(&binary, None);
    if asset.is_none() {
        latest_version = current_version;
    }

    logcln(&format!("current: {current_version}"), Category::Info);
    logcln(&format!("latest: {latest_version}"), Category::Info);

    if version_compare::compare_to(current_version, latest_version, Cmp::Ge).unwrap() {
        logln("already at latest");
        return Ok(());
    }

    if !Confirm::new()
        .with_prompt("upgrade to latest?")
        .default(true)
        .interact()?
    {
        return Ok(());
    }

    let cur_path_str = get_cur_path_str()?;

    let bak_path_str = get_bak_path_str(&cur_path_str);
    let tmp_path_str = get_tmp_path_str(&cur_path_str);

    // copying preserves fs permissions
    copy(&cur_path_str, &tmp_path_str)?;

    // create will then truncate the existing file
    let tmp_path_file = File::create(&tmp_path_str)?;

    self_update::Download::from_url(&asset.unwrap().download_url)
        .set_header(reqwest::header::ACCEPT, "application/octet-stream".parse()?)
        .download_to(&tmp_path_file)?;

    self_update::Move::from_source(Path::new(&tmp_path_str))
        // windows requires this; unix is optional
        .replace_using_temp(Path::new(&bak_path_str))
        .to_dest(Path::new(&cur_path_str))?;

    Ok(())
}
