use std::collections::HashSet;

use crate::known_properties::KnownProperties;
use crate::report::{Report, push_error, push_warning_level};

mod at_rule_descriptors;
mod background;
mod border;
mod borders;
mod colors;
mod fonts;
mod grid;
mod misc;
mod numbers;
mod syntax;
#[cfg(test)]
mod tests;
mod url_function;
mod wide_keywords;

use at_rule_descriptors::{
    is_color_profile_descriptor, is_counter_style_descriptor, is_font_face_descriptor,
    is_font_palette_values_descriptor, is_page_descriptor, is_property_descriptor,
    is_scroll_timeline_descriptor, is_view_timeline_descriptor, is_view_transition_descriptor,
    validate_color_profile_descriptor_rendering_intent, validate_property_descriptor_inherits,
    validate_property_descriptor_syntax, validate_view_transition_descriptor_navigation,
};
use background::{
    is_any_percentage_token, is_background_image_like_token, validate_background,
    validate_background_image,
};
use border::{BorderRedefinitionTracker, track_border_redefinitions};
#[cfg(test)]
pub(crate) use border::{border_side_component_flags, for_each_affected_border_longhand};
use borders::{
    is_border_style_token, is_border_width_token, is_length_token, validate_border_color_aggregate,
    validate_border_shorthand, validate_border_style_aggregate, validate_border_width_aggregate,
    validate_outline, validate_outline_style, validate_outline_width,
};
use colors::{
    is_valid_color_token, validate_background_color, validate_border_side_color, validate_color,
    validate_outline_color,
};
#[cfg(test)]
use colors::{is_valid_hsl_like_function, is_valid_rgb_like_function};
use fonts::{validate_font, validate_font_family, validate_font_optical_sizing};
use grid::validate_grid_template_tracks;
#[cfg(test)]
use misc::{is_css_identifier_token, is_integer_token};
use misc::{
    validate_aspect_ratio, validate_azimuth, validate_clip, validate_content,
    validate_counter_list, validate_cue, validate_cue_side, validate_cursor, validate_elevation,
    validate_filter, validate_float, validate_list_style, validate_overflow_clip_margin,
    validate_pause, validate_pause_after, validate_play_during, validate_quotes,
    validate_text_decoration, validate_voice_family, validate_zoom,
};
use numbers::{parse_css_number, scan_css_number_end, split_number_and_unit};
use syntax::{
    contains_var_or_env_outside_strings, find_embedded_declaration_start,
    is_balanced_function_call_token, is_property_ident_char,
    is_single_top_level_curly_block_with_var_or_env, is_single_valued_property,
    is_valid_css_value_syntax, is_valid_property_name, is_vendor_extension_property,
    iter_declaration_statements_skipping_nested_blocks, normalize_property_name,
    split_top_level_value_tokens, strip_important, unescape_css_identifier_css1_compat,
    unescape_css_identifier_greedy, validate_max_tokens, validate_single_token,
};
pub(crate) use url_function::is_valid_url_function_token;
use wide_keywords::has_css_wide_keyword_mixed;
pub(crate) use wide_keywords::is_css_wide_keyword;
#[cfg(test)]
pub(crate) use wide_keywords::is_css_wide_keywordish_token;

#[cfg(test)]
use crate::{Config, validate_css_declarations_text};

#[cfg(test)]
use crate::report::Severity;

