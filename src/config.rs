use std::{
  fs,
  fs::File,
  path::PathBuf,
  process::{Command as StdCommand, Output},
};

use anyhow::{anyhow, Result};
use directories::ProjectDirs;
use indexmap::IndexMap;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config(IndexMap<String, RawEntry>);

#[derive(Deserialize)]
struct RawEntry {
  get: Option<Command>,
  set: Command,
}

#[derive(Deserialize)]
pub struct Command {
  cmd: String,
  args: Vec<String>,
}

pub struct Entry {
  pub name: String,
  get: Option<Command>,
  set: Command,
}

impl From<Config> for Vec<Entry> {
  fn from(config: Config) -> Self {
    config
      .0
      .into_iter()
      .map(|(name, raw_entry)| Entry {
        name,
        get: raw_entry.get,
        set: raw_entry.set,
      })
      .collect()
  }
}

impl Entry {
  pub fn set(&self, file: File) -> Result<Output> {
    let Command { cmd, args } = &self.set;

    Ok(StdCommand::new(cmd).args(args).stdin(file).output()?)
  }

  pub fn get(&self) -> Result<Option<Output>> {
    if let Some(Command { cmd, args }) = self.get.as_ref() {
      Ok(Some(StdCommand::new(cmd).args(args).output()?))
    } else {
      Ok(None)
    }
  }
}

fn config_dir() -> Result<PathBuf> {
  Ok(
    ProjectDirs::from("com", "enricozb", "clip-rs")
      .ok_or_else(|| anyhow!("could not find config directory"))?
      .config_dir()
      .to_owned(),
  )
}

pub fn entries() -> Result<Vec<Entry>> {
  let path = config_dir()?.join("config.toml");
  let config_str = fs::read_to_string(path)?;

  let config = Config(toml::from_str(&config_str)?);

  Ok(config.into())
}
