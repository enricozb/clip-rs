use std::io::{self, Read, Seek, SeekFrom, Write};

use anyhow::Result;

use crate::config::{self, Entry};

pub struct Clipboard {
  entries: Vec<Entry>,
}

impl Clipboard {
  pub fn new() -> Result<Self> {
    Ok(Self {
      entries: config::entries()?,
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

      entry.set(tempfile);
    }

    Ok(())
  }

  pub fn paste(&self) -> Result<()> {
    for entry in &self.entries {
      if let Some(output) = entry.get() {
        io::stdout().write_all(&output)?;

        break;
      }
    }

    Ok(())
  }
}
