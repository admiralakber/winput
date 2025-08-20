# winput ✨

Slick, minimal GUI input box for Wayland/Sway (GTK4). Type, hit Enter, get clean stdout. Pipe it into power.

Inspired by `i3-input` (see the man page). Designed to feel native on Sway with a clean, flat aesthetic.

Note: This project was fully created by the Cursor agent CLI running GPT‑5, directed by `admiralakber`.

## Features
- Modern, flat dark UI (theme via flags)
- Monospace default (Inconsolata) for minimal distraction
- Enter: print to stdout, exit 0; Escape: exit 1, no output
- Tooltips, placeholder, initial text, font, colors, radius, size

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
  -H, --height <HEIGHT>                Window height (px) [default: 56]
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

### Quick bindings (simple and effective)
```bash
# mark and jump
bindsym $mod+Shift+g exec winput --placeholder "mark" | xargs -I{} swaymsg "mark {}"
bindsym $mod+g        exec winput --placeholder "jump to mark" | xargs -I{} swaymsg '[con_mark="{}"] focus'
```

Tips:
- If you prefer to skip executing when input is empty, add `-r` to xargs.
- If Sway’s environment lacks `~/.cargo/bin` in PATH, use the absolute path: `$HOME/.cargo/bin/winput`.

General command runner binding:
```bash
bindsym $mod+g exec sh -c 'cmd="$(winput --placeholder Run:)"; [ -n "$cmd" ] && sh -c "$cmd"'
```

### Pipe to `swaymsg` directly
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
