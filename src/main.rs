mod clipboard;
mod config;

use anyhow::Result;
use atty::{self, Stream::Stdin};

use crate::clipboard::Clipboard;

enum Mode {
  Copy,
  Paste,
}

fn mode() -> Mode {
  if atty::is(Stdin) {
    Mode::Paste
  } else {
    Mode::Copy
  }
}

fn main() -> Result<()> {
  let clipboard = Clipboard::new()?;

  match mode() {
    Mode::Copy => clipboard.copy()?,
    Mode::Paste => clipboard.paste()?,
  }

  Ok(())
}
