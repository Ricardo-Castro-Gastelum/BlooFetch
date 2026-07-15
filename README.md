<p align="center">
  <img src="logo.png" alt="BlooFetch" width="200">
</p>

<h1 align="center">BlooFetch</h1>

<p align="center">
  A beautiful terminal decoration with clock, system stats, and Spotify
</p>

<p align="center">
  <a href="#installation">Installation</a> •
  <a href="#usage">Usage</a> •
  <a href="#features">Features</a> •
  <a href="#license">License</a>
</p>

---

## Preview

```
┌────────────────────────────────────────────────┐
│ Ram:  24.9% [████░░░░░░░░░░░░░░░░] 3.4/13.5GB │
│ Cpu:   0.0% [░░░░░░░░░░░░░░░░░░░░] 0%         │
│ Disk:  48.4% [█████████░░░░░░░░░░░] 229.8/474GB│
│                                                │
│            ██   ██████      ██   ██  ██        │
│            ██   ██          ██   ██  ██  ██    │
│           ██   ██████            ██████  ████  │
│            ██   ██  ██      ██       ██        │
│            ██   ██████      ██       ██  ████  │
│                                                │
│               ♫ Song - Artist                  │
└────────────────────────────────────────────────┘
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
# Show once (default)
bloofetch

# Auto-refresh every second
bloofetch --loop

# Custom refresh interval (seconds)
bloofetch --loop --interval 5

# Custom width
bloofetch --width 60
```

## Features

- **Big ASCII Clock** - Large digital clock in the center
- **System Stats** - CPU, RAM, and Disk usage with progress bars
- **Spotify Integration** - Shows current song with playback controls
- **Auto-refresh** - Update display in real-time
- **Cross-platform** - Linux, macOS, Windows

## Detected System Info

| Stat | Source |
|------|--------|
| CPU | Global CPU usage |
| RAM | Used/Total memory |
| Disk | Root filesystem usage |
| Spotify | D-Bus MPRIS2 interface |

## License

[Apache-2.0](LICENSE)
