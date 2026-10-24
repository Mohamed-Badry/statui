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

Open the releases in the sidebar and install the binary for your device.

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
default_interval = 3    
# you shouldn't ping your APIs so often this is just for the demo

# Default timeout (in seconds) for any single request
default_timeout = 5

# -----------------------------------------------
# statui :: Endpoints
# -----------------------------------------------
# Each [[endpoints]] block is a new site to monitor.

# ===============================================
# Standard GET & HEAD Checks
# ===============================================

[[endpoints]]
# A simple endpoint. It will use the default interval and timeout.
name = "Google"
url = "https://www.google.com"

[[endpoints]]
# A more complex endpoint that overrides global settings.
name = "Rust Language"
url = "https://www.rust-lang.org"
interval = 10       # Ping this specific API only every 10 seconds
method = "HEAD"     # Use HTTP HEAD instead of GET to save bandwidth

[[endpoints]]
name = "docs.rs"
url = "https://www.docs.rs"

# ===============================================
# POST: JSON Body & Inline Headers
# ===============================================

[[endpoints]]
# Native TOML table serialized automatically to JSON.
# Shows headers defined using inline table syntax: { key = "val" }
name = "HTTPBin POST (JSON)"
url = "https://httpbin.org/post"
method = "POST"
interval = 5
body = { service = "statui-probe", ping = true, count = 1 }
headers = { "Authorization" = "Bearer demo-token-123", "X-Monitor" = "statui" }

# ===============================================
# POST: External File Payload
# ===============================================

[[endpoints]]
# Loads body bytes directly from an external file on disk.
# Point to any file in your project or presets.
name = "HTTPBin File Payload"
url = "https://httpbin.org/post"
method = "POST"
interval = 10
body = { file = "presets/ai_llm.toml" }

# ===============================================
# PUT: Raw String Body
# ===============================================

[[endpoints]]
# Plain raw string payload (ideal for plain text, CSV, or custom protocols).
name = "HTTPBin PUT (Raw)"
url = "https://httpbin.org/put"
method = "PUT"
body = "sensor_id=42&status=healthy&uptime=99.9"
headers = { "Content-Type" = "application/x-www-form-urlencoded" }

# ===============================================
# PATCH: Multi-line Headers Sub-Table
# ===============================================

[[endpoints]]
# Shows headers defined as a separate TOML sub-table.
# Great for endpoints with lots of headers so lines don't get too long.
name = "HTTPBin PATCH (Subtable)"
url = "https://httpbin.org/patch"
method = "PATCH"
body = { operation = "patch_status", is_active = true }

[endpoints.headers]
Authorization = "Bearer env-secret-placeholder"
X-Environment = "staging"
X-Custom-Client = "statui-v0.2.3"

# ===============================================
# DELETE Check
# ===============================================

[[endpoints]]
# Testing DELETE method support
name = "HTTPBin DELETE"
url = "https://httpbin.org/delete"
method = "DELETE"
interval = 10
headers = { "X-Audit-Reason" = "healthcheck-cleanup" }

# ===============================================
# Edge Cases & Failure Demos
# ===============================================

[[endpoints]]
# Failing endpoint (Connection Error demo)
name = "Broken API"
url = "https://www.my-non-existent-api.monkey/api"

[[endpoints]]
# Self-signed certificate demo
name = "BadSSL Self Signed"
url = "https://self-signed.badssl.com/"
skip_cert_verification = true
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
