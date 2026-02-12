use super::*;

#[test]
fn modern_at_rules_custom_media_custom_selector_and_nest_are_accepted() {
    let css = r#"
@custom-media --narrow (max-width: 30em);
@custom-selector :--btn button, .button;

.foo {
  @nest .bar & { color: red; }
}
"#;
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert!(report.messages.is_empty(), "{report:?}");
}

#[test]
fn supports_queries_accept_calc_nan_and_infinity() {
    for css in [
        "@supports (scale: calc(NaN)) {}",
        "@supports (scale: calc(infinity)) {}",
    ] {
        let report = validate_css_text(css, &Config::default()).unwrap();
        assert_eq!(report.errors, 0, "{css}: {report:?}");
        assert_eq!(report.warnings, 0, "{css}: {report:?}");
        assert!(report.messages.is_empty(), "{css}: {report:?}");
    }
}

#[test]
fn property_at_rule_descriptor_block_is_accepted() {
    let css = r#"
@property --x {
    syntax: "<length>";
    inherits: true;
    initial-value: 0px;
}
"#;
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert!(report.messages.is_empty(), "{report:?}");
}

#[test]
fn css4_profile_accepts_empty_initial_value_in_property_at_rule() {
    let css = r#"
@property --x {
    syntax: "<length>";
    inherits: false;
    initial-value: ;
}
"#;
    let report = validate_css_text(
        css,
        &Config {
            ..Config::default()
        },
    )
    .unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert!(report.messages.is_empty(), "{report:?}");
}

#[test]
fn page_margin_at_rules_are_accepted() {
    let css = r#"
@page {
  size: A4;
  @bottom-left { content: "x"; }
  @top-right-corner { content: counter(page); }
}
"#;
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert!(report.messages.is_empty(), "{report:?}");
}

#[test]
fn property_at_rule_rejects_invalid_inherits_descriptor_value() {
    let css = r#"
@property --x {
    syntax: "<length>";
    inherits: nope;
    initial-value: 0px;
}
"#;
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert_eq!(report.messages.len(), 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Invalid value for property “inherits”."
    );
}

#[test]
fn font_palette_values_at_rule_descriptor_block_is_accepted() {
    let css = r#"
@font-palette-values --my-palette {
    font-family: "My Font";
    base-palette: 1;
    override-colors: 0 #00f, 1 #f00;
}
"#;
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert!(report.messages.is_empty(), "{report:?}");
}

#[test]
fn font_palette_values_at_rule_rejects_unknown_descriptor() {
    let css = r#"
@font-palette-values --my-palette {
    no-such-descriptor: 1;
    font-family: "My Font";
}
"#;
    let config = Config {
        profile: Some("css3".to_string()),
        ..Config::default()
    };
    let report = validate_css_text(css, &config).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert_eq!(report.messages.len(), 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Unknown property “no-such-descriptor”."
    );
}

#[test]
fn counter_style_at_rule_descriptor_block_is_accepted() {
    let css = r#"
@counter-style thumbs {
    system: fixed 1;
    symbols: "👍" "👎";
    suffix: " ";
}
"#;
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert!(report.messages.is_empty(), "{report:?}");
}

#[test]
fn counter_style_at_rule_rejects_unknown_descriptor() {
    let css = r#"
@counter-style thumbs {
    no-such-descriptor: 1;
    system: cyclic;
}
"#;
    let config = Config {
        profile: Some("css3".to_string()),
        ..Config::default()
    };
    let report = validate_css_text(css, &config).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert_eq!(report.messages.len(), 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Unknown property “no-such-descriptor”."
    );
}

#[test]
fn color_profile_at_rule_descriptor_block_is_accepted() {
    let css = r#"
@color-profile --my-profile {
    src: url("my.icc");
    rendering-intent: relative-colorimetric;
}

a {
    color: color(display-p3 1 0 0);
}
"#;
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert!(report.messages.is_empty(), "{report:?}");
}

#[test]
fn color_profile_at_rule_rejects_unknown_descriptor() {
    let css = r#"
@color-profile --my-profile {
    no-such-descriptor: 1;
    src: url("my.icc");
}
"#;
    let config = Config {
        profile: Some("css3".to_string()),
        ..Config::default()
    };
    let report = validate_css_text(css, &config).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert_eq!(report.messages.len(), 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Unknown property “no-such-descriptor”."
    );
}

#[test]
fn color_profile_at_rule_rejects_invalid_rendering_intent_value() {
    let css = r#"
@color-profile --my-profile {
    src: url("my.icc");
    rendering-intent: nope;
}
"#;
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert_eq!(report.messages.len(), 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Invalid value for property “rendering-intent”."
    );
}

#[test]
fn view_transition_at_rule_descriptor_block_is_accepted() {
    let css = r#"
@view-transition {
    navigation: auto;
    types: fast slow;
}

a { color: red; }
"#;
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert!(report.messages.is_empty(), "{report:?}");
}

