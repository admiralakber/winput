# winput

Slick, minimal GUI input box for Wayland/Sway (GTK4). Type text, press Enter → winput prints to stdout and exits. Escape exits with code 1 and no output. Perfect for piping into `xargs` or `swaymsg`.

Inspired by `i3-input` (see the man page). Designed to feel native on Sway with a clean, flat aesthetic.

Note: This project was fully created by the Cursor agent CLI running GPT‑5, directed by `admiralakber`.

## Features
- Flat, modern dark UI (fully themeable)
- Monospace look with Inconsolata by default (configurable)
- Enter: print to stdout, exit 0
- Escape: exit 1 (no output)
- Tooltip, placeholder, initial text, size, font, colors, and rounding via CLI flags

## Build and run (NixOS recommended)

### With Nix shell
```bash
nix-shell --pure
cargo build --release
./target/release/winput --placeholder "Run:" --tooltip "Type and press Enter"
```

### Without Nix
Install GTK4 development libraries for your distro, then:
```bash
cargo build --release
./target/release/winput
```

## Install locally
```bash
# From the repo root
cargo install --path .
# Then run
winput --placeholder "Search:"
```

## CLI
```text
Usage: winput [OPTIONS]

Options:
  -p, --placeholder <PLACEHOLDER>     Placeholder text [default: Type and press Enter]
  -t, --text <TEXT>                    Initial text value
  -T, --tooltip <TOOLTIP>              Tooltip text (on hover)
  -w, --width <WIDTH>                  Window width (px) [default: 520]
  -h, --height <HEIGHT>                Window height (px) [default: 56]
  -f, --font <FONT>                    Font family [default: Inconsolata]
  -s, --font-size <FONT_SIZE>          Font size (pt) [default: 14]
      --bg <BG>                        Background color [default: #1e1e2e]
      --fg <FG>                        Foreground color [default: #cdd6f4]
      --accent <ACCENT>                Accent/caret color [default: #89b4fa]
      --radius <RADIUS>                Corner radius [default: 8]
      --decorated                      Show window decorations (off by default)
      --newline                        Print a trailing newline
  -V, --version                        Print version
  -h, --help                           Print help
```

## Sway usage

Analogue to `i3-input` for marking and jumping between windows:

```bash
# Mark the focused container with a name you type
winput --placeholder "mark name" | xargs -r -I{} swaymsg mark -- "{}"

# Jump focus to a container by its mark
winput --placeholder "jump to mark" | xargs -r -I{} swaymsg '[con_mark="{}"] focus'
```

General command runner binding:
```bash
bindsym $mod+g exec sh -c 'cmd="$(winput --placeholder Run:)"; [ -n "$cmd" ] && sh -c "$cmd"'
```

Pipe to `swaymsg` directly:
```bash
winput --placeholder "swaymsg command" --newline | xargs -r -I{} swaymsg -- {}
```

Search-style prompt with accent color:
```bash
winput --placeholder "Search" --width 640 --height 60 --accent "#a6e3a1"
```

## Reference
- `i3-input` man page: https://manpages.ubuntu.com/manpages/bionic/man1/i3-input.1.html

## License
MIT
