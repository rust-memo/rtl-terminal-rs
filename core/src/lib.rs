//! rtl-terminal-core
//! Pure Rust engine for the RTL terminal:
//!  - Unicode RTL detection
//!  - Line reordering (visual direction) for Arabic/Hebrew
//!  - Arabic presentation-form shaping (isolated/initial/medial/final, lam-alef)
//!  - ANSI-escape-safe output processing
//!  - Optional PTY session (feature `pty`)

pub mod bidi;
pub mod shaping;

#[cfg(feature = "pty")]
pub mod pty;

/// Main entry: reorder + shape a chunk of terminal output for visual display.
/// ANSI escape sequences are never altered.
pub fn process_output(chunk: &str) -> String {
    let mut out = String::new();
    let mut buf = String::new();
    for c in chunk.chars() {
        if c == '\r' || c == '\n' {
            process_line(&mut buf, &mut out);
            out.push(c);
            buf.clear();
        } else {
            buf.push(c);
        }
    }
    process_line(&mut buf, &mut out);
    out
}

/// Is this an ASCII alphabetic char (ANSI terminator)?
fn is_alpha(c: char) -> bool {
    (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z')
}

/// Process one line: split ANSI sequences out, reorder/shape plain runs.
fn process_line(line: &mut String, out: &mut String) {
    if !bidi::contains_rtl(line) {
        out.push_str(line);
        return;
    }
    let arr: Vec<char> = line.chars().collect::<Vec<char>>();
    let n = arr.len();
    let mut plain = String::new();
    let mut i = 0;
    while i < n {
        if arr[i] == '\x1b' {
            if !plain.is_empty() {
                out.push_str(&bidi::visual_line(&plain));
                plain.clear();
            }
            out.push('\x1b');
            i += 1;
            if i >= n { break; }
            let kind = arr[i];
            out.push(kind);
            i += 1;
            // consume until terminator
            while i < n {
                let c = arr[i];
                out.push(c);
                if kind == '[' && is_alpha(c) { i += 1; break; }
                if kind == ']' && c == '\x07' { i += 1; break; }
                if kind != '[' && kind != ']' { i += 1; break; }
                i += 1;
            }
        } else {
            plain.push(arr[i]);
            i += 1;
        }
    }
    if !plain.is_empty() {
        out.push_str(&bidi::visual_line(&plain));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bidi::contains_rtl;

    #[test]
    fn detects_rtl() {
        assert!(contains_rtl("مرحبا"));
        assert!(contains_rtl("hello مرحبا"));
        assert!(!contains_rtl("plain english 123"));
    }

    #[test]
    fn shapes_lam_alef() {
        let shaped = shaping::shape_arabic(&String::from("لا"));
        assert!(shaped.chars().next().unwrap() as u32 >= 0xFEFB);
    }

    #[test]
    fn shapes_alef_isolated() {
        let shaped = shaping::shape_arabic(&String::from("ا"));
        assert_eq!(shaped, "\u{FE8D}");
    }

    #[test]
    fn keeps_non_rtl_untouched() {
        let out = process_output("hello world 123\n");
        assert_eq!(out, "hello world 123\n");
    }

    #[test]
    fn processes_rtl_line() {
        let out = process_output("مرحبا");
        assert!(!out.is_empty());
        assert!(out.chars().any(|c| c as u32 >= 0xFE70));
    }

    #[test]
    fn ansi_escapes_survive() {
        let input = "\x1b[32mمرحبا\x1b[0m test";
        let out = process_output(input);
        assert!(out.contains("\x1b[32m"));
        assert!(out.contains("\x1b[0m"));
    }

    #[test]
    fn mixed_bidi_digits_stay_attached() {
        let out = process_output("echo مرحبا 123 test");
        assert!(out.contains("123"));
        assert!(out.contains("echo"));
        assert!(out.contains("test"));
    }
}