use crate::report::{Report, push_error};
use crate::strutil::ascii_lowercase_cow;

use super::{is_balanced_function_call_token, is_valid_color_token, split_number_and_unit};

pub(super) fn validate_outline_style(tokens: &[&str], css4_profile: bool, report: &mut Report) {
    let [token] = tokens else {
        push_error(report, "Invalid value for property “outline-style”.");
        return;
    };
    let tl = ascii_lowercase_cow(token.trim());
    match tl.as_ref() {
        "auto" if css4_profile => {}
        "none" | "hidden" | "dotted" | "dashed" | "solid" | "double" | "groove" | "ridge"
        | "inset" | "outset" => {}
        _ => push_error(report, "Invalid value for property “outline-style”."),
    }
}

pub(super) fn validate_outline_width(tokens: &[&str], report: &mut Report) {
    let [token] = tokens else {
        push_error(report, "Invalid value for property “outline-width”.");
        return;
    };
    let tl = ascii_lowercase_cow(token.trim());
    let tl = tl.as_ref();
    if matches!(tl, "thin" | "medium" | "thick") || tl == "0" || is_length_token(tl) {
        return;
    }
    push_error(report, "Invalid value for property “outline-width”.");
}

pub(super) fn validate_outline(
    tokens: &[&str],
    css1_escapes: bool,
    css4_profile: bool,
    lenient: bool,
    report: &mut Report,
) {
    if tokens.is_empty() || tokens.len() > 3 {
        push_error(report, "Invalid value for property “outline”.");
        return;
    }

    let mut saw_color = false;
    let mut saw_style = false;
    let mut saw_width = false;

    for t in tokens {
        let raw = t.trim();
        if raw.is_empty() {
            continue;
        }
        let tl = ascii_lowercase_cow(raw);

        let is_color = tl.as_ref() == "invert" || is_valid_color_token(raw, css1_escapes, lenient);
        let is_style = matches!(
            tl.as_ref(),
            "none"
                | "hidden"
                | "dotted"
                | "dashed"
                | "solid"
                | "double"
                | "groove"
                | "ridge"
                | "inset"
                | "outset"
        ) || (css4_profile && tl.as_ref() == "auto");
        let is_width = matches!(tl.as_ref(), "thin" | "medium" | "thick")
            || tl.as_ref() == "0"
            || is_length_token(tl.as_ref());

        if is_color {
            if saw_color {
                push_error(report, "Invalid value for property “outline”.");
                return;
            }
            saw_color = true;
            continue;
        }
        if is_style {
            if saw_style {
                push_error(report, "Invalid value for property “outline”.");
                return;
            }
            saw_style = true;
            continue;
        }
        if is_width {
            if saw_width {
                push_error(report, "Invalid value for property “outline”.");
                return;
            }
            saw_width = true;
            continue;
        }

        push_error(report, "Invalid value for property “outline”.");
        return;
    }
}

pub(super) fn validate_border_shorthand(
    tokens: &[&str],
    css1_escapes: bool,
    lenient: bool,
    report: &mut Report,
) {
    if !(1..=3).contains(&tokens.len()) {
        push_error(report, "Invalid value for property “border”.");
        return;
    }

    let mut opts: Vec<[bool; 3]> = Vec::with_capacity(tokens.len());
    for &t in tokens {
        let tl = ascii_lowercase_cow(t.trim());
        let tl = tl.as_ref();
        let can_width = is_border_width_token(tl);
        let can_style = is_border_style_token(tl)
            || (tl.starts_with("var(") && is_balanced_function_call_token(tl, "var"))
            || (tl.starts_with("env(") && is_balanced_function_call_token(tl, "env"));
        let can_color = is_border_color_token(tl, css1_escapes, lenient);
        if !(can_width || can_style || can_color) {
            push_error(report, "Invalid value for property “border”.");
            return;
        }
        opts.push([can_width, can_style, can_color]);
    }

    fn assign_component(opts: &[[bool; 3]], idx: usize, used: &mut [bool; 3]) -> bool {
        if idx >= opts.len() {
            return true;
        }
        for k in 0..3 {
            if !opts[idx][k] || used[k] {
                continue;
            }
            used[k] = true;
            if assign_component(opts, idx + 1, used) {
                return true;
            }
            used[k] = false;
        }
        false
    }

    let mut used = [false; 3];
    if !assign_component(&opts, 0, &mut used) {
        push_error(report, "Invalid value for property “border”.");
    }
}

pub(super) fn validate_border_width_aggregate(tokens: &[&str], report: &mut Report) {
    if !(1..=4).contains(&tokens.len()) {
        push_error(report, "Invalid value for property “border-width”.");
        return;
    }
    for t in tokens {
        let tl = ascii_lowercase_cow(t);
        if !is_border_width_token(tl.as_ref()) {
            push_error(report, "Invalid value for property “border-width”.");
            return;
        }
    }
}

pub(super) fn validate_border_style_aggregate(tokens: &[&str], report: &mut Report) {
    if !(1..=4).contains(&tokens.len()) {
        push_error(report, "Invalid value for property “border-style”.");
        return;
    }
    for t in tokens {
        let tl = ascii_lowercase_cow(t);
        if !is_border_style_token(tl.as_ref()) {
            push_error(report, "Invalid value for property “border-style”.");
            return;
        }
    }
}

pub(super) fn validate_border_color_aggregate(
    tokens: &[&str],
    css1_escapes: bool,
    lenient: bool,
    report: &mut Report,
) {
    if !(1..=4).contains(&tokens.len()) {
        push_error(report, "Invalid value for property “border-color”.");
        return;
    }
    for t in tokens {
        let tl = ascii_lowercase_cow(t);
        if !is_border_color_token(tl.as_ref(), css1_escapes, lenient) {
            push_error(report, "Invalid value for property “border-color”.");
            return;
        }
    }
}

pub(super) fn is_border_width_token(t: &str) -> bool {
    matches!(t, "thin" | "medium" | "thick" | "0") || is_length_token(t)
}

pub(super) fn is_length_token(t: &str) -> bool {
    let t = ascii_lowercase_cow(t.trim());
    let t = t.as_ref();

    if t.starts_with("var(") {
        return is_balanced_function_call_token(t, "var");
    }
    if t.starts_with("env(") {
        return is_balanced_function_call_token(t, "env");
    }
    if t.starts_with("calc(") {
        return is_balanced_function_call_token(t, "calc");
    }
    if t.starts_with("min(") {
        return is_balanced_function_call_token(t, "min");
    }
    if t.starts_with("max(") {
        return is_balanced_function_call_token(t, "max");
    }
    if t.starts_with("clamp(") {
        return is_balanced_function_call_token(t, "clamp");
    }

    // Minimal length: `<number><unit>` with common units.
    let (num, unit) = split_number_and_unit(t);
    let Some(num) = num else {
        return false;
    };

    // CSS allows unitless zeros for lengths (including signed/decimal zeros like `-0`, `+0`,
    // `0.0`, etc).
    if unit.is_empty() {
        return num == 0.0;
    }

    matches!(
        unit,
        // Absolute.
        "px" | "pt" | "pc" | "cm" | "mm" | "in" | "q"
        // Font-relative.
        | "em" | "rem" | "ex" | "ch" | "lh" | "rlh"
        // Viewport-relative.
        | "vw" | "vh" | "vi" | "vb" | "vmin" | "vmax"
        | "svw" | "svh" | "svi" | "svb" | "svmin" | "svmax"
        | "lvw" | "lvh" | "lvi" | "lvb" | "lvmin" | "lvmax"
        | "dvw" | "dvh" | "dvi" | "dvb" | "dvmin" | "dvmax"
        // Container query.
        | "cqw" | "cqh" | "cqi" | "cqb" | "cqmin" | "cqmax"
    )
}

pub(super) fn is_border_style_token(t: &str) -> bool {
    matches!(
        t,
        "none"
            | "hidden"
            | "dotted"
            | "dashed"
            | "solid"
            | "double"
            | "groove"
            | "ridge"
            | "inset"
            | "outset"
    )
}

fn is_border_color_token(t: &str, css1_escapes: bool, lenient: bool) -> bool {
    is_valid_color_token(t, css1_escapes, lenient)
}
