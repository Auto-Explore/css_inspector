use std::borrow::Cow;

use crate::config::{Config, css1_escapes_from_config, warning_level_from_config};
use crate::declarations::validate_declarations;
use crate::errors::ValidatorError;
use crate::imports::has_top_level_import_url;
use crate::known_properties::known_properties_for_config;
use crate::parser::{
    RuleBlockKind, at_rule_decl_list_context_flags, contains_invalid_top_level_chars,
    contains_unknown_at_rule, count_brace_balance_errors, count_stray_top_level_declaration_errors,
    ends_with_stray_backslash, iter_rule_blocks, strip_css_comments, warn_on_other_media_rules,
};
use crate::report::{Report, push_error, push_error_times, push_warning_level};
use crate::selectors::{
    selector_pseudo_version_from_config, validate_selector_prelude,
    warn_on_conflicting_attribute_selectors,
};

#[cfg(test)]
use crate::report::Severity;

#[cfg(test)]
use crate::{Fetcher, validate_css_text_with_fetcher};

pub fn validate_css_text(css: &str, config: &Config) -> Result<Report, ValidatorError> {
    validate_css_text_internal(css, config, true)
}

pub(crate) fn validate_css_text_stripped(
    css: &str,
    config: &Config,
    warn_on_top_level_imports: bool,
    report: &mut Report,
) {
    let warning_level = warning_level_from_config(config);
    let pseudo_version = selector_pseudo_version_from_config(config);

    // The upstream autotest suite treats top-level `@import` as producing a single warning in
    // text-validation mode (i.e., when not validating via URI with fetching enabled).
    if warn_on_top_level_imports && has_top_level_import_url(css) {
        push_warning_level(
            report,
            warning_level,
            0,
            "Imported style sheets are not checked.",
        );
    }

    let known_properties = known_properties_for_config(config);
    let css1_escapes = css1_escapes_from_config(config);
    let css4_profile =
        matches!(config.profile.as_deref(), Some(p) if p.eq_ignore_ascii_case("css4"));

    warn_on_other_media_rules(css, config, warning_level, report);

    let brace_errors = count_brace_balance_errors(css);
    push_error_times(report, "Unbalanced braces.", brace_errors);
    if contains_invalid_top_level_chars(css) {
        push_error(report, "Invalid input.");
    }
    if ends_with_stray_backslash(css) {
        push_error(report, "Invalid escape at end of input.");
    }
    if contains_unknown_at_rule(css) {
        push_error(report, "Unknown at-rule.");
    }
    let stray_decl_errors = count_stray_top_level_declaration_errors(css);
    push_error_times(
        report,
        "Stray declaration outside a rule.",
        stray_decl_errors,
    );

    for block in iter_rule_blocks(css) {
        if block.kind == RuleBlockKind::QualifiedRule {
            validate_selector_prelude(block.prelude, pseudo_version, warning_level, report);
            warn_on_conflicting_attribute_selectors(block.prelude, warning_level, report);
        }
        let (
            in_page_at_rule,
            in_font_face_at_rule,
            in_property_at_rule,
            in_font_palette_values_at_rule,
            in_counter_style_at_rule,
            in_color_profile_at_rule,
            in_view_transition_at_rule,
        ) =
            at_rule_decl_list_context_flags(block.kind, block.prelude);
        validate_declarations(
            block.body,
            known_properties,
            warning_level,
            css1_escapes,
            in_page_at_rule,
            in_font_face_at_rule,
            in_property_at_rule,
            in_font_palette_values_at_rule,
            in_counter_style_at_rule,
            in_color_profile_at_rule,
            in_view_transition_at_rule,
            css4_profile,
            report,
        );
    }
}

pub(crate) fn strip_comments_or_push_error<'a>(
    input: &'a str,
    report: &mut Report,
) -> Option<Cow<'a, str>> {
    strip_comments_or_push_error_with(input, report, "Unclosed comment.")
}

pub(crate) fn strip_comments_or_push_error_with<'a>(
    input: &'a str,
    report: &mut Report,
    message: &'static str,
) -> Option<Cow<'a, str>> {
    let (out, ok) = strip_css_comments(input);
    if !ok {
        push_error(report, message);
        return None;
    }
    Some(out)
}

#[cfg(test)]
mod strip_comments_or_push_error_tests {
    use std::borrow::Cow;

    use super::{
        Report, Severity, strip_comments_or_push_error, strip_comments_or_push_error_with,
    };

    #[test]
    fn returns_some_when_comments_strip_successfully() {
        let mut report = Report::default();
        let out = strip_comments_or_push_error("a { color: red }", &mut report);
        assert!(matches!(out, Some(Cow::Borrowed(_))));
        assert_eq!(report.errors, 0);
        assert!(report.messages.is_empty());
    }

    #[test]
    fn returns_owned_when_comment_stripping_changes_input() {
        let mut report = Report::default();
        let out = strip_comments_or_push_error("a/*x*/b", &mut report);
        assert!(matches!(out, Some(Cow::Owned(_))));
        assert_eq!(out.unwrap().as_ref(), "a b");
        assert_eq!(report.errors, 0);
        assert!(report.messages.is_empty());
    }

