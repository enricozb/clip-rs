use std::{
  io::{self, Read, Seek, SeekFrom, Write},
  process::Output,
};

use anyhow::{anyhow, Error, Result};

use crate::config::{self, Entry};

pub struct Clipboard {
  entries: Vec<Entry>,

  debug: bool,
  strict: bool,
}

impl Clipboard {
  pub fn new(debug: bool, strict: bool) -> Result<Self> {
    Ok(Self {
      entries: config::entries()?,
      debug,
      strict,
    })
  }

  pub fn copy(&self) -> Result<()> {
    let mut stdin = Vec::new();
    io::stdin().read_to_end(&mut stdin)?;

    for entry in &self.entries {
      let mut tempfile = tempfile::tempfile()?;
      tempfile.write_all(&stdin)?;
      tempfile.flush()?;
      tempfile.seek(SeekFrom::Start(0))?;

      match (self.debug, self.strict, entry.set(tempfile)) {
        (true, _, Err(error)) => eprintln_error("get", &entry.name, &error),
        (true, _, Ok(Output { status, stderr, .. })) if !status.success() => {
          eprintln_status("get", &entry.name, &stderr);
        }

        (_, true, Err(error)) => return error_error("get", &entry.name, &error),
        (_, true, Ok(Output { status, stderr, .. })) if !status.success() => {
          return error_status("get", &entry.name, &stderr);
        }

        _ => (),
      };
    }

    Ok(())
  }

  pub fn paste(&self) -> Result<()> {
    for entry in &self.entries {
      match (self.debug, self.strict, entry.get()) {
        (true, _, Err(error)) => eprintln_error("set", &entry.name, &error),
        (true, _, Ok(Some(Output { status, stderr, .. }))) if !status.success() => {
          eprintln_status("set", &entry.name, &stderr);
        }

        (_, true, Err(error)) => return error_error("set", &entry.name, &error),
        (_, true, Ok(Some(Output { status, stderr, .. }))) if !status.success() => {
          return error_status("set", &entry.name, &stderr);
        }

        (_, _, Ok(Some(Output { status, stdout, .. }))) if status.success() => {
          io::stdout().write_all(&stdout)?;
          break;
        }

        _ => (),
      }
    }

    Ok(())
  }
}

fn eprintln_error(method: &str, name: &str, error: &Error) {
  eprintln!("Couldn't {method} clipboard for '{name}': {error}");
}

fn eprintln_status(method: &str, name: &str, stderr: &[u8]) {
  eprintln!(
    "Clipboard '{name}' {method} exited with non-zero status: {}",
    String::from_utf8_lossy(stderr)
  );
}

fn error_error(method: &str, name: &str, error: &Error) -> Result<()> {
  Err(anyhow!("Couldn't {method} clipboard for '{name}': {error}"))
}

fn error_status(method: &str, name: &str, stderr: &[u8]) -> Result<()> {
  Err(anyhow!(
    "Clipboard '{name}' {method} exited with non-zero status: {}",
    String::from_utf8_lossy(stderr)
  ))
}
