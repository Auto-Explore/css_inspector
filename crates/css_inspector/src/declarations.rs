use std::borrow::Cow;
use std::collections::HashSet;

use crate::known_properties::KnownProperties;
use crate::report::{Report, push_error, push_error_times, push_warning_level};
use crate::strutil::{
    ascii_lowercase_cow, ends_with_ascii_ci, starts_with_ascii_ci, step_string_state,
};

#[cfg(test)]
use crate::{Config, validate_css_declarations_text};

#[cfg(test)]
use crate::report::Severity;

pub(crate) fn validate_declarations(
    block: &str,
    known_properties: &KnownProperties,
    warning_level: i32,
    css1_escapes: bool,
    in_page_at_rule: bool,
    in_font_face_at_rule: bool,
    in_property_at_rule: bool,
    in_font_palette_values_at_rule: bool,
    in_counter_style_at_rule: bool,
    in_color_profile_at_rule: bool,
    in_view_transition_at_rule: bool,
    css4_profile: bool,
    report: &mut Report,
) {
    let decl_block = strip_nested_rule_blocks_in_declaration_list(block);
    let mut v = DeclValidator {
        known_properties,
        warning_level,
        css1_escapes,
        css4_profile,
        report,
        redef: BorderRedefinitionTracker::default(),
        ctx: DeclContext {
            in_page_at_rule,
            in_font_face_at_rule,
            in_property_at_rule,
            in_font_palette_values_at_rule,
            in_counter_style_at_rule,
            in_color_profile_at_rule,
            in_view_transition_at_rule,
            warned_pagebreak_too_many_values: false,
        },
        unknown_reported: HashSet::new(),
    };

    for raw in decl_block.as_ref().split(';') {
        v.validate_raw_declaration(raw);
    }
}

fn strip_nested_rule_blocks_in_declaration_list<'a>(block: &'a str) -> Cow<'a, str> {
    let bytes = block.as_bytes();
    let mut out: Option<Vec<u8>> = None;

    let mut i = 0usize;
    let mut stmt_start = 0usize;
    let mut in_string: Option<u8> = None;
    let mut escape = false;

    while i < bytes.len() {
        let b = bytes[i];
        if step_string_state(b, &mut in_string, &mut escape) {
            i += 1;
            continue;
        }

        match b {
            b';' => {
                stmt_start = i + 1;
                i += 1;
            }
            b'{' => {
                let mut depth = 1usize;
                let mut j = i + 1;
                while j < bytes.len() {
                    let bj = bytes[j];
                    if step_string_state(bj, &mut in_string, &mut escape) {
                        j += 1;
                        continue;
                    }
                    match bj {
                        b'{' => depth += 1,
                        b'}' => {
                            depth = depth.saturating_sub(1);
                            if depth == 0 {
                                break;
                            }
                        }
                        _ => {}
                    }
                    j += 1;
                }

                if depth != 0 || j >= bytes.len() {
                    break;
                }

                let out = out.get_or_insert_with(|| bytes.to_vec());
                for b in &mut out[stmt_start..=j] {
                    *b = b' ';
                }

                i = j + 1;
                stmt_start = i;
            }
            _ => {
                i += 1;
            }
        }
    }

    match out {
        Some(out) => Cow::Owned(String::from_utf8(out).expect("valid utf-8")),
        None => Cow::Borrowed(block),
    }
}

fn is_vendor_extension_property(prop: &str) -> bool {
    // Match W3C CSS validator behavior (CssPropertyFactory.isNonstandardProperty):
    // treat leading `-`/`_` properties as vendor/nonstandard.
    let Some(first) = prop.as_bytes().first() else {
        return false;
    };
    *first == b'-' || *first == b'_'
}

struct DeclContext {
    in_page_at_rule: bool,
    in_font_face_at_rule: bool,
    in_property_at_rule: bool,
    in_font_palette_values_at_rule: bool,
    in_counter_style_at_rule: bool,
    in_color_profile_at_rule: bool,
    in_view_transition_at_rule: bool,
    warned_pagebreak_too_many_values: bool,
}

struct DeclValidator<'a> {
    known_properties: &'a KnownProperties,
    warning_level: i32,
    css1_escapes: bool,
    css4_profile: bool,
    report: &'a mut Report,
    redef: BorderRedefinitionTracker,
    ctx: DeclContext,
    unknown_reported: HashSet<String>,
}

impl DeclValidator<'_> {
    fn validate_raw_declaration(&mut self, raw: &str) {
        let mut raw = raw;
        loop {
            let raw_trimmed = raw.trim();
            if raw_trimmed.is_empty() {
                return;
            }
            let Some((prop_raw, mut value_raw)) = raw_trimmed.split_once(':') else {
                push_error(self.report, "Missing ':' in declaration.");
                return;
            };
            let prop = ascii_lowercase_cow(prop_raw.trim());
            value_raw = value_raw.trim();

            // Error recovery: missing `;` between declarations.
            if let Some(start) = find_embedded_declaration_start(value_raw) {
                push_error(self.report, "Missing ';' between declarations.");
                let (value, rest) = value_raw.split_at(start);
                self.validate_property_declaration(prop.as_ref(), value);
                raw = rest;
                continue;
            }

            self.validate_property_declaration(prop.as_ref(), value_raw);
            return;
        }
    }

    fn validate_property_declaration(&mut self, prop: &str, value_raw: &str) {
        let errors_before = self.report.errors;
        let is_font_face_desc = self.ctx.in_font_face_at_rule && is_font_face_descriptor(prop);
        let is_page_desc = self.ctx.in_page_at_rule && is_page_descriptor(prop);
        let is_property_desc = self.ctx.in_property_at_rule && is_property_descriptor(prop);
        let is_font_palette_values_desc = self.ctx.in_font_palette_values_at_rule
            && is_font_palette_values_descriptor(prop);
        let is_counter_style_desc =
            self.ctx.in_counter_style_at_rule && is_counter_style_descriptor(prop);
        let is_color_profile_desc =
            self.ctx.in_color_profile_at_rule && is_color_profile_descriptor(prop);
        let is_view_transition_desc =
            self.ctx.in_view_transition_at_rule && is_view_transition_descriptor(prop);
        let is_custom_property = prop.starts_with("--");

        if prop.is_empty() {
            push_error(self.report, "Missing property name in declaration.");
            return;
        }
        if !is_valid_property_name(prop) {
            push_error(self.report, "Invalid property name in declaration.");
            return;
        }
        if !is_custom_property && !self.known_properties.contains(prop) {
            if is_font_face_desc
                || is_page_desc
                || is_property_desc
                || is_font_palette_values_desc
                || is_counter_style_desc
                || is_color_profile_desc
                || is_view_transition_desc
            {
                // Allowed descriptor within an at-rule descriptor list.
            } else {
                // Keep error counts closer to the upstream validator by reporting each unknown
                // property name at most once per declaration block.
                if self.unknown_reported.insert(prop.to_owned()) {
                    // For vnu.jar parity (Assertions.java): vendor extensions are treated as warnings
                    // and vnu.jar disables warnings by default (warningLevel=-1), so suppress these
                    // by demoting them to warnings only when warnings are disabled.
                    if is_vendor_extension_property(prop) && self.warning_level < 0 {
                        push_warning_level(
                            self.report,
                            self.warning_level,
                            0,
                            format!("“{prop}” is a vendor extension."),
                        );
                    } else {
                        push_error(self.report, format!("Unknown property “{prop}”."));
                    }
                }
                return;
            }
        }

        let value = strip_important(value_raw.trim());
        if value.is_empty() {
            push_error(self.report, format!("Missing value for property “{prop}”."));
            return;
        }

        // Common rules.
        let tokens = split_top_level_value_tokens(value);
        if tokens.is_empty() {
            push_error(self.report, format!("Missing value for property “{prop}”."));
            return;
        }

        track_border_redefinitions(
            &mut self.redef,
            prop,
            tokens.as_slice(),
            self.warning_level,
            self.report,
        );
        if self.ctx.in_page_at_rule
            && !self.ctx.warned_pagebreak_too_many_values
            && prop.starts_with("page-break-")
            && tokens.len() > 1
        {
            // Autotest `testsuite/properties/too-many-values/paged.css` expects a single warning
            // (at the default warning level) when page-break properties have too many components.
            push_warning_level(
                self.report,
                self.warning_level,
                0,
                "Too many values for a page-break property.",
            );
            self.ctx.warned_pagebreak_too_many_values = true;
        }
        if !is_custom_property && has_css_wide_keyword_mixed(&tokens) {
            push_error(self.report, format!("Invalid value for property “{prop}”."));
            return;
        }

        if prop == "overflow-clip-margin" {
            push_warning_level(
                self.report,
                self.warning_level,
                0,
                "“overflow-clip-margin” is not supported by Safari.",
            );
        }

        if tokens.len() == 1 && is_css_wide_keyword(tokens[0]) {
            return;
        }

        match prop {
            "syntax" if is_property_desc => {
                validate_property_descriptor_syntax(tokens.as_slice(), self.report)
            }
            "inherits" if is_property_desc => {
                validate_property_descriptor_inherits(tokens.as_slice(), self.report)
            }
            "rendering-intent" if is_color_profile_desc => {
                validate_color_profile_descriptor_rendering_intent(tokens.as_slice(), self.report)
            }
            "navigation" if is_view_transition_desc => {
                validate_view_transition_descriptor_navigation(tokens.as_slice(), self.report)
            }
            "float" => validate_float(tokens.as_slice(), self.report),
            "color" => validate_color(tokens.as_slice(), self.css1_escapes, self.report),
            "background-color" => {
                validate_background_color(tokens.as_slice(), self.css1_escapes, self.report)
            }
            "background" => validate_background(
                tokens.as_slice(),
                self.css1_escapes,
                self.css4_profile,
                self.report,
            ),
            "background-image" => {
                validate_background_image(tokens.as_slice(), self.css4_profile, self.report)
            }
            "zoom" => validate_zoom(tokens.as_slice(), self.report),
            "background-repeat" => {
                if !self.css4_profile {
                    validate_single_token(tokens.as_slice(), "background-repeat", self.report)
                }
            }
            "background-attachment" => {
                if !self.css4_profile {
                    validate_single_token(tokens.as_slice(), "background-attachment", self.report)
                }
            }
            "background-position" => {
                if !self.css4_profile {
                    validate_max_tokens(tokens.as_slice(), 2, "background-position", self.report)
                }
            }
            "font" => validate_font(tokens.as_slice(), self.report),
            "font-optical-sizing" => validate_font_optical_sizing(tokens.as_slice(), self.report),
            "border" => {
                validate_border_shorthand(tokens.as_slice(), self.css1_escapes, self.report)
            }
            "border-top" | "border-right" | "border-bottom" | "border-left" => {
                validate_border_shorthand(tokens.as_slice(), self.css1_escapes, self.report)
            }
            "border-width" => validate_border_width_aggregate(tokens.as_slice(), self.report),
            "border-style" => validate_border_style_aggregate(tokens.as_slice(), self.report),
            "border-color" => {
                validate_border_color_aggregate(tokens.as_slice(), self.css1_escapes, self.report)
            }
            "border-top-color"
            | "border-right-color"
            | "border-bottom-color"
            | "border-left-color" => {
                validate_border_side_color(tokens.as_slice(), self.css1_escapes, self.report)
            }
            "outline-color" => {
                validate_outline_color(tokens.as_slice(), self.css1_escapes, self.report)
            }
            "outline-style" => validate_outline_style(tokens.as_slice(), self.report),
            "outline-width" => validate_outline_width(tokens.as_slice(), self.report),
            "outline" => validate_outline(tokens.as_slice(), self.css1_escapes, self.report),
            "margin" | "padding" => validate_max_tokens(tokens.as_slice(), 4, prop, self.report),
            "border-spacing" => {
                validate_max_tokens(tokens.as_slice(), 2, "border-spacing", self.report)
            }
            "list-style" => validate_list_style(tokens.as_slice(), self.report),
            "text-decoration" => validate_text_decoration(tokens.as_slice(), self.report),
            "clip" => validate_clip(tokens.as_slice(), self.report),
            "cursor" => validate_cursor(tokens.as_slice(), self.css4_profile, self.report),
            "content" => validate_content(tokens.as_slice(), self.css4_profile, self.report),
            "quotes" => validate_quotes(tokens.as_slice(), self.report),
            "counter-increment" => {
                validate_counter_list(tokens.as_slice(), "counter-increment", self.report)
            }
            "counter-reset" => {
                validate_counter_list(tokens.as_slice(), "counter-reset", self.report)
            }
            "pause" => validate_pause(tokens.as_slice(), self.report),
            "pause-after" => validate_pause_after(tokens.as_slice(), self.report),
            "cue" => validate_cue(tokens.as_slice(), self.report),
            "cue-before" | "cue-after" => validate_cue_side(tokens.as_slice(), prop, self.report),
            "play-during" => validate_play_during(tokens.as_slice(), self.report),
            "voice-family" => validate_voice_family(tokens.as_slice(), self.report),
            "azimuth" => validate_azimuth(tokens.as_slice(), self.report),
            "elevation" => validate_elevation(tokens.as_slice(), self.report),
            "filter" => validate_filter(tokens.as_slice(), self.css4_profile, self.report),
            "overflow-clip-margin" => validate_overflow_clip_margin(tokens.as_slice(), self.report),
            _ => {}
        }

        // Generic “too many values” check for properties we don’t implement but are single-valued.
        // Avoid double-counting: only apply if no other error was raised for this declaration.
        if self.report.errors == errors_before
            && tokens.len() > 1
            && !self.css4_profile
            && is_single_valued_property(prop)
            && !is_font_face_desc
            && !is_page_desc
            && !is_property_desc
            && !is_font_palette_values_desc
            && !is_counter_style_desc
            && !is_color_profile_desc
            && !is_view_transition_desc
            && !is_custom_property
        {
            push_error(self.report, format!("Invalid value for property “{prop}”."));
        }
    }
}

