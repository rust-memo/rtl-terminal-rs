//! Arabic presentation-form shaping (isolated/initial/medial/final + lam-alef).

struct VariantSet {
    iso: u64,
    ini: u64,
    med: u64,
    fin: u64,
    present: bool,
}

/// Shape-table lookup; iso=0 means the char has no presentation forms.
fn variants(c: char) -> VariantSet {
    let z = VariantSet { iso: 0, ini: 0, med: 0, fin: 0, present: false };
    match c {
        '\u{0622}' => VariantSet { iso: 0xFE81, fin: 0xFE82, present: true, ini: 0, med: 0 },
        '\u{0623}' => VariantSet { iso: 0xFE83, fin: 0xFE84, present: true, ini: 0, med: 0 },
        '\u{0624}' => VariantSet { iso: 0xFE85, fin: 0xFE86, present: true, ini: 0, med: 0 },
        '\u{0625}' => VariantSet { iso: 0xFE87, fin: 0xFE88, present: true, ini: 0, med: 0 },
        '\u{0626}' => VariantSet { iso: 0xFE89, ini: 0xFE8B, med: 0xFE8C, fin: 0xFE8A, present: true },
        '\u{0627}' => VariantSet { iso: 0xFE8D, fin: 0xFE8E, present: true, ini: 0, med: 0 },
        '\u{0628}' => VariantSet { iso: 0xFE8F, ini: 0xFE91, med: 0xFE92, fin: 0xFE90, present: true },
        '\u{0629}' => VariantSet { iso: 0xFE93, fin: 0xFE94, present: true, ini: 0, med: 0 },
        '\u{062A}' => VariantSet { iso: 0xFE95, ini: 0xFE97, med: 0xFE98, fin: 0xFE96, present: true },
        '\u{062B}' => VariantSet { iso: 0xFE99, ini: 0xFE9B, med: 0xFE9C, fin: 0xFE9A, present: true },
        '\u{062C}' => VariantSet { iso: 0xFE9D, ini: 0xFE9F, med: 0xFEA0, fin: 0xFE9E, present: true },
        '\u{062D}' => VariantSet { iso: 0xFEA1, ini: 0xFEA3, med: 0xFEA4, fin: 0xFEA2, present: true },
        '\u{062E}' => VariantSet { iso: 0xFEA5, ini: 0xFEA7, med: 0xFEA8, fin: 0xFEA6, present: true },
        '\u{062F}' => VariantSet { iso: 0xFEA9, fin: 0xFEAA, present: true, ini: 0, med: 0 },
        '\u{0630}' => VariantSet { iso: 0xFEAB, fin: 0xFEAC, present: true, ini: 0, med: 0 },
        '\u{0631}' => VariantSet { iso: 0xFEAD, fin: 0xFEAE, present: true, ini: 0, med: 0 },
        '\u{0632}' => VariantSet { iso: 0xFEAF, fin: 0xFEB0, present: true, ini: 0, med: 0 },
        '\u{0633}' => VariantSet { iso: 0xFEB1, ini: 0xFEB3, med: 0xFEB4, fin: 0xFEB2, present: true },
        '\u{0634}' => VariantSet { iso: 0xFEB5, ini: 0xFEB7, med: 0xFEB8, fin: 0xFEB6, present: true },
        '\u{0635}' => VariantSet { iso: 0xFEB9, ini: 0xFEBB, med: 0xFEBC, fin: 0xFEBA, present: true },
        '\u{0636}' => VariantSet { iso: 0xFEBD, ini: 0xFEBF, med: 0xFEC0, fin: 0xFEBE, present: true },
        '\u{0637}' => VariantSet { iso: 0xFEC1, ini: 0xFEC3, med: 0xFEC4, fin: 0xFEC2, present: true },
        '\u{0638}' => VariantSet { iso: 0xFEC5, ini: 0xFEC7, med: 0xFEC8, fin: 0xFEC6, present: true },
        '\u{0639}' => VariantSet { iso: 0xFEC9, ini: 0xFECB, med: 0xFECC, fin: 0xFECA, present: true },
        '\u{063A}' => VariantSet { iso: 0xFECD, ini: 0xFECF, med: 0xFED0, fin: 0xFECE, present: true },
        '\u{0641}' => VariantSet { iso: 0xFED1, ini: 0xFED3, med: 0xFED4, fin: 0xFED2, present: true },
        '\u{0642}' => VariantSet { iso: 0xFED5, ini: 0xFED7, med: 0xFED8, fin: 0xFED6, present: true },
        '\u{0643}' => VariantSet { iso: 0xFED9, ini: 0xFEDB, med: 0xFEDC, fin: 0xFEDA, present: true },
        '\u{0644}' => VariantSet { iso: 0xFEDF, ini: 0xFEE1, med: 0xFEE2, fin: 0xFEDE, present: true },
        '\u{0645}' => VariantSet { iso: 0xFEE3, ini: 0xFEE5, med: 0xFEE6, fin: 0xFEE4, present: true },
        '\u{0646}' => VariantSet { iso: 0xFEE7, ini: 0xFEE9, med: 0xFEEA, fin: 0xFEE8, present: true },
        '\u{0647}' => VariantSet { iso: 0xFEEB, ini: 0xFEED, med: 0xFEEE, fin: 0xFEEC, present: true },
        '\u{0648}' => VariantSet { iso: 0xFEED, fin: 0xFEEE, present: true, ini: 0, med: 0 },
        '\u{0649}' => VariantSet { iso: 0xFEF1, fin: 0xFEF0, present: true, ini: 0, med: 0 },
        '\u{064A}' => VariantSet { iso: 0xFEF3, ini: 0xFEF5, med: 0xFEF6, fin: 0xFEF4, present: true },
        _ => z,
    }
}

