<div align="center">

# 🖥️ RTL Terminal — RS

**A professional, lightweight, Rust-powered terminal with full Arabic / RTL support**

Built with [Tauri v2](https://v2.tauri.app) · Rust core engine · xterm.js frontend · Real PTY shell

[![build-windows](https://github.com/rust-memo/rtl-terminal-rs/actions/workflows/build-windows.yml/badge.svg)](https://github.com/rust-memo/rtl-terminal-rs/actions/workflows/build-windows.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-stable-orange.svg)](https://rustup.rs)

</div>

---

## ✨ Why RTL Terminal RS?

| | Electron v1.x | **Rust / Tauri v2** ✅ |
|---|---|---|
| App size | ~143 MB | **~10-15 MB** |
| RAM usage | ~300 MB | **~60 MB** |
| Shell bridge | `child_process` (no PTY) | **Real PTY** via `portable-pty` |
| Security | node integration | Hardened IPC commands only |
| Frontend | Bundled Chromium | Native system WebView (WebView2 / WebKitGTK) |
| Startup time | ~1.5s | **~300ms** |

## 🏗️ Architecture

```
┌─────────────────────────────────────────────┐
│  WebView Frontend (system WebView)          │
│  xterm.js + RTL/BiDi JS layer — offline     │
└──────────────────┬──────────────────────────┘
                   │  Tauri IPC (commands + events)
┌──────────────────▼──────────────────────────┐
│  Rust Process — Tauri v2                    │
│  pty_spawn / pty_write / pty_resize / kill  │
└──────────────────┬──────────────────────────┘
                   │
┌──────────────────▼──────────────────────────┐
│  rtl-terminal-core (pure Rust)              │
│  • BiDi engine — visual line reordering     │
│  • Arabic shaping (isolated/initial/medial) │
│  • Lam-Alef ligatures                       │
│  • ANSI-safe processing                     │
│  • Real PTY via portable-pty                │
└─────────────────────────────────────────────┘
```

### Project layout
```
rtl-terminal-rs/
├── core/                  # Rust engine crate (pure, testable everywhere)
│   ├── src/bidi.rs        #   RTL detection + visual reordering
│   ├── src/shaping.rs     #   Arabic presentation forms + lam-alef
│   ├── src/pty.rs         #   Cross-platform PTY (portable-pty)
│   └── src/lib.rs         #   process_output() — ANSI-safe pipeline
├── src-tauri/             # Tauri v2 app shell
│   ├── src/main.rs        #   IPC commands ↔ PTY session
│   ├── tauri.conf.json
│   ├── capabilities/      #   Least-privilege permissions
│   └── icons/             #   Custom icon set
├── frontend/              # WebView UI (offline, no CDN)
│   ├── index.html         #   Professional Arabic-first design
│   ├── app.js             #   IPC transport + WS fallback
│   ├── rtl-bidi.js        #   JS mirror of the Rust BiDi engine
│   └── vendor/            #   xterm.js + fit addon (vendored)
└── .github/workflows/     # Windows CI build + release
```

## 🚀 Download (Windows)

> Go to [**Releases**](https://github.com/rust-memo/rtl-terminal-rs/releases) and grab `RTL-Terminal_*_x64-setup.exe`

Built automatically by GitHub Actions on every version tag — NSIS installer + portable binary.

First launch: if SmartScreen warns (unsigned binary) → `More info` → `Run anyway`, or right-click → Properties → **Unblock**.

## 🔨 Build from source

### Prerequisites
- **Rust** stable: `rustup.rs`
- **Linux**: `libwebkit2gtk-4.1-dev build-essential libssl-dev libxdo-dev`
- **Windows**: WebView2 runtime (pre-installed on Win10/11)

### Build
```bash
# Install Tauri CLI
cargo install tauri-cli --version ^2

# Build (produces .exe / .AppImage / .deb)
cd src-tauri
cargo tauri build

# Development mode (hot reload)
cargo tauri dev
```

### Test the core engine (runs everywhere — no GUI needed)
```bash
cd core
cargo test
```
```
running 7 tests
test tests::ansi_escapes_survive .............. ok
test tests::keeps_non_rtl_untouched ........... ok
test tests::processes_rtl_line ................ ok
test tests::mixed_bidi_digits_stay_attached ... ok
test tests::shapes_alef_isolated .............. ok
test tests::detects_rtl ....................... ok
test tests::shapes_lam_alef ................... ok

test result: ok. 7 passed; 0 failed
```

## 🌍 RTL / Arabic features

- **Full BiDi reordering** — RTL lines render correctly right-to-left
- **Arabic shaping** — isolated / initial / medial / final letter forms
- **Lam-Alef ligature** — لا لأ لإ لآ
- **Mixed text** — digits & latin stay in logical order inside Arabic
- **ANSI-safe** — colors never break, even in mixed lines
- **Smart input** — `dir="auto"` box, switches direction as you type
- **One-click toggle** — flip RTL/LTR or disable shaping live

## 🔐 Security model

- No Node.js in the webview — pure `contextIsolation`
- Frontend talks to Rust only via explicit Tauri commands
- Capabilities file limits exposure to `core:default` + event APIs
- PTY runs with user privileges, no elevation

## 🤖 CI/CD

`.github/workflows/build-windows.yml` — on every `v*` tag:
1. Builds with `cargo tauri build --target x86_64-pc-windows-msvc`
2. Uploads NSIS installer + portable exe to a **draft GitHub Release**

## 📄 License

MIT © [rust-memo](https://github.com/rust-memo)
