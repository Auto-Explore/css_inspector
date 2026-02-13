use crate::report::{Report, push_error};
use crate::strutil::ascii_lowercase_cow;
use crate::strutil::{skip_css_escape, step_string_state};

use super::{
    is_any_percentage_token, is_balanced_function_call_token, is_css_wide_keyword, is_length_token,
    is_valid_property_name, parse_css_number,
};

pub(super) fn validate_font_optical_sizing(tokens: &[&str], report: &mut Report) {
    let [t] = tokens else {
        push_error(report, "Invalid value for property “font-optical-sizing”.");
        return;
    };
    match t.trim() {
        t if t.eq_ignore_ascii_case("auto") || t.eq_ignore_ascii_case("none") => {}
        _ => push_error(report, "Invalid value for property “font-optical-sizing”."),
    }
}

pub(super) fn validate_font_family(value: &str, css4_profile: bool, report: &mut Report) {
    // Minimal font-family validation to avoid rejecting valid font-family names that contain
    // css-wide keywords as identifier tokens (e.g. `inherit foo`), while still treating
    // css-wide keywords as invalid *when used alone as a font family* in a comma-separated list
    // (e.g. `inherit, serif`).
    //
    // `validate_property_declaration()` already accepts css-wide keywords as *entire* values
    // (e.g. `font-family: inherit`) before reaching this validator.
    let value = value.trim();
    if value.is_empty() {
        push_error(report, "Invalid value for property “font-family”.");
        return;
    }

    fn split_top_level_commas<'a>(value: &'a str) -> Vec<&'a str> {
        let bytes = value.as_bytes();
        let mut out: Vec<&'a str> = Vec::new();
        let mut start = 0usize;
        let mut i = 0usize;
        let mut in_string: Option<u8> = None;
        let mut escape = false;
        let mut paren_depth: usize = 0;

        while i < bytes.len() {
            let b = bytes[i];
            if in_string.is_none() && b == b'\\' {
                // Skip escaped characters outside strings so sequences like `\\,` or `\\\"` do
                // not act as comma/string delimiters.
                skip_css_escape(bytes, &mut i);
                continue;
            }
            if step_string_state(b, &mut in_string, &mut escape) {
                i += 1;
                continue;
            }
            match b {
                b'(' => paren_depth += 1,
                b')' => paren_depth = paren_depth.saturating_sub(1),
                b',' if paren_depth == 0 => {
                    out.push(&value[start..i]);
                    start = i + 1;
                }
                _ => {}
            }
            i += 1;
        }
        out.push(&value[start..]);
        out
    }

    fn is_string_token(t: &str) -> bool {
        let t = t.trim();
        if t.len() < 2 {
            return false;
        }
        let bytes = t.as_bytes();
        let q = bytes[0];
        (q == b'"' || q == b'\'') && bytes[bytes.len() - 1] == q
    }

    fn is_js_template_placeholder_token(t: &str) -> bool {
        let t = t.trim();
        t.starts_with("${") && t.ends_with('}') && t.len() >= 3
    }

    for raw_component in split_top_level_commas(value) {
        let component = raw_component.trim();
        if component.is_empty() {
            push_error(report, "Invalid value for property “font-family”.");
            return;
        }

        let spans = split_top_level_value_token_spans(component);
        if spans.is_empty() {
            push_error(report, "Invalid value for property “font-family”.");
            return;
        }

        if spans.len() == 1 {
            let t = component[spans[0].0..spans[0].1].trim();
            if is_css_wide_keyword(t) {
                push_error(report, "Invalid value for property “font-family”.");
                return;
            }
            if is_string_token(t) {
                continue;
            }
            if css4_profile && is_js_template_placeholder_token(t) {
                continue;
            }
            if !is_valid_property_name(t) {
                push_error(report, "Invalid value for property “font-family”.");
                return;
            }
            continue;
        }

        // For unquoted family names, a component is 1+ identifiers.
        // Disallow strings mixed with identifiers like `"inherit" foo`.
        for span in spans {
            let t = component[span.0..span.1].trim();
            if css4_profile && is_js_template_placeholder_token(t) {
                continue;
            }
            if is_string_token(t) || !is_valid_property_name(t) {
                push_error(report, "Invalid value for property “font-family”.");
                return;
            }
        }
    }
}

