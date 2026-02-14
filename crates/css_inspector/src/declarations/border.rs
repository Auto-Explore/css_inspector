use rustc_hash::FxHashSet;

use crate::report::{Report, push_warning_level};
use crate::strutil::ascii_lowercase_cow;

use super::{is_border_style_token, is_border_width_token};

pub(super) struct BorderRedefinitionTracker {
    seen: FxHashSet<&'static str>,
}

impl Default for BorderRedefinitionTracker {
    fn default() -> Self {
        // border-{color,width,style} expands to at most 4 sides each.
        Self {
            seen: FxHashSet::with_capacity_and_hasher(12, Default::default()),
        }
    }
}

pub(super) fn track_border_redefinitions(
    tracker: &mut BorderRedefinitionTracker,
    prop: &str,
    tokens: &[&str],
    warning_level: i32,
    report: &mut Report,
) {
    if warning_level < 2 {
        return;
    }
    for_each_affected_border_longhand(prop, tokens, |p| {
        if !tracker.seen.insert(p) {
            push_warning_level(report, warning_level, 2, "Property redefined.");
        }
    });
}

pub(crate) fn for_each_affected_border_longhand(
    prop: &str,
    tokens: &[&str],
    mut f: impl FnMut(&'static str),
) -> bool {
    // Note: for the purposes of redefinition warnings (autotest `bugs/233.css`), we intentionally
    // treat `border-{color,width,style}` as affecting as many “slots” as values were provided
    // (1..=4), rather than eagerly expanding to all four sides.
    let prop = ascii_lowercase_cow(prop);
    const BORDER_COLOR: [&str; 4] = [
        "border-top-color",
        "border-right-color",
        "border-bottom-color",
        "border-left-color",
    ];
    const BORDER_STYLE: [&str; 4] = [
        "border-top-style",
        "border-right-style",
        "border-bottom-style",
        "border-left-style",
    ];
    const BORDER_WIDTH: [&str; 4] = [
        "border-top-width",
        "border-right-width",
        "border-bottom-width",
        "border-left-width",
    ];

    fn side_index(side: &str) -> Option<usize> {
        match side {
            "top" => Some(0),
            "right" => Some(1),
            "bottom" => Some(2),
            "left" => Some(3),
            _ => None,
        }
    }

    if prop.as_ref() == "border" {
        let (has_width, has_style, has_color) = border_side_component_flags(tokens);
        for i in 0..4 {
            if has_width {
                f(BORDER_WIDTH[i]);
            }
            if has_style {
                f(BORDER_STYLE[i]);
            }
            if has_color {
                f(BORDER_COLOR[i]);
            }
        }
        return true;
    }

    if let Some(rest) = prop.as_ref().strip_prefix("border-") {
        if let Some((side, kind)) = rest.split_once('-') {
            let Some(idx) = side_index(side) else {
                return false;
            };
            let longhand = match kind {
                "color" => BORDER_COLOR[idx],
                "style" => BORDER_STYLE[idx],
                "width" => BORDER_WIDTH[idx],
                _ => return false,
            };
            f(longhand);
            return true;
        } else if let Some(idx) = side_index(rest) {
            let (has_width, has_style, has_color) = border_side_component_flags(tokens);
            if has_width {
                f(BORDER_WIDTH[idx]);
            }
            if has_style {
                f(BORDER_STYLE[idx]);
            }
            if has_color {
                f(BORDER_COLOR[idx]);
            }
            return true;
        }
    }

    let aggregate = match prop.as_ref() {
        "border-color" => &BORDER_COLOR,
        "border-style" => &BORDER_STYLE,
        "border-width" => &BORDER_WIDTH,
        _ => return false,
    };

    let n = tokens.len().clamp(1, 4);
    for &p in &aggregate[..n] {
        f(p);
    }
    true
}

pub(crate) fn border_side_component_flags(tokens: &[&str]) -> (bool, bool, bool) {
    let mut has_width = false;
    let mut has_style = false;
    let mut has_color = false;
    for &t in tokens {
        let tl = ascii_lowercase_cow(t);
        if is_border_width_token(tl.as_ref()) {
            has_width = true;
        } else if is_border_style_token(tl.as_ref()) {
            has_style = true;
        } else {
            has_color = true;
        }
        if has_width && has_style && has_color {
            break;
        }
    }
    (has_width, has_style, has_color)
}