pub(crate) fn validate_declarations(
    block: &str,
    known_properties: &KnownProperties,
    settings: DeclValidationSettings,
    at_rule_context: DeclAtRuleContext,
    report: &mut Report,
) {
    let mut v = DeclValidator {
        known_properties,
        warning_level: settings.warning_level,
        css1_escapes: settings.css1_escapes,
        css4_profile: settings.css4_profile,
        lenient: settings.lenient,
        report,
        redef: BorderRedefinitionTracker::default(),
        ctx: DeclContext {
            in_page_at_rule: at_rule_context.in_page_at_rule,
            in_font_face_at_rule: at_rule_context.in_font_face_at_rule,
            in_property_at_rule: at_rule_context.in_property_at_rule,
            in_font_palette_values_at_rule: at_rule_context.in_font_palette_values_at_rule,
            in_counter_style_at_rule: at_rule_context.in_counter_style_at_rule,
            in_color_profile_at_rule: at_rule_context.in_color_profile_at_rule,
            in_view_transition_at_rule: at_rule_context.in_view_transition_at_rule,
            in_scroll_timeline_at_rule: at_rule_context.in_scroll_timeline_at_rule,
            in_view_timeline_at_rule: at_rule_context.in_view_timeline_at_rule,
            warned_pagebreak_too_many_values: false,
        },
        unknown_reported: HashSet::new(),
    };

    for raw in iter_declaration_statements_skipping_nested_blocks(block, known_properties) {
        v.validate_raw_declaration(raw);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct DeclValidationSettings {
    pub(crate) warning_level: i32,
    pub(crate) css1_escapes: bool,
    pub(crate) css4_profile: bool,
    pub(crate) lenient: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct DeclAtRuleContext {
    pub(crate) in_page_at_rule: bool,
    pub(crate) in_font_face_at_rule: bool,
    pub(crate) in_property_at_rule: bool,
    pub(crate) in_font_palette_values_at_rule: bool,
    pub(crate) in_counter_style_at_rule: bool,
    pub(crate) in_color_profile_at_rule: bool,
    pub(crate) in_view_transition_at_rule: bool,
    pub(crate) in_scroll_timeline_at_rule: bool,
    pub(crate) in_view_timeline_at_rule: bool,
}

struct DeclContext {
    in_page_at_rule: bool,
    in_font_face_at_rule: bool,
    in_property_at_rule: bool,
    in_font_palette_values_at_rule: bool,
    in_counter_style_at_rule: bool,
    in_color_profile_at_rule: bool,
    in_view_transition_at_rule: bool,
    in_scroll_timeline_at_rule: bool,
    in_view_timeline_at_rule: bool,
    warned_pagebreak_too_many_values: bool,
}

struct DeclValidator<'a> {
    known_properties: &'a KnownProperties,
    warning_level: i32,
    css1_escapes: bool,
    css4_profile: bool,
    lenient: bool,
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
            if raw_trimmed.starts_with('@') {
                // At-rules are permitted syntactically within declaration lists. They are
                // validated separately (or ignored) by higher-level rule parsing.
                return;
            }
            let Some((prop_raw, mut value_raw)) = raw_trimmed.split_once(':') else {
                push_error(self.report, "Missing ':' in declaration.");
                return;
            };
            let prop_raw = prop_raw.trim();
            if prop_raw.is_empty() {
                push_error(self.report, "Missing property name in declaration.");
                return;
            }
            if !is_valid_property_name(prop_raw) {
                push_error(self.report, "Invalid property name in declaration.");
                return;
            }
            let prop = normalize_property_name(prop_raw, self.css1_escapes);
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
        let is_font_palette_values_desc =
            self.ctx.in_font_palette_values_at_rule && is_font_palette_values_descriptor(prop);
        let is_counter_style_desc =
            self.ctx.in_counter_style_at_rule && is_counter_style_descriptor(prop);
        let is_color_profile_desc =
            self.ctx.in_color_profile_at_rule && is_color_profile_descriptor(prop);
        let is_view_transition_desc =
            self.ctx.in_view_transition_at_rule && is_view_transition_descriptor(prop);
        let is_scroll_timeline_desc =
            self.ctx.in_scroll_timeline_at_rule && is_scroll_timeline_descriptor(prop);
        let is_view_timeline_desc =
            self.ctx.in_view_timeline_at_rule && is_view_timeline_descriptor(prop);
        let is_custom_property = prop.starts_with("--");

        if prop.is_empty() {
            push_error(self.report, "Missing property name in declaration.");
            return;
        }

        if !self.lenient && !is_custom_property && !self.known_properties.contains(prop) {
            if is_font_face_desc
                || is_page_desc
                || is_property_desc
                || is_font_palette_values_desc
                || is_counter_style_desc
                || is_color_profile_desc
                || is_view_transition_desc
                || is_scroll_timeline_desc
                || is_view_timeline_desc
            {
                // Allowed descriptor within an at-rule descriptor list.
            } else {
                // Keep error counts closer to the upstream validator by reporting each unknown
                // property name at most once per declaration block.
                if self.unknown_reported.insert(prop.to_owned()) {
                    // For vnu.jar parity (Assertions.java): vendor extensions are treated as
                    // warnings and vnu.jar disables warnings by default (warningLevel=-1), so
                    // suppress these by demoting them to warnings only when warnings are disabled.
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
            if is_custom_property {
                // Custom properties can be empty token streams.
                return;
            }
            if is_property_desc && prop.eq_ignore_ascii_case("initial-value") && self.css4_profile {
                // CSS Properties and Values API: empty `initial-value` is treated as a single
                // space character (wpt/css-properties-values-api/at-property-optional-initial-value.html).
                return;
            }
            push_error(self.report, format!("Missing value for property “{prop}”."));
            return;
        }
        if !is_custom_property && !is_valid_css_value_syntax(value, self.css4_profile) {
            push_error(self.report, format!("Invalid value for property “{prop}”."));
            return;
        }

        // CSS values may contain curly-brace blocks as component values. For standard properties,
        // browsers accept a single `{...}` block token when it contains `var()`/`env()`, deferring
        // full validation until after substitution.
        if !is_custom_property
            && self.css4_profile
            && is_single_top_level_curly_block_with_var_or_env(value)
        {
            return;
        }

        // Common rules.
        let tokens = split_top_level_value_tokens(value);
        if tokens.is_empty() {
            if is_custom_property {
                // Custom properties can be arbitrary token streams (including comma-only values).
                return;
            }
            if is_property_desc && prop.eq_ignore_ascii_case("initial-value") && self.css4_profile {
                return;
            }
            push_error(self.report, format!("Missing value for property “{prop}”."));
            return;
        }
        let skip_css_wide_keyword_mixed =
            !is_custom_property && self.css4_profile && matches!(prop, "font" | "font-family");
        if !skip_css_wide_keyword_mixed
            && !is_custom_property
            && has_css_wide_keyword_mixed(&tokens)
        {
            push_error(self.report, format!("Invalid value for property “{prop}”."));
            return;
        }

        if self.lenient && !is_custom_property && !self.known_properties.contains(prop) {
            if is_font_face_desc
                || is_page_desc
                || is_property_desc
                || is_font_palette_values_desc
                || is_counter_style_desc
                || is_color_profile_desc
                || is_view_transition_desc
                || is_scroll_timeline_desc
                || is_view_timeline_desc
            {
                // Allowed descriptor within an at-rule descriptor list.
            } else {
                // Keep report size closer to upstream behavior by reporting each unknown property
                // name at most once per declaration block.
                if self.unknown_reported.insert(prop.to_owned()) {
                    if is_vendor_extension_property(prop) {
                        push_warning_level(
                            self.report,
                            self.warning_level,
                            1,
                            format!("“{prop}” is a vendor extension."),
                        );
                    } else {
                        push_warning_level(
                            self.report,
                            self.warning_level,
                            1,
                            format!("Unknown property “{prop}”."),
                        );
                    }
                }
                return;
            }
        }

        if !is_custom_property
            && self.css4_profile
            && contains_var_or_env_outside_strings(value)
            && !value.contains('{')
            && !value.contains('}')
        {
            // In CSS4/WPT mode, values containing substitution functions (e.g. `var()`/`env()`)
            // are treated as potentially valid until substitution happens.
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

        if prop == "overflow-clip-margin" && !self.lenient {
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
            "float" => validate_float(tokens.as_slice(), self.css4_profile, self.report),
            "color" => validate_color(
                tokens.as_slice(),
                self.css1_escapes,
                self.lenient,
                self.report,
            ),
            "background-color" => validate_background_color(
                tokens.as_slice(),
                self.css1_escapes,
                self.lenient,
                self.report,
            ),
            "aspect-ratio" => validate_aspect_ratio(value, self.report),
            "background" => validate_background(
                tokens.as_slice(),
                self.css1_escapes,
                self.css4_profile,
                self.lenient,
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
            "font" => validate_font(value, self.css4_profile, self.report),
            "font-family" => validate_font_family(value, self.css4_profile, self.report),
            "font-optical-sizing" => validate_font_optical_sizing(tokens.as_slice(), self.report),
            "border" => validate_border_shorthand(
                tokens.as_slice(),
                self.css1_escapes,
                self.lenient,
                self.report,
            ),
            "border-top" | "border-right" | "border-bottom" | "border-left" => {
                validate_border_shorthand(
                    tokens.as_slice(),
                    self.css1_escapes,
                    self.lenient,
                    self.report,
                )
            }
            "border-width" => validate_border_width_aggregate(tokens.as_slice(), self.report),
            "border-style" => validate_border_style_aggregate(tokens.as_slice(), self.report),
            "border-color" => validate_border_color_aggregate(
                tokens.as_slice(),
                self.css1_escapes,
                self.lenient,
                self.report,
            ),
            "border-top-color"
            | "border-right-color"
            | "border-bottom-color"
            | "border-left-color" => validate_border_side_color(
                tokens.as_slice(),
                self.css1_escapes,
                self.lenient,
                self.report,
            ),
            "outline-color" => validate_outline_color(
                tokens.as_slice(),
                self.css1_escapes,
                self.css4_profile,
                self.lenient,
                self.report,
            ),
            "outline-style" => {
                validate_outline_style(tokens.as_slice(), self.css4_profile, self.report)
            }
            "outline-width" => validate_outline_width(tokens.as_slice(), self.report),
            "outline" => validate_outline(
                tokens.as_slice(),
                self.css1_escapes,
                self.css4_profile,
                self.lenient,
                self.report,
            ),
            "margin" | "padding" => validate_max_tokens(tokens.as_slice(), 4, prop, self.report),
            "border-spacing" => {
                validate_max_tokens(tokens.as_slice(), 2, "border-spacing", self.report)
            }
            "list-style" => validate_list_style(tokens.as_slice(), self.lenient, self.report),
            "text-decoration" => validate_text_decoration(
                tokens.as_slice(),
                self.css1_escapes,
                self.css4_profile,
                self.lenient,
                self.report,
            ),
            "clip" => validate_clip(tokens.as_slice(), self.report),
            "cursor" => validate_cursor(tokens.as_slice(), self.css4_profile, self.report),
            "content" => validate_content(
                tokens.as_slice(),
                self.css4_profile,
                self.lenient,
                self.report,
            ),
            "quotes" => validate_quotes(tokens.as_slice(), self.report),
            "grid-template-columns" | "grid-template-rows" => {
                validate_grid_template_tracks(value, prop, self.lenient, self.report)
            }
            "counter-increment" => validate_counter_list(
                tokens.as_slice(),
                "counter-increment",
                self.css4_profile,
                self.report,
            ),
            "counter-reset" => validate_counter_list(
                tokens.as_slice(),
                "counter-reset",
                self.css4_profile,
                self.report,
            ),
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
            && !is_scroll_timeline_desc
            && !is_view_timeline_desc
            && !is_custom_property
        {
            push_error(self.report, format!("Invalid value for property “{prop}”."));
        }
    }
}
