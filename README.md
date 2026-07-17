<p align="center">
  <img src="src/logo/BlooCava.png" alt="BlooFetch Logo" width="400">
</p>

<p align="center">
  <strong>Terminal clock with system statistics</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.3.0-blue" alt="Version">
  <img src="https://img.shields.io/badge/license-MIT-green" alt="License">
  <img src="https://img.shields.io/badge/language-Rust-orange" alt="Rust">
</p>

---

## Installation

### Option 1: cargo install (if you have Rust)

```bash
cargo install bloofetch
```

### Option 2: Install script (Linux/macOS)

```bash
curl -sSL https://raw.githubusercontent.com/Ricardo-Castro-Gastelum/BlooFetch/main/install.sh | bash
```

### Option 3: Download binary

Go to [Releases](https://github.com/Ricardo-Castro-Gastelum/BlooFetch/releases) and download the binary for your platform.

## Usage

```bash
# Run with default settings
bloofetch

# Run with accent color
bloofetch -c cyan
bloofetch --color red
```

### Controls

| Key | Action |
|-----|--------|
| `q` / `Esc` | Quit |
| `p` | Toggle particles |

## Options

| Flag | Description |
|------|-------------|
| `-c`, `--color <COLOR>` | Set accent color |
| `-h`, `--help` | Print help |

### Available Colors

| Color | Preview |
|-------|---------|
| `white` | Default |
| `red` | Red accent |
| `green` | Green accent |
| `yellow` | Yellow accent |
| `blue` | Blue accent |
| `magenta` | Magenta accent |
| `cyan` | Cyan accent |
| `gray` | Gray accent |

## Features

- Full-screen TUI clock (no solid background)
- Real-time system stats (CPU, RAM, Disk)
- 8 accent colors via CLI flag
- Block-style ASCII digits
- Floating particles background (toggle with `p`)
- Auto-updates every second
- Fast startup (~1MB binary)
- Cross-platform (Linux, macOS, Windows)

## License

[MIT](LICENSE)