#[test]
fn view_transition_at_rule_rejects_unknown_descriptor() {
    let css = r#"
@view-transition {
    no-such-descriptor: 1;
    navigation: auto;
}
"#;
    let config = Config {
        profile: Some("css3".to_string()),
        ..Config::default()
    };
    let report = validate_css_text(css, &config).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert_eq!(report.messages.len(), 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Unknown property “no-such-descriptor”."
    );
}

#[test]
fn view_transition_at_rule_rejects_invalid_navigation_value() {
    let css = r#"
@view-transition {
    navigation: maybe;
}
"#;
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert_eq!(report.messages.len(), 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Invalid value for property “navigation”."
    );
}

#[test]
fn scroll_timeline_at_rule_descriptor_block_is_accepted() {
    let css = r#"
@scroll-timeline --my-scroll {
    source: auto;
    orientation: block;
    scroll-offsets: 0%, 100%;
    time-range: 1s;
}

a { color: red; }
"#;
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert!(report.messages.is_empty(), "{report:?}");
}

#[test]
fn scroll_timeline_at_rule_rejects_unknown_descriptor() {
    let css = r#"
@scroll-timeline --my-scroll {
    no-such-descriptor: 1;
    source: auto;
}
"#;
    let config = Config {
        profile: Some("css3".to_string()),
        ..Config::default()
    };
    let report = validate_css_text(css, &config).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert_eq!(report.messages.len(), 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Unknown property “no-such-descriptor”."
    );
}

#[test]
fn view_timeline_at_rule_descriptor_block_is_accepted() {
    let css = r#"
@view-timeline --my-view {
    subject: auto;
    axis: block;
    inset: auto;
    time-range: 1s;
}

a { color: red; }
"#;
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert!(report.messages.is_empty(), "{report:?}");
}

#[test]
fn view_timeline_at_rule_rejects_unknown_descriptor() {
    let css = r#"
@view-timeline --my-view {
    no-such-descriptor: 1;
    subject: auto;
}
"#;
    let config = Config {
        profile: Some("css3".to_string()),
        ..Config::default()
    };
    let report = validate_css_text(css, &config).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert_eq!(report.messages.len(), 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Unknown property “no-such-descriptor”."
    );
}

#[test]
fn css4_phase2_complex_stylesheet_is_accepted_under_css4_profile() {
    let css = r#"
@property --gap {
    syntax: "<length>";
    inherits: true;
    initial-value: 1cqi;
}

@font-palette-values --my-palette {
    font-family: "My Font";
    base-palette: 1;
    override-colors: 0 #00f, 1 #f00;
}

@counter-style thumbs {
    system: fixed 1;
    symbols: "👍" "👎";
    suffix: " ";
}

@font-feature-values My Font {
    @stylistic { alt-a: 1; }
    @annotation { note: 2; }
}

@keyframes fade {
    from {
        color: rgb(0 0 0 / 50%);
        outline-width: 1cqi;
    }
    to { color: #000; }
}

@starting-style {
    .card { opacity: 0; }
}

@scope (.card) to (.card .content) {
    .card {
        margin-trim: inherit;
        font-palette: --my-palette;
        line-clamp: inherit;
    }
    .card::part(button) { color: rgb(0 0 0 / 50%); }

    .card {
        @media (width >= 1024px) {
            reading-order: inherit;
            outline-width: 1cqi;
        }
    }
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
fn css4_phase2_complex_stylesheet_reports_expected_errors_under_css4_profile() {
    let css = r#"
@property --x {
    syntax: "<length>";
    inherits: maybe;
    initial-value: 0px;
}

@font-palette-values --p {
    no-such-descriptor: 1;
    font-family: "My Font";
}

@counter-style x {
    no-such-descriptor: 1;
    system: cyclic;
}

@nope { a { color: red; } }

.bad::unknown(foo) { color: red; }

.bad {
    color: rgb(0 0 / 50%);
    outline-width: 1nope;
    no-such-prop: 1;
}
"#;
    let config = Config {
        profile: Some("css4".to_string()),
        ..Config::default()
    };
    let report = validate_css_text(css, &config).unwrap();
    assert_eq!(report.warnings, 0, "{report:?}");
    assert_eq!(report.errors, 2, "{report:?}");
    assert_eq!(report.messages.len(), 2, "{report:?}");

    let expected: std::collections::BTreeSet<String> = [
        "Invalid value for property “inherits”.",
        "Invalid value for property “outline-width”.",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();

    let got: std::collections::BTreeSet<String> =
        report.messages.iter().map(|m| m.message.clone()).collect();
    assert_eq!(got, expected, "{report:?}");
    assert!(
        report
            .messages
            .iter()
            .all(|m| matches!(m.severity, Severity::Error))
    );
}
