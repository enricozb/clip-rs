# clip-rs: an extensible clipboard

`clip-rs` works by reading clipboard definitions from `~/.config/clip-rs/config.toml`.
The expected format is one table per clipboard entry, with two keys, `get` and `set`.
Each of those keys expects an inline table, with the keys `cmd` and `args`. For example,
```toml
[xsel]
get = { cmd = "xsel", args = ["-b", "-o"] }
set = { cmd = "xsel", args = ["-b", "-i"] }

[file]
get = { cmd = "cat", args = ["/tmp/clipboard"] }
set = { cmd = "tee", args = ["/tmp/clipboard"] }

[osc_52]
set = { cmd = "bash", args = ["-c", "printf '\\e]52;;%s\\a' $(base64 | tr -d '\\n') >/dev/tty"] }
```

As seen above, the getters and setters define how `clip-rs` interfaces with the specified
clipboards. The `get` key is optional, and if omitted, this clipboard will only be used for setting.
This is useful in the case of using an [OSC 52] sequence to copy text over an SSH session.

## Usage
`clip-rs` has two modes, copy and paste. It infers that it is in copy mode when being piped into,
and in paste mode when not being piped into. This is done by checking whether stdin is a tty.

## Flags
All flags have a short form using the starting letter.

- `--copy`: forces copy mode
- `--paste`: forces paste mode
- `--debug`: print out any errors with the clipboard commands that are executed
- `--strict`: fail if any clipboard commands exits with a non-zero status

[OSC 52]: https://jdhao.github.io/2021/01/05/nvim_copy_from_remote_via_osc52/
