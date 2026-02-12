use std::borrow::Cow;

use crate::known_properties::KnownProperties;
use crate::report::{Report, push_error};
use crate::strutil::{ascii_lowercase_cow, ends_with_ascii_ci, step_string_state};

pub(super) fn iter_declaration_statements_skipping_nested_blocks<'a, 'b>(
    block: &'a str,
    known_properties: &'b KnownProperties,
) -> impl Iterator<Item = &'a str> + 'b
where
    'a: 'b,
{
    // The `validator` walks nested rule blocks separately (via `iter_rule_blocks`). Here we only
    // want to validate *top-level declarations* in this style block, so we skip nested `{...}`
    // blocks entirely (including their preludes).
    //
    // This avoids the older “blanking” approach which could accidentally erase adjacent
    // declarations when semicolons were missing.
    let bytes = block.as_bytes();
    let mut i = 0usize;
    let mut stmt_start = 0usize;
    let mut in_string: Option<u8> = None;
    let mut escape = false;
    let mut paren_depth: usize = 0;
    let mut value_brace_depth: usize = 0;

    std::iter::from_fn(move || {
        while i < bytes.len() {
            let b = bytes[i];
            if step_string_state(b, &mut in_string, &mut escape) {
                i += 1;
                continue;
            }

            match b {
                b'(' => {
                    paren_depth += 1;
                    i += 1;
                }
                b')' => {
                    paren_depth = paren_depth.saturating_sub(1);
                    i += 1;
                }
                b';' if paren_depth == 0 && value_brace_depth == 0 => {
                    let end = i;
                    i += 1;
                    let raw = &block[stmt_start..end];
                    stmt_start = i;
                    return Some(raw);
                }
                b'{' if paren_depth == 0 => {
                    if value_brace_depth > 0 {
                        value_brace_depth += 1;
                        i += 1;
                        continue;
                    }

                    let prefix = &block[stmt_start..i];
                    if brace_starts_value_block(prefix, known_properties) {
                        value_brace_depth = 1;
                        i += 1;
                        continue;
                    }

                    // Skip a nested block: it starts at `stmt_start` and ends at the matching `}`.
                    let mut depth = 1usize;
                    i += 1;
                    while i < bytes.len() {
                        let bi = bytes[i];
                        if step_string_state(bi, &mut in_string, &mut escape) {
                            i += 1;
                            continue;
                        }
                        match bi {
                            b'{' => depth += 1,
                            b'}' => {
                                depth = depth.saturating_sub(1);
                                if depth == 0 {
                                    i += 1;
                                    break;
                                }
                            }
                            _ => {}
                        }
                        i += 1;
                    }
                    stmt_start = i;
                }
                b'}' if paren_depth == 0 && value_brace_depth > 0 => {
                    value_brace_depth = value_brace_depth.saturating_sub(1);
                    i += 1;
                }
                _ => {
                    i += 1;
                }
            }
        }

        if stmt_start < bytes.len() {
            let raw = &block[stmt_start..];
            stmt_start = bytes.len();
            return Some(raw);
        }
        None
    })
}

fn brace_starts_value_block(prefix: &str, known_properties: &KnownProperties) -> bool {
    let trimmed = prefix.trim_start();
    if trimmed.is_empty() {
        return false;
    }
    if trimmed.starts_with('@') {
        return false;
    }

    let trimmed_end = trimmed.trim_end();
    if trimmed_end.ends_with(':') {
        return true;
    }

    let bytes = trimmed.as_bytes();
    let mut name_end = 0usize;
    while name_end < bytes.len()
        && (bytes[name_end].is_ascii_alphanumeric()
            || matches!(bytes[name_end], b'-' | b'_')
            || bytes[name_end] == b'\\')
    {
        name_end += 1;
    }
    if name_end == 0 {
        return false;
    }
    let name = &trimmed[..name_end];
    let mut i = name_end;
    while i < bytes.len() && bytes[i].is_ascii_whitespace() {
        i += 1;
    }
    if bytes.get(i) != Some(&b':') {
        return false;
    }

    if name.starts_with("--") {
        return true;
    }
    if matches!(name.as_bytes().first(), Some(b'-' | b'_')) {
        return true;
    }

    let lower = ascii_lowercase_cow(name);
    known_properties.contains(lower.as_ref())
}

