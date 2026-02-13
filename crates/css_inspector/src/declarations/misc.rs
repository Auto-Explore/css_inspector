use crate::report::{Report, push_error, push_error_times};
use crate::strutil::{ascii_lowercase_cow, starts_with_ascii_ci};

use super::{
    is_any_percentage_token, is_background_image_like_token, is_balanced_function_call_token,
    is_length_token, is_valid_color_token, is_valid_property_name, is_valid_url_function_token,
    parse_css_number, scan_css_number_end, split_number_and_unit,
};

pub(super) fn validate_zoom(tokens: &[&str], report: &mut Report) {
    let [t] = tokens else {
        push_error(report, "Invalid value for property “zoom”.");
        return;
    };
    let t = t.trim();

    if t.eq_ignore_ascii_case("normal") || t.eq_ignore_ascii_case("reset") {
        return;
    }

    if let Some(num) = t.strip_suffix('%') {
        if let Some(v) = parse_css_number(num)
            && v >= 0.0 {
                return;
            }
    } else if let Some(v) = parse_css_number(t)
        && v >= 0.0 {
            return;
        }

    push_error(report, "Invalid value for property “zoom”.");
}

pub(super) fn validate_aspect_ratio(value: &str, report: &mut Report) {
    let v = value.trim();
    if v.is_empty() {
        push_error(report, "Invalid value for property “aspect-ratio”.");
        return;
    }

    #[inline]
    fn skip_ws(bytes: &[u8], mut i: usize) -> usize {
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        i
    }

    fn parse_positive_number_at(s: &str, bytes: &[u8], start: usize) -> Option<(f64, usize)> {
        let start = skip_ws(bytes, start);
        let end = start + scan_css_number_end(bytes.get(start..)?)?;
        let n = s[start..end].parse::<f64>().ok()?;
        (n.is_finite() && n > 0.0).then_some((n, end))
    }

    fn parse_ratio(s: &str) -> Option<()> {
        let bytes = s.as_bytes();
        let (_, mut i) = parse_positive_number_at(s, bytes, 0)?;
        i = skip_ws(bytes, i);
        if i == bytes.len() {
            return Some(());
        }
        if bytes[i] != b'/' {
            return None;
        }
        let (_, mut i) = parse_positive_number_at(s, bytes, i + 1)?;
        i = skip_ws(bytes, i);
        (i == bytes.len()).then_some(())
    }

    if starts_with_ascii_ci(v, "auto") {
        let after = &v[4..];
        if after.is_empty() {
            return;
        }
        if after
            .as_bytes()
            .first()
            .is_some_and(|b| b.is_ascii_whitespace())
            && parse_ratio(after).is_some()
        {
            return;
        }
    }

    if parse_ratio(v).is_some() {
        return;
    }

    push_error(report, "Invalid value for property “aspect-ratio”.");
}

pub(super) fn validate_overflow_clip_margin(tokens: &[&str], report: &mut Report) {
    if !(1..=2).contains(&tokens.len()) {
        push_error(report, "Invalid value for property “overflow-clip-margin”.");
        return;
    }

    let mut has_box = false;
    let mut has_length = false;

    for &t in tokens {
        let tl = ascii_lowercase_cow(t);
        let t = tl.as_ref();

        if is_overflow_clip_margin_visual_box(t) {
            if has_box {
                push_error(report, "Invalid value for property “overflow-clip-margin”.");
                return;
            }
            has_box = true;
            continue;
        }

        if is_overflow_clip_margin_lengthish_token(t) {
            if has_length {
                push_error(report, "Invalid value for property “overflow-clip-margin”.");
                return;
            }
            has_length = true;
            continue;
        }

        push_error(report, "Invalid value for property “overflow-clip-margin”.");
        return;
    }

    // Disallow `foo foo` cases.
    if tokens.len() == 2 && !(has_box && has_length) {
        push_error(report, "Invalid value for property “overflow-clip-margin”.");
    }
}

fn is_overflow_clip_margin_visual_box(t: &str) -> bool {
    matches!(
        t,
        "content-box" | "padding-box" | "border-box" | "margin-box"
    )
}

