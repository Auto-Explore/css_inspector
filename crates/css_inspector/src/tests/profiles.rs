use super::*;

#[test]
fn css4_profile_relaxes_single_value_heuristic_for_multitoken_properties() {
    let css = r#"a { contain-intrinsic-size: 10px 20px; }"#;
    let config = Config {
        profile: Some("css4".to_string()),
        ..Config::default()
    };
    let report = validate_css_text(css, &config).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");

    let report =
        validate_css_declarations_text("contain-intrinsic-size: 10px 20px;", &config).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
}

#[test]
fn css4_profile_accepts_modern_background_filter_and_content_values() {
    let css = r#"
:root { --c: red; --img: linear-gradient(red, blue); --txt: "x"; }

a {
  color: var(--c);
  background-image: linear-gradient(red, blue);
  background: linear-gradient(red, blue) center / cover no-repeat fixed;
  filter: blur(5px) drop-shadow(0 0 10px black);
  content: var(--txt);
}
"#;
    let config = Config {
        profile: Some("css4".to_string()),
        ..Config::default()
    };
    let report = validate_css_text(css, &config).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert!(report.messages.is_empty(), "{report:?}");
}

#[test]
fn css4_profile_accepts_var_in_known_property_values() {
    let config = Config {
        profile: Some("css4".to_string()),
        ..Config::default()
    };
    let report = validate_css_text("a { outline-style: var(--x); }", &config).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert!(report.messages.is_empty(), "{report:?}");
}

#[test]
fn css4_profile_accepts_single_curly_block_value_containing_var() {
    let config = Config {
        profile: Some("css4".to_string()),
        ..Config::default()
    };
    let report = validate_css_text("a { color: {var(--x)}; }", &config).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert!(report.messages.is_empty(), "{report:?}");
}

#[test]
fn custom_properties_accept_empty_and_comma_only_values() {
    let css = r#"a { --x: ; --y: ,; color: red; }"#;
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert!(report.messages.is_empty(), "{report:?}");
}

#[test]
fn custom_property_names_allow_unicode_and_escapes() {
    let css = r#"
a {
  --\fffd: green;
  --a-长-name-that-might-be-longer-than-you\27 d-normally-use: green;
  color: var(--a-长-name-that-might-be-longer-than-you\27 d-normally-use);
}
"#;
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert!(report.messages.is_empty(), "{report:?}");
}

#[test]
fn css4_profile_accepts_modern_cursor_values() {
    let css = r#"
a { cursor: grab; }
b { cursor: zoom-in; }
c { cursor: url(foo.png) 2 2, pointer; }
d { cursor: url(foo.png) 2 2, url(bar.png) 4 4, grabbing; }
"#;
    let config = Config {
        profile: Some("css4".to_string()),
        ..Config::default()
    };
    let report = validate_css_text(css, &config).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert!(report.messages.is_empty(), "{report:?}");
}

#[test]
fn css4_profile_accepts_cursor_none_keyword() {
    let config = Config {
        profile: Some("css4".to_string()),
        ..Config::default()
    };
    let report = validate_css_text("a { cursor: none; }", &config).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert!(report.messages.is_empty(), "{report:?}");
}

#[test]
fn css3_profile_rejects_css4_only_cursor_values() {
    let css = r#"a { cursor: grab; }"#;
    let config = Config {
        profile: Some("css3".to_string()),
        ..Config::default()
    };
    let report = validate_css_text(css, &config).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(report.messages.len(), 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Invalid value for property “cursor”."
    );
}

#[test]
fn css3_profile_rejects_modern_background_filter_and_content_values() {
    let css = r#"
a {
  background-image: linear-gradient(red, blue);
  filter: blur(5px);
  content: var(--txt);
}
"#;
    let config = Config {
        profile: Some("css3".to_string()),
        ..Config::default()
    };
    let report = validate_css_text(css, &config).unwrap();
    assert_eq!(report.warnings, 0, "{report:?}");
    assert_eq!(report.errors, 3, "{report:?}");

    let got: std::collections::BTreeSet<String> =
        report.messages.iter().map(|m| m.message.clone()).collect();
    let expected: std::collections::BTreeSet<String> = [
        "Invalid value for property “background-image”.",
        "Invalid value for property “filter”.",
        "Invalid value for property “content”.",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();
    assert_eq!(got, expected, "{report:?}");
}

#[test]
fn value_syntax_validation_rejects_unbalanced_parentheses_for_unvalidated_properties() {
    let css = r#"a { width: calc(1px; }"#;
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert_eq!(report.messages.len(), 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Invalid value for property “width”."
    );
}

#[test]
fn value_syntax_validation_rejects_unbalanced_brackets_for_unvalidated_properties() {
    let css = r#"a { width: [1px; }"#;
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert_eq!(report.messages.len(), 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Invalid value for property “width”."
    );
}

#[test]
fn css4_profile_demotes_unknown_at_rules_to_warnings() {
    let css = "@three-dee { a { color: red; } }";

    let strict = Config {
        profile: Some("css3".to_string()),
        ..Config::default()
    };
    let report = validate_css_text(css, &strict).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.message == "Unknown at-rule."),
        "{report:?}"
    );

    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert!(report.messages.is_empty(), "{report:?}");

    let report = validate_css_text(
        css,
        &Config {
            warning: Some("1".to_string()),
            ..Config::default()
        },
    )
    .unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 1, "{report:?}");
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.message == "Unknown at-rule."),
        "{report:?}"
    );
}

#[test]
fn css4_profile_demotes_unknown_properties_to_warnings() {
    let css = "a { no-such-prop: 1; }";

    let strict = Config {
        profile: Some("css3".to_string()),
        ..Config::default()
    };
    let report = validate_css_text(css, &strict).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.message == "Unknown property “no-such-prop”."),
        "{report:?}"
    );

    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert!(report.messages.is_empty(), "{report:?}");

    let report = validate_css_text(
        css,
        &Config {
            warning: Some("1".to_string()),
            ..Config::default()
        },
    )
    .unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 1, "{report:?}");
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.message == "Unknown property “no-such-prop”."),
        "{report:?}"
    );
}