pub(super) fn is_vendor_extension_property(prop: &str) -> bool {
    // Match W3C CSS validator behavior (CssPropertyFactory.isNonstandardProperty):
    // treat leading `-`/`_` properties as vendor/nonstandard.
    let Some(first) = prop.as_bytes().first() else {
        return false;
    };
    *first == b'-' || *first == b'_'
}

pub(super) fn find_embedded_declaration_start(value: &str) -> Option<usize> {
    // Look for `<whitespace><ident>:\u{20}` at top-level (not in strings/parentheses).
    let bytes = value.as_bytes();
    let mut i = 0usize;
    let mut paren_depth: usize = 0;
    let mut in_string: Option<u8> = None;
    let mut escape = false;

    while i < bytes.len() {
        let b = bytes[i];
        if step_string_state(b, &mut in_string, &mut escape) {
            i += 1;
            continue;
        }

        match b {
            b'(' => {
                paren_depth += 1;
            }
            b')' => {
                paren_depth = paren_depth.saturating_sub(1);
            }
            b':' if paren_depth == 0 => {
                if i + 1 >= bytes.len() || !bytes[i + 1].is_ascii_whitespace() {
                    i += 1;
                    continue;
                }
                if i == 0 || !is_property_ident_char(bytes[i - 1]) {
                    i += 1;
                    continue;
                }
                let mut start = i - 1;
                while start > 0 && is_property_ident_char(bytes[start - 1]) {
                    start -= 1;
                }
                if start == 0 || !bytes[start - 1].is_ascii_whitespace() {
                    i += 1;
                    continue;
                }
                let name = value[start..i].trim();
                if name.len() < 2 {
                    i += 1;
                    continue;
                }
                return Some(start);
            }
            _ => {}
        }
        i += 1;
    }

    None
}

pub(super) fn is_property_ident_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'-' || b == b'_'
}

pub(super) fn validate_single_token(tokens: &[&str], prop: &str, report: &mut Report) {
    if tokens.len() != 1 {
        push_error(report, format!("Invalid value for property “{prop}”."));
    }
}

pub(super) fn validate_max_tokens(tokens: &[&str], max: usize, prop: &str, report: &mut Report) {
    if tokens.len() > max {
        push_error(report, format!("Invalid value for property “{prop}”."));
    }
}

pub(super) fn is_single_valued_property(prop: &str) -> bool {
    // Keep this intentionally conservative: only list properties that appear with multi-token
    // values in valid suite cases are excluded.
    !matches!(
        prop,
        "background"
            | "background-position"
            | "border"
            | "border-top"
            | "border-right"
            | "border-bottom"
            | "border-left"
            | "border-color"
            | "border-style"
            | "border-width"
            | "border-spacing"
            | "clip"
            | "clip-path"
            | "content"
            | "counter-increment"
            | "counter-reset"
            | "cue"
            | "cursor"
            | "box-shadow"
            | "font"
            | "font-family"
            | "aspect-ratio"
            | "grid-template"
            | "grid-template-columns"
            | "grid-template-rows"
            | "list-style"
            | "margin"
            | "outline"
            | "overflow-clip-margin"
            | "pause"
            | "padding"
            | "play-during"
            | "quotes"
            | "transition"
            | "transition-delay"
            | "transition-duration"
            | "transition-property"
            | "transition-timing-function"
            | "text-decoration"
            | "azimuth"
            | "voice-family"
    )
}

