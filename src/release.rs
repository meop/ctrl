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
use crate::log::Category;

#[derive(Subcommand)]
pub(super) enum Command {
    /// Revert to local backup release
    Revert {},

    /// Sync to latest release
    Sync {},
}

pub(super) trait Invoke {
    fn run(&self) -> Result<(), Box<dyn Error>>;
}

impl Invoke for Command {
    fn run(&self) -> Result<(), Box<dyn Error>> {
        match self {
            Command::Revert {} => revert(),
            Command::Sync {} => sync(),
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

fn sync() -> Result<(), Box<dyn Error>> {
    let releases = self_update::backends::github::ReleaseList::configure()
        .repo_owner("meop")
        .repo_name("ctrl")
        .build()?
        .fetch()?;

    if releases.len() == 0 {
        logcln("no releases found", Category::Info);
        return Ok(());
    }

    let running_version = self_update::cargo_crate_version!();
    let latest_version = &releases[0].version;

    logcln(
        &format!("running version: {running_version}"),
        Category::Info,
    );
    logcln(&format!("latest version: {latest_version}"), Category::Info);

    if version_compare::compare_to(running_version, latest_version, Cmp::Ge).unwrap() {
        logcln(
            "running version is already at or above latest version",
            Category::Info,
        );
        return Ok(());
    }

    if !Confirm::new()
        .with_prompt("upgrade to latest version?")
        .default(true)
        .interact()?
    {
        return Ok(());
    }

    let binary = format!("{}-ctrl", self_update::get_target());

    if let Some(asset) = releases[0].asset_for(&binary) {
        let cur_path_str = get_cur_path_str()?;

        let bak_path_str = get_bak_path_str(&cur_path_str);
        let tmp_path_str = get_tmp_path_str(&cur_path_str);

        // copying preserves fs permissions
        copy(&cur_path_str, &tmp_path_str)?;

        // create will then truncate the existing file
        let tmp_path_file = File::create(&tmp_path_str)?;

        self_update::Download::from_url(&asset.download_url)
            .set_header(reqwest::header::ACCEPT, "application/octet-stream".parse()?)
            .download_to(&tmp_path_file)?;

        self_update::Move::from_source(Path::new(&tmp_path_str))
            // windows requires this; unix is optional
            .replace_using_temp(Path::new(&bak_path_str))
            .to_dest(Path::new(&cur_path_str))?;
    } else {
        logcln(
            &format!("latest version does not contain target: {binary}"),
            Category::Info,
        );
        return Ok(());
    }

    Ok(())
}