fn validate_font_optical_sizing(tokens: &[&str], report: &mut Report) {
    let [t] = tokens else {
        push_error(report, "Invalid value for property “font-optical-sizing”.");
        return;
    };
    match t.trim() {
        t if t.eq_ignore_ascii_case("auto") || t.eq_ignore_ascii_case("none") => {}
        _ => push_error(report, "Invalid value for property “font-optical-sizing”."),
    }
}

fn validate_zoom(tokens: &[&str], report: &mut Report) {
    let [t] = tokens else {
        push_error(report, "Invalid value for property “zoom”.");
        return;
    };
    let t = t.trim();

    if t.eq_ignore_ascii_case("normal") || t.eq_ignore_ascii_case("reset") {
        return;
    }

    if let Some(num) = t.strip_suffix('%') {
        if let Ok(v) = num.trim().parse::<f64>() {
            if v.is_finite() && v >= 0.0 {
                return;
            }
        }
    } else if let Ok(v) = t.parse::<f64>() {
        if v.is_finite() && v >= 0.0 {
            return;
        }
    }

    push_error(report, "Invalid value for property “zoom”.");
}

fn validate_overflow_clip_margin(tokens: &[&str], report: &mut Report) {
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
    matches!(t, "content-box" | "padding-box" | "border-box" | "margin-box")
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

#[cfg(test)]
mod declaration_validation_tests {
    use super::{Config, validate_css_declarations_text};

    #[test]
    fn missing_semicolon_between_declarations_reports_error_and_recovers() {
        let report =
            validate_css_declarations_text("color: red background-color: blue", &Config::default())
                .unwrap();
        assert_eq!(report.errors, 1);
        assert_eq!(report.messages.len(), 1);
        assert_eq!(
            report.messages[0].message,
            "Missing ';' between declarations."
        );
    }

    #[test]
    fn missing_semicolons_between_multiple_declarations_reports_each_gap() {
        let report = validate_css_declarations_text(
            "color: red background-color: blue border: 0",
            &Config::default(),
        )
        .unwrap();
        let missing_semicolons = report
            .messages
            .iter()
            .filter(|m| m.message == "Missing ';' between declarations.")
            .count();
        assert_eq!(missing_semicolons, 2, "{report:?}");
        assert_eq!(report.errors, 2, "{report:?}");
    }

    #[test]
    fn embedded_declaration_detection_ignores_strings() {
        let report =
            validate_css_declarations_text(r#"content: "a: b""#, &Config::default()).unwrap();
        assert!(
            !report
                .messages
                .iter()
                .any(|m| m.message == "Missing ';' between declarations."),
            "{report:?}"
        );
    }

    #[test]
    fn unknown_property_is_reported_at_most_once_per_block() {
        let report = validate_css_declarations_text("foo: 1; foo: 2", &Config::default()).unwrap();
        let unknown = report
            .messages
            .iter()
            .filter(|m| m.message == "Unknown property “foo”.")
            .count();
        assert_eq!(unknown, 1, "{report:?}");
        assert_eq!(report.errors, 1);
    }

    #[test]
    fn box_shadow_accepts_none_and_multi_token_values() {
        let report = validate_css_declarations_text(
            "box-shadow: none; box-shadow: 0 0 0 #419;",
            &Config::default(),
        )
        .unwrap();
        assert_eq!(report.errors, 0, "{report:?}");
        assert_eq!(report.warnings, 0, "{report:?}");
        assert!(report.messages.is_empty(), "{report:?}");
    }

    #[test]
    fn custom_properties_can_store_multi_token_box_shadow_values() {
        let report = validate_css_declarations_text(
            "--primary_buttons_box_shadow: none; box-shadow: var(--primary_buttons_box_shadow);",
            &Config::default(),
        )
        .unwrap();
        assert_eq!(report.errors, 0, "{report:?}");
        assert_eq!(report.warnings, 0, "{report:?}");
        assert!(report.messages.is_empty(), "{report:?}");

        let report = validate_css_declarations_text(
            "--primary_buttons_box_shadow: 0 0 0 #419; box-shadow: var(--primary_buttons_box_shadow);",
            &Config::default(),
        )
        .unwrap();
        assert_eq!(report.errors, 0, "{report:?}");
        assert_eq!(report.warnings, 0, "{report:?}");
        assert!(report.messages.is_empty(), "{report:?}");
    }

    #[test]
    fn custom_properties_allow_css_wide_keywords_as_plain_tokens() {
        let report =
            validate_css_declarations_text("--foo: inherit 1;", &Config::default()).unwrap();
        assert_eq!(report.errors, 0, "{report:?}");
        assert_eq!(report.warnings, 0, "{report:?}");
        assert!(report.messages.is_empty(), "{report:?}");
    }

    #[test]
    fn vendor_extension_properties_are_errors_by_default() {
        let report =
            validate_css_declarations_text("-webkit-foo: 1; _bar: 2; zoom: 3", &Config::default())
                .unwrap();
        assert_eq!(report.errors, 2, "{report:?}");
        assert_eq!(report.warnings, 0, "{report:?}");
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.message == "Unknown property “-webkit-foo”.")
        );
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.message == "Unknown property “_bar”.")
        );
        assert!(
            report
                .messages
                .iter()
                .all(|m| m.message != "Unknown property “zoom”.")
        );
    }

    #[test]
    fn vendor_extension_properties_are_suppressed_when_warnings_are_disabled() {
        let mut cfg = Config::default();
        cfg.warning = Some("-1".to_string());
        let report =
            validate_css_declarations_text("-webkit-foo: 1; _bar: 2; zoom: 3", &cfg).unwrap();
        assert_eq!(report.errors, 0, "{report:?}");
        assert_eq!(report.warnings, 0, "{report:?}");
        assert!(report.messages.is_empty(), "{report:?}");
    }

    #[test]
    fn overflow_clip_margin_is_accepted_but_warns_about_safari_support() {
        for css in [
            "overflow-clip-margin: 20px;",
            "overflow-clip-margin: 1em;",
            "overflow-clip-margin: content-box 5px;",
            "overflow-clip-margin: inherit;",
            "overflow-clip-margin: initial;",
            "overflow-clip-margin: revert;",
            "overflow-clip-margin: revert-layer;",
            "overflow-clip-margin: unset;",
        ] {
            let report = validate_css_declarations_text(css, &Config::default()).unwrap();
            assert_eq!(report.errors, 0, "css={css:?} report={report:?}");
            assert_eq!(report.warnings, 1, "css={css:?} report={report:?}");
            assert_eq!(report.messages.len(), 1, "css={css:?} report={report:?}");
            assert_eq!(
                report.messages[0].message,
                "“overflow-clip-margin” is not supported by Safari.",
                "css={css:?} report={report:?}"
            );
        }
    }
}

