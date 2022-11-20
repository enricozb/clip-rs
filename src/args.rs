use atty::Stream;
use clap::Parser;

// TODO(enricozb): consider args:
//   --debug (to print stderr of commands, quiet otherwise)
//   --strict (fail if any of the clipboard commands fail)

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
  #[arg(short, long)]
  copy: bool,

  #[arg(short, long)]
  paste: bool,
}

impl Args {
  pub fn mode(&self) -> Mode {
    match (self.copy, self.paste, atty::is(Stream::Stdin)) {
      (true, true, _) => panic!("provide only one of --copy and --paste"),
      (true, _, _) => Mode::Copy,
      (_, true, _) => Mode::Paste,
      (_, _, true) => Mode::Paste,
      _ => Mode::Copy,
    }
  }
}

pub enum Mode {
  Copy,
  Paste,
}
