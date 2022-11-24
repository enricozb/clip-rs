use atty::Stream;
use clap::Parser;

/// clip-rs: an extensible clipboard
///
/// `clip-rs` works by reading clipboard definitions from `~/.config/clip-rs/config.toml`.
/// The expected format is one table per clipboard entry, with two keys, `get` and `set`.
/// Each of those keys expects an inline table, with the keys `cmd` and `args`. For example,
/// ```toml
/// [xsel]
/// get = { cmd = "xsel", args = ["-b", "-o"] }
/// set = { cmd = "xsel", args = ["-b", "-i"] }
///
/// [file]
/// get = { cmd = "cat", args = ["/tmp/clipboard"] }
/// set = { cmd = "tee", args = ["/tmp/clipboard"] }
///
/// [osc_52]
/// set = { cmd = "bash", args = ["-c", "printf '\\e]52;;%s\\a' $(base64 | tr -d '\\n') >/dev/tty"] }
/// ```
///
/// As seen above, the getters and setters define how `clip-rs` interfaces with the specified
/// clipboards. The `get` key is optional, and if omitted, this clipboard will only be used for setting.
/// This is useful in the case of using an [OSC 52] sequence to copy text over an SSH session.
///
/// [OSC 52]: https://jdhao.github.io/2021/01/05/nvim_copy_from_remote_via_osc52/
#[allow(clippy::struct_excessive_bools)]
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
  /// Force copy mode
  #[arg(short, long)]
  copy: bool,

  /// Force paste mode
  #[arg(short, long)]
  paste: bool,

  /// Print out errors to stderr
  #[arg(short, long)]
  pub debug: bool,

  /// Exit with non-zero status code on an error
  #[arg(short, long)]
  pub strict: bool,
}

impl Args {
  pub fn mode(&self) -> Mode {
    match (self.copy, self.paste, atty::is(Stream::Stdin)) {
      (true, true, _) => panic!("provide only one of --copy and --paste"),
      (true, _, _) => Mode::Copy,
      (_, true, _) | (_, _, true) => Mode::Paste,
      _ => Mode::Copy,
    }
  }
}

pub enum Mode {
  Copy,
  Paste,
}