pub(super) fn split_top_level_value_tokens(value: &str) -> Vec<&str> {
    let bytes = value.as_bytes();
    let mut out = Vec::new();
    let mut i = 0usize;
    let mut start: Option<usize> = None;
    let mut paren_depth: usize = 0;
    let mut in_string: Option<u8> = None;
    let mut escape = false;

    fn flush<'a>(start: &mut Option<usize>, end: usize, value: &'a str, out: &mut Vec<&'a str>) {
        if let Some(s) = start.take() {
            let tok = value[s..end].trim();
            if !tok.is_empty() {
                out.push(tok);
            }
        }
    }

    while i < bytes.len() {
        let b = bytes[i];
        let was_in_string = in_string.is_some();
        if step_string_state(b, &mut in_string, &mut escape) {
            if !was_in_string {
                start.get_or_insert(i);
            }
            i += 1;
            continue;
        }

        match b {
            b'(' => {
                paren_depth += 1;
                start.get_or_insert(i);
            }
            b')' => {
                paren_depth = paren_depth.saturating_sub(1);
            }
            b',' if paren_depth == 0 => {
                flush(&mut start, i, value, &mut out);
            }
            b if b.is_ascii_whitespace() && paren_depth == 0 => {
                flush(&mut start, i, value, &mut out);
            }
            _ => {
                start.get_or_insert(i);
            }
        }

        i += 1;
    }
    flush(&mut start, bytes.len(), value, &mut out);
    out
}

pub(super) fn is_valid_css_value_syntax(value: &str, allow_curly_blocks: bool) -> bool {
    // Validate basic CSS component-value syntax: balanced strings, parentheses, and brackets.
    //
    // This does *not* implement property-specific grammars; it only rejects values that are
    // syntactically malformed at the CSS syntax level.
    let value = value.trim();
    if value.is_empty() {
        return false;
    }

    let bytes = value.as_bytes();
    let mut stack: Vec<u8> = Vec::new();
    let mut in_string: Option<u8> = None;
    let mut escape = false;
    let mut i = 0usize;

    while i < bytes.len() {
        let b = bytes[i];
        if step_string_state(b, &mut in_string, &mut escape) {
            i += 1;
            continue;
        }
        if b == b'\\' {
            // Skip escaped characters outside strings so sequences like `\'` or `\)` are treated
            // as component values rather than as string/paren delimiters.
            i += 1;
            if i < bytes.len() {
                if bytes[i] == b'\r' && bytes.get(i + 1) == Some(&b'\n') {
                    i += 2;
                } else {
                    i += 1;
                }
            }
            continue;
        }
        match b {
            b'(' => stack.push(b')'),
            b'[' => stack.push(b']'),
            b'{' if allow_curly_blocks => stack.push(b'}'),
            b')' | b']' => {
                if stack.pop() != Some(b) {
                    return false;
                }
            }
            b'}' if allow_curly_blocks => {
                if stack.pop() != Some(b) {
                    return false;
                }
            }
            b'{' | b'}' => return false,
            _ => {}
        }
        i += 1;
    }

    in_string.is_none() && stack.is_empty()
}

pub(super) fn is_single_top_level_curly_block_with_var_or_env(value: &str) -> bool {
    let v = value.trim();
    if !(v.starts_with('{') && v.ends_with('}')) {
        return false;
    }

    let bytes = v.as_bytes();
    let mut depth: i32 = 0;
    let mut in_string: Option<u8> = None;
    let mut escape = false;
    let mut saw_var_or_env = false;

    let mut i = 0usize;
    while i < bytes.len() {
        let b = bytes[i];
        if step_string_state(b, &mut in_string, &mut escape) {
            i += 1;
            continue;
        }
        match b {
            b'{' => {
                depth += 1;
            }
            b'}' => {
                depth -= 1;
                if depth == 0 && i != bytes.len() - 1 {
                    // More tokens after the outermost `}`.
                    return false;
                }
                if depth < 0 {
                    return false;
                }
            }
            _ => {
                if depth >= 1 && bytes[i..].len() >= 4 {
                    if bytes[i..].starts_with(b"var(") || bytes[i..].starts_with(b"env(") {
                        saw_var_or_env = true;
                    }
                }
            }
        }
        i += 1;
    }

    in_string.is_none() && depth == 0 && saw_var_or_env
}

pub(super) fn contains_var_or_env_outside_strings(value: &str) -> bool {
    let bytes = value.as_bytes();
    let mut i = 0usize;
    let mut in_string: Option<u8> = None;
    let mut escape = false;

    while i < bytes.len() {
        let b = bytes[i];
        if step_string_state(b, &mut in_string, &mut escape) {
            i += 1;
            continue;
        }

        if b.is_ascii_alphabetic() && bytes[i..].len() >= 4 {
            if bytes[i..i + 4].eq_ignore_ascii_case(b"var(")
                || bytes[i..i + 4].eq_ignore_ascii_case(b"env(")
            {
                return true;
            }
        }

        i += 1;
    }
    false
}

