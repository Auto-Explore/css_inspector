use crate::report::{Report, push_error};
use crate::strutil::ascii_lowercase_cow;

use super::{
    is_any_percentage_token, is_balanced_function_call_token, is_css_wide_keyword, is_length_token,
    parse_css_number,
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

pub(super) fn validate_font(tokens: &[&str], css4_profile: bool, report: &mut Report) {
    // Minimal validation to catch invalid shorthands in the autotest suite (`bugs/289.css` and
    // `properties/inherit/error/font.css`), while accepting common valid forms.
    match tokens {
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
        for t in tokens {
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
        if let Some((s, lh)) = tok.split_once('/') {
            if is_font_size_token(s) && is_line_height_token(lh) {
                size_idx = Some(i);
                family_start = i + 1;
                break;
            }
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
