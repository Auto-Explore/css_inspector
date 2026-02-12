use crate::report::{Report, push_error};
use crate::strutil::ascii_lowercase_cow;

pub(super) fn is_font_face_descriptor(prop: &str) -> bool {
    matches!(
        prop,
        "font-family"
            | "src"
            | "font-style"
            | "font-weight"
            | "font-stretch"
            | "unicode-range"
            | "font-variant"
            | "font-feature-settings"
            | "font-display"
    )
}

pub(super) fn is_page_descriptor(prop: &str) -> bool {
    matches!(prop, "size" | "marks" | "bleed")
}

pub(super) fn is_property_descriptor(prop: &str) -> bool {
    matches!(prop, "syntax" | "inherits" | "initial-value")
}

pub(super) fn is_font_palette_values_descriptor(prop: &str) -> bool {
    matches!(prop, "font-family" | "base-palette" | "override-colors")
}

pub(super) fn is_counter_style_descriptor(prop: &str) -> bool {
    matches!(
        prop,
        "system"
            | "symbols"
            | "additive-symbols"
            | "negative"
            | "prefix"
            | "suffix"
            | "range"
            | "pad"
            | "speak-as"
            | "fallback"
    )
}

pub(super) fn is_color_profile_descriptor(prop: &str) -> bool {
    matches!(prop, "src" | "rendering-intent")
}

pub(super) fn is_view_transition_descriptor(prop: &str) -> bool {
    matches!(prop, "navigation" | "types")
}

pub(super) fn is_scroll_timeline_descriptor(prop: &str) -> bool {
    matches!(
        prop,
        "source" | "orientation" | "scroll-offsets" | "time-range"
    )
}

pub(super) fn is_view_timeline_descriptor(prop: &str) -> bool {
    matches!(prop, "subject" | "axis" | "inset" | "time-range")
}

pub(super) fn validate_property_descriptor_syntax(tokens: &[&str], report: &mut Report) {
    let [t] = tokens else {
        push_error(report, "Invalid value for property “syntax”.");
        return;
    };
    let t = t.trim();
    if t.len() >= 2 {
        let bytes = t.as_bytes();
        let q = bytes[0];
        if (q == b'"' || q == b'\'') && bytes[bytes.len() - 1] == q {
            return;
        }
    }
    push_error(report, "Invalid value for property “syntax”.");
}

pub(super) fn validate_property_descriptor_inherits(tokens: &[&str], report: &mut Report) {
    let [t] = tokens else {
        push_error(report, "Invalid value for property “inherits”.");
        return;
    };
    let t = t.trim();
    if t.eq_ignore_ascii_case("true") || t.eq_ignore_ascii_case("false") {
        return;
    }
    push_error(report, "Invalid value for property “inherits”.");
}

pub(super) fn validate_color_profile_descriptor_rendering_intent(
    tokens: &[&str],
    report: &mut Report,
) {
    let [t] = tokens else {
        push_error(report, "Invalid value for property “rendering-intent”.");
        return;
    };
    let t = t.trim();
    if matches!(
        ascii_lowercase_cow(t).as_ref(),
        "auto" | "perceptual" | "relative-colorimetric" | "saturation" | "absolute-colorimetric"
    ) {
        return;
    }
    push_error(report, "Invalid value for property “rendering-intent”.");
}

pub(super) fn validate_view_transition_descriptor_navigation(tokens: &[&str], report: &mut Report) {
    let [t] = tokens else {
        push_error(report, "Invalid value for property “navigation”.");
        return;
    };
    let t = t.trim();
    if t.eq_ignore_ascii_case("auto") || t.eq_ignore_ascii_case("none") {
        return;
    }
    push_error(report, "Invalid value for property “navigation”.");
}