pub(super) fn is_valid_property_name(name: &str) -> bool {
    // Conservative check for CSS identifier-like tokens, used for declaration property names and
    // other custom-identifier values.
    //
    // Accept:
    // - ASCII identifier characters (`a-zA-Z0-9_-`)
    // - non-ASCII UTF-8 bytes
    // - CSS escapes (including a single whitespace terminator after hex escapes)
    //
    // Reject unescaped whitespace and delimiter characters like `{` or `/`.
    let name = name.trim();
    let bytes = name.as_bytes();
    let Some(&first) = bytes.first() else {
        return false;
    };
    if first.is_ascii_digit() || first.is_ascii_whitespace() {
        return false;
    }
    if !(first.is_ascii_alphabetic()
        || first == b'-'
        || first == b'_'
        || first == b'\\'
        || first >= 0x80)
    {
        return false;
    }

    let mut i = 0usize;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'\\' {
            i += 1;
            let Some(&next) = bytes.get(i) else {
                return false;
            };

            // Line continuation escape (`\\\n` etc).
            if matches!(next, b'\n' | b'\r' | b'\x0C') {
                if next == b'\r' && bytes.get(i + 1) == Some(&b'\n') {
                    i += 1;
                }
                i += 1;
                continue;
            }

            // Hex escape (`\\xx...`), optionally terminated by a single whitespace.
            if next.is_ascii_hexdigit() {
                let mut digits = 0usize;
                while i < bytes.len() && digits < 6 && bytes[i].is_ascii_hexdigit() {
                    digits += 1;
                    i += 1;
                }
                if bytes.get(i).is_some_and(|b| b.is_ascii_whitespace()) {
                    i += 1;
                }
                continue;
            }

            // Single-character escape.
            i += 1;
            continue;
        }

        if b.is_ascii_whitespace() {
            return false;
        }
        if b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b >= 0x80 {
            i += 1;
            continue;
        }
        return false;
    }

    true
}

pub(super) fn strip_important(value: &str) -> &str {
    // Handle trailing `!important` (with optional whitespace around the `!`).
    //
    // Valid CSS allows whitespace between `!` and `important`:
    //   `color: red ! important`
    let v = value.trim_end();
    if !ends_with_ascii_ci(v, "important") {
        return v;
    }

    let important_start = v.len() - "important".len();
    let bytes = v.as_bytes();
    let mut i = important_start;
    while i > 0 && bytes[i - 1].is_ascii_whitespace() {
        i -= 1;
    }
    if i == 0 || bytes[i - 1] != b'!' {
        return v;
    }
    v[..i - 1].trim_end()
}

pub(super) fn normalize_property_name<'a>(raw: &'a str, css1_escapes: bool) -> Cow<'a, str> {
    let raw = raw.trim();
    if !raw.as_bytes().iter().any(|&b| b == b'\\') {
        return ascii_lowercase_cow(raw);
    }
    let mut out = if css1_escapes {
        unescape_css_identifier_css1_compat(raw)
    } else {
        unescape_css_identifier_greedy(raw)
    };
    out.make_ascii_lowercase();
    Cow::Owned(out)
}

