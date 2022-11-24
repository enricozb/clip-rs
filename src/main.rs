mod args;
mod clipboard;
mod config;

use anyhow::Result;
use clap::Parser;

use crate::{
  args::{Args, Mode},
  clipboard::Clipboard,
};

fn main() -> Result<()> {
  let args = Args::parse();
  let clipboard = Clipboard::new(args.debug, args.strict)?;

  match args.mode() {
    Mode::Copy => clipboard.copy()?,
    Mode::Paste => clipboard.paste()?,
  }

  Ok(())
}
