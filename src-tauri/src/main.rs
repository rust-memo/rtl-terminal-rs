//! RTL-Terminal-RS — Tauri v2 desktop app.
//!
//! Architecture:
//!   WebView frontend (xterm.js + RTL/BiDi JS layer)
//!        ↕  Tauri IPC (commands + events)
//!   Rust process — spawns & drives a real PTY shell via `rtl-terminal-core`
//!
//! Professional notes:
//!  - Shell is a real PTY (portable-pty): fully interactive, resize-aware.
//!  - Frontend gets `pty:data` events; sends `pty:write`, `pty:resize`,
//!    `pty:kill` commands.
//!  - All the visual RTL transform stays in the JS layer (see frontend/app.js).
//!    Rust core also exposes the same engine for headless/tests.

use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager, State};

/// Shared PTY session protected by a mutex (single-window app for now).
struct PtyState(Mutex<Option<rtl_terminal_core::pty::PtySession>>);

/// The default shell for the platform (PowerShell on Windows, bash on Linux).
#[tauri::command]
fn default_shell() -> (String, Vec<String>) {
    rtl_terminal_core::pty::default_shell()
}

#[tauri::command]
fn pty_spawn(
    app: AppHandle,
    state: State<PtyState>,
    shell: String,
    args: Vec<String>,
    cols: u16,
    rows: u16,
) -> Result<(), String> {
    let mut guard = state.0.lock().map_err(|_| "state lock poisoned")?;
    if guard.is_some() {
        return Ok(()); // already running
    }
    let session = rtl_terminal_core::pty::spawn(shell, args, cols, rows)?;
    // Reader thread: pump PTY output into the WebView via events.
    // The session is moved into the thread so master reads stay exclusive.
    let app2 = app.clone();
    std::thread::spawn(move || {
        let mut buf = String::new();
        loop {
            match rtl_terminal_core::pty::read_chunk(&session, &mut buf, 4096) {
                Ok(n) => {
                    if n > 0 {
                        let _ = app2.emit("pty:data", buf.clone());
                        buf.clear();
                    } else {
                        std::thread::sleep(std::time::Duration::from_millis(16));
                    }
                }
                Err(_) => break,
            }
        }
        let _ = app.emit("pty:data", "\r\n\x1b[90m[shell exited]\x1b[0m\r\n");
    });
    *guard = Some(session);
    Ok(())
}

#[tauri::command]
fn pty_write(state: State<PtyState>, data: String) -> Result<(), String> {
    let guard = state.0.lock().map_err(|_| "state lock poisoned")?;
    if let Some(session) = guard.as_ref() {
        rtl_terminal_core::pty::write(session, data)?;
    }
    Ok(())
}

#[tauri::command]
fn pty_resize(state: State<PtyState>, cols: u16, rows: u16) -> Result<(), String> {
    let guard = state.0.lock().map_err(|_| "state lock poisoned")?;
    if let Some(session) = guard.as_ref() {
        rtl_terminal_core::pty::resize(session, cols, rows)?;
    }
    Ok(())
}

#[tauri::command]
fn pty_kill(state: State<PtyState>) -> Result<(), String> {
    let mut guard = state.0.lock().map_err(|_| "state lock poisoned")?;
    if let Some(session) = guard.take() {
        rtl_terminal_core::pty::kill(session);
    }
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(PtyState(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            pty_spawn, pty_write, pty_resize, pty_kill
        ])
        .run(tauri::generate_context!())
        .expect("error while running rtl-terminal");
}