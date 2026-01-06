# HyperFetch

<div align="center">

**A blazingly fast, feature-rich system information tool written in Rust**

*An enhanced alternative to fastfetch with more features and better output*

[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

</div>

## Features

### License

HyperFetch is licensed under the **GNU GPLv3**. The ASCII art logos are sourced from the **fastfetch** project and are covered by the **MIT License** (see `LICENSE.mit`). This means the overall project is dual‑licensed: the code is GPLv3, while the logo assets are MIT‑licensed.

- **GPLv3** – Full source code, all Rust modules, and documentation.
- **MIT** – All files under `src/logos/ascii/` (the original fastfetch ASCII art).

When redistributing HyperFetch, include both `LICENSE` (GPLv3) and `LICENSE.mit` files.






## Installation

### Quick Install
To download and install the latest release directly (replace `v1.0.0` with the latest version):

```bash
# Download
wget https://github.com/compiledkernel-idk/hyperfetch/releases/download/v1.0.0/hyperfetch-linux-x86_64-v1.0.0.tar.gz

# Extract
tar -xzvf hyperfetch-linux-x86_64-v1.0.0.tar.gz

# Install to /usr/local/bin
sudo install -Dm755 hyperfetch /usr/local/bin/hyperfetch
```

### From Source
```bash
git clone https://github.com/compiledkernel-idk/hyperfetch
cd hyperfetch
cargo build --release
sudo cp target/release/hyperfetch /usr/local/bin/
```

### Requirements
- wget
- Rust 1.70+
- Optional: Nerd Font for icons
- git

## Usage

### Basic Usage
```bash
# Display system information
hyperfetch

# Use a different color theme
hyperfetch --color dracula

# Show with benchmark
hyperfetch --benchmark

# View top processes
hyperfetch --processes

# Show color palette
hyperfetch --colors

# Hide ASCII logo
hyperfetch --no-logo

# Override distro logo
hyperfetch --logo ubuntu

# JSON output
hyperfetch --json

# Disable Nerd Font icons
hyperfetch --no-icons
```

### Available Options
```
-l, --logo <LOGO>      Override distro logo (arch, debian, ubuntu, etc.)
-c, --color <THEME>    Color theme (default, dracula, nord, gruvbox, catppuccin, monokai)
-s, --small            Compact output mode
-a, --all              Show all available modules
    --no-logo          Hide ASCII art logo
    --benchmark        Show quick benchmark score
    --processes        Show top CPU-consuming processes
    --colors           Show terminal color palette
    --json             Output as JSON
    --no-icons         Disable icons (for terminals without Nerd Fonts)
-h, --help             Print help
-V, --version          Print version
```

##  Color Themes

HyperFetch includes several built-in color themes:

- **Default** - Blue and cyan
- **Dracula** - Purple and cyan theme
- **Nord** - Polar blue theme
- **Gruvbox** - Retro warm colors
- **Catppuccin** - Pastel theme
- **Monokai** - Classic editor theme

##  Supported Logos

HyperFetch includes 25+ ASCII art logos for popular distros:

**Arch-based:** Arch, Manjaro, EndeavourOS, Garuda, CachyOS, Artix

**Debian-based:** Debian, Ubuntu, Mint, Pop!_OS, Elementary, Kali, MX Linux

**Red Hat-based:** Fedora, RHEL, CentOS, Rocky, AlmaLinux

**Other:** Gentoo, NixOS, Void, Alpine, OpenSUSE, Slackware, and more

*ASCII art sourced from the [fastfetch official repository](https://github.com/fastfetch-cli/fastfetch)*

##  System Information Modules

- **User & Host** - Username and hostname
- **OS** - Distribution and architecture
- **Kernel** - Kernel version
- **CPU** - Model, cores, threads, frequency
- **GPU** - Vendor, model, driver
- **Memory** - RAM usage with progress bar
- **Disk** - Storage usage with progress bar
- **Uptime** - System uptime
- **Shell** - Shell name and version
- **Desktop** - DE/WM and display server
- **Display** - Resolution and refresh rate
- **Battery** - Charge level and status (laptops)
- **Packages** - Count from multiple package managers
- **Terminal** - Terminal emulator detection
- **Network** - Local IP address



##  Development

### Building from Source
```bash
cargo build --release
```

### Running Tests
```bash
cargo test
```

### Project Structure
```
hyperfetch/
├── src/
│   ├── main.rs              # Entry point
│   ├── cli.rs               # CLI argument parsing
│   ├── modules/             # System detection modules
│   │   ├── os.rs
│   │   ├── cpu.rs
│   │   ├── gpu.rs
│   │   ├── memory.rs
│   │   └── ... (14 modules)
│   ├── logos/               # ASCII art logos
│   │   └── ascii/           # Logo files from fastfetch
│   ├── output/              # Rendering engine
│   │   ├── colors.rs        # Theme system
│   │   ├── progress.rs      # Progress bars
│   │   └── icons.rs         # Nerd Font icons
│   └── features/            # Extra features
│       ├── benchmark.rs     # Performance benchmark
│       ├── processes.rs     # Top processes viewer
│       └── colors_preview.rs # Color palette
└── Cargo.toml
```

##  License

MIT License - see LICENSE file for details

##  Acknowledgments

- **ASCII Art** - Logos sourced from [fastfetch](https://github.com/fastfetch-cli/fastfetch)


##  Contributing

Contributions are welcome! Feel free to:
- Report bugs
- Suggest features
- Submit pull requests

##  Contact

- GitHub Issues: [Report a bug or request a feature](https://github.com/compiledkernel-idk/hyperfetch/issues)

---