pub(super) fn validate_font(value: &str, css4_profile: bool, report: &mut Report) {
    // Minimal validation to catch invalid shorthands in the autotest suite (`bugs/289.css` and
    // `properties/inherit/error/font.css`), while accepting common valid forms.
    let spans = split_top_level_value_token_spans(value);
    let tokens: Vec<&str> = spans.iter().map(|(s, e)| &value[*s..*e]).collect();
    match tokens.as_slice() {
        [] => {
            push_error(report, "Invalid value for property “font”.");
            return;
        }
        [t] => {
            let t = t.trim();
            if is_css_wide_keyword(t) {
                return;
            }
            let tl = ascii_lowercase_cow(t);
            if matches!(
                tl.as_ref(),
                "caption" | "icon" | "menu" | "message-box" | "small-caption" | "status-bar"
            ) {
                return;
            }
        }
        _ => {}
    }

    if css4_profile {
        for t in &tokens {
            let lower = ascii_lowercase_cow(t.trim());
            let lower = lower.as_ref();
            if (lower.starts_with("var(") && is_balanced_function_call_token(lower, "var"))
                || (lower.starts_with("env(") && is_balanced_function_call_token(lower, "env"))
            {
                return;
            }
        }
    }

    let mut size_idx: Option<usize> = None;
    let mut family_start: usize = 0;

    for (i, raw) in tokens.iter().enumerate() {
        let tok = raw.trim();
        if let Some((s, lh)) = tok.split_once('/')
            && is_font_size_token(s)
            && is_line_height_token(lh)
        {
            size_idx = Some(i);
            family_start = i + 1;
            break;
        }
        if is_font_size_token(tok) {
            size_idx = Some(i);
            if i + 2 < tokens.len()
                && tokens[i + 1].trim() == "/"
                && is_line_height_token(tokens[i + 2])
            {
                family_start = i + 3;
            } else {
                family_start = i + 1;
            }
            break;
        }
    }

    let Some(size_idx) = size_idx else {
        push_error(report, "Invalid value for property “font”.");
        return;
    };

    for &t in &tokens[..size_idx] {
        let tl = ascii_lowercase_cow(t.trim());
        if !is_font_prefix_token(tl.as_ref()) {
            push_error(report, "Invalid value for property “font”.");
            return;
        }
    }
    if family_start >= tokens.len() {
        push_error(report, "Invalid value for property “font”.");
        return;
    }

    // In strict (non-`css4`) profiles, keep validation conservative: require a plausible
    // size+family structure, but do not attempt to fully validate the `font-family` tail.
    //
    // This keeps parity with upstream autotest expectations that intentionally include
    // odd-but-tokenizable family tails in some “too many values” cases.
    if !css4_profile {
        return;
    }

    // In `css4` mode, validate the `font-family` tail. Map `family_start` back to the original
    // string (preserving commas) and reuse `validate_font_family()`.
    if family_start >= spans.len() {
        push_error(report, "Invalid value for property “font”.");
        return;
    }
    let family_value = &value[spans[family_start].0..];
    let mut fam_report = Report::default();
    validate_font_family(family_value, css4_profile, &mut fam_report);
    if fam_report.errors > 0 {
        push_error(report, "Invalid value for property “font”.");
    }
}

fn is_font_prefix_token(t: &str) -> bool {
    matches!(
        t,
        "normal" | "italic" | "oblique" | "small-caps" | "bold" | "bolder" | "lighter"
    ) || t
        .parse::<i32>()
        .is_ok_and(|v| matches!(v, 100 | 200 | 300 | 400 | 500 | 600 | 700 | 800 | 900))
}

fn is_font_size_token(t: &str) -> bool {
    let tl = ascii_lowercase_cow(t.trim());
    let tl = tl.as_ref();
    matches!(
        tl,
        "xx-small"
            | "x-small"
            | "small"
            | "medium"
            | "large"
            | "x-large"
            | "xx-large"
            | "larger"
            | "smaller"
    ) || tl == "0"
        || is_length_token(tl)
        || is_any_percentage_token(tl)
}

fn is_line_height_token(t: &str) -> bool {
    let tl = ascii_lowercase_cow(t.trim());
    let tl = tl.as_ref();
    tl == "normal"
        || is_length_token(tl)
        || is_any_percentage_token(tl)
        || parse_css_number(tl).is_some()
}

fn split_top_level_value_token_spans(value: &str) -> Vec<(usize, usize)> {
    // Same tokenization as `split_top_level_value_tokens()`, but preserves byte spans (after
    // trimming) so we can slice the original `value` while keeping delimiters like commas.
    let bytes = value.as_bytes();
    let mut out: Vec<(usize, usize)> = Vec::new();
    let mut i = 0usize;
    let mut start: Option<usize> = None;
    let mut paren_depth: usize = 0;
    let mut in_string: Option<u8> = None;
    let mut escape = false;

    fn flush(start: &mut Option<usize>, end: usize, value: &str, out: &mut Vec<(usize, usize)>) {
        let Some(s) = start.take() else {
            return;
        };
        let raw = &value[s..end];
        let raw_trim_start = raw.trim_start();
        let trimmed_start = s + (raw.len() - raw_trim_start.len());
        let trimmed = raw_trim_start.trim_end();
        let trimmed_end = trimmed_start + trimmed.len();
        if trimmed_end > trimmed_start {
            out.push((trimmed_start, trimmed_end));
        }
    }

    while i < bytes.len() {
        let b = bytes[i];
        if in_string.is_none() && b == b'\\' {
            // Skip escaped characters outside strings so sequences like `\\\"` don't start a
            // string, and `\\,` doesn't act as a comma separator.
            start.get_or_insert(i);
            skip_css_escape(bytes, &mut i);
            continue;
        }
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

#[cfg(test)]
mod font_family_validation_tests {
    use super::{is_valid_property_name, validate_font_family};
    use crate::report::Report;

    #[test]
    fn escaped_quote_is_allowed_as_identifier_token() {
        assert!(is_valid_property_name(r#"\""#));
        let mut report = Report::default();
        validate_font_family(r#"\""#, true, &mut report);
        assert_eq!(report.errors, 0, "{report:?}");
    }
}
