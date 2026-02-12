use crate::strutil::{ascii_lowercase_cow, starts_with_ascii_ci, step_string_state};

pub(crate) fn is_valid_url_function_token(token: &str) -> bool {
    let t = token.trim();
    if !starts_with_ascii_ci(t, "url(") || !t.ends_with(')') || t.len() < 5 {
        return false;
    }
    let mut rest = t[4..t.len() - 1].trim();

    // CSS Values 4 allows empty `url()` values; they normalize to `url("")`.
    if rest.is_empty() {
        return true;
    }

    if rest.starts_with('"') || rest.starts_with('\'') {
        let bytes = rest.as_bytes();
        let q = bytes[0];
        let mut i = 1usize;
        let mut escape = false;
        let mut closed = false;

        while i < bytes.len() {
            let b = bytes[i];
            if escape {
                escape = false;
            } else if b == b'\\' {
                escape = true;
            } else if b == q {
                i += 1;
                closed = true;
                break;
            }
            i += 1;
        }

        if !closed {
            return false;
        }

        rest = rest[i..].trim_start();
    } else {
        // Unquoted URL token: parse up to the first unescaped whitespace.
        let bytes = rest.as_bytes();
        let mut i = 0usize;
        let mut escape = false;
        while i < bytes.len() {
            let b = bytes[i];
            if escape {
                escape = false;
                i += 1;
                continue;
            }
            if b == b'\\' {
                escape = true;
                i += 1;
                continue;
            }
            if b.is_ascii_whitespace() {
                break;
            }
            i += 1;
        }
        if escape {
            return false;
        }
        let url = &rest[..i];
        // Unquoted URLs must not end in an escaping backslash (would escape the closing `)`).
        if url.ends_with('\\') {
            return false;
        }
        rest = rest[i..].trim_start();
    }

    // URL modifiers: `url("..." cross-origin(anonymous))`
    while !rest.is_empty() {
        let Some(open) = rest.find('(') else {
            return false;
        };
        let name = rest[..open].trim();
        if name.is_empty() || !name.eq_ignore_ascii_case("cross-origin") {
            return false;
        }

        let bytes = rest.as_bytes();
        let mut i = open;
        let mut depth: i32 = 0;
        let mut in_string: Option<u8> = None;
        let mut escape = false;

        while i < bytes.len() {
            let b = bytes[i];
            if step_string_state(b, &mut in_string, &mut escape) {
                i += 1;
                continue;
            }
            match b {
                b'(' => depth += 1,
                b')' => {
                    depth -= 1;
                    if depth == 0 {
                        i += 1;
                        break;
                    }
                }
                _ => {}
            }
            i += 1;
        }

        if depth != 0 || in_string.is_some() {
            return false;
        }

        let func = &rest[..i];
        let func_lower = ascii_lowercase_cow(func);
        let func_lower = func_lower.as_ref();
        if !super::is_balanced_function_call_token(func_lower, "cross-origin") {
            return false;
        }

        rest = rest[i..].trim_start();
    }

    true
}
