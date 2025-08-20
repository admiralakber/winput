# winput

Minimal, slick GUI input box for Wayland/Sway (GTK4). Type text, press Enter → winput prints to stdout and exits. Escape exits with code 1 and no output. Perfect for piping into xargs or swaymsg.

## Features
- Flat, modern, dark style (customizable)
- Inconsolata monospace by default (configurable)
- Press Enter: print text to stdout, exit 0
- Press Escape: exit 1 without printing
- Tooltip, placeholder, initial text, size, colors, and rounding are configurable via CLI flags

## Build and run (NixOS recommended)

### With Nix shell
```bash
nix-shell --pure
cargo build --release
./target/release/winput --placeholder "Run:" --tooltip "Type and press Enter"
```

### Without Nix
You need GTK4 development libraries installed on your system.
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

Execute what you type:
```bash
bindsym $mod+g exec sh -c 'cmd="$(winput --placeholder Run:)"; [ -n "$cmd" ] && sh -c "$cmd"'
```

Pipe to swaymsg:
```bash
bindsym $mod+e exec sh -c 'winput --placeholder "swaymsg command" --newline | xargs -r -I{} swaymsg -- {}'
```

Search-style prompt with accent color:
```bash
winput --placeholder "Search" --width 640 --height 60 --accent "#a6e3a1"
```

## License
MIT
