//! Optional PTY session (feature "pty") — cross-platform via portable-pty.
//!
//! Design: a dedicated reader thread pumps shell output into a shared
//! `Arc<RwLock<VecDeque>>` buffer. The UI polls `poll()` — no event plumbing.

use std::collections::VecDeque;
use std::io::Read;
use std::io::Write;
use std::sync::{Arc, RwLock};
use portable_pty::{native_pty_system, CommandBuilder, PtySize};

/// One interactive shell session bridged over a PTY master pipe.
pub struct PtySession {
    pub master: Box<dyn portable_pty::MasterPty + Send>,
    pub child: Box<dyn portable_pty::Child + Send + Sync>,
    pub writer: Box<dyn std::io::Write + Send>,
    pub outbuf: Arc<RwLock<VecDeque<Vec<u8>>>>,
}

fn home_cwd() -> String {
    std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string())
}

/// The default shell for the platform. Returns (program, args).
pub fn default_shell() -> (String, Vec<String>) {
    if cfg!(windows) {
        let sysroot = std::env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".to_string());
        let ps = sysroot + r"\\System32\\WindowsPowerShell\\v1.0\\powershell.exe";
        if std::fs::metadata(&ps).is_ok() {
            return (ps, vec!["-NoLogo".into(), "-NoProfile".into()]);
        }
        let comspec = std::env::var("COMSPEC").unwrap_or_else(|_| "cmd.exe".to_string());
        return (comspec, vec![]);
    }
    let sh = std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());
    (sh, vec!["-i".into()])
}

/// Spawn a shell in a PTY, pump its output into a shared buffer, return the session.
pub fn spawn(shell: String, args: Vec<String>, cols: u16, rows: u16) -> Result<PtySession, String> {
    let pty_sys = native_pty_system();
    let size = PtySize { rows, cols, pixel_width: 0, pixel_height: 0 };
    let mut pair = pty_sys.openpty(size).map_err(|e| e.to_string())?;
    let mut cmd = CommandBuilder::new(&shell);
    for a in args {
        cmd.arg(a);
    }
    cmd.cwd(home_cwd());
    cmd.env("TERM", "xterm-256color");
    let child = pair.slave.spawn_command(cmd).map_err(|e| e.to_string())?;
    let mut reader = pair.master.try_clone_reader().map_err(|e| e.to_string())?;
    let writer = pair.master.take_writer().map_err(|e| e.to_string())?;
    let master = pair.master;
    let outbuf = Arc::new(RwLock::new(VecDeque::new()));
    // Reader thread: block-read master, push chunks into the shared buffer.
    let out = outbuf.clone();
    std::thread::spawn(move || {
        let mut buf = vec![0u8; 8192];
        loop {
            match reader.read(&mut buf) {
                Ok(n) => {
                    if n > 0 {
                        let mut guard = out.write().expect("outbuf lock");
                        let mut q = &mut *guard;
                        q.push_back(buf[..n].to_vec());
                    }
                }
                Err(_) => break,
            }
        }
    });
    Ok(PtySession { master, child, writer, outbuf })
}

/// Write input bytes into the shell.
pub fn write(sess: &mut PtySession, data: &str) -> Result<(), String> {
    sess.writer.write_all(data.as_bytes()).map_err(|e| e.to_string())?;
    sess.writer.flush().map_err(|e| e.to_string())?;
    Ok(())
}

/// Drain pending output (bounded). Safe to call from any thread.
pub fn poll(sess: &PtySession, max_chars: usize) -> String {
    let mut guard = sess.outbuf.write().expect("outbuf lock");
    let mut q = &mut *guard;
    let mut out = String::new();
    while !q.is_empty() && out.len() < max_chars {
        let chunk = q.pop_front();
        if let Some(c) = chunk {
            out.push_str(&String::from_utf8_lossy(&c));
        } else {
            break;
        }
    }
    out
}

/// Resize the PTY (called from UI resize events).
pub fn resize(sess: &PtySession, cols: u16, rows: u16) -> Result<(), String> {
    sess.master
        .resize(PtySize { rows, cols, pixel_width: 0, pixel_height: 0 })
        .map_err(|e| e.to_string())
}

/// Kill the child process.
pub fn kill(sess: &mut PtySession) {
    let _ = sess.child.kill();
}