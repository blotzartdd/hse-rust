use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use cargo_toml::Manifest;
use colored::Colorize;
use indoc::formatdoc;
use version_compare::Version;

use crate::repository::repo::Repository;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const ROVER_RELATIVE_PATH: &'static str = "tools/rover";

fn parse_version(version: &str) -> Result<Version> {
    Version::from(version.as_ref()).ok_or(anyhow!("Coult not parse version {version}"))
}

fn get_version_in_repo(repo: &Repository) -> Result<String> {
    let toml_path = repo
        .get_path()
        .join(PathBuf::from(ROVER_RELATIVE_PATH))
        .join("Cargo.toml");
    let manifest = Manifest::from_path(&toml_path).context(format!(
        "Could not parse rover's Cargo.toml by path {}",
        toml_path.display()
    ))?;

    Ok(manifest
        .package
        .ok_or(anyhow!("no package in Cargo.toml"))?
        .version()
        .to_owned())
}

pub fn ensure_version_is_latest(path: &Path) -> Result<()> {
    let repo = Repository::from_path(path)?;
    let latest_version_str = get_version_in_repo(&repo)?;
    let latest_version = parse_version(&latest_version_str)?;
    let current_version = parse_version(VERSION)?;

    if current_version < latest_version {
        let rover_path = repo.get_path().join(ROVER_RELATIVE_PATH);
        bail!(formatdoc! {"
            New version of rover is available: {version_change}.
            Please install `{install_cmd}`",
            version_change = format!("{current_version} -> {latest_version}").green(),
            install_cmd = format!("cargo install --path {}", rover_path.display()).bold(),
        });
    }
    Ok(())
}
