//! RTL-Terminal-RS — Tauri v2 desktop app.
//!
//! Architecture: WebView frontend (xterm.js + RTL layer) ⇄ Tauri IPC ⇄ Rust PTY.
//!
//! Poll-based protocol (no events): the frontend polls `pty_poll()` every 30 ms,
//! sends input via `pty_write()`, resizes via `pty_resize()`.

use std::sync::Mutex;
use tauri::State;

/// Managed app state — exactly one PTY session (single window).
struct PtyState(Mutex<Option<rtl_terminal_core::pty::PtySession>>);

/// Start the default shell in a PTY. Safe to call repeatedly.
#[tauri::command]
fn pty_start(state: State<PtyState>) -> Result<(), String> {
    let mut guard = state.inner().0.lock().map_err(|_| "state lock poisoned".to_string())?;
    if guard.is_some() {
        return Ok(());
    }
    let (shell, args) = rtl_terminal_core::pty::default_shell();
    let session = rtl_terminal_core::pty::spawn(shell, args, 100, 30)?;
    *guard = Some(session);
    Ok(())
}

/// Drain pending shell output (returns up to ~64 KB).
#[tauri::command]
fn pty_poll(state: State<PtyState>) -> String {
    let result = state.inner().0.lock().map_err(|_| "state lock poisoned".to_string());
    match result {
        Ok(guard) => match guard.as_ref() {
            Some(sess) => rtl_terminal_core::pty::poll(sess, 65536),
            None => String::new(),
        },
        Err(_) => String::new(),
    }
}

/// Send input bytes to the shell.
#[tauri::command]
fn pty_write(state: State<PtyState>, data: String) -> Result<(), String> {
    let mut guard = state.inner().0.lock().map_err(|_| "state lock poisoned".to_string())?;
    match &mut *guard {
        Some(sess) => rtl_terminal_core::pty::write(sess, data)?,
        None => {}
    }
    Ok(())
}

/// Resize the PTY (called from UI resize events).
#[tauri::command]
fn pty_resize(state: State<PtyState>, cols: u16, rows: u16) -> Result<(), String> {
    let guard = state.inner().0.lock().map_err(|_| "state lock poisoned".to_string())?;
    match guard.as_ref() {
        Some(sess) => rtl_terminal_core::pty::resize(sess, cols, rows)?,
        None => {}
    }
    Ok(())
}

/// Kill the shell and mark the session dead.
#[tauri::command]
fn pty_kill(state: State<PtyState>) -> Result<(), String> {
    let mut guard = state.inner().0.lock().map_err(|_| "state lock poisoned".to_string())?;
    match &mut *guard {
        Some(sess) => rtl_terminal_core::pty::kill(sess),
        None => {}
    }
    *guard = None; // clear the session after killing
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(PtyState(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![
            pty_start, pty_poll, pty_write, pty_resize, pty_kill
        ])
        .run(tauri::generate_context!())
        .expect("error while running rtl-terminal");
}