fn is_overflow_clip_margin_lengthish_token(t: &str) -> bool {
    let t = t.trim();
    let (n, unit) = split_number_and_unit(t);

    if let Some(n) = n {
        // CSS allows unitless zeros where lengths are expected.
        if unit.is_empty() {
            return n == 0.0;
        }

        if unit == "%" {
            return false;
        }

        // Be lenient about units: accept any ASCII alphabetic unit.
        if unit.as_bytes().iter().all(|b| b.is_ascii_alphabetic()) {
            return true;
        }
    }

    starts_with_ascii_ci(t, "calc(")
        || starts_with_ascii_ci(t, "min(")
        || starts_with_ascii_ci(t, "max(")
        || starts_with_ascii_ci(t, "clamp(")
        || starts_with_ascii_ci(t, "var(")
        || starts_with_ascii_ci(t, "env(")
}

pub(super) fn validate_float(tokens: &[&str], css4_profile: bool, report: &mut Report) {
    let [token] = tokens else {
        push_error(report, "Invalid value for property “float”.");
        return;
    };
    let ok = ["left", "right", "none", "inherit", "initial", "unset"]
        .iter()
        .any(|v| token.eq_ignore_ascii_case(v));
    let ok = ok
        || (css4_profile
            && ["inline-start", "inline-end"]
                .iter()
                .any(|v| token.eq_ignore_ascii_case(v)));
    if !ok {
        push_error(report, "Invalid value for property “float”.");
    }
}