/// Letters that do NOT connect to the following letter (context rule).
fn breaks_context(c: char) -> bool {
    match c {
        '\u{0627}' | '\u{0622}' | '\u{0623}' | '\u{0625}' | '\u{0629}' | '\u{062F}' |
        '\u{0630}' | '\u{0631}' | '\u{0632}' | '\u{0648}' | '\u{0649}' => true,
        _ => false,
    }
}

/// Shape an Arabic run to correct presentation forms (+ lam-alef ligatures).
pub fn shape_arabic(word: &String) -> String {
    let arr: Vec<char> = word.chars().collect::<Vec<char>>();
    let n = arr.len();
    let mut out = String::new();
    let mut i = 0usize;
    while i < n {
        let c = arr[i];
        if c == '\u{0644}' && i + 1 < n {
            let nxt = arr[i + 1];
            let mut lig: u64 = 0;
            if nxt == '\u{0627}' { lig = 0xFEFB; }
            else if nxt == '\u{0623}' { lig = 0xFEF7; }
            else if nxt == '\u{0625}' { lig = 0xFEF9; }
            else if nxt == '\u{0622}' { lig = 0xFEF5; }
            if lig != 0 {
                let prev_joins = i > 0usize && !breaks_context(arr[i - 1]);
                out.push(char::from_u32(if prev_joins { lig + 1 } else { lig } as u32).unwrap());
                i += 2;
                continue;
            }
        }
        let v = variants(c);
        if !v.present {
            out.push(c);
            i += 1;
            continue;
        }
        let has_prev = i > 0usize;
        let has_next = i + 1 < n;
        let joins_prev = has_prev && !breaks_context(arr[i - 1]);
        let joins_next = has_next && !breaks_context(c);
        let code: u64 = if joins_prev && joins_next && v.med != 0 {
            v.med
        } else if joins_prev && v.fin != 0 {
            v.fin
        } else if joins_next && v.ini != 0 {
            v.ini
        } else {
            v.iso
        };
        out.push(char::from_u32(code as u32).unwrap());
        i += 1;
    }
    out
}