use crate::report::{Report, push_error};
use crate::strutil::ascii_lowercase_cow;

use super::{
    is_balanced_function_call_token, is_css_wide_keyword, is_length_token, is_valid_color_token,
    is_valid_url_function_token, parse_css_number,
};

pub(super) fn validate_background(
    tokens: &[&str],
    css1_escapes: bool,
    css4_profile: bool,
    lenient: bool,
    report: &mut Report,
) {
    if tokens.is_empty() {
        push_error(report, "Invalid value for property “background”.");
        return;
    }

    // Test-driven validation for the `background` shorthand (kept intentionally conservative).
    // In the `css4` profile we relax this substantially to avoid false errors on modern shorthands
    // (gradients, `center / cover`, etc).
    let mut colors = 0usize;
    for (idx, t) in tokens.iter().enumerate() {
        let raw = t.trim();
        if raw.is_empty() {
            continue;
        }
        if is_quoted_string_token(raw) {
            push_error(report, "Invalid value for property “background”.");
            return;
        }
        if raw == "/" {
            if css4_profile && idx > 0 && idx + 1 < tokens.len() {
                continue;
            }
            push_error(report, "Invalid value for property “background”.");
            return;
        }
        let lower = ascii_lowercase_cow(raw);
        let lower = lower.as_ref();
        if lower.ends_with("deg") {
            // Used in `bugs/289.css` invalid-value fuzzing.
            push_error(report, "Invalid value for property “background”.");
            return;
        }

        // URL tokens must be syntactically valid if present.
        if lower.starts_with("url(") {
            if !is_valid_url_function_token(raw) {
                push_error(report, "Invalid value for property “background”.");
                return;
            }
            continue;
        }

        if is_valid_color_token(raw, css1_escapes, lenient) {
            colors += 1;
            if colors > 1 {
                push_error(report, "Invalid value for property “background”.");
                return;
            }
            continue;
        }

        if matches!(
            lower,
            "none"
                | "repeat"
                | "repeat-x"
                | "repeat-y"
                | "no-repeat"
                | "scroll"
                | "fixed"
                | "local"
                | "left"
                | "center"
                | "right"
                | "top"
                | "bottom"
        ) {
            continue;
        }

        if lower == "0" || is_length_token(lower) || is_any_percentage_token(lower) {
            continue;
        }

        if css4_profile {
            if is_background_image_like_token(lower) {
                continue;
            }
            if matches!(
                lower,
                "space"
                    | "round"
                    | "repeat-space"
                    | "repeat-round"
                    | "cover"
                    | "contain"
                    | "border-box"
                    | "padding-box"
                    | "content-box"
                    | "text"
            ) {
                continue;
            }
            if lower.starts_with("var(") && is_balanced_function_call_token(lower, "var") {
                continue;
            }
            if lower.starts_with("env(") && is_balanced_function_call_token(lower, "env") {
                continue;
            }
            if lower.contains('(') && !lower.ends_with(')') {
                push_error(report, "Invalid value for property “background”.");
                return;
            }
            continue;
        }

        push_error(report, "Invalid value for property “background”.");
        return;
    }
}

pub(super) fn is_any_percentage_token(t: &str) -> bool {
    let Some(num) = t.trim().strip_suffix('%') else {
        return false;
    };
    parse_css_number(num).is_some()
}

fn is_quoted_string_token(t: &str) -> bool {
    let bytes = t.as_bytes();
    if bytes.len() < 2 {
        return false;
    }
    let q = bytes[0];
    (q == b'"' || q == b'\'') && bytes[bytes.len() - 1] == q
}

pub(super) fn validate_background_image(tokens: &[&str], css4_profile: bool, report: &mut Report) {
    if tokens.is_empty() {
        push_error(report, "Invalid value for property “background-image”.");
        return;
    }

    if !css4_profile {
        let [raw] = tokens else {
            push_error(report, "Invalid value for property “background-image”.");
            return;
        };
        let raw = raw.trim();
        let lower = ascii_lowercase_cow(raw);
        let lower = lower.as_ref();
        if lower == "none" || is_css_wide_keyword(lower) {
            return;
        }
        if lower.starts_with("url(") {
            if is_valid_url_function_token(raw) {
                return;
            }
        }
        push_error(report, "Invalid value for property “background-image”.");
        return;
    }

    for t in tokens {
        let raw = t.trim();
        if raw.is_empty() {
            continue;
        }
        let lower = ascii_lowercase_cow(raw);
        let lower = lower.as_ref();
        if lower == "none" {
            continue;
        }
        if lower.starts_with("url(") {
            if is_valid_url_function_token(raw) {
                continue;
            }
            push_error(report, "Invalid value for property “background-image”.");
            return;
        }
        if lower.starts_with("var(") && is_balanced_function_call_token(lower, "var") {
            continue;
        }
        if lower.starts_with("env(") && is_balanced_function_call_token(lower, "env") {
            continue;
        }
        if is_background_image_like_token(lower) {
            continue;
        }
        push_error(report, "Invalid value for property “background-image”.");
        return;
    }
}

pub(super) fn is_background_image_like_token(lower: &str) -> bool {
    // Cover common `<image>` functions used in modern CSS.
    const FUNCTIONS: [&str; 14] = [
        "linear-gradient",
        "repeating-linear-gradient",
        "radial-gradient",
        "repeating-radial-gradient",
        "conic-gradient",
        "repeating-conic-gradient",
        "image-set",
        "-webkit-image-set",
        "-webkit-cross-fade",
        "cross-fade",
        "filter",
        "element",
        "image",
        "paint",
    ];

    FUNCTIONS
        .iter()
        .any(|name| lower.starts_with(*name) && is_balanced_function_call_token(lower, name))
}
