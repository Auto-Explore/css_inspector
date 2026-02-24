use super::*;

// ---------------------------------------------------------------------------
// Valid CSS baseline
// ---------------------------------------------------------------------------

#[test]
fn valid_css_produces_no_errors_or_warnings() {
    let css = r#"
body {
  color: red;
  font-size: 14px;
  margin: 0 auto;
  background-color: #fff;
}
h1 {
  font-weight: bold;
  text-decoration: underline;
}
a:hover {
  color: blue;
  opacity: 0.8;
}
@media screen and (max-width: 768px) {
  body { font-size: 12px; }
}
"#;
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert!(report.messages.is_empty(), "{report:?}");
}

// ---------------------------------------------------------------------------
// Comment errors
// ---------------------------------------------------------------------------

#[test]
fn unclosed_comment_reports_error() {
    let css = "/* This comment is never closed\nbody { color: red; }";
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Unclosed comment."
    );
}

#[test]
fn unclosed_comment_swallows_subsequent_errors() {
    // When a comment is never closed the rest of the input is consumed as
    // comment text, so downstream errors are not reachable.
    let css = r#"/* unclosed comment

body {
  colour: red;
  font-size abc;
  margin: 10zz;
}

color: blue;

@foobar { }

h1[broken {
  padding: ;
}

a {
  z-index: hello;
"#;
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Unclosed comment."
    );
}

// ---------------------------------------------------------------------------
// Brace balance
// ---------------------------------------------------------------------------

#[test]
fn unbalanced_braces_reports_error() {
    let css = "body { color: red;\na { font-size: 12px; } } }";
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Unbalanced braces."
    );
}

// ---------------------------------------------------------------------------
// Unknown properties (css3 profile - strict)
// ---------------------------------------------------------------------------

#[test]
fn css3_profile_reports_multiple_misspelled_properties() {
    let css = r#"h1 {
  colour: red;
  bckground: blue;
  fonnt-size: 14px;
  txt-decoration: underline;
}"#;
    let config = Config {
        profile: Some("css3".to_string()),
        ..Config::default()
    };
    let report = validate_css_text(css, &config).unwrap();
    assert_eq!(report.errors, 4, "{report:?}");

    let got: std::collections::BTreeSet<String> =
        report.messages.iter().map(|m| m.message.clone()).collect();
    let expected: std::collections::BTreeSet<String> = [
        "Unknown property \u{201c}colour\u{201d}.",
        "Unknown property \u{201c}bckground\u{201d}.",
        "Unknown property \u{201c}fonnt-size\u{201d}.",
        "Unknown property \u{201c}txt-decoration\u{201d}.",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();
    assert_eq!(got, expected, "{report:?}");
}

#[test]
fn css4_profile_silently_accepts_unknown_properties_by_default() {
    let css = r#"h1 {
  colour: red;
  bckground: blue;
  fonnt-size: 14px;
  txt-decoration: underline;
}"#;
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert!(report.messages.is_empty(), "{report:?}");
}

// ---------------------------------------------------------------------------
// Invalid property values
// ---------------------------------------------------------------------------

#[test]
fn invalid_color_value_reports_error() {
    let css = "body { color: notacolor; }";
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Invalid value for property \u{201c}color\u{201d}."
    );
}

#[test]
fn invalid_float_value_reports_error() {
    let css = "div { float: center; }";
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Invalid value for property \u{201c}float\u{201d}."
    );
}

#[test]
fn invalid_border_style_value_reports_error() {
    let css = "div { border-style: wiggly; }";
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Invalid value for property \u{201c}border-style\u{201d}."
    );
}

// ---------------------------------------------------------------------------
// Declaration syntax errors
// ---------------------------------------------------------------------------

#[test]
fn missing_colon_in_declarations_reports_errors() {
    let css = "p { color red; font-size 14px; }";
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 2, "{report:?}");
    assert!(
        report
            .messages
            .iter()
            .all(|m| m.message == "Missing ':' in declaration."),
        "{report:?}"
    );
}