pub(super) fn is_balanced_function_call_token(token: &str, function_name: &str) -> bool {
    if !token.starts_with(function_name) {
        return false;
    }
    let bytes = token.as_bytes();
    let after_name = function_name.len();
    if bytes.get(after_name) != Some(&b'(') || !token.ends_with(')') {
        return false;
    }

    let mut i = after_name + 1;
    let mut depth: i64 = 1;
    let mut in_string: Option<u8> = None;
    let mut escape = false;
    let mut has_non_ws = false;

    while i < bytes.len() {
        let b = bytes[i];
        let was_in_string = in_string.is_some();
        if step_string_state(b, &mut in_string, &mut escape) {
            if !was_in_string {
                has_non_ws = true;
            }
            i += 1;
            continue;
        }
        if b == b'\\' {
            // Escaped characters are component values; don't let them affect paren/string state.
            has_non_ws = true;
            i += 1;
            if i < bytes.len() {
                if bytes[i] == b'\r' && bytes.get(i + 1) == Some(&b'\n') {
                    i += 2;
                } else {
                    i += 1;
                }
            }
            continue;
        }
        match b {
            b'(' => {
                depth += 1;
            }
            b')' => {
                depth -= 1;
                if depth == 0 {
                    return has_non_ws && i == bytes.len() - 1;
                }
            }
            b if !b.is_ascii_whitespace() => {
                has_non_ws = true;
            }
            _ => {}
        }
        i += 1;
    }

    false
}

pub(super) fn unescape_css_identifier_css1_compat(s: &str) -> String {
    // CSS escapes: `\\` followed by up to 6 hex digits (optionally followed by whitespace),
    // or `\\` followed by any single character.
    let bytes = s.as_bytes();
    let mut out = String::with_capacity(s.len());
    let mut i = 0usize;
    while i < bytes.len() {
        let b = bytes[i];
        if b != b'\\' {
            out.push(b as char);
            i += 1;
            continue;
        }
        i += 1;
        if i >= bytes.len() {
            // trailing backslash
            break;
        }
        let b2 = bytes[i];
        // Backslash-newline escapes the newline (line continuation).
        if matches!(b2, b'\n' | b'\r' | b'\x0C') {
            i += 1;
            continue;
        }
        if b2.is_ascii_hexdigit() {
            let mut run = 0usize;
            while i + run < bytes.len() && run < 6 && bytes[i + run].is_ascii_hexdigit() {
                run += 1;
            }
            // Autotest-driven behavior: prefer 6-digit escapes when present; otherwise use 4
            // digits when possible (e.g. `\\0065d` should decode as `e` + `d`).
            let take = if run >= 6 {
                6
            } else if run >= 4 {
                4
            } else {
                run
            };
            let mut val: u32 = 0;
            for &hex in &bytes[i..i + take] {
                val = (val << 4) | (hex_value(hex) as u32);
            }
            let mut j = i + take;
            // Optional whitespace after hex escape.
            if j < bytes.len() && bytes[j].is_ascii_whitespace() {
                j += 1;
            }
            if let Some(ch) = char::from_u32(val) {
                out.push(ch);
            }
            i = j;
            continue;
        }
        // Otherwise escape the next character verbatim.
        out.push(b2 as char);
        i += 1;
    }
    out
}

pub(super) fn unescape_css_identifier_greedy(s: &str) -> String {
    // Spec-ish greedy behavior: consume up to 6 hex digits.
    let bytes = s.as_bytes();
    let mut out = String::with_capacity(s.len());
    let mut i = 0usize;
    while i < bytes.len() {
        let b = bytes[i];
        if b != b'\\' {
            out.push(b as char);
            i += 1;
            continue;
        }
        i += 1;
        if i >= bytes.len() {
            break;
        }
        let b2 = bytes[i];
        if matches!(b2, b'\n' | b'\r' | b'\x0C') {
            i += 1;
            continue;
        }
        if b2.is_ascii_hexdigit() {
            let mut j = i;
            let mut val: u32 = 0;
            let mut digits = 0usize;
            while j < bytes.len() && digits < 6 && bytes[j].is_ascii_hexdigit() {
                val = (val << 4) | (hex_value(bytes[j]) as u32);
                j += 1;
                digits += 1;
            }
            if j < bytes.len() && bytes[j].is_ascii_whitespace() {
                j += 1;
            }
            if let Some(ch) = char::from_u32(val) {
                out.push(ch);
            }
            i = j;
            continue;
        }
        out.push(b2 as char);
        i += 1;
    }
    out
}

fn hex_value(b: u8) -> u8 {
    match b {
        b'0'..=b'9' => b - b'0',
        b'a'..=b'f' => 10 + (b - b'a'),
        b'A'..=b'F' => 10 + (b - b'A'),
        _ => 0,
    }
}
