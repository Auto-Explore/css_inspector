pub(super) fn scan_css_number_end(bytes: &[u8]) -> Option<usize> {
    // CSS <number> grammar subset (per requested test cases):
    //   [+|-]? (
    //       [0-9]+ ('.' [0-9]+)?
    //     | '.' [0-9]+
    //   ) ([eE] [+|-]? [0-9]+)?
    //
    // Notably invalid:
    // - "12." (no digits after '.')
    // - "+-12.2" (multiple leading signs)
    // - "12.1.1" (extra characters remain)
    let mut i = 0usize;
    if matches!(bytes.first(), Some(b'+' | b'-')) {
        i += 1;
    }

    let mut saw_digit = false;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        saw_digit = true;
        i += 1;
    }

    if i < bytes.len() && bytes[i] == b'.' {
        i += 1;
        let frac_start = i;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
        if i == frac_start {
            return None;
        }
    } else if !saw_digit {
        return None;
    }

    if i < bytes.len() && matches!(bytes[i], b'e' | b'E') {
        // Only treat `e`/`E` as an exponent marker if it’s followed by an exponent with at least
        // one digit; otherwise it belongs to the unit/identifier part (e.g. `1em`).
        let mut j = i + 1;
        if j < bytes.len() && matches!(bytes[j], b'+' | b'-') {
            j += 1;
        }
        let exp_start = j;
        while j < bytes.len() && bytes[j].is_ascii_digit() {
            j += 1;
        }
        if j != exp_start {
            i = j;
        }
    }

    Some(i)
}

pub(super) fn parse_css_number(token: &str) -> Option<f64> {
    let t = token.trim();
    if t.is_empty() {
        return None;
    }
    let end = scan_css_number_end(t.as_bytes())?;
    if end != t.len() {
        return None;
    }
    let v = t.parse::<f64>().ok()?;
    v.is_finite().then_some(v)
}

pub(super) fn split_number_and_unit(s: &str) -> (Option<f64>, &str) {
    let s = s.trim();
    if s.is_empty() {
        return (None, "");
    }
    let Some(end) = scan_css_number_end(s.as_bytes()) else {
        return (None, "");
    };
    let (n, u) = s.split_at(end);
    let num = n.parse::<f64>().ok().filter(|v| v.is_finite());
    (num, u)
}
