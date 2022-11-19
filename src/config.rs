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
  name: String,
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
  pub fn set(&self, file: File) {
    let Command { cmd, args } = &self.set;
    if let Err(error) = StdCommand::new(cmd).args(args).stdin(file).output() {
      eprintln!("Couldn't set clipboard for '{}': {error}", self.name);
    }
  }

  pub fn get(&self) -> Option<Vec<u8>> {
    let Command { cmd, args } = self.get.as_ref()?;
    match StdCommand::new(cmd).args(args).output() {
      Ok(Output { status, stdout, .. }) if status.success() => return Some(stdout),
      Ok(Output { stderr, .. }) => eprintln!(
        "Couldn't get clipboard for '{}': {}",
        self.name,
        String::from_utf8_lossy(&stderr)
      ),
      Err(error) => eprintln!("Couldn't get clipboard for '{}': {error}", self.name),
    }

    None
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
