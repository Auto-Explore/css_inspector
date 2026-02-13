use crate::report::{Report, push_error};
use crate::strutil::step_string_state;

use super::{is_property_ident_char, split_number_and_unit};

pub(super) fn validate_grid_template_tracks(
    value: &str,
    prop: &str,
    lenient: bool,
    report: &mut Report,
) {
    let v = value.trim();
    if v.is_empty() || !grid_template_value_balanced(v) {
        push_error(report, format!("Invalid value for property “{prop}”."));
        return;
    }

    let bytes = v.as_bytes();
    let mut i = 0usize;
    let mut token_count = 0usize;
    let mut keyword_only = false;
    let mut subgrid_mode = false;

    while i < bytes.len() {
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i >= bytes.len() {
            break;
        }

        if keyword_only {
            push_error(report, format!("Invalid value for property “{prop}”."));
            return;
        }

        let b = bytes[i];
        if matches!(b, b',' | b']' | b')') {
            push_error(report, format!("Invalid value for property “{prop}”."));
            return;
        }

        if b == b'[' {
            let start = i;
            i += 1;
            let mut depth: i32 = 1;
            let mut in_string: Option<u8> = None;
            let mut escape = false;

            while i < bytes.len() {
                let bi = bytes[i];
                if step_string_state(bi, &mut in_string, &mut escape) {
                    i += 1;
                    continue;
                }
                match bi {
                    b'[' => depth += 1,
                    b']' => {
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
                push_error(report, format!("Invalid value for property “{prop}”."));
                return;
            }

            let tok = &v[start..i];
            if !is_valid_grid_line_name_list(tok, lenient) {
                push_error(report, format!("Invalid value for property “{prop}”."));
                return;
            }

            token_count += 1;
            continue;
        }

        let start = i;
        let mut paren_depth: i32 = 0;
        let mut in_string: Option<u8> = None;
        let mut escape = false;

        while i < bytes.len() {
            let bi = bytes[i];
            if step_string_state(bi, &mut in_string, &mut escape) {
                i += 1;
                continue;
            }
            match bi {
                b'(' => paren_depth += 1,
                b')' => {
                    if paren_depth == 0 {
                        break;
                    }
                    paren_depth -= 1;
                }
                b'[' if paren_depth == 0 => break,
                b',' if paren_depth == 0 => {
                    push_error(report, format!("Invalid value for property “{prop}”."));
                    return;
                }
                b if b.is_ascii_whitespace() && paren_depth == 0 => break,
                _ => {}
            }
            i += 1;
        }

        if paren_depth != 0 || in_string.is_some() {
            push_error(report, format!("Invalid value for property “{prop}”."));
            return;
        }

        let tok = v[start..i].trim();
        if tok.is_empty() {
            continue;
        }

        if subgrid_mode {
            if let Some((name, args)) = split_function_token(tok)
                && name.eq_ignore_ascii_case("repeat") {
                    let Some((count, tracks)) = split_on_single_top_level_comma(args) else {
                        push_error(report, format!("Invalid value for property “{prop}”."));
                        return;
                    };
                    let count = count.trim();
                    let tracks = tracks.trim();
                    if count.is_empty() || tracks.is_empty() {
                        push_error(report, format!("Invalid value for property “{prop}”."));
                        return;
                    }
                    let count_ok = count.eq_ignore_ascii_case("auto-fill")
                        || count.eq_ignore_ascii_case("auto-fit")
                        || count.parse::<usize>().is_ok_and(|n| n > 0);
                    if !count_ok || !is_valid_subgrid_line_name_sequence(tracks, lenient) {
                        push_error(report, format!("Invalid value for property “{prop}”."));
                        return;
                    }
                    token_count += 1;
                    continue;
                }

            // In `subgrid` mode, only `[line-names]` or `repeat(<count>, [line-names])` are
            // allowed after the `subgrid` keyword.
            push_error(report, format!("Invalid value for property “{prop}”."));
            return;
        }

        if tok.eq_ignore_ascii_case("none") || tok.eq_ignore_ascii_case("masonry") {
            if token_count != 0 {
                push_error(report, format!("Invalid value for property “{prop}”."));
                return;
            }
            keyword_only = true;
            token_count += 1;
            continue;
        }

        if tok.eq_ignore_ascii_case("subgrid") {
            if token_count != 0 {
                push_error(report, format!("Invalid value for property “{prop}”."));
                return;
            }
            subgrid_mode = true;
            token_count += 1;
            continue;
        }

        if tok.eq_ignore_ascii_case("auto")
            || tok.eq_ignore_ascii_case("min-content")
            || tok.eq_ignore_ascii_case("max-content")
        {
            token_count += 1;
            continue;
        }

        if lenient
            && (tok.eq_ignore_ascii_case("fit-content") || tok.eq_ignore_ascii_case("grid-lanes"))
        {
            token_count += 1;
            continue;
        }

        if is_grid_track_breadth_token(tok) {
            token_count += 1;
            continue;
        }

        if let Some((name, args)) = split_function_token(tok) {
            if name.eq_ignore_ascii_case("repeat") {
                let Some((count, tracks)) = split_on_single_top_level_comma(args) else {
                    push_error(report, format!("Invalid value for property “{prop}”."));
                    return;
                };
                let count = count.trim();
                let tracks = tracks.trim();
                if count.is_empty() || tracks.is_empty() {
                    push_error(report, format!("Invalid value for property “{prop}”."));
                    return;
                }
                let count_ok = count.eq_ignore_ascii_case("auto-fill")
                    || count.eq_ignore_ascii_case("auto-fit")
                    || count.parse::<usize>().is_ok_and(|n| n > 0);
                if !count_ok {
                    push_error(report, format!("Invalid value for property “{prop}”."));
                    return;
                }
                token_count += 1;
                continue;
            }

            if name.eq_ignore_ascii_case("minmax") {
                let Some((a, b)) = split_on_single_top_level_comma(args) else {
                    push_error(report, format!("Invalid value for property “{prop}”."));
                    return;
                };
                if a.trim().is_empty() || b.trim().is_empty() {
                    push_error(report, format!("Invalid value for property “{prop}”."));
                    return;
                }
                token_count += 1;
                continue;
            }

            if name.eq_ignore_ascii_case("fit-content") {
                if args.trim().is_empty() || args_has_top_level_comma(args) {
                    push_error(report, format!("Invalid value for property “{prop}”."));
                    return;
                }
                token_count += 1;
                continue;
            }

            // Be permissive with math/custom-property functions commonly used in track sizing.
            if name.eq_ignore_ascii_case("calc")
                || name.eq_ignore_ascii_case("min")
                || name.eq_ignore_ascii_case("max")
                || name.eq_ignore_ascii_case("clamp")
                || name.eq_ignore_ascii_case("var")
                || name.eq_ignore_ascii_case("env")
            {
                if args.trim().is_empty() {
                    push_error(report, format!("Invalid value for property “{prop}”."));
                    return;
                }
                token_count += 1;
                continue;
            }
        }

        push_error(report, format!("Invalid value for property “{prop}”."));
        return;
    }

    if token_count == 0 {
        push_error(report, format!("Invalid value for property “{prop}”."));
    }
}

fn is_valid_subgrid_line_name_sequence(s: &str, allow_empty: bool) -> bool {
    let v = s.trim();
    if v.is_empty() {
        return false;
    }

    let bytes = v.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i >= bytes.len() {
            break;
        }
        if bytes[i] != b'[' {
            return false;
        }

        let start = i;
        i += 1;
        let mut depth: i32 = 1;
        let mut in_string: Option<u8> = None;
        let mut escape = false;

        while i < bytes.len() {
            let bi = bytes[i];
            if step_string_state(bi, &mut in_string, &mut escape) {
                i += 1;
                continue;
            }
            match bi {
                b'[' => depth += 1,
                b']' => {
                    depth -= 1;
                    i += 1;
                    if depth == 0 {
                        break;
                    }
                    continue;
                }
                _ => {}
            }
            i += 1;
        }
        if depth != 0 || in_string.is_some() {
            return false;
        }

        let tok = &v[start..i];
        if !is_valid_grid_line_name_list(tok, allow_empty) {
            return false;
        }
    }
    true
}

fn grid_template_value_balanced(s: &str) -> bool {
    let bytes = s.as_bytes();
    let mut paren_depth: i32 = 0;
    let mut bracket_depth: i32 = 0;
    let mut in_string: Option<u8> = None;
    let mut escape = false;

    for &b in bytes {
        if step_string_state(b, &mut in_string, &mut escape) {
            continue;
        }
        match b {
            b'(' => paren_depth += 1,
            b')' => {
                paren_depth -= 1;
                if paren_depth < 0 {
                    return false;
                }
            }
            b'[' => bracket_depth += 1,
            b']' => {
                bracket_depth -= 1;
                if bracket_depth < 0 {
                    return false;
                }
            }
            _ => {}
        }
    }

    in_string.is_none() && paren_depth == 0 && bracket_depth == 0
}

fn split_function_token(token: &str) -> Option<(&str, &str)> {
    let token = token.trim();
    if !token.ends_with(')') {
        return None;
    }
    let open = token.find('(')?;
    let (name, rest) = token.split_at(open);
    let name = name.trim();
    if name.is_empty() {
        return None;
    }
    let args = &rest[1..rest.len() - 1];
    Some((name, args))
}

fn split_on_single_top_level_comma(args: &str) -> Option<(&str, &str)> {
    let bytes = args.as_bytes();
    let mut i = 0usize;
    let mut paren_depth: i32 = 0;
    let mut bracket_depth: i32 = 0;
    let mut in_string: Option<u8> = None;
    let mut escape = false;
    let mut comma: Option<usize> = None;

    while i < bytes.len() {
        let b = bytes[i];
        if step_string_state(b, &mut in_string, &mut escape) {
            i += 1;
            continue;
        }

        match b {
            b'(' => paren_depth += 1,
            b')' => paren_depth = paren_depth.saturating_sub(1),
            b'[' => bracket_depth += 1,
            b']' => bracket_depth = bracket_depth.saturating_sub(1),
            b',' if paren_depth == 0 && bracket_depth == 0 => {
                if comma.is_some() {
                    return None;
                }
                comma = Some(i);
            }
            _ => {}
        }
        i += 1;
    }

    let pos = comma?;
    Some((&args[..pos], &args[pos + 1..]))
}

fn args_has_top_level_comma(args: &str) -> bool {
    let bytes = args.as_bytes();
    let mut i = 0usize;
    let mut paren_depth: i32 = 0;
    let mut bracket_depth: i32 = 0;
    let mut in_string: Option<u8> = None;
    let mut escape = false;

    while i < bytes.len() {
        let b = bytes[i];
        if step_string_state(b, &mut in_string, &mut escape) {
            i += 1;
            continue;
        }

        match b {
            b'(' => paren_depth += 1,
            b')' => paren_depth = paren_depth.saturating_sub(1),
            b'[' => bracket_depth += 1,
            b']' => bracket_depth = bracket_depth.saturating_sub(1),
            b',' if paren_depth == 0 && bracket_depth == 0 => return true,
            _ => {}
        }
        i += 1;
    }

    false
}

fn is_valid_grid_line_name_list(tok: &str, allow_empty: bool) -> bool {
    let t = tok.trim();
    let Some(inner) = t.strip_prefix('[').and_then(|s| s.strip_suffix(']')) else {
        return false;
    };
    let inner = inner.trim();
    if inner.is_empty() {
        return allow_empty;
    }
    inner.split_whitespace().all(|name| {
        let bytes = name.as_bytes();
        let Some(&first) = bytes.first() else {
            return false;
        };
        if !(first.is_ascii_alphabetic() || first == b'-' || first == b'_') {
            return false;
        }
        bytes.iter().all(|&b| is_property_ident_char(b))
    })
}

fn is_grid_track_breadth_token(tok: &str) -> bool {
    let t = tok.trim();
    let (n, unit) = split_number_and_unit(t);
    let Some(n) = n else {
        return false;
    };
    if !n.is_finite() {
        return false;
    }

    if unit.is_empty() {
        return n == 0.0;
    }
    if unit == "%" {
        return true;
    }

    // `fr` values and lengths use ASCII alphabetic units.
    unit.as_bytes().iter().all(|b| b.is_ascii_alphabetic())
}
