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
    assert_eq!(report.messages[0].message, "Unclosed comment.");
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
    assert_eq!(report.messages[0].message, "Unclosed comment.");
}

// ---------------------------------------------------------------------------
// Brace balance
// ---------------------------------------------------------------------------

#[test]
fn unbalanced_braces_reports_error() {
    let css = "body { color: red;\na { font-size: 12px; } } }";
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(report.messages[0].message, "Unbalanced braces.");
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
    assert!(
        report.errors >= 3,
        "expected at least 3 stray declaration errors: {report:?}"
    );
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
    assert_eq!(report.messages[0].message, "Invalid selector.");
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
    assert_eq!(report.messages[0].message, "Invalid selector.");
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
    assert_eq!(report.messages[0].message, "Unknown at-rule.");
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

// ---------------------------------------------------------------------------
// Cursor keyword fixes (CSS UI 3/4)
// ---------------------------------------------------------------------------

#[test]
fn css3_profile_accepts_all_css3_cursor_keywords() {
    let keywords = [
        "auto",
        "default",
        "none",
        "context-menu",
        "help",
        "pointer",
        "progress",
        "wait",
        "cell",
        "crosshair",
        "text",
        "vertical-text",
        "alias",
        "copy",
        "move",
        "no-drop",
        "not-allowed",
        "grab",
        "grabbing",
        "e-resize",
        "n-resize",
        "ne-resize",
        "nw-resize",
        "s-resize",
        "se-resize",
        "sw-resize",
        "w-resize",
        "ew-resize",
        "ns-resize",
        "nesw-resize",
        "nwse-resize",
        "col-resize",
        "row-resize",
        "all-scroll",
        "zoom-in",
        "zoom-out",
    ];
    let config = Config {
        profile: Some("css3".to_string()),
        ..Config::default()
    };
    for kw in keywords {
        let css = format!("a {{ cursor: {kw}; }}");
        let report = validate_css_text(&css, &config).unwrap();
        assert_eq!(
            report.errors, 0,
            "cursor: {kw} should be valid in css3: {report:?}"
        );
    }
}

#[test]
fn cursor_rejects_nonstandard_keywords() {
    let config = Config {
        profile: Some("css4".to_string()),
        ..Config::default()
    };
    for kw in [
        "n-all-scroll",
        "s-all-scroll",
        "e-all-scroll",
        "w-all-scroll",
    ] {
        let css = format!("a {{ cursor: {kw}; }}");
        let report = validate_css_text(&css, &config).unwrap();
        assert_eq!(
            report.errors, 1,
            "cursor: {kw} should be rejected: {report:?}"
        );
    }
}

#[test]
fn cursor_rejects_invalid_keyword() {
    let css = "a { cursor: banana; }";
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Invalid value for property “cursor”."
    );
}

// ---------------------------------------------------------------------------
// outline-color: invert (removed in CSS UI 4)
// ---------------------------------------------------------------------------

#[test]
fn css4_profile_rejects_outline_color_invert() {
    let css = "a { outline-color: invert; }";
    let config = Config {
        profile: Some("css4".to_string()),
        ..Config::default()
    };
    let report = validate_css_text(css, &config).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Invalid value for property “outline-color”."
    );
}

#[test]
fn css3_profile_accepts_outline_color_invert() {
    let css = "a { outline-color: invert; }";
    let config = Config {
        profile: Some("css3".to_string()),
        ..Config::default()
    };
    let report = validate_css_text(css, &config).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
}

// ---------------------------------------------------------------------------
// resize
// ---------------------------------------------------------------------------

#[test]
fn resize_accepts_valid_values() {
    let config = Config::default();
    for kw in ["none", "both", "horizontal", "vertical"] {
        let css = format!("a {{ resize: {kw}; }}");
        let report = validate_css_text(&css, &config).unwrap();
        assert_eq!(report.errors, 0, "resize: {kw} should be valid: {report:?}");
    }
}

#[test]
fn resize_block_inline_css4_only() {
    let css3 = Config {
        profile: Some("css3".to_string()),
        ..Config::default()
    };
    let css4 = Config {
        profile: Some("css4".to_string()),
        ..Config::default()
    };
    for kw in ["block", "inline"] {
        let css = format!("a {{ resize: {kw}; }}");
        let r3 = validate_css_text(&css, &css3).unwrap();
        assert_eq!(
            r3.errors, 1,
            "resize: {kw} should be invalid in css3: {r3:?}"
        );
        let r4 = validate_css_text(&css, &css4).unwrap();
        assert_eq!(r4.errors, 0, "resize: {kw} should be valid in css4: {r4:?}");
    }
}

#[test]
fn resize_rejects_invalid_value() {
    let css = "a { resize: diagonal; }";
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Invalid value for property “resize”."
    );
}

// ---------------------------------------------------------------------------
// user-select
// ---------------------------------------------------------------------------

#[test]
fn user_select_accepts_valid_values() {
    for kw in ["auto", "text", "none", "contain", "all"] {
        let css = format!("a {{ user-select: {kw}; }}");
        let report = validate_css_text(&css, &Config::default()).unwrap();
        assert_eq!(
            report.errors, 0,
            "user-select: {kw} should be valid: {report:?}"
        );
    }
}

#[test]
fn user_select_rejects_invalid_value() {
    let css = "a { user-select: maybe; }";
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Invalid value for property “user-select”."
    );
}

// ---------------------------------------------------------------------------
// appearance
// ---------------------------------------------------------------------------

