//! Optional PTY session (feature "pty") — cross-platform via portable-pty.

use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::io::Read;

/// One interactive shell session bridged over a PTY master pipe.
pub struct PtySession {
    pub master: Box<dyn portable_pty::MasterPty + Send>,
    pub writer: Option<Box<dyn std::io::Write + Send>>,
    pub reader: Box<dyn Read + Send>,
    pub child: Box<dyn portable_pty::Child + Send + Sync>,
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

fn home_cwd() -> String {
    std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string())
}

/// Spawn a shell in a PTY. On success a session with a master handle is returned.
pub fn spawn(shell: String, args: Vec<String>, cols: u16, rows: u16) -> Result<PtySession, String> {
    let pty_sys = native_pty_system();
    let size = PtySize { rows, cols, pixel_width: 0, pixel_height: 0 };
    let pair = pty_sys.openpty(size).map_err(|e| e.to_string())?;
    let mut cmd = CommandBuilder::new(&shell);
    for a in args {
        cmd.arg(a);
    }
    cmd.cwd(home_cwd());
    cmd.env("TERM", "xterm-256color");
    let child = pair.slave.spawn_command(cmd).map_err(|e| e.to_string())?;
    let master = pair.master;
    let reader = master.try_clone_reader().map_err(|e| e.to_string())?;
    let writer = master.take_writer().map_err(|e| e.to_string())?;
    drop(pair.slave);
    Ok(PtySession { master, writer: Some(writer), reader, child })
}

/// Write input bytes into the shell.
pub fn write(sess: &PtySession, data: &str) -> Result<(), String> {
    if let Some(w) = &sess.writer {
        use std::io::Write;
        w.write_all(data.as_bytes()).map_err(|e| e.to_string())?;
        w.flush().map_err(|e| e.to_string())?;
        return Ok(());
    }
    Err("no writer".to_string())
}

/// Read up to `max` bytes; caller polls in a loop with a small sleep.
pub fn read_chunk(sess: &PtySession, buf: &mut String, max: usize) -> Result<usize, String> {
    let mut tmp = vec![0u8; max];
    match sess.reader.read(&mut tmp) {
        Ok(n) => {
            if n > 0 {
                buf.push_str(&String::from_utf8_lossy(&tmp[..n]));
            }
            Ok(n)
        }
        Err(e) => Err(e.to_string()),
    }
}

/// Resize the PTY (called from UI resize events).
pub fn resize(sess: &PtySession, cols: u16, rows: u16) -> Result<(), String> {
    sess.master
        .resize(PtySize { rows, cols, pixel_width: 0, pixel_height: 0 })
        .map_err(|e| e.to_string())
}

/// Kill the child process.
pub fn kill(sess: &PtySession) {
    let _ = sess.child.kill();
}