use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    #[serde(default)]
    problems: Vec<PathBuf>,
    #[serde(default)]
    tools: Vec<PathBuf>,
    #[serde(with = "tuple_vec_map", default)]
    copy: Vec<(PathBuf, PathBuf)>,
    #[serde(default)]
    skip_entries: Vec<PathBuf>,
    #[serde(default)]
    add_to_toml: Vec<PathBuf>,
    #[serde(default)]
    do_not_delete: Vec<PathBuf>,
}

impl Config {
    pub fn from_yml(path: &Path) -> Result<Self> {
        let file = File::open(path).context("no yml file with config")?;
        let config: Config = serde_yaml::from_reader(file).context("cannot read file from yml")?;
        for entry in config.get_do_not_delete() {
            if entry.iter().count() != 1 {
                bail!("compose doesn't support do-not-delete on paths, only directories or files: {entry:?}")
            }
        }
        Ok(config)
    }

    pub fn get_problems(&self) -> &[PathBuf] {
        self.problems.as_slice()
    }

    pub fn get_tools(&self) -> &[PathBuf] {
        self.tools.as_slice()
    }

    pub fn get_copy(&self) -> &[(PathBuf, PathBuf)] {
        self.copy.as_slice()
    }

    pub fn get_add_to_toml(&self) -> &[PathBuf] {
        self.add_to_toml.as_slice()
    }

    pub fn get_skipped(&self) -> &[PathBuf] {
        self.skip_entries.as_slice()
    }

    pub fn get_do_not_delete(&self) -> &[PathBuf] {
        self.do_not_delete.as_slice()
    }
}
