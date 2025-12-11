<div align="center">

<!-- ASCII Art Logo -->
```text
███████╗████████╗ █████╗ ████████╗██╗   ██╗██╗
██╔════╝╚══██╔══╝██╔══██╗╚══██╔══╝██║   ██║██║
███████╗   ██║   ███████║   ██║   ██║   ██║██║
╚════██║   ██║   ██╔══██║   ██║   ██║   ██║██║
███████║   ██║   ██║  ██║   ██║   ╚██████╔╝██║
╚══════╝   ╚═╝   ╚═╝  ╚═╝   ╚═╝    ╚═════╝ ╚═╝
```
</div>

A lightweight, async TUI for monitoring API health.

<!-- Badges -->
<div align="center">
<a href="https://github.com/Mohamed-Badry/statui/actions"><img src="https://img.shields.io/github/actions/workflow/status/Mohamed-Badry/statui/release.yml?style=flat-square" alt="Build Status"></a>
<a href="https://crates.io/crates/statui"><img src="https://img.shields.io/crates/v/statui?style=flat-square" alt="Crates.io"></a>
<a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue?style=flat-square" alt="License"></a>
</div>

---

<!-- Demo GIF -->
<p align="center">
  <img src="https://raw.githubusercontent.com/Mohamed-Badry/statui/main/assets/demo.gif" alt="Statui Demo" width="900">
</p>

## Overview

Statui is a terminal-based dashboard for monitoring HTTP endpoints in real-time. Built for developers and ML engineers who need to keep an eye on API availability, latency trends, and status codes without leaving the terminal.

It features a high-performance async backend that ensures the UI never freezes, even when monitoring dozens of slow endpoints simultaneously.

### Features
- Real-time monitoring with visual sparklines and live latency tracking
- Async backend powered by `tokio` for non-blocking I/O
- Inspector panel to show details about specific endpoints with headers, history, and error logs
- Configurable via TOML files, presets, and CLI arguments
- Compiles to a single static binary with zero runtime dependencies

---

## Installation

### Option 1: Quick Install (Binary)
Use `cargo-binstall` to get the pre-built binary instantly:
```bash
cargo binstall statui
```

### Option 2: Install using cargo
Installs statui from crates.io and compiles it for your device:
```bash
cargo install statui
```

### Option 3: From Source
```bash
cargo install --git https://github.com/Mohamed-Badry/statui statui
```

### Option 4: From Github

Open on the releases in the sidebar and install the binary for your device.

---

## Quick Start

1. Create a config: \
Statui looks for a `statui.toml` in your current directory. 

2. Run:
```bash
statui
``` 

3. Try a Preset: \
Need to monitor something quickly? Copy one of the presets in `presets` and run it:
```bash
# Monitors package registries for various languages.
statui dev_registries.toml
```

---

## Key Bindings

| Key | Action |
| :--- | :--- |
| `q` | Quit |
| `j` / `k` | Scroll Down/Scroll Up |
| `i` | Toggle Inspector Pane (Endpoint Details) |

---

## Configuration

Statui uses a layered configuration system. It loads settings in this order (last one wins):

1.  Built-in defaults (60s interval, 5s timeout)
2.  Global config (`~/.config/statui/config.toml` on Linux/Mac)
3.  Local config (`./statui.toml`)
4.  CLI arguments (`statui my_config.toml`)

### Example `statui.toml`
```toml
# -----------------------------------------------
# statui :: Global Settings
# -----------------------------------------------
# All endpoints will use these values by default

# Default interval (in seconds) to ping all endpoints
default_interval = 30

# Default timeout (in seconds) for any single request
default_timeout = 5

# -----------------------------------------------
# statui :: Endpoints
# -----------------------------------------------
# Each [[endpoints]] block is a new site to monitor.

[[endpoints]]
# A simple endpoint. It will use the default interval and timeout.
name = "Google"
url = "https://www.google.com"


[[endpoints]]
# A more complex endpoint that overrides global settings.
name = "Rust Language"
url = "https://www.rust-lang.org"

# -- Optional per-endpoint settings --
# Uncomment these to override the globals for "Rust Language" only
interval = 10       # Ping this specific API only every 10 seconds
timeout = 10        # Give it a longer 10-second timeout
method = "HEAD"     # Use HTTP HEAD instead of GET to save bandwidth

[[endpoints]]
# Failing endpoint (Connection Error)
name = "Broken API"
url = "https://www.my-non-existent-api.monkey/api"


[[endpoints]]
# Another simple endpoint to fill the table a bit.
name = "docs.rs"
url = "https://www.docs.rs"
```

---

## Presets

Statui comes with pre-configured lists for popular domains:

* AI & LLMs: `presets/ai_llm.toml` (OpenAI, Anthropic, Replicate)
* Cloud Infra: `presets/cloud_infra.toml` (AWS, Vercel, Heroku, DNS)
* Dev Registries: `presets/dev_registries.toml` (Crates.io, PyPI, NPM)

---

## Roadmap

Statui is still evolving. Here are some features currently in the works:

* Search/Filter: Press `/` to filter endpoints by name or status
* Extended HTTP Methods: Full support for POST, PUT, DELETE, PATCH with request body configuration
* Request Bodies & Headers: Define custom headers and JSON/form payloads in the config file
* Authentication: Built-in support for Bearer tokens, API keys, and Basic Auth
* Theming: Customize colors and styles via `theme.toml`
* Custom Keybindings: Remap controls through `keymap.toml`
* Column Sorting: Sort endpoints by name, status, latency, or last checked time
* Export & Logging: Save historical data to CSV or JSON for analysis

Contributions and feature requests are welcome. Feel free to open an issue or PR.

---

## Architecture

Statui follows a simple async-sync bridge architecture:

* The Backend (`tokio`): Spawns a dedicated lightweight task for every endpoint. These tasks (using `reqwest`) handle networking, timing, and retries independently.

* The Frontend (`ratatui`): Runs on the main thread. It receives updates via a thread-safe channel (`mpsc`) and renders the UI state.

This ensures that a timeout on one API never stutters the UI or delays checks for other APIs.

---

<div align="center">
  Built with Rust 🦀 
</div>