    #[test]
    fn returns_none_and_pushes_error_on_unclosed_comment() {
        let mut report = Report::default();
        let out = strip_comments_or_push_error("a{/*", &mut report);
        assert!(out.is_none());
        assert_eq!(report.errors, 1);
        assert_eq!(report.messages.len(), 1);
        assert_eq!(report.messages[0].severity, Severity::Error);
        assert_eq!(report.messages[0].message, "Unclosed comment.");
    }

    #[test]
    fn custom_message_is_used_when_provided() {
        let mut report = Report::default();
        let out = strip_comments_or_push_error_with("a{/*", &mut report, "Custom.");
        assert!(out.is_none());
        assert_eq!(report.errors, 1);
        assert_eq!(report.messages.len(), 1);
        assert_eq!(report.messages[0].message, "Custom.");
    }

    #[test]
    fn custom_message_is_ignored_on_success() {
        let mut report = Report::default();
        let out = strip_comments_or_push_error_with("a { color: red }", &mut report, "Custom.");
        assert!(out.is_some());
        assert_eq!(report.errors, 0);
        assert!(report.messages.is_empty());
    }
}

fn validate_css_text_internal(
    css: &str,
    config: &Config,
    warn_on_top_level_imports: bool,
) -> Result<Report, ValidatorError> {
    let mut report = Report::default();
    let Some(stripped) = strip_comments_or_push_error(css, &mut report) else {
        return Ok(report);
    };
    validate_css_text_stripped(
        stripped.as_ref(),
        config,
        warn_on_top_level_imports,
        &mut report,
    );

    Ok(report)
}

/// Validate a CSS declaration list (e.g. the contents of an HTML `style=""` attribute).
///
/// This is intentionally conservative and exists primarily for embedding the Rust CSS validator
/// into the Rust HTML validator.
pub fn validate_css_declarations_text(
    decls: &str,
    config: &Config,
) -> Result<Report, ValidatorError> {
    let mut report = Report::default();
    let Some(stripped) = strip_comments_or_push_error(decls, &mut report) else {
        return Ok(report);
    };
    let stripped = stripped.as_ref();
    let known_properties = known_properties_for_config(config);
    let warning_level = warning_level_from_config(config);
    let css1_escapes = css1_escapes_from_config(config);
    let css4_profile =
        matches!(config.profile.as_deref(), Some(p) if p.eq_ignore_ascii_case("css4"));
    validate_declarations(
        stripped,
        known_properties,
        warning_level,
        css1_escapes,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        css4_profile,
        &mut report,
    );
    Ok(report)
}

#[cfg(test)]
mod unclosed_comment_entrypoint_tests {
    use super::{
        Config, Fetcher, Severity, ValidatorError, validate_css_declarations_text,
        validate_css_text, validate_css_text_with_fetcher,
    };

    #[test]
    fn validate_css_text_reports_unclosed_comment() {
        let report = validate_css_text("a{/*", &Config::default()).unwrap();
        assert_eq!(report.errors, 1);
        assert_eq!(report.warnings, 0);
        assert_eq!(report.messages.len(), 1);
        assert_eq!(report.messages[0].severity, Severity::Error);
        assert_eq!(report.messages[0].message, "Unclosed comment.");
    }

    #[test]
    fn validate_css_declarations_text_reports_unclosed_comment() {
        let report = validate_css_declarations_text("color: red; /*", &Config::default()).unwrap();
        assert_eq!(report.errors, 1);
        assert_eq!(report.warnings, 0);
        assert_eq!(report.messages.len(), 1);
        assert_eq!(report.messages[0].severity, Severity::Error);
        assert_eq!(report.messages[0].message, "Unclosed comment.");
    }

    #[test]
    fn validate_css_text_with_fetcher_reports_unclosed_comment_without_fetching() {
        struct PanicFetcher;
        impl Fetcher for PanicFetcher {
            fn fetch(&self, _uri: &str) -> Result<Vec<u8>, ValidatorError> {
                panic!("fetch should not be called");
            }
        }

        let report =
            validate_css_text_with_fetcher("a{/*", None, &Config::default(), &PanicFetcher)
                .unwrap();
        assert_eq!(report.errors, 1);
        assert_eq!(report.messages.len(), 1);
        assert_eq!(report.messages[0].message, "Unclosed comment.");
    }
}

#[cfg(test)]
mod syntax_error_regression_tests {
    use crate::{Config, validate_css_text};

    #[test]
    fn reports_errors_for_stray_garbage_between_rules() {
        let css = r#"b {text-decoration: underline solid black;}
/* .disabled-attribute{display: block;} */   %
!
)
/* .another > .disabled > .attribute {box-shadow: 10px 10px 20px RGBA(255,255,255,0);border: 1px solid hsla(0,0%,100%,.2) !important;} */

span.greentext {color: green;} "#;

        let report = validate_css_text(css, &Config::default()).unwrap();

        assert!(!report.valid(), "{report:?}");
        assert!(report.errors > 0, "{report:?}");
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.message == "Invalid selector."),
            "{report:?}"
        );
    }
}
