<p align="center">
  <img src="logo.png" alt="BlooFetch" width="200">
</p>

<h1 align="center">BlooFetch</h1>

<p align="center">
  A beautiful full-screen terminal decoration with clock, system stats, and Spotify
</p>

<p align="center">
  <a href="#installation">Installation</a> •
  <a href="#usage">Usage</a> •
  <a href="#features">Features</a> •
  <a href="#license">License</a>
</p>

---

## Preview

A full-screen TUI application that shows:
- Big ASCII clock in the center
- System stats (CPU, RAM, Disk) as percentages
- Clean, minimal design

```
     Ram: 24.9%
     Cpu: 3.2%
     Disk: 48.4%

          ████████          ████████
          ██    ██          ██    ██
          ██    ██                ██
          ██    ██          ████████
          ██    ██          ██
          ██    ██          ██
          ████████          ████████


     Press q to quit
```

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
# Run the full-screen app
bloofetch

# Press q or Esc to quit
```

## Features

- **Full-screen TUI** - Takes over your terminal like cava
- **Big ASCII Clock** - Large digital clock in the center
- **System Stats** - CPU, RAM, and Disk as simple percentages
- **Minimal Design** - Clean, no borders, no clutter
- **Cross-platform** - Linux, macOS

## Controls

| Key | Action |
|-----|--------|
| `q` | Quit |
| `Esc` | Quit |

## License

[Apache-2.0](LICENSE)
