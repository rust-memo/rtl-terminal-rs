//! Unicode bidi detection + visual line reordering (logical -> visual).

use crate::shaping::shape_arabic;

const RTL_START: u32 = 0x0590;
const RTL_END: u32 = 0x08FF;
const RTL_FB_START: u32 = 0xFB1D;
const RTL_FB_END: u32 = 0xFDFF;
const RTL_FE_START: u32 = 0xFE70;
const RTL_FE_END: u32 = 0xFEFC;

fn char_is_rtl(c: char) -> bool {
    let u = c as u32;
    (u >= RTL_START && u <= RTL_END) ||
    (u >= RTL_FB_START && u <= RTL_FB_END) ||
    (u >= RTL_FE_START && u <= RTL_FE_END)
}

/// True when the string contains RTL (Hebrew/Arabic-family) characters.
pub fn contains_rtl(s: &str) -> bool {
    for c in s.chars() {
        if char_is_rtl(c) { return true; }
    }
    false
}

fn is_digit(c: char) -> bool {
    (c >= '0' && c <= '9') || (c >= '\u{0660}' && c <= '\u{0669}')
}

struct Token {
    text: String,
    rtl: bool,
}

/// Reorder + shape a single line into visual order.
pub fn visual_line(line: &String) -> String {
    if !contains_rtl(line) {
        return line.to_string();
    }
    let mut tokens: Vec<Token> = Vec::new();
    let mut cur = String::new();
    let mut cur_rtl = false;
    let mut started = false;
    for c in line.chars() {
        let r = char_is_rtl(c);
        let neutral = c == ' ' || c == '\t' || is_digit(c);
        if !started {
            cur.push(c);
            cur_rtl = r;
            started = true;
        } else if neutral || r == cur_rtl {
            cur.push(c);
        } else {
            tokens.push(Token { text: cur, rtl: cur_rtl });
            cur = String::new();
            cur.push(c);
            cur_rtl = r;
        }
    }
    if started {
        tokens.push(Token { text: cur, rtl: cur_rtl });
    }

    let mut rtl_count = 0usize;
    for t in &mut tokens {
        if t.rtl {
            t.text = shape_arabic(&t.text);
            rtl_count += 1;
        }
    }
    if rtl_count * 2 >= tokens.len() {
        tokens.reverse();
    }
    let mut out = String::new();
    for t in tokens {
        out.push_str(&t.text);
    }
    out
}