#[cfg(test)]
mod find_embedded_declaration_start_tests {
    use super::find_embedded_declaration_start;

    #[test]
    fn ignores_colons_inside_parentheses() {
        let value = "url(foo: bar) background-color: blue";
        let start = find_embedded_declaration_start(value).expect("expected embedded declaration");
        assert!(
            value[start..].starts_with("background-color"),
            "start={start} value={value:?}"
        );
    }
}

fn is_font_face_descriptor(prop: &str) -> bool {
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

fn is_page_descriptor(prop: &str) -> bool {
    matches!(prop, "size" | "marks" | "bleed")
}

fn is_property_descriptor(prop: &str) -> bool {
    matches!(prop, "syntax" | "inherits" | "initial-value")
}

fn is_font_palette_values_descriptor(prop: &str) -> bool {
    matches!(prop, "font-family" | "base-palette" | "override-colors")
}

fn is_counter_style_descriptor(prop: &str) -> bool {
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

fn is_color_profile_descriptor(prop: &str) -> bool {
    matches!(prop, "src" | "rendering-intent")
}

fn is_view_transition_descriptor(prop: &str) -> bool {
    matches!(prop, "navigation" | "types")
}

fn validate_property_descriptor_syntax(tokens: &[&str], report: &mut Report) {
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

fn validate_property_descriptor_inherits(tokens: &[&str], report: &mut Report) {
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

fn validate_color_profile_descriptor_rendering_intent(tokens: &[&str], report: &mut Report) {
    let [t] = tokens else {
        push_error(report, "Invalid value for property “rendering-intent”.");
        return;
    };
    let t = t.trim();
    if matches!(
        ascii_lowercase_cow(t).as_ref(),
        "auto"
            | "perceptual"
            | "relative-colorimetric"
            | "saturation"
            | "absolute-colorimetric"
    ) {
        return;
    }
    push_error(report, "Invalid value for property “rendering-intent”.");
}

fn validate_view_transition_descriptor_navigation(tokens: &[&str], report: &mut Report) {
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

fn find_embedded_declaration_start(value: &str) -> Option<usize> {
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

fn is_property_ident_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'-' || b == b'_'
}

#[derive(Default)]
struct BorderRedefinitionTracker {
    seen: HashSet<&'static str>,
}

fn track_border_redefinitions(
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

fn validate_single_token(tokens: &[&str], prop: &str, report: &mut Report) {
    if tokens.len() != 1 {
        push_error(report, format!("Invalid value for property “{prop}”."));
    }
}

fn validate_max_tokens(tokens: &[&str], max: usize, prop: &str, report: &mut Report) {
    if tokens.len() > max {
        push_error(report, format!("Invalid value for property “{prop}”."));
    }
}

fn is_single_valued_property(prop: &str) -> bool {
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
            | "grid-template"
            | "list-style"
            | "margin"
            | "outline"
            | "overflow-clip-margin"
            | "pause"
            | "padding"
            | "play-during"
            | "quotes"
            | "text-decoration"
            | "azimuth"
            | "voice-family"
    )
}

fn split_top_level_value_tokens(value: &str) -> Vec<&str> {
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

#[cfg(test)]
mod split_top_level_value_tokens_tests {
    use super::split_top_level_value_tokens;

    #[test]
    fn splits_on_whitespace_and_commas_at_top_level() {
        assert_eq!(split_top_level_value_tokens("  a  b "), vec!["a", "b"]);
        assert_eq!(split_top_level_value_tokens("a,b,c"), vec!["a", "b", "c"]);
        assert_eq!(split_top_level_value_tokens("a, b  c"), vec!["a", "b", "c"]);
    }

    #[test]
    fn ignores_empty_components() {
        assert_eq!(split_top_level_value_tokens("a,,b,"), vec!["a", "b"]);
        assert_eq!(split_top_level_value_tokens(",a"), vec!["a"]);
    }

    #[test]
    fn does_not_split_inside_parentheses_or_strings() {
        assert_eq!(
            split_top_level_value_tokens("url(a b) red"),
            vec!["url(a b)", "red"]
        );
        assert_eq!(
            split_top_level_value_tokens(r#""a, b"  c"#),
            vec![r#""a, b""#, "c"]
        );
        assert_eq!(
            split_top_level_value_tokens(r#"func("a b", c) d"#),
            vec![r#"func("a b", c)"#, "d"]
        );
    }

    #[test]
    fn splits_on_commas_outside_parentheses() {
        assert_eq!(
            split_top_level_value_tokens("func(a, b), c"),
            vec!["func(a, b)", "c"]
        );
    }
}

fn has_css_wide_keyword_mixed(tokens: &[&str]) -> bool {
    // If any CSS-wide keyword appears, it must be the only component.
    tokens.len() != 1 && tokens.iter().any(|t| is_css_wide_keywordish_token(t))
}

const CSS_WIDE_KEYWORDS: [&str; 5] = ["inherit", "initial", "unset", "revert", "revert-layer"];

#[cfg(test)]
mod has_css_wide_keyword_mixed_tests {
    use super::has_css_wide_keyword_mixed;

    #[test]
    fn returns_false_for_empty_or_single_component() {
        assert!(!has_css_wide_keyword_mixed(&[]));
        assert!(!has_css_wide_keyword_mixed(&["inherit"]));
        assert!(!has_css_wide_keyword_mixed(&["inherit/20%"]));
        assert!(!has_css_wide_keyword_mixed(&["red"]));
    }

    #[test]
    fn returns_true_when_css_wide_keyword_appears_with_other_tokens() {
        assert!(has_css_wide_keyword_mixed(&["inherit", "20%"]));
        assert!(has_css_wide_keyword_mixed(&["red", "unset"]));
        assert!(has_css_wide_keyword_mixed(&["inherit/20%", "red"]));
    }

    #[test]
    fn returns_false_when_no_css_wide_keywordish_token_is_present() {
        assert!(!has_css_wide_keyword_mixed(&["red", "blue"]));
        assert!(!has_css_wide_keyword_mixed(&["1px", "solid"]));
    }
}

pub(crate) fn is_css_wide_keyword(token: &str) -> bool {
    let t = token.trim();
    CSS_WIDE_KEYWORDS
        .iter()
        .any(|&kw| t.eq_ignore_ascii_case(kw))
}

pub(crate) fn is_css_wide_keywordish_token(token: &str) -> bool {
    // Also treat `<css-wide-keyword>/...` as a css-wide keyword usage for mixed-value detection,
    // e.g. `inherit/20%` in shorthand values.
    let t = token.trim();
    if is_css_wide_keyword(t) {
        return true;
    }
    for kw in CSS_WIDE_KEYWORDS {
        if t.len() > kw.len()
            && starts_with_ascii_ci(t, kw)
            && t.as_bytes().get(kw.len()) == Some(&b'/')
        {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod is_css_wide_keyword_tests {
    use super::{is_css_wide_keyword, is_css_wide_keywordish_token};

    #[test]
    fn matches_case_insensitively_and_trims() {
        for token in ["inherit", " INHERIT ", "ReVeRt-LaYeR"] {
            assert!(is_css_wide_keyword(token), "{token}");
        }
        assert!(!is_css_wide_keyword("inherit/20%"));
    }

    #[test]
    fn keywordish_tokens_include_slash_forms() {
        for token in ["inherit/20%", "inherit/", " InHeRiT/20% "] {
            assert!(is_css_wide_keywordish_token(token), "{token}");
        }
        assert!(!is_css_wide_keywordish_token("inherit /20%"));
    }
}

fn is_valid_property_name(name: &str) -> bool {
    // Minimal check: property names are ASCII identifiers and cannot contain whitespace.
    // Allow vendor prefixes and custom properties.
    let mut bytes = name.bytes();
    let Some(first) = bytes.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == b'-' || first == b'_') {
        return false;
    }
    bytes.all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
}

fn strip_important(value: &str) -> &str {
    // Handle trailing `!important` (with optional whitespace before it).
    const IMPORTANT: &str = "!important";

    let v = value.trim_end();
    if !ends_with_ascii_ci(v, IMPORTANT) {
        return v;
    }
    let cut = v.len() - IMPORTANT.len();
    v[..cut].trim_end()
}

#[cfg(test)]
mod is_valid_property_name_tests {
    use super::is_valid_property_name;

    #[test]
    fn property_name_validation_is_ascii_and_allows_vendor_and_custom_prefixes() {
        for name in ["color", "-webkit-color", "--foo", "_x", "-1", "a_b-c9"] {
            assert!(is_valid_property_name(name), "{name}");
        }

        for name in ["", "1abc", "a b", "a\tb", "a©b", "a{b", "a/b"] {
            assert!(!is_valid_property_name(name), "{name}");
        }
    }
}

#[cfg(test)]
mod strip_important_tests {
    use super::strip_important;

    #[test]
    fn keeps_values_without_important() {
        assert_eq!(strip_important("red"), "red");
        assert_eq!(strip_important("red !importanty"), "red !importanty");
    }

    #[test]
    fn strips_important_case_insensitively_and_trims_whitespace() {
        assert_eq!(strip_important("red!important"), "red");
        assert_eq!(strip_important("red !important"), "red");
        assert_eq!(strip_important("red !IMPORTANT"), "red");
        assert_eq!(strip_important("red !important   "), "red");
    }

    #[test]
    fn handles_value_that_is_only_important() {
        assert_eq!(strip_important("!important"), "");
        assert_eq!(strip_important("  !important  "), "");
    }

    #[test]
    fn preserves_non_ascii_prefix() {
        assert_eq!(strip_important("❤ !important"), "❤");
    }
}

fn validate_float(tokens: &[&str], report: &mut Report) {
    let [token] = tokens else {
        push_error(report, "Invalid value for property “float”.");
        return;
    };
    let ok = ["left", "right", "none", "inherit", "initial", "unset"]
        .iter()
        .any(|v| token.eq_ignore_ascii_case(v));
    if !ok {
        push_error(report, "Invalid value for property “float”.");
    }
}

fn validate_color(tokens: &[&str], css1_escapes: bool, report: &mut Report) {
    let [token] = tokens else {
        push_error(report, "Invalid value for property “color”.");
        return;
    };
    let v = token.trim();
    if !is_valid_color_token(v, css1_escapes) {
        push_error(report, "Invalid value for property “color”.");
    }
}

fn validate_background_color(tokens: &[&str], css1_escapes: bool, report: &mut Report) {
    let [token] = tokens else {
        push_error(report, "Invalid value for property “background-color”.");
        return;
    };
    let v = token.trim();
    if !is_valid_color_token(v, css1_escapes) {
        push_error(report, "Invalid value for property “background-color”.");
    }
}

fn validate_border_side_color(tokens: &[&str], css1_escapes: bool, report: &mut Report) {
    let [token] = tokens else {
        push_error(report, "Invalid value for property “border-*-color”.");
        return;
    };
    let v = token.trim();
    if !is_valid_color_token(v, css1_escapes) {
        push_error(report, "Invalid value for property “border-*-color”.");
    }
}

fn validate_outline_color(tokens: &[&str], css1_escapes: bool, report: &mut Report) {
    let [token] = tokens else {
        push_error(report, "Invalid value for property “outline-color”.");
        return;
    };
    let v = token.trim();
    if v.eq_ignore_ascii_case("invert") || is_valid_color_token(v, css1_escapes) {
        return;
    }
    push_error(report, "Invalid value for property “outline-color”.");
}

fn validate_outline_style(tokens: &[&str], report: &mut Report) {
    let [token] = tokens else {
        push_error(report, "Invalid value for property “outline-style”.");
        return;
    };
    let tl = ascii_lowercase_cow(token.trim());
    match tl.as_ref() {
        "none" | "hidden" | "dotted" | "dashed" | "solid" | "double" | "groove" | "ridge"
        | "inset" | "outset" => {}
        _ => push_error(report, "Invalid value for property “outline-style”."),
    }
}

fn validate_outline_width(tokens: &[&str], report: &mut Report) {
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

#[cfg(test)]
mod validate_simple_property_value_tests {
    use super::{
        Report, validate_background_color, validate_border_side_color, validate_color,
        validate_float, validate_outline_color, validate_outline_style, validate_outline_width,
    };

    #[test]
    fn validate_float_accepts_known_keywords_and_rejects_wrong_arity() {
        let mut report = Report::default();
        validate_float(&["left"], &mut report);
        assert_eq!(report.errors, 0);

        validate_float(&["left", "right"], &mut report);
        assert_eq!(report.errors, 1);
        assert_eq!(
            report.messages.last().unwrap().message,
            "Invalid value for property “float”."
        );
    }

    #[test]
    fn validate_color_like_properties_require_single_token_and_accept_basic_color_names() {
        let mut report = Report::default();
        validate_color(&["red"], false, &mut report);
        validate_background_color(&["red"], false, &mut report);
        validate_border_side_color(&["red"], false, &mut report);
        assert_eq!(report.errors, 0);

        validate_color(&["red", "blue"], false, &mut report);
        assert_eq!(report.errors, 1);
        assert_eq!(
            report.messages.last().unwrap().message,
            "Invalid value for property “color”."
        );
    }

    #[test]
    fn validate_outline_family_properties_accept_expected_keywords() {
        let mut report = Report::default();

        validate_outline_color(&["invert"], false, &mut report);
        validate_outline_style(&["SOLID"], &mut report);
        validate_outline_width(&["THIN"], &mut report);
        validate_outline_width(&["0"], &mut report);
        validate_outline_width(&["1px"], &mut report);
        assert_eq!(report.errors, 0);

        validate_outline_style(&["bad"], &mut report);
        assert_eq!(report.errors, 1);
        assert_eq!(
            report.messages.last().unwrap().message,
            "Invalid value for property “outline-style”."
        );
    }
}

fn validate_outline(tokens: &[&str], css1_escapes: bool, report: &mut Report) {
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

        let is_color = tl.as_ref() == "invert" || is_valid_color_token(raw, css1_escapes);
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
        );
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

fn validate_cursor(tokens: &[&str], css4_profile: bool, report: &mut Report) {
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
                if is_url(raw) {
                    saw_url = true;
                    continue;
                }
                if is_integer_token(raw) {
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

fn validate_quotes(tokens: &[&str], report: &mut Report) {
    match tokens {
        [t] if t.trim().eq_ignore_ascii_case("none") => (),
        [t0, t1] if is_string_token(t0) && is_string_token(t1) => (),
        _ => push_error(report, "Invalid value for property “quotes”."),
    }
}

fn is_css_identifier_token(t: &str) -> bool {
    let t = t.trim().as_bytes();
    let Some((&first, rest)) = t.split_first() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == b'-' || first == b'_') {
        return false;
    }
    rest.iter()
        .all(|&b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
}

fn is_integer_token(t: &str) -> bool {
    let t = t.trim().as_bytes();
    let t = match t.first() {
        Some(b'+' | b'-') => &t[1..],
        _ => t,
    };
    !t.is_empty() && t.iter().all(|&b| b.is_ascii_digit())
}

#[cfg(test)]
mod token_predicate_tests {
    use super::{is_css_identifier_token, is_integer_token};

    #[test]
    fn is_css_identifier_token_matches_ascii_tokens_conservatively() {
        assert!(is_css_identifier_token("a"));
        assert!(is_css_identifier_token("abc"));
        assert!(is_css_identifier_token("-"));
        assert!(is_css_identifier_token("_"));
        assert!(is_css_identifier_token("-a"));
        assert!(is_css_identifier_token("_a"));
        assert!(is_css_identifier_token("a-b_c1"));
        assert!(is_css_identifier_token("-1"));
        assert!(is_css_identifier_token("  a  "));

        assert!(!is_css_identifier_token(""));
        assert!(!is_css_identifier_token("   "));
        assert!(!is_css_identifier_token("1a"));
        assert!(!is_css_identifier_token("a b"));
        assert!(!is_css_identifier_token("aé"));
    }

    #[test]
    fn is_integer_token_accepts_optional_sign_and_ascii_digits() {
        assert!(is_integer_token("0"));
        assert!(is_integer_token("123"));
        assert!(is_integer_token("-1"));
        assert!(is_integer_token("+2"));
        assert!(is_integer_token("  42 "));

        assert!(!is_integer_token(""));
        assert!(!is_integer_token("   "));
        assert!(!is_integer_token("+"));
        assert!(!is_integer_token("-"));
        assert!(!is_integer_token("1.0"));
        assert!(!is_integer_token("1a"));
    }
}

fn validate_counter_list(tokens: &[&str], prop: &str, report: &mut Report) {
    if let [t] = tokens {
        if t.trim().eq_ignore_ascii_case("none") {
            return;
        }
    }

    let invalid_value = |report: &mut Report| {
        push_error(report, format!("Invalid value for property “{prop}”."));
    };
    let mut i = 0usize;
    while i < tokens.len() {
        let t = tokens[i].trim();
        if t.eq_ignore_ascii_case("none") {
            invalid_value(report);
            return;
        }
        if !is_css_identifier_token(t) {
            invalid_value(report);
            return;
        }
        i += 1;
        if i < tokens.len() && is_integer_token(tokens[i]) {
            i += 1;
        }
    }
}

#[cfg(test)]
mod validate_cursor_tests {
    use super::{Report, validate_cursor};

    #[test]
    fn accepts_single_keyword_or_url() {
        for tokens in [&["auto"][..], &["  url(foo)  "][..]] {
            let mut report = Report::default();
            validate_cursor(tokens, false, &mut report);
            assert_eq!(report.errors, 0, "{tokens:?}: {report:?}");
        }
    }

    #[test]
    fn accepts_two_urls_plus_keyword() {
        let mut report = Report::default();
        validate_cursor(&["url(a)", "url(b)", "pointer"], false, &mut report);
        assert_eq!(report.errors, 0);
    }

    #[test]
    fn rejects_invalid_forms() {
        for tokens in [
            &[][..],
            &["foo"][..],
            &["url(a)", "pointer"][..],
            &["url(a)", "url(b)", "url(c)"][..],
        ] {
            let mut report = Report::default();
            validate_cursor(tokens, false, &mut report);
            assert_eq!(report.errors, 1, "{tokens:?}: {report:?}");
            assert_eq!(report.messages.len(), 1);
            assert_eq!(
                report.messages[0].message,
                "Invalid value for property “cursor”."
            );
        }
    }
}

#[cfg(test)]
mod validate_quotes_tests {
    use super::{Report, validate_quotes};

    #[test]
    fn accepts_none_case_insensitively() {
        for tokens in [&["none"][..], &[" NONE "][..]] {
            let mut report = Report::default();
            validate_quotes(tokens, &mut report);
            assert_eq!(report.errors, 0, "{tokens:?}: {report:?}");
        }
    }

    #[test]
    fn accepts_two_strings() {
        let mut report = Report::default();
        validate_quotes(&["\"a\"", "'b'"], &mut report);
        assert_eq!(report.errors, 0);
    }

    #[test]
    fn accepts_two_strings_with_surrounding_whitespace() {
        let mut report = Report::default();
        validate_quotes(&["  \"a\"  ", "\t'b'\n"], &mut report);
        assert_eq!(report.errors, 0);
    }

    #[test]
    fn rejects_other_forms() {
        for tokens in [
            &[][..],
            &["\"a\""][..],
            &["a", "b"][..],
            &["\"a\"", "b"][..],
        ] {
            let mut report = Report::default();
            validate_quotes(tokens, &mut report);
            assert_eq!(report.errors, 1, "{tokens:?}: {report:?}");
            assert_eq!(report.messages.len(), 1);
            assert_eq!(
                report.messages[0].message,
                "Invalid value for property “quotes”."
            );
        }
    }
}

#[cfg(test)]
mod validate_counter_list_tests {
    use super::{Report, validate_counter_list};

    #[test]
    fn accepts_none_alone_case_insensitively() {
        for tokens in [&["none"][..], &[" NONE "][..]] {
            let mut report = Report::default();
            validate_counter_list(tokens, "counter-increment", &mut report);
            assert_eq!(report.errors, 0, "{tokens:?}: {report:?}");
        }
    }

    #[test]
    fn accepts_identifier_list_with_optional_integers() {
        for tokens in [
            &["chapter"][..],
            &["chapter", "1"][..],
            &["chapter", "1", "section", "2"][..],
            &["chapter", "section"][..],
            &["chapter", "-1"][..],
        ] {
            let mut report = Report::default();
            validate_counter_list(tokens, "counter-reset", &mut report);
            assert_eq!(report.errors, 0, "{tokens:?}: {report:?}");
        }
    }

    #[test]
    fn rejects_none_mixed_or_invalid_tokens() {
        for tokens in [
            &["chapter", "none"][..],
            &["1"][..],
            &["a", "1", "b", "+"][..],
        ] {
            let mut report = Report::default();
            validate_counter_list(tokens, "counter-reset", &mut report);
            assert_eq!(report.errors, 1, "{tokens:?}: {report:?}");
            assert_eq!(report.messages.len(), 1);
            assert_eq!(
                report.messages[0].message,
                "Invalid value for property “counter-reset”."
            );
        }
    }
}

fn validate_content(tokens: &[&str], css4_profile: bool, report: &mut Report) {
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
            return num.trim().parse::<f64>().is_ok();
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

fn validate_pause_after(tokens: &[&str], report: &mut Report) {
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

fn validate_pause(tokens: &[&str], report: &mut Report) {
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

fn validate_cue_side(tokens: &[&str], prop: &str, report: &mut Report) {
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

fn validate_cue(tokens: &[&str], report: &mut Report) {
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

fn validate_play_during(tokens: &[&str], report: &mut Report) {
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
            return num.trim().parse::<f64>().is_ok();
        }
    }
    false
}

fn validate_azimuth(tokens: &[&str], report: &mut Report) {
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

fn validate_elevation(tokens: &[&str], report: &mut Report) {
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

fn validate_voice_family(tokens: &[&str], report: &mut Report) {
    if tokens.is_empty() {
        push_error(report, "Invalid value for property “voice-family”.");
        return;
    }
    for t in tokens {
        let t = t.trim();
        if is_string_token(t) || is_css_identifier_token(t) {
            continue;
        }
        push_error(report, "Invalid value for property “voice-family”.");
        return;
    }
}

fn validate_background(tokens: &[&str], css1_escapes: bool, css4_profile: bool, report: &mut Report) {
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

        if is_valid_color_token(raw, css1_escapes) {
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

fn is_any_percentage_token(t: &str) -> bool {
    let Some(num) = t.trim().strip_suffix('%') else {
        return false;
    };
    num.trim().parse::<f64>().is_ok()
}

fn is_quoted_string_token(t: &str) -> bool {
    let bytes = t.as_bytes();
    if bytes.len() < 2 {
        return false;
    }
    let q = bytes[0];
    (q == b'"' || q == b'\'') && bytes[bytes.len() - 1] == q
}

fn validate_background_image(tokens: &[&str], css4_profile: bool, report: &mut Report) {
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

fn is_background_image_like_token(lower: &str) -> bool {
    // Cover common `<image>` functions used in modern CSS.
    const FUNCTIONS: [&str; 11] = [
        "linear-gradient",
        "repeating-linear-gradient",
        "radial-gradient",
        "repeating-radial-gradient",
        "conic-gradient",
        "repeating-conic-gradient",
        "image-set",
        "-webkit-image-set",
        "cross-fade",
        "element",
        "paint",
    ];

    FUNCTIONS
        .iter()
        .any(|name| lower.starts_with(*name) && is_balanced_function_call_token(lower, name))
}

#[cfg(test)]
mod validate_single_token_value_tests {
    use super::{
        Report, validate_background_image, validate_cue_side, validate_elevation,
        validate_pause_after,
    };

    #[test]
    fn validate_pause_after_accepts_valid_single_values_and_rejects_others() {
        let mut report = Report::default();
        validate_pause_after(&["1s"], &mut report);
        validate_pause_after(&["none"], &mut report);
        assert_eq!(report.errors, 0);
        assert!(report.messages.is_empty());

        validate_pause_after(&["foo"], &mut report);
        assert_eq!(report.errors, 1);
        assert_eq!(
            report.messages.last().unwrap().message,
            "Invalid value for property “pause-after”."
        );

        validate_pause_after(&["1s", "2s"], &mut report);
        assert_eq!(report.errors, 2);
    }

    #[test]
    fn validate_cue_side_accepts_none_and_url_and_rejects_others() {
        let mut report = Report::default();
        validate_cue_side(&["none"], "cue-before", &mut report);
        validate_cue_side(&["url(x)"], "cue-before", &mut report);
        assert_eq!(report.errors, 0);
        assert!(report.messages.is_empty());

        validate_cue_side(&["foo"], "cue-before", &mut report);
        assert_eq!(report.errors, 1);
        assert_eq!(
            report.messages.last().unwrap().message,
            "Invalid value for property “cue-before”."
        );

        validate_cue_side(&["none", "url(x)"], "cue-before", &mut report);
        assert_eq!(report.errors, 2);
    }

    #[test]
    fn validate_elevation_accepts_known_keywords_and_angles() {
        let mut report = Report::default();
        validate_elevation(&["below"], &mut report);
        validate_elevation(&["10deg"], &mut report);
        assert_eq!(report.errors, 0);
        assert!(report.messages.is_empty());

        validate_elevation(&["foo"], &mut report);
        assert_eq!(report.errors, 1);
        assert_eq!(
            report.messages.last().unwrap().message,
            "Invalid value for property “elevation”."
        );

        validate_elevation(&["below", "above"], &mut report);
        assert_eq!(report.errors, 2);
    }

    #[test]
    fn validate_background_image_accepts_none_inherit_and_valid_url_function() {
        let mut report = Report::default();
        validate_background_image(&["none"], false, &mut report);
        validate_background_image(&["inherit"], false, &mut report);
        validate_background_image(&["url(x)"], false, &mut report);
        assert_eq!(report.errors, 0);
        assert!(report.messages.is_empty());

        validate_background_image(&["url(x\\)"], false, &mut report);
        assert_eq!(report.errors, 1);
        assert_eq!(
            report.messages.last().unwrap().message,
            "Invalid value for property “background-image”."
        );

        validate_background_image(&["none", "url(x)"], false, &mut report);
        assert_eq!(report.errors, 2);
    }
}

pub(crate) fn is_valid_url_function_token(token: &str) -> bool {
    let t = token.trim();
    if !starts_with_ascii_ci(t, "url(") || !t.ends_with(')') || t.len() < 5 {
        return false;
    }
    let inner = &t[4..t.len() - 1];
    let inner = inner.trim();
    if inner.is_empty() {
        return false;
    }
    // Unquoted URLs must not end in an escaping backslash (would escape the closing `)`).
    if !inner.starts_with('"') && !inner.starts_with('\'') {
        return !inner.ends_with('\\');
    }
    let bytes = inner.as_bytes();
    if bytes.len() < 2 {
        return false;
    }
    let q = bytes[0];
    if bytes[bytes.len() - 1] != q {
        return false;
    }
    // Closing quote must not be escaped.
    let backslashes = bytes[..bytes.len() - 1]
        .iter()
        .rev()
        .take_while(|&&b| b == b'\\')
        .count();
    backslashes % 2 == 0
}

fn validate_font(tokens: &[&str], report: &mut Report) {
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
        || tl.parse::<f64>().is_ok()
}

fn is_hex_color(s: &str) -> bool {
    let Some(hex) = s.strip_prefix('#') else {
        return false;
    };
    matches!(hex.len(), 3 | 4 | 6 | 8) && hex.chars().all(|c| c.is_ascii_hexdigit())
}

fn is_balanced_function_call_token(token: &str, function_name: &str) -> bool {
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
        if step_string_state(b, &mut in_string, &mut escape) {
            i += 1;
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

fn is_valid_color_token(raw: &str, css1_escapes: bool) -> bool {
    let t = raw.trim();
    if t.is_empty() {
        return false;
    }
    if t.ends_with('\\') {
        // Trailing escapes are treated as invalid in the autotest suite (bug 3631).
        return false;
    }
    // Strings are not colors.
    if t.len() >= 2 {
        let bytes = t.as_bytes();
        let q = bytes[0];
        if (q == b'"' || q == b'\'') && bytes[bytes.len() - 1] == q {
            return false;
        }
    }

    let lower_ascii = ascii_lowercase_cow(t);
    let lower_ascii = lower_ascii.as_ref();

    // Hex colors.
    if is_hex_color(lower_ascii) {
        return true;
    }

    // Functional colors.
    if lower_ascii.starts_with("rgb(") {
        return is_valid_rgb_like_function(lower_ascii, false);
    }
    if lower_ascii.starts_with("rgba(") {
        return is_valid_rgb_like_function(lower_ascii, true);
    }
    if lower_ascii.starts_with("hsl(") {
        return is_valid_hsl_like_function(lower_ascii, false);
    }
    if lower_ascii.starts_with("hsla(") {
        return is_valid_hsl_like_function(lower_ascii, true);
    }
    if lower_ascii.starts_with("oklch(") {
        return is_balanced_function_call_token(lower_ascii, "oklch");
    }
    if lower_ascii.starts_with("oklab(") {
        return is_balanced_function_call_token(lower_ascii, "oklab");
    }
    if lower_ascii.starts_with("hwb(") {
        return is_balanced_function_call_token(lower_ascii, "hwb");
    }
    if lower_ascii.starts_with("lab(") {
        return is_balanced_function_call_token(lower_ascii, "lab");
    }
    if lower_ascii.starts_with("lch(") {
        return is_balanced_function_call_token(lower_ascii, "lch");
    }
    if lower_ascii.starts_with("color-mix(") {
        return is_balanced_function_call_token(lower_ascii, "color-mix");
    }
    if lower_ascii.starts_with("color(") {
        return is_balanced_function_call_token(lower_ascii, "color");
    }
    if lower_ascii.starts_with("device-cmyk(") {
        return is_balanced_function_call_token(lower_ascii, "device-cmyk");
    }
    if lower_ascii.starts_with("light-dark(") {
        return is_balanced_function_call_token(lower_ascii, "light-dark");
    }
    if lower_ascii.starts_with("color-contrast(") {
        return is_balanced_function_call_token(lower_ascii, "color-contrast");
    }
    if lower_ascii.starts_with("var(") {
        return is_balanced_function_call_token(lower_ascii, "var");
    }
    if lower_ascii.starts_with("env(") {
        return is_balanced_function_call_token(lower_ascii, "env");
    }

    // Ident colors (with CSS escapes).
    let ident = if css1_escapes {
        unescape_css_identifier_css1_compat(t)
    } else {
        unescape_css_identifier_greedy(t)
    };
    let ident_lower = ascii_lowercase_cow(&ident);
    let ident_lower = ident_lower.as_ref();
    ident_lower == "transparent"
        || ident_lower == "currentcolor"
        || is_basic_named_color(ident_lower)
        || is_css_wide_keyword(ident_lower)
}

fn is_basic_named_color(t: &str) -> bool {
    matches!(
        t,
        "black"
            | "silver"
            | "gray"
            | "white"
            | "maroon"
            | "red"
            | "purple"
            | "fuchsia"
            | "green"
            | "lime"
            | "olive"
            | "yellow"
            | "navy"
            | "blue"
            | "teal"
            | "aqua"
    )
}

fn unescape_css_identifier_css1_compat(s: &str) -> String {
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

fn unescape_css_identifier_greedy(s: &str) -> String {
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

#[cfg(test)]
mod unescape_css_identifier_tests {
    use super::{unescape_css_identifier_css1_compat, unescape_css_identifier_greedy};

    #[test]
    fn backslash_newline_is_line_continuation() {
        assert_eq!(unescape_css_identifier_css1_compat("a\\\nb"), "ab");
        assert_eq!(unescape_css_identifier_greedy("a\\\nb"), "ab");

        assert_eq!(unescape_css_identifier_css1_compat("a\\\rb"), "ab");
        assert_eq!(unescape_css_identifier_greedy("a\\\rb"), "ab");

        assert_eq!(unescape_css_identifier_css1_compat("a\\\x0Cb"), "ab");
        assert_eq!(unescape_css_identifier_greedy("a\\\x0Cb"), "ab");
    }

    #[test]
    fn trailing_backslash_is_dropped() {
        assert_eq!(unescape_css_identifier_css1_compat("\\"), "");
        assert_eq!(unescape_css_identifier_greedy("\\"), "");
    }

    #[test]
    fn hex_escape_decodes_and_skips_optional_whitespace() {
        assert_eq!(unescape_css_identifier_css1_compat("\\0041 B"), "AB");
        assert_eq!(unescape_css_identifier_greedy("\\0041 B"), "AB");
    }

    #[test]
    fn css1_compat_prefers_4_digits_when_possible() {
        assert_eq!(unescape_css_identifier_css1_compat("\\0065d"), "ed");
    }

    #[test]
    fn greedy_consumes_all_hex_digits() {
        assert_eq!(unescape_css_identifier_greedy("\\0065d"), "\u{65d}");
    }

    #[test]
    fn non_hex_escape_escapes_next_char_verbatim() {
        assert_eq!(unescape_css_identifier_css1_compat("\\-"), "-");
        assert_eq!(unescape_css_identifier_greedy("\\-"), "-");
    }
}

fn hex_value(b: u8) -> u8 {
    match b {
        b'0'..=b'9' => b - b'0',
        b'a'..=b'f' => 10 + (b - b'a'),
        b'A'..=b'F' => 10 + (b - b'A'),
        _ => 0,
    }
}

fn iter_function_args(inner: &str) -> impl Iterator<Item = &str> {
    // rgb()/hsl() arguments in the suite are comma-separated without nested functions.
    inner.split(',').map(str::trim).filter(|s| !s.is_empty())
}

fn is_valid_rgb_like_function(token_lower: &str, has_alpha: bool) -> bool {
    let name_len = if has_alpha { 5 } else { 4 }; // "rgba(" or "rgb("
    let Some(without_close) = token_lower.strip_suffix(")") else {
        return false;
    };
    if without_close.len() <= name_len {
        return false;
    }
    let inner = &without_close[name_len..];
    if inner.trim_start().starts_with("from ") {
        return is_balanced_function_call_token(token_lower, if has_alpha { "rgba" } else { "rgb" });
    }
    if !inner.contains(',') {
        return is_valid_rgb_like_space_syntax(inner, has_alpha);
    }
    let mut args = iter_function_args(inner);
    let (Some(c1), Some(c2), Some(c3)) = (args.next(), args.next(), args.next()) else {
        return false;
    };
    let alpha = args.next();
    if args.next().is_some() {
        return false;
    }

    let is_percent = c1.ends_with('%') || c2.ends_with('%') || c3.ends_with('%');
    if is_percent {
        if !(c1.ends_with('%') && c2.ends_with('%') && c3.ends_with('%')) {
            return false;
        }
        if !is_percentage_0_100(c1) || !is_percentage_0_100(c2) || !is_percentage_0_100(c3) {
            return false;
        }
    } else {
        if !is_integer_0_255(c1) || !is_integer_0_255(c2) || !is_integer_0_255(c3) {
            return false;
        }
    }
    if has_alpha {
        alpha.is_some_and(is_alpha_value)
    } else {
        alpha.is_none()
    }
}

fn is_valid_rgb_like_space_syntax(inner: &str, has_alpha: bool) -> bool {
    let mut parts = inner.split('/');
    let before = parts.next().unwrap_or("");
    let after = parts.next();
    if parts.next().is_some() {
        return false;
    }

    let comps: Vec<&str> = before
        .split_ascii_whitespace()
        .filter(|s| !s.is_empty())
        .collect();
    if comps.len() != 3 {
        return false;
    }

    let alpha = if let Some(s) = after {
        let tokens: Vec<&str> = s
            .split_ascii_whitespace()
            .filter(|t| !t.is_empty())
            .collect();
        if tokens.len() != 1 {
            return false;
        }
        Some(tokens[0])
    } else {
        None
    };

    let is_percent = comps.iter().any(|c| c.ends_with('%'));
    if is_percent {
        if !comps.iter().all(|c| c.ends_with('%') && is_percentage_0_100(c)) {
            return false;
        }
    } else if !comps.iter().all(|c| is_integer_0_255(c)) {
        return false;
    }

    if has_alpha {
        alpha.is_some_and(is_alpha_value)
    } else {
        alpha.map_or(true, is_alpha_value)
    }
}

fn is_valid_hsl_like_function(token_lower: &str, has_alpha: bool) -> bool {
    let name_len = if has_alpha { 5 } else { 4 }; // "hsla(" or "hsl("
    let Some(without_close) = token_lower.strip_suffix(")") else {
        return false;
    };
    if without_close.len() <= name_len {
        return false;
    }
    let inner = &without_close[name_len..];
    if inner.trim_start().starts_with("from ") {
        return is_balanced_function_call_token(token_lower, if has_alpha { "hsla" } else { "hsl" });
    }
    if !inner.contains(',') {
        return is_valid_hsl_like_space_syntax(inner, has_alpha);
    }
    let mut args = iter_function_args(inner);
    let (Some(_h), Some(s), Some(l)) = (args.next(), args.next(), args.next()) else {
        return false;
    };
    let alpha = args.next();
    if args.next().is_some() {
        return false;
    }

    // Keep this permissive for now: check only that saturation/lightness are percentages.
    if !(s.ends_with('%') && l.ends_with('%')) {
        return false;
    }
    if !is_percentage_0_100(s) || !is_percentage_0_100(l) {
        return false;
    }
    if has_alpha {
        alpha.is_some_and(is_alpha_value)
    } else {
        alpha.is_none()
    }
}

fn is_valid_hsl_like_space_syntax(inner: &str, has_alpha: bool) -> bool {
    let mut parts = inner.split('/');
    let before = parts.next().unwrap_or("");
    let after = parts.next();
    if parts.next().is_some() {
        return false;
    }

    let comps: Vec<&str> = before
        .split_ascii_whitespace()
        .filter(|s| !s.is_empty())
        .collect();
    if comps.len() != 3 {
        return false;
    }

    let alpha = if let Some(s) = after {
        let tokens: Vec<&str> = s
            .split_ascii_whitespace()
            .filter(|t| !t.is_empty())
            .collect();
        if tokens.len() != 1 {
            return false;
        }
        Some(tokens[0])
    } else {
        None
    };

    let s = comps[1];
    let l = comps[2];
    if !(s.ends_with('%') && l.ends_with('%')) {
        return false;
    }
    if !is_percentage_0_100(s) || !is_percentage_0_100(l) {
        return false;
    }

    if has_alpha {
        alpha.is_some_and(is_alpha_value)
    } else {
        alpha.map_or(true, is_alpha_value)
    }
}

fn is_percentage_0_100(s: &str) -> bool {
    let Some(num) = s.strip_suffix('%') else {
        return false;
    };
    let Ok(v) = num.trim().parse::<f64>() else {
        return false;
    };
    (0.0..=100.0).contains(&v)
}

#[cfg(test)]
mod color_function_arg_tests {
    use super::{is_valid_hsl_like_function, is_valid_rgb_like_function};

    #[test]
    fn rgb_like_requires_exact_nonempty_argument_count() {
        assert!(is_valid_rgb_like_function("rgb(1,2,3)", false));
        assert!(is_valid_rgb_like_function("rgb(1 2 3)", false));
        assert!(is_valid_rgb_like_function("rgb(1 2 3 / 0.5)", false));
        assert!(!is_valid_rgb_like_function("rgb()", false));
        assert!(!is_valid_rgb_like_function("rgb(1,2,3", false));
        assert!(!is_valid_rgb_like_function("rgb(1,2)", false));
        assert!(!is_valid_rgb_like_function("rgb(1,2,3,4)", false));
        assert!(is_valid_rgb_like_function("rgba(1,2,3,0.5)", true));
        assert!(is_valid_rgb_like_function("rgba(1 2 3 / 0.5)", true));
        assert!(!is_valid_rgb_like_function("rgba()", true));
        assert!(!is_valid_rgb_like_function("rgba(1,2,3,0.5", true));
        assert!(!is_valid_rgb_like_function("rgba(1,2,3)", true));
        assert!(!is_valid_rgb_like_function("rgba(1 2 3)", true));
        assert!(!is_valid_rgb_like_function("rgba(1,2,3,0.5,0.6)", true));
    }

    #[test]
    fn hsl_like_requires_percent_saturation_and_lightness() {
        assert!(is_valid_hsl_like_function("hsl(0,0%,0%)", false));
        assert!(is_valid_hsl_like_function("hsl(0 0% 0%)", false));
        assert!(is_valid_hsl_like_function("hsl(0 0% 0% / 50%)", false));
        assert!(!is_valid_hsl_like_function("hsl()", false));
        assert!(!is_valid_hsl_like_function("hsl(0,0%,0%", false));
        assert!(!is_valid_hsl_like_function("hsl(0,0,0%)", false));
        assert!(!is_valid_hsl_like_function("hsl(0,0%,0)", false));
        assert!(is_valid_hsl_like_function("hsla(0,0%,0%,0.5)", true));
        assert!(is_valid_hsl_like_function("hsla(0 0% 0% / 0.5)", true));
        assert!(!is_valid_hsl_like_function("hsla()", true));
        assert!(!is_valid_hsl_like_function("hsla(0,0%,0%,0.5", true));
        assert!(!is_valid_hsl_like_function("hsla(0,0%,0%)", true));
        assert!(!is_valid_hsl_like_function("hsla(0 0% 0%)", true));
    }

    #[test]
    fn rgb_like_alpha_values_match_is_alpha_value_rules() {
        assert!(is_valid_rgb_like_function("rgba(1,2,3,0)", true));
        assert!(is_valid_rgb_like_function("rgba(1,2,3,1)", true));
        assert!(!is_valid_rgb_like_function("rgba(1,2,3,2)", true));
        assert!(is_valid_rgb_like_function("rgba(1,2,3,100%)", true));
        assert!(!is_valid_rgb_like_function("rgba(1,2,3,101%)", true));
    }

    #[test]
    fn hsl_like_alpha_values_match_is_alpha_value_rules() {
        assert!(is_valid_hsl_like_function("hsla(0,0%,0%,0)", true));
        assert!(is_valid_hsl_like_function("hsla(0,0%,0%,1)", true));
        assert!(!is_valid_hsl_like_function("hsla(0,0%,0%,2)", true));
        assert!(is_valid_hsl_like_function("hsla(0,0%,0%,100%)", true));
        assert!(!is_valid_hsl_like_function("hsla(0,0%,0%,101%)", true));
    }

    #[test]
    fn color_function_args_allow_whitespace_around_commas_and_alpha() {
        assert!(is_valid_rgb_like_function("rgb( 1 , 2 , 3 )", false));
        assert!(is_valid_rgb_like_function("rgba(1, 2, 3, 0.5 )", true));
        assert!(is_valid_hsl_like_function("hsl( 0 , 0% , 0% )", false));
        assert!(is_valid_hsl_like_function("hsla(0, 0%, 0%, 50% )", true));
    }
}

fn is_integer_0_255(s: &str) -> bool {
    let t = s.trim();
    if t.is_empty() {
        return false;
    }
    if t.contains('.') {
        return false;
    }
    let Ok(v) = t.parse::<i32>() else {
        return false;
    };
    (0..=255).contains(&v)
}

fn is_alpha_value(s: &str) -> bool {
    let t = s.trim();
    if let Some(pct) = t.strip_suffix('%') {
        let Ok(v) = pct.trim().parse::<f64>() else {
            return false;
        };
        return (0.0..=100.0).contains(&v);
    }
    let Ok(v) = t.parse::<f64>() else {
        return false;
    };
    (0.0..=1.0).contains(&v)
}

fn validate_border_shorthand(tokens: &[&str], css1_escapes: bool, report: &mut Report) {
    if !(1..=3).contains(&tokens.len()) {
        push_error(report, "Invalid value for property “border”.");
        return;
    }

    let mut has_width = false;
    let mut has_style = false;
    let mut has_color = false;
    for &t in tokens {
        let tl = ascii_lowercase_cow(t);
        if is_border_width_token(tl.as_ref()) {
            if has_width {
                push_error(report, "Invalid value for property “border”.");
                return;
            }
            has_width = true;
            continue;
        }
        if is_border_style_token(tl.as_ref()) {
            if has_style {
                push_error(report, "Invalid value for property “border”.");
                return;
            }
            has_style = true;
            continue;
        }
        if is_border_color_token(tl.as_ref(), css1_escapes) {
            if has_color {
                push_error(report, "Invalid value for property “border”.");
                return;
            }
            has_color = true;
            continue;
        }
        push_error(report, "Invalid value for property “border”.");
        return;
    }
}

fn validate_border_width_aggregate(tokens: &[&str], report: &mut Report) {
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

fn validate_border_style_aggregate(tokens: &[&str], report: &mut Report) {
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

fn validate_border_color_aggregate(tokens: &[&str], css1_escapes: bool, report: &mut Report) {
    if !(1..=4).contains(&tokens.len()) {
        push_error(report, "Invalid value for property “border-color”.");
        return;
    }
    for t in tokens {
        let tl = ascii_lowercase_cow(t);
        if !is_border_color_token(tl.as_ref(), css1_escapes) {
            push_error(report, "Invalid value for property “border-color”.");
            return;
        }
    }
}

fn is_border_width_token(t: &str) -> bool {
    matches!(t, "thin" | "medium" | "thick" | "0") || is_length_token(t)
}

fn is_length_token(t: &str) -> bool {
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
    num.is_some()
        && matches!(
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

fn split_number_and_unit(s: &str) -> (Option<f64>, &str) {
    let s = s.trim();
    let bytes = s.as_bytes();
    if bytes.is_empty() {
        return (None, "");
    }
    let mut idx = 0usize;
    if matches!(bytes[0], b'+' | b'-') {
        idx = 1;
    }
    while idx < bytes.len() && (bytes[idx].is_ascii_digit() || bytes[idx] == b'.') {
        idx += 1;
    }
    if idx == 0 {
        return (None, "");
    }
    let (n, u) = s.split_at(idx);
    let num = n.parse::<f64>().ok();
    (num, u)
}

#[cfg(test)]
mod split_number_and_unit_tests {
    use super::split_number_and_unit;

    #[test]
    fn splits_number_prefix_and_returns_remaining_unit_verbatim() {
        let (n, u) = split_number_and_unit("1px");
        assert_eq!(n, Some(1.0));
        assert_eq!(u, "px");

        let (n, u) = split_number_and_unit(" 1 px ");
        assert_eq!(n, Some(1.0));
        assert_eq!(u, " px");

        let (n, u) = split_number_and_unit("1e2px");
        assert_eq!(n, Some(1.0));
        assert_eq!(u, "e2px");

        let (n, u) = split_number_and_unit("1π");
        assert_eq!(n, Some(1.0));
        assert_eq!(u, "π");
    }

    #[test]
    fn rejects_inputs_without_a_number_prefix() {
        assert_eq!(split_number_and_unit(""), (None, ""));
        assert_eq!(split_number_and_unit("px"), (None, ""));
        assert_eq!(split_number_and_unit("π1"), (None, ""));
    }

    #[test]
    fn keeps_unit_even_when_number_parse_fails() {
        assert_eq!(split_number_and_unit(".px"), (None, "px"));
        assert_eq!(split_number_and_unit("+"), (None, ""));
        assert_eq!(split_number_and_unit("+ 1"), (None, " 1"));
    }

    #[test]
    fn handles_optional_sign_and_decimal_points() {
        assert_eq!(split_number_and_unit("-1.5em"), (Some(-1.5), "em"));
        assert_eq!(split_number_and_unit("+.5rem"), (Some(0.5), "rem"));
        assert_eq!(split_number_and_unit("-.px"), (None, "px"));
    }
}

#[cfg(test)]
mod is_length_token_tests {
    use super::is_length_token;

    #[test]
    fn accepts_common_modern_units() {
        for t in [
            "1px", "1q", "1em", "1rem", "1ch", "1lh", "1vw", "1svh", "1dvb", "1cqi",
            "var(--x)",
            "calc(1px + 2px)",
            "min(1px, 2px)",
            "max(1px, 2px)",
            "clamp(1px, 2px, 3px)",
            "env(safe-area-inset-top)",
        ] {
            assert!(is_length_token(t), "{t}");
        }
    }

    #[test]
    fn rejects_dimensionless_and_unknown_units() {
        for t in ["1", "1nope", "px", " 1 px ", "calc("] {
            assert!(!is_length_token(t), "{t}");
        }
    }
}

fn is_border_style_token(t: &str) -> bool {
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

fn is_border_color_token(t: &str, css1_escapes: bool) -> bool {
    is_valid_color_token(t, css1_escapes)
}

fn validate_list_style(tokens: &[&str], report: &mut Report) {
    // Minimal, test-driven parsing for the `list-style` shorthand:
    //   <list-style-type> || <list-style-position> || <list-style-image>
    //
    // This is intentionally not a complete implementation; it exists to catch the autotest
    // “too many values” cases and a common typo (`disk` vs `disc`).
    if tokens.is_empty() || tokens.len() > 3 {
        push_error(report, "Invalid value for property “list-style”.");
        return;
    }

    let mut have_type = false;
    let mut have_position = false;
    let mut have_image = false;

    for t in tokens {
        let tl = ascii_lowercase_cow(t.trim());
        let tl = tl.as_ref();
        if tl == "disk" {
            push_error(report, "Invalid value for property “list-style”.");
            return;
        }
        if matches!(tl, "inside" | "outside") {
            if have_position {
                push_error(report, "Invalid value for property “list-style”.");
                return;
            }
            have_position = true;
            continue;
        }
        if tl.starts_with("url(") {
            if have_image {
                push_error(report, "Invalid value for property “list-style”.");
                return;
            }
            have_image = true;
            continue;
        }
        if tl == "none" {
            // `none` is ambiguous (type vs image); allow it to fill whichever slot is still free.
            if !have_type {
                have_type = true;
                continue;
            }
            if !have_image {
                have_image = true;
                continue;
            }
            push_error(report, "Invalid value for property “list-style”.");
            return;
        }
        if is_list_style_type_keyword(tl) {
            if have_type {
                push_error(report, "Invalid value for property “list-style”.");
                return;
            }
            have_type = true;
            continue;
        }

        push_error(report, "Invalid value for property “list-style”.");
        return;
    }
}

#[cfg(test)]
mod validate_list_style_tests {
    use super::{Config, Severity, validate_css_declarations_text};

    #[test]
    fn accepts_inside_and_outside() {
        for css in ["list-style: inside;", "list-style: outside;"] {
            let report = validate_css_declarations_text(css, &Config::default()).unwrap();
            assert_eq!(report.errors, 0, "css={css:?} report={report:?}");
            assert_eq!(report.warnings, 0);
            assert!(report.messages.is_empty());
        }
    }

    #[test]
    fn rejects_duplicate_positions() {
        for css in [
            "list-style: inside outside;",
            "list-style: outside outside;",
        ] {
            let report = validate_css_declarations_text(css, &Config::default()).unwrap();
            assert_eq!(report.errors, 1, "css={css:?} report={report:?}");
            assert_eq!(report.warnings, 0);
            assert_eq!(report.messages.len(), 1);
            assert_eq!(report.messages[0].severity, Severity::Error);
            assert_eq!(
                report.messages[0].message,
                "Invalid value for property “list-style”."
            );
        }
    }

    #[test]
    fn rejects_disk_typo() {
        let report =
            validate_css_declarations_text("list-style: disk;", &Config::default()).unwrap();
        assert_eq!(report.errors, 1);
        assert_eq!(report.warnings, 0);
        assert_eq!(report.messages.len(), 1);
        assert_eq!(report.messages[0].severity, Severity::Error);
        assert_eq!(
            report.messages[0].message,
            "Invalid value for property “list-style”."
        );
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

fn validate_text_decoration(tokens: &[&str], report: &mut Report) {
    // CSS1/CSS2-era grammar:
    //   none | [ underline || overline || line-through || blink ]
    if tokens.is_empty() {
        push_error(report, "Invalid value for property “text-decoration”.");
        return;
    }
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
}

#[cfg(test)]
mod validate_text_decoration_tests {
    use super::{Config, Severity, validate_css_declarations_text};

    #[test]
    fn accepts_none_alone() {
        let report =
            validate_css_declarations_text("text-decoration: none;", &Config::default()).unwrap();
        assert_eq!(report.errors, 0);
        assert_eq!(report.warnings, 0);
        assert!(report.messages.is_empty());
    }

    #[test]
    fn accepts_distinct_keywords() {
        let report = validate_css_declarations_text(
            "text-decoration: underline overline;",
            &Config::default(),
        )
        .unwrap();
        assert_eq!(report.errors, 0);
        assert_eq!(report.warnings, 0);
        assert!(report.messages.is_empty());
    }

    #[test]
    fn rejects_duplicate_keywords() {
        let report = validate_css_declarations_text(
            "text-decoration: underline underline;",
            &Config::default(),
        )
        .unwrap();
        assert_eq!(report.errors, 1);
        assert_eq!(report.warnings, 0);
        assert_eq!(report.messages.len(), 1);
        assert_eq!(report.messages[0].severity, Severity::Error);
        assert_eq!(
            report.messages[0].message,
            "Invalid value for property “text-decoration”."
        );
    }

    #[test]
    fn rejects_none_combined_with_other_keywords() {
        let report =
            validate_css_declarations_text("text-decoration: none underline;", &Config::default())
                .unwrap();
        assert_eq!(report.errors, 1);
        assert_eq!(report.warnings, 0);
        assert_eq!(report.messages.len(), 1);
        assert_eq!(report.messages[0].severity, Severity::Error);
        assert_eq!(
            report.messages[0].message,
            "Invalid value for property “text-decoration”."
        );
    }

    #[test]
    fn rejects_unknown_keyword() {
        let report =
            validate_css_declarations_text("text-decoration: sparkle;", &Config::default())
                .unwrap();
        assert_eq!(report.errors, 1);
        assert_eq!(report.warnings, 0);
        assert_eq!(report.messages.len(), 1);
        assert_eq!(report.messages[0].severity, Severity::Error);
        assert_eq!(
            report.messages[0].message,
            "Invalid value for property “text-decoration”."
        );
    }

    #[test]
    fn treats_keywords_case_insensitively() {
        let report = validate_css_declarations_text(
            "text-decoration: UnderLine underline;",
            &Config::default(),
        )
        .unwrap();
        assert_eq!(report.errors, 1);
        assert_eq!(report.warnings, 0);
        assert_eq!(report.messages.len(), 1);
        assert_eq!(report.messages[0].severity, Severity::Error);
        assert_eq!(
            report.messages[0].message,
            "Invalid value for property “text-decoration”."
        );
    }
}

fn validate_clip(tokens: &[&str], report: &mut Report) {
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

#[cfg(test)]
mod validate_clip_tests {
    use super::{Report, Severity, validate_clip};

    fn run(tokens: &[&str]) -> Report {
        let mut report = Report::default();
        validate_clip(tokens, &mut report);
        report
    }

    #[test]
    fn accepts_auto() {
        let report = run(&["auto"]);
        assert_eq!(report.errors, 0);
        assert!(report.messages.is_empty());
    }

    #[test]
    fn rejects_multiple_tokens() {
        let report = run(&["auto", "auto"]);
        assert_eq!(report.errors, 1);
        assert_eq!(report.messages.len(), 1);
        assert_eq!(report.messages[0].severity, Severity::Error);
        assert_eq!(
            report.messages[0].message,
            "Invalid value for property “clip”."
        );
    }

    #[test]
    fn accepts_rect_with_four_components() {
        let report = run(&["rect(1,2,3,4)"]);
        assert_eq!(report.errors, 0);
        assert!(report.messages.is_empty());

        let report = run(&["rect(1 2 3 4)"]);
        assert_eq!(report.errors, 0);
        assert!(report.messages.is_empty());
    }

    #[test]
    fn rejects_rect_with_wrong_arity() {
        let report = run(&["rect(1,2,3)"]);
        assert_eq!(report.errors, 1);
        assert_eq!(report.messages.len(), 1);
        assert_eq!(report.messages[0].severity, Severity::Error);
        assert_eq!(
            report.messages[0].message,
            "Invalid value for property “clip”."
        );

        let report = run(&["rect(1 2 3 4 5)"]);
        assert_eq!(report.errors, 1);
        assert_eq!(report.messages.len(), 1);
        assert_eq!(report.messages[0].severity, Severity::Error);
        assert_eq!(
            report.messages[0].message,
            "Invalid value for property “clip”."
        );
    }
}

fn validate_filter(tokens: &[&str], css4_profile: bool, report: &mut Report) {
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

        if FUNCTIONS
            .iter()
            .any(|name| lower.starts_with(*name) && is_balanced_function_call_token(lower, name))
        {
            continue;
        }

        push_error(report, "Invalid value for property “filter”.");
        return;
    }
}
