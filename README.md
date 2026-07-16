<p align="center">
  <img src="logo.png" alt="BlooFetch" width="200">
</p>

<h1 align="center">BlooFetch</h1>

<p align="center">
  Full-screen terminal clock with system stats
</p>

<p align="center">
  <a href="#installation">Installation</a> •
  <a href="#usage">Usage</a> •
  <a href="#options">Options</a> •
  <a href="#license">License</a>
</p>

---

## Installation

### Option 1: cargo install (if you have Rust)

```bash
cargo install bloofetch
```

### Option 2: Install script (Linux/macOS)

```bash
curl -sSL https://raw.githubusercontent.com/Ricardo-Castro-Gastelum/bloofetch/main/install.sh | bash
```

### Option 3: Download binary

Go to [Releases](https://github.com/Ricardo-Castro-Gastelum/bloofetch/releases) and download the binary for your platform.

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
- Auto-updates every second
- Fast startup (~1MB binary)
- Cross-platform (Linux, macOS, Windows)

## License

[Apache-2.0](LICENSE)