pub(super) fn validate_cursor(tokens: &[&str], css4_profile: bool, report: &mut Report) {
    let is_url = |t: &str| starts_with_ascii_ci(t.trim(), "url(");
    let is_keyword = |t: &str| {
        let tl = ascii_lowercase_cow(t.trim());
        let base = matches!(
            tl.as_ref(),
            "auto"
                | "crosshair"
                | "default"
                | "pointer"
                | "move"
                | "e-resize"
                | "ne-resize"
                | "nw-resize"
                | "n-resize"
                | "se-resize"
                | "sw-resize"
                | "s-resize"
                | "w-resize"
                | "text"
                | "wait"
                | "help"
                | "progress"
        );
        if base {
            return true;
        }

        css4_profile
            && matches!(
                tl.as_ref(),
                "grab"
                    | "grabbing"
                    | "zoom-in"
                    | "zoom-out"
                    | "none"
                    | "not-allowed"
                    | "no-drop"
                    | "context-menu"
                    | "cell"
                    | "vertical-text"
                    | "alias"
                    | "copy"
                    | "all-scroll"
                    | "col-resize"
                    | "row-resize"
                    | "n-all-scroll"
                    | "s-all-scroll"
                    | "e-all-scroll"
                    | "w-all-scroll"
                    | "nesw-resize"
                    | "nwse-resize"
                    | "ns-resize"
                    | "ew-resize"
            )
    };

    match tokens {
        [t] if is_keyword(t) || is_url(t) => {}
        // Autotest `properties/ok/ui.css` expects a two-URL + keyword form to be valid.
        [t0, t1, t2] if is_url(t0) && is_url(t1) && is_keyword(t2) => {}
        _ if css4_profile && tokens.len() >= 2 && is_keyword(tokens[tokens.len() - 1]) => {
            // Allow `url(...) x y, url(...) x y, keyword` style lists where the coordinates
            // (and commas) get split into tokens.
            let mut saw_url = false;
            for &t in &tokens[..tokens.len() - 1] {
                let raw = t.trim();
                if raw.is_empty() {
                    continue;
                }
                let lower = ascii_lowercase_cow(raw);
                let lower = lower.as_ref();
                if is_url(raw) || is_background_image_like_token(lower) {
                    saw_url = true;
                    continue;
                }
                if is_integer_token(raw) {
                    continue;
                }
                if lower.starts_with("calc(") && is_balanced_function_call_token(lower, "calc") {
                    continue;
                }
                push_error(report, "Invalid value for property “cursor”.");
                return;
            }
            if !saw_url {
                push_error(report, "Invalid value for property “cursor”.");
            }
        }
        _ => push_error(report, "Invalid value for property “cursor”."),
    }
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

pub(super) fn validate_quotes(tokens: &[&str], report: &mut Report) {
    match tokens {
        [t] if t.trim().eq_ignore_ascii_case("none") => (),
        [t] if t.trim().eq_ignore_ascii_case("auto") => (),
        [t0, t1] if is_string_token(t0) && is_string_token(t1) => (),
        _ if tokens.len() >= 2
            && tokens.len().is_multiple_of(2)
            && tokens.iter().all(|t| is_string_token(t)) => {}
        _ => push_error(report, "Invalid value for property “quotes”."),
    }
}

pub(super) fn is_css_identifier_token(t: &str) -> bool {
    is_valid_property_name(t)
}

pub(super) fn is_integer_token(t: &str) -> bool {
    let t = t.trim().as_bytes();
    let t = match t.first() {
        Some(b'+' | b'-') => &t[1..],
        _ => t,
    };
    !t.is_empty() && t.iter().all(|&b| b.is_ascii_digit())
}

pub(super) fn validate_counter_list(
    tokens: &[&str],
    prop: &str,
    css4_profile: bool,
    report: &mut Report,
) {
    if let [t] = tokens
        && t.trim().eq_ignore_ascii_case("none") {
            return;
        }

    let invalid_value = |report: &mut Report| {
        push_error(report, format!("Invalid value for property “{prop}”."));
    };
    fn is_reversed_counter_token(token: &str) -> bool {
        let token = token.trim();
        if !starts_with_ascii_ci(token, "reversed(") || !token.ends_with(')') {
            return false;
        }
        let lower = ascii_lowercase_cow(token);
        let lower = lower.as_ref();
        if !is_balanced_function_call_token(lower, "reversed") {
            return false;
        }
        let Some(open) = token.find('(') else {
            return false;
        };
        let args = token[open + 1..token.len() - 1].trim();
        is_css_identifier_token(args)
    }

    fn is_counter_value_token(token: &str, css4_profile: bool) -> bool {
        if is_integer_token(token) {
            return true;
        }
        if !css4_profile {
            return false;
        }
        let lower = ascii_lowercase_cow(token.trim());
        let lower = lower.as_ref();
        (lower.starts_with("calc(") && is_balanced_function_call_token(lower, "calc"))
            || (lower.starts_with("var(") && is_balanced_function_call_token(lower, "var"))
            || (lower.starts_with("env(") && is_balanced_function_call_token(lower, "env"))
    }

    let mut i = 0usize;
    while i < tokens.len() {
        let t = tokens[i].trim();
        if t.eq_ignore_ascii_case("none") {
            invalid_value(report);
            return;
        }
        if !(is_css_identifier_token(t) || (css4_profile && is_reversed_counter_token(t))) {
            invalid_value(report);
            return;
        }
        i += 1;
        if i < tokens.len() && is_counter_value_token(tokens[i], css4_profile) {
            i += 1;
        }
    }
}

pub(super) fn validate_content(
    tokens: &[&str],
    css4_profile: bool,
    lenient: bool,
    report: &mut Report,
) {
    if tokens.is_empty() {
        push_error(report, "Invalid value for property “content”.");
        return;
    }
    if tokens.len() == 1 {
        let tl = ascii_lowercase_cow(tokens[0].trim());
        match tl.as_ref() {
            "normal" | "none" => return,
            _ => {}
        }
    }
    for t in tokens {
        let raw = t.trim();
        if raw.eq_ignore_ascii_case("normal") || raw.eq_ignore_ascii_case("none") {
            // `normal`/`none` cannot be mixed with other components.
            push_error(report, "Invalid value for property “content”.");
            return;
        }
        if is_string_token(raw) {
            continue;
        }
        if starts_with_ascii_ci(raw, "url(")
            || starts_with_ascii_ci(raw, "counter(")
            || (css4_profile && starts_with_ascii_ci(raw, "counters("))
            || starts_with_ascii_ci(raw, "attr(")
        {
            continue;
        }
        if css4_profile {
            let lower = ascii_lowercase_cow(raw);
            let lower = lower.as_ref();
            if lower.starts_with("var(") && is_balanced_function_call_token(lower, "var") {
                continue;
            }
            if lower.starts_with("env(") && is_balanced_function_call_token(lower, "env") {
                continue;
            }
        }
        if lenient {
            let lower = ascii_lowercase_cow(raw);
            let lower = lower.as_ref();
            if lower.starts_with("leader(") && is_balanced_function_call_token(lower, "leader") {
                continue;
            }
            if lower.starts_with("string(") && is_balanced_function_call_token(lower, "string") {
                continue;
            }
            if is_background_image_like_token(lower) {
                continue;
            }
        }
        if raw.eq_ignore_ascii_case("open-quote")
            || raw.eq_ignore_ascii_case("close-quote")
            || raw.eq_ignore_ascii_case("no-open-quote")
            || raw.eq_ignore_ascii_case("no-close-quote")
        {
            continue;
        }
        push_error(report, "Invalid value for property “content”.");
        return;
    }
}

fn is_time_token(t: &str) -> bool {
    let t = ascii_lowercase_cow(t.trim());
    let t = t.as_ref();
    if t == "0" {
        return true;
    }
    for unit in ["ms", "s"] {
        if let Some(num) = t.strip_suffix(unit) {
            if num.trim().is_empty() {
                return false;
            }
            return parse_css_number(num).is_some();
        }
    }
    false
}

fn is_pause_keyword(t: &str) -> bool {
    let t = t.trim();
    t.eq_ignore_ascii_case("none")
        || t.eq_ignore_ascii_case("x-weak")
        || t.eq_ignore_ascii_case("weak")
        || t.eq_ignore_ascii_case("medium")
        || t.eq_ignore_ascii_case("strong")
        || t.eq_ignore_ascii_case("x-strong")
}

pub(super) fn validate_pause_after(tokens: &[&str], report: &mut Report) {
    let [t] = tokens else {
        push_error(report, "Invalid value for property “pause-after”.");
        return;
    };
    let t = t.trim();
    if is_time_token(t) || is_pause_keyword(t) {
        return;
    }
    push_error(report, "Invalid value for property “pause-after”.");
}

pub(super) fn validate_pause(tokens: &[&str], report: &mut Report) {
    if tokens.is_empty() || tokens.len() > 2 {
        push_error(report, "Invalid value for property “pause”.");
        return;
    }
    for t in tokens {
        let t = t.trim();
        if !(is_time_token(t) || is_pause_keyword(t)) {
            push_error(report, "Invalid value for property “pause”.");
            return;
        }
    }
}

pub(super) fn validate_cue_side(tokens: &[&str], prop: &str, report: &mut Report) {
    let [t] = tokens else {
        push_error(report, format!("Invalid value for property “{prop}”."));
        return;
    };
    let t = t.trim();
    if t.eq_ignore_ascii_case("none") || starts_with_ascii_ci(t, "url(") {
        return;
    }
    push_error(report, format!("Invalid value for property “{prop}”."));
}

pub(super) fn validate_cue(tokens: &[&str], report: &mut Report) {
    if tokens.is_empty() || tokens.len() > 2 {
        push_error(report, "Invalid value for property “cue”.");
        return;
    }
    for t in tokens {
        let t = t.trim();
        if t.eq_ignore_ascii_case("none") || starts_with_ascii_ci(t, "url(") {
            continue;
        }
        push_error(report, "Invalid value for property “cue”.");
        return;
    }
}

pub(super) fn validate_play_during(tokens: &[&str], report: &mut Report) {
    if tokens.is_empty() || tokens.len() > 3 {
        push_error(report, "Invalid value for property “play-during”.");
        return;
    }
    let t0 = tokens[0].trim();
    let is_url = starts_with_ascii_ci(t0, "url(");
    let is_single_keyword = t0.eq_ignore_ascii_case("none") || t0.eq_ignore_ascii_case("auto");
    if tokens.len() == 1 && (is_url || is_single_keyword) {
        return;
    }
    if !is_url {
        push_error(report, "Invalid value for property “play-during”.");
        return;
    }
    let mut saw_mix = false;
    let mut saw_repeat = false;
    for t in &tokens[1..] {
        let t = t.trim();
        if t.eq_ignore_ascii_case("mix") && !saw_mix {
            saw_mix = true;
        } else if t.eq_ignore_ascii_case("repeat") && !saw_repeat {
            saw_repeat = true;
        } else {
            push_error(report, "Invalid value for property “play-during”.");
            return;
        }
    }
}

fn is_angle_token(t: &str) -> bool {
    let t = ascii_lowercase_cow(t.trim());
    let t = t.as_ref();
    for unit in ["deg", "grad", "rad", "turn"] {
        if let Some(num) = t.strip_suffix(unit) {
            if num.trim().is_empty() {
                return false;
            }
            return parse_css_number(num).is_some();
        }
    }
    false
}

pub(super) fn validate_azimuth(tokens: &[&str], report: &mut Report) {
    if tokens.is_empty() || tokens.len() > 2 {
        push_error(report, "Invalid value for property “azimuth”.");
        return;
    }
    if tokens.len() == 1 && is_angle_token(tokens[0]) {
        return;
    }
    if tokens.len() == 2 && (is_angle_token(tokens[0]) || is_angle_token(tokens[1])) {
        push_error(report, "Invalid value for property “azimuth”.");
        return;
    }

    let t0 = ascii_lowercase_cow(tokens[0].trim());
    let t0 = t0.as_ref();
    let allowed_dir = |t: &str| {
        matches!(
            t,
            "left-side"
                | "far-left"
                | "left"
                | "center-left"
                | "center"
                | "center-right"
                | "right"
                | "far-right"
                | "right-side"
        )
    };
    let single = |t: &str| matches!(t, "inherit" | "leftwards" | "rightwards");

    if tokens.len() == 1 {
        if single(t0) || t0 == "behind" || allowed_dir(t0) {
            return;
        }
        push_error(report, "Invalid value for property “azimuth”.");
        return;
    }

    let t1 = ascii_lowercase_cow(tokens[1].trim());
    let t1 = t1.as_ref();
    if single(t0) || single(t1) {
        push_error(report, "Invalid value for property “azimuth”.");
        return;
    }
    if t0 == "behind" && allowed_dir(t1) {
        return;
    }
    if allowed_dir(t0) && t1 == "behind" {
        return;
    }
    push_error(report, "Invalid value for property “azimuth”.");
}

pub(super) fn validate_elevation(tokens: &[&str], report: &mut Report) {
    let [t] = tokens else {
        push_error(report, "Invalid value for property “elevation”.");
        return;
    };
    let tl = ascii_lowercase_cow(t.trim());
    let tl = tl.as_ref();
    if is_angle_token(tl) || matches!(tl, "below" | "level" | "above" | "higher" | "lower") {
        return;
    }
    push_error(report, "Invalid value for property “elevation”.");
}

pub(super) fn validate_voice_family(tokens: &[&str], report: &mut Report) {
    if tokens.is_empty() {
        push_error(report, "Invalid value for property “voice-family”.");
        return;
    }
    let mut can_take_integer = false;
    for t in tokens {
        let t = t.trim();
        if is_integer_token(t) {
            if !can_take_integer {
                push_error(report, "Invalid value for property “voice-family”.");
                return;
            }
            can_take_integer = false;
            continue;
        }
        if is_string_token(t) || is_css_identifier_token(t) {
            can_take_integer = true;
            continue;
        }
        push_error(report, "Invalid value for property “voice-family”.");
        return;
    }
}

pub(super) fn validate_list_style(tokens: &[&str], lenient: bool, report: &mut Report) {
    // Minimal, test-driven parsing for the `list-style` shorthand:
    //   <list-style-type> || <list-style-position> || <list-style-image>
    //
    // This is intentionally not a complete implementation; it exists to catch the autotest
    // “too many values” cases and a common typo (`disk` vs `disc`).
    let tokens: Vec<&str> = tokens
        .iter()
        .map(|t| t.trim())
        .filter(|t| !t.is_empty())
        .collect();

    if tokens.is_empty() || tokens.len() > 3 {
        push_error(report, "Invalid value for property “list-style”.");
        return;
    }

    if tokens.iter().any(|t| t.eq_ignore_ascii_case("disk")) {
        push_error(report, "Invalid value for property “list-style”.");
        return;
    }

    fn is_position_token(token: &str) -> bool {
        token.eq_ignore_ascii_case("inside") || token.eq_ignore_ascii_case("outside")
    }

    fn is_image_token(token: &str) -> bool {
        token.eq_ignore_ascii_case("none") || starts_with_ascii_ci(token, "url(")
    }

    fn is_custom_list_style_type_ident(token: &str) -> bool {
        // Counter-style names are custom identifiers.
        is_valid_property_name(token)
    }

    fn is_type_token(token: &str, lenient: bool) -> bool {
        if token.eq_ignore_ascii_case("none") {
            return true;
        }
        let lower = ascii_lowercase_cow(token);
        let lower = lower.as_ref();
        if is_list_style_type_keyword(lower) {
            return true;
        }
        if lenient {
            if is_string_token(token) {
                return true;
            }
            if is_custom_list_style_type_ident(token) {
                return true;
            }
        }
        false
    }

    fn search(
        idx: usize,
        tokens: &[&str],
        lenient: bool,
        have_type: bool,
        have_position: bool,
        have_image: bool,
    ) -> bool {
        if idx >= tokens.len() {
            return true;
        }
        let token = tokens[idx];

        if !have_position && is_position_token(token)
            && search(idx + 1, tokens, lenient, have_type, true, have_image) {
                return true;
            }
        if !have_image && is_image_token(token)
            && search(idx + 1, tokens, lenient, have_type, have_position, true) {
                return true;
            }
        if !have_type && is_type_token(token, lenient)
            && search(idx + 1, tokens, lenient, true, have_position, have_image) {
                return true;
            }

        false
    }

    if !search(0, &tokens, lenient, false, false, false) {
        push_error(report, "Invalid value for property “list-style”.");
    }
}

fn is_list_style_type_keyword(t: &str) -> bool {
    matches!(
        t,
        "disc"
            | "circle"
            | "square"
            | "decimal"
            | "decimal-leading-zero"
            | "lower-roman"
            | "upper-roman"
            | "lower-greek"
            | "lower-alpha"
            | "upper-alpha"
            | "lower-latin"
            | "upper-latin"
            | "armenian"
            | "georgian"
    )
}

pub(super) fn validate_text_decoration(
    tokens: &[&str],
    css1_escapes: bool,
    css4_profile: bool,
    lenient: bool,
    report: &mut Report,
) {
    if tokens.is_empty() {
        push_error(report, "Invalid value for property “text-decoration”.");
        return;
    }

    // CSS1/CSS2-era grammar:
    //   none | [ underline || overline || line-through || blink ]
    if !css4_profile {
        if tokens.len() == 1 && tokens[0].trim().eq_ignore_ascii_case("none") {
            return;
        }

        let mut seen = 0u8;
        for t in tokens {
            let tl = ascii_lowercase_cow(t.trim());
            let tl = tl.as_ref();
            if tl == "none" {
                push_error(report, "Invalid value for property “text-decoration”.");
                return;
            }
            let key: u8 = match tl {
                "underline" => 1,
                "overline" => 2,
                "line-through" => 4,
                "blink" => 8,
                _ => {
                    push_error(report, "Invalid value for property “text-decoration”.");
                    return;
                }
            };
            if (seen & key) != 0 {
                push_error(report, "Invalid value for property “text-decoration”.");
                return;
            }
            seen |= key;
        }
        return;
    }

    // CSS Text Decoration 4-ish shorthand:
    //   <'text-decoration-line'> || <'text-decoration-style'> || <'text-decoration-color'>
    //   || <'text-decoration-thickness'>
    //
    // Be permissive about tokenization; WPT expects a variety of modern forms.
    if tokens.len() == 1 && tokens[0].trim().eq_ignore_ascii_case("none") {
        return;
    }

    let mut line_seen = 0u8;
    let mut has_style = false;
    let mut has_color = false;
    let mut has_thickness = false;

    for t in tokens {
        let raw = t.trim();
        let tl = ascii_lowercase_cow(raw);
        let tl = tl.as_ref();

        if tl == "none" {
            push_error(report, "Invalid value for property “text-decoration”.");
            return;
        }

        let line_key: u8 = match tl {
            "underline" => 1,
            "overline" => 2,
            "line-through" => 4,
            "blink" => 8,
            "spelling-error" => 16,
            "grammar-error" => 32,
            _ => 0,
        };
        if line_key != 0 {
            if (line_seen & line_key) != 0 {
                push_error(report, "Invalid value for property “text-decoration”.");
                return;
            }
            line_seen |= line_key;
            continue;
        }

        if matches!(tl, "solid" | "double" | "dotted" | "dashed" | "wavy") {
            if has_style {
                push_error(report, "Invalid value for property “text-decoration”.");
                return;
            }
            has_style = true;
            continue;
        }

        if matches!(tl, "auto" | "from-font")
            || tl == "0"
            || is_length_token(tl)
            || is_any_percentage_token(tl)
        {
            if has_thickness {
                push_error(report, "Invalid value for property “text-decoration”.");
                return;
            }
            has_thickness = true;
            continue;
        }

        if is_valid_color_token(raw, css1_escapes, lenient) {
            if has_color {
                push_error(report, "Invalid value for property “text-decoration”.");
                return;
            }
            has_color = true;
            continue;
        }

        push_error(report, "Invalid value for property “text-decoration”.");
        return;
    }
}

pub(super) fn validate_clip(tokens: &[&str], report: &mut Report) {
    const ERR: &str = "Invalid value for property “clip”.";

    let [t] = tokens else {
        push_error(report, ERR);
        return;
    };
    let t = t.trim();
    if t.eq_ignore_ascii_case("auto") {
        return;
    }

    let ok = starts_with_ascii_ci(t, "rect(") && t.ends_with(')') && t.len() >= 6 && {
        let inner = t[5..t.len() - 1].trim();
        let parts = if inner.contains(',') {
            inner
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .take(5)
                .count()
        } else {
            inner.split_whitespace().take(5).count()
        };
        parts == 4
    };

    if !ok {
        push_error(report, ERR);
    }
}

pub(super) fn validate_filter(tokens: &[&str], css4_profile: bool, report: &mut Report) {
    // The autotest suite includes a Microsoft `progid:` filter expression that the upstream
    // validator reports as 4 errors (bug 750). Preserve that behavior.
    if tokens.is_empty() {
        push_error(report, "Invalid value for property “filter”.");
        return;
    }

    // `split_top_level_value_tokens` splits on whitespace, and the prefix contains no whitespace.
    // This makes checking only the first token equivalent to checking `tokens.join(" ")`.
    if tokens
        .first()
        .is_some_and(|t| starts_with_ascii_ci(t.trim(), "progid:dximagetransform.microsoft."))
    {
        push_error_times(report, "Invalid value for property “filter”.", 4);
        return;
    }

    if !css4_profile {
        push_error(report, "Invalid value for property “filter”.");
        return;
    }

    if tokens.len() == 1 && tokens[0].trim().eq_ignore_ascii_case("none") {
        return;
    }

    const FUNCTIONS: [&str; 11] = [
        "blur",
        "brightness",
        "contrast",
        "drop-shadow",
        "grayscale",
        "hue-rotate",
        "invert",
        "opacity",
        "saturate",
        "sepia",
        "url",
    ];

    const OPTIONAL_ARG_FUNCTIONS: [&str; 7] = [
        "brightness",
        "contrast",
        "grayscale",
        "invert",
        "opacity",
        "saturate",
        "sepia",
    ];

    for t in tokens {
        let raw = t.trim();
        if raw.is_empty() {
            continue;
        }
        let lower = ascii_lowercase_cow(raw);
        let lower = lower.as_ref();

        if lower.starts_with("url(") {
            if is_valid_url_function_token(raw) {
                continue;
            }
            push_error(report, "Invalid value for property “filter”.");
            return;
        }

        if lower.starts_with("var(") && is_balanced_function_call_token(lower, "var") {
            continue;
        }
        if lower.starts_with("env(") && is_balanced_function_call_token(lower, "env") {
            continue;
        }

        if FUNCTIONS.iter().any(|name| {
            if !lower.starts_with(*name) {
                return false;
            }
            if is_balanced_function_call_token(lower, name) {
                return true;
            }
            if !OPTIONAL_ARG_FUNCTIONS.contains(name) {
                return false;
            }

            // Some filter functions accept an optional parameter, where an empty argument
            // list implies the default.
            let bytes = lower.as_bytes();
            let after_name = name.len();
            if bytes.get(after_name) != Some(&b'(') || !lower.ends_with(')') {
                return false;
            }
            let inner = &lower[after_name + 1..lower.len() - 1];
            inner.bytes().all(|b| b.is_ascii_whitespace())
        }) {
            continue;
        }

        push_error(report, "Invalid value for property “filter”.");
        return;
    }
}