#[test]
fn missing_property_values_report_errors() {
    let css = "h1 { color: ; font-size: ; margin: ; }";
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 3, "{report:?}");

    let got: std::collections::BTreeSet<String> =
        report.messages.iter().map(|m| m.message.clone()).collect();
    let expected: std::collections::BTreeSet<String> = [
        "Missing value for property \u{201c}color\u{201d}.",
        "Missing value for property \u{201c}font-size\u{201d}.",
        "Missing value for property \u{201c}margin\u{201d}.",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();
    assert_eq!(got, expected, "{report:?}");
}

#[test]
fn missing_semicolons_between_declarations_reports_errors() {
    let css = "h1 { color: red\n  font-size: 14px\n  margin: 0\n}";
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 2, "{report:?}");
    assert!(
        report
            .messages
            .iter()
            .all(|m| m.message == "Missing ';' between declarations."),
        "{report:?}"
    );
}

// ---------------------------------------------------------------------------
// Stray declarations at top level
// ---------------------------------------------------------------------------

#[test]
fn stray_declarations_outside_rules_report_errors() {
    let css = "color: red;\nfont-size: 14px;\nbody { margin: 0; }\nbackground: blue;";
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert!(report.errors >= 3, "expected at least 3 stray declaration errors: {report:?}");
    assert!(
        report
            .messages
            .iter()
            .filter(|m| m.message == "Stray declaration outside a rule.")
            .count()
            >= 3,
        "{report:?}"
    );
}

// ---------------------------------------------------------------------------
// Invalid selectors
// ---------------------------------------------------------------------------

#[test]
fn css3_profile_rejects_bogus_pseudo_element() {
    let css = "a::nonexistent-pseudo-element { color: red; }";
    let config = Config {
        profile: Some("css3".to_string()),
        ..Config::default()
    };
    let report = validate_css_text(css, &config).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Invalid selector."
    );
}

#[test]
fn css3_profile_rejects_bogus_pseudo_class() {
    let css = "div:bogus-pseudo-class { margin: 0; }";
    let config = Config {
        profile: Some("css3".to_string()),
        ..Config::default()
    };
    let report = validate_css_text(css, &config).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Invalid selector."
    );
}

// ---------------------------------------------------------------------------
// Unknown at-rules
// ---------------------------------------------------------------------------

#[test]
fn css3_profile_reports_unknown_at_rule() {
    let css = "@foobar { body { color: red; } }";
    let config = Config {
        profile: Some("css3".to_string()),
        ..Config::default()
    };
    let report = validate_css_text(css, &config).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Unknown at-rule."
    );
}

#[test]
fn css4_profile_silently_accepts_unknown_at_rules_by_default() {
    let css = "@foobar { body { color: red; } }\n@nonsense url(\"test.css\");";
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert!(report.messages.is_empty(), "{report:?}");
}

// ---------------------------------------------------------------------------
// Trailing escape
// ---------------------------------------------------------------------------

#[test]
fn trailing_backslash_reports_invalid_escape() {
    let css = "body { color: red; }\\";
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Invalid escape at end of input."
    );
}

// ---------------------------------------------------------------------------
// Warnings
// ---------------------------------------------------------------------------

#[test]
fn conflicting_attribute_selectors_produce_warning() {
    let css = r#"span[hello="Cleveland"][hello="Columbus"] { color: red; }"#;
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Conflicting attribute selector constraints."
    );
}

#[test]
fn css3_import_without_fetcher_produces_warning() {
    let css = "@import url(\"other.css\");\nbody { color: red; }";
    let config = Config {
        profile: Some("css3".to_string()),
        ..Config::default()
    };
    let report = validate_css_text(css, &config).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Imported style sheets are not checked."
    );
}

#[test]
fn import_warning_suppressed_by_negative_warning_level() {
    let css = "@import url(\"other.css\");\nbody { color: red; }";
    let config = Config {
        profile: Some("css3".to_string()),
        warning: Some("-1".to_string()),
        ..Config::default()
    };
    let report = validate_css_text(css, &config).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert!(report.messages.is_empty(), "{report:?}");
}