#[test]
fn appearance_accepts_valid_values() {
    for kw in ["none", "auto"] {
        let css = format!("a {{ appearance: {kw}; }}");
        let report = validate_css_text(&css, &Config::default()).unwrap();
        assert_eq!(
            report.errors, 0,
            "appearance: {kw} should be valid: {report:?}"
        );
    }
}

#[test]
fn appearance_rejects_invalid_value() {
    let css = "a { appearance: button; }";
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Invalid value for property “appearance”."
    );
}

// ---------------------------------------------------------------------------
// caret-color
// ---------------------------------------------------------------------------

#[test]
fn caret_color_accepts_auto_and_color() {
    for val in ["auto", "red", "#ff0000", "rgb(255,0,0)"] {
        let css = format!("a {{ caret-color: {val}; }}");
        let report = validate_css_text(&css, &Config::default()).unwrap();
        assert_eq!(
            report.errors, 0,
            "caret-color: {val} should be valid: {report:?}"
        );
    }
}

#[test]
fn caret_color_rejects_invalid_value() {
    let css = "a { caret-color: fancy; }";
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Invalid value for property “caret-color”."
    );
}

// ---------------------------------------------------------------------------
// caret-shape
// ---------------------------------------------------------------------------

#[test]
fn caret_shape_accepts_valid_values() {
    for kw in ["auto", "bar", "block", "underscore"] {
        let css = format!("a {{ caret-shape: {kw}; }}");
        let report = validate_css_text(&css, &Config::default()).unwrap();
        assert_eq!(
            report.errors, 0,
            "caret-shape: {kw} should be valid: {report:?}"
        );
    }
}

#[test]
fn caret_shape_rejects_invalid_value() {
    let css = "a { caret-shape: triangle; }";
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Invalid value for property “caret-shape”."
    );
}

// ---------------------------------------------------------------------------
// caret-animation
// ---------------------------------------------------------------------------

#[test]
fn caret_animation_accepts_valid_values() {
    for kw in ["auto", "manual"] {
        let css = format!("a {{ caret-animation: {kw}; }}");
        let report = validate_css_text(&css, &Config::default()).unwrap();
        assert_eq!(
            report.errors, 0,
            "caret-animation: {kw} should be valid: {report:?}"
        );
    }
}

#[test]
fn caret_animation_rejects_invalid_value() {
    let css = "a { caret-animation: fast; }";
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Invalid value for property “caret-animation”."
    );
}

// ---------------------------------------------------------------------------
// caret (shorthand)
// ---------------------------------------------------------------------------

#[test]
fn caret_shorthand_accepts_combined_values() {
    let cases = [
        "a { caret: auto; }",
        "a { caret: red; }",
        "a { caret: red bar; }",
        "a { caret: bar manual; }",
        "a { caret: #00ff00 underscore manual; }",
    ];
    for css in cases {
        let report = validate_css_text(css, &Config::default()).unwrap();
        assert_eq!(report.errors, 0, "{css} should be valid: {report:?}");
    }
}

#[test]
fn caret_shorthand_rejects_invalid() {
    let css = "a { caret: fancy blinky; }";
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Invalid value for property “caret”."
    );
}

// ---------------------------------------------------------------------------
// accent-color
// ---------------------------------------------------------------------------

#[test]
fn accent_color_accepts_auto_and_color() {
    for val in ["auto", "red", "#abcdef"] {
        let css = format!("a {{ accent-color: {val}; }}");
        let report = validate_css_text(&css, &Config::default()).unwrap();
        assert_eq!(
            report.errors, 0,
            "accent-color: {val} should be valid: {report:?}"
        );
    }
}

#[test]
fn accent_color_rejects_invalid_value() {
    let css = "a { accent-color: sparkly; }";
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Invalid value for property “accent-color”."
    );
}

// ---------------------------------------------------------------------------
// pointer-events
// ---------------------------------------------------------------------------

#[test]
fn pointer_events_accepts_auto_and_none() {
    for kw in ["auto", "none"] {
        let css = format!("a {{ pointer-events: {kw}; }}");
        let report = validate_css_text(&css, &Config::default()).unwrap();
        assert_eq!(
            report.errors, 0,
            "pointer-events: {kw} should be valid: {report:?}"
        );
    }
}

#[test]
fn pointer_events_rejects_invalid_value() {
    let css = "a { pointer-events: maybe; }";
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Invalid value for property “pointer-events”."
    );
}

// ---------------------------------------------------------------------------
// nav-up / nav-right / nav-down / nav-left
// ---------------------------------------------------------------------------

#[test]
fn nav_direction_accepts_auto() {
    for prop in ["nav-up", "nav-right", "nav-down", "nav-left"] {
        let css = format!("a {{ {prop}: auto; }}");
        let report = validate_css_text(&css, &Config::default()).unwrap();
        assert_eq!(report.errors, 0, "{prop}: auto should be valid: {report:?}");
    }
}

#[test]
fn nav_direction_rejects_invalid_value() {
    let css = "a { nav-up: nowhere; }";
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Invalid value for property “nav-up”."
    );
}

// ---------------------------------------------------------------------------
// outline-offset
// ---------------------------------------------------------------------------

#[test]
fn outline_offset_accepts_length() {
    for val in ["0", "5px", "1em", "-2px"] {
        let css = format!("a {{ outline-offset: {val}; }}");
        let report = validate_css_text(&css, &Config::default()).unwrap();
        assert_eq!(
            report.errors, 0,
            "outline-offset: {val} should be valid: {report:?}"
        );
    }
}

#[test]
fn outline_offset_rejects_invalid() {
    let css = "a { outline-offset: wide; }";
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Invalid value for property “outline-offset”."
    );
}
