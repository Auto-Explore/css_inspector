use super::*;

#[test]
fn nested_rules_in_style_blocks_are_validated() {
    let css = r#"
a {
  color: red;
  .child { no-such-prop: 1; }
  @media (min-width: 1px) { no-such-prop2: 2; }
	}
	"#;
    let config = Config {
        profile: Some("css3".to_string()),
        ..Config::default()
    };
    let report = validate_css_text(css, &config).unwrap();
    assert_eq!(report.warnings, 0, "{report:?}");
    assert_eq!(report.errors, 2, "{report:?}");
    let got: std::collections::BTreeSet<String> =
        report.messages.iter().map(|m| m.message.clone()).collect();
    let expected: std::collections::BTreeSet<String> = [
        "Unknown property “no-such-prop”.",
        "Unknown property “no-such-prop2”.",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();
    assert_eq!(got, expected, "{report:?}");
}

#[test]
fn style_blocks_with_only_nested_rules_produce_no_declaration_errors() {
    let css = r#"
a {
  .child { color: red; }
  @media screen { b { color: blue; } }
}
"#;
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert!(report.messages.is_empty(), "{report:?}");
}

#[test]
fn nested_blocks_do_not_hide_following_declarations_in_style_blocks() {
    let css = r#"
a {
  color: red;
  .child { color: blue; }
  @media screen { b { color: green; } }
  no-such-prop: 1;
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
        "Unknown property “no-such-prop”."
    );
}

#[test]
fn extracts_decls_from_qualified_rules() {
    let css = "p{color:red}a{margin:0; padding: 1px}";
    let blocks: Vec<_> = iter_rule_blocks(css).collect();
    assert_eq!(blocks.len(), 2);
    assert_eq!(blocks[0].kind, RuleBlockKind::QualifiedRule);
    assert_eq!(blocks[0].prelude, "p");
    assert_eq!(blocks[0].body, "color:red");
    assert_eq!(blocks[1].kind, RuleBlockKind::QualifiedRule);
    assert_eq!(blocks[1].prelude, "a");
    assert_eq!(blocks[1].body, "margin:0; padding: 1px");
}

#[test]
fn extracts_decls_inside_media_nested_rules() {
    let css = "@media screen { p { margin: 0; } }";
    let blocks: Vec<_> = iter_rule_blocks(css).collect();
    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].kind, RuleBlockKind::QualifiedRule);
    assert_eq!(blocks[0].prelude, "p");
    assert_eq!(blocks[0].body, " margin: 0; ");
}

#[test]
fn iter_rule_blocks_ignores_semicolons_inside_parens() {
    let css = "@supports (--a: var(--b);) { p { color: red; } }";
    let blocks: Vec<_> = iter_rule_blocks(css).collect();
    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].kind, RuleBlockKind::QualifiedRule);
    assert_eq!(blocks[0].prelude, "p");
    assert_eq!(blocks[0].body.trim(), "color: red;");
}

#[test]
fn iter_rule_blocks_ignores_value_blocks_inside_supports_prelude() {
    let css = "@supports not ({ color: green; }) { html { color: red; } }";
    let blocks: Vec<_> = iter_rule_blocks(css).collect();
    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].kind, RuleBlockKind::QualifiedRule);
    assert_eq!(blocks[0].prelude, "html");
    assert_eq!(blocks[0].body.trim(), "color: red;");
}

#[test]
fn escaped_delimiters_in_at_rule_preludes_do_not_confuse_block_parsing() {
    let css = r#"
@import 'abc' layer(\{\});
@counter-style abc\{\}oops {}
@font-feature-values abc\{\}oops {}
@font-palette-values --abc\{\}oops {}
@keyframes abc\{\}oops {}
@layer abc\;oops\!;
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
fn iter_rule_blocks_does_not_yield_blocks_inside_values() {
    let css = ".a { color:{var(--x)}; }";
    let blocks: Vec<_> = iter_rule_blocks(css).collect();
    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].kind, RuleBlockKind::QualifiedRule);
    assert_eq!(blocks[0].prelude, ".a");
    assert_eq!(blocks[0].body.trim(), "color:{var(--x)};");
}

#[test]
fn treats_font_face_as_declaration_list() {
    let css = "@font-face { font-family: X; src: url(x); }";
    let blocks: Vec<_> = iter_rule_blocks(css).collect();
    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].kind, RuleBlockKind::AtRuleDeclList);
    assert_eq!(blocks[0].prelude, "@font-face");
    assert_eq!(blocks[0].body, " font-family: X; src: url(x); ");
}

#[test]
fn treats_font_face_as_declaration_list_case_insensitively() {
    let css = "@FONT-FACE { font-family: X; }";
    let blocks: Vec<_> = iter_rule_blocks(css).collect();
    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].kind, RuleBlockKind::AtRuleDeclList);
}

#[test]
fn treats_page_as_declaration_list() {
    let css = "@page { margin: 0; }";
    let blocks: Vec<_> = iter_rule_blocks(css).collect();
    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].kind, RuleBlockKind::AtRuleDeclList);
    assert_eq!(blocks[0].prelude, "@page");
    assert_eq!(blocks[0].body, " margin: 0; ");
}

#[test]
fn treats_page_as_declaration_list_case_insensitively() {
    let css = "@PAGE { margin: 0; }";
    let blocks: Vec<_> = iter_rule_blocks(css).collect();
    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].kind, RuleBlockKind::AtRuleDeclList);
}

#[test]
fn treats_page_as_declaration_list_with_whitespace_after_at() {
    let css = "@   PaGe { margin: 0; }";
    let blocks: Vec<_> = iter_rule_blocks(css).collect();
    assert_eq!(blocks.len(), 1);
    assert_eq!(blocks[0].kind, RuleBlockKind::AtRuleDeclList);
    assert_eq!(blocks[0].prelude, "@   PaGe");
    assert_eq!(blocks[0].body, " margin: 0; ");
}

#[test]
fn css2_accepts_font_face_with_numeric_font_weight() {
    let report = validate_css_text(
        "@font-face { font-weight: 700; }",
        &Config {
            profile: Some("css2".to_string()),
            ..Config::default()
        },
    )
    .unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert!(report.messages.is_empty(), "{report:?}");
}

#[test]
fn font_face_allows_src_descriptor_and_page_warns_once_for_page_break_too_many_values() {
    // `src` is not a normal CSS property, but is allowed inside @font-face.
    let config = Config {
        profile: Some("css3".to_string()),
        ..Config::default()
    };
    let report = validate_css_text("a { src: url(x); }", &config).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.message == "Unknown property “src”."),
        "{report:?}"
    );

    let report = validate_css_text("@FoNt-FaCe { src: url(x); }", &config).unwrap();
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.message == "Unknown property “src”."),
        "{report:?}"
    );

    let report = validate_css_text("@page { src: url(x); }", &config).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.message == "Unknown property “src”."),
        "{report:?}"
    );

    // Page-break warnings should only be emitted inside @page.
    let report =
        validate_css_text("@font-face { page-break-before: always always; }", &config).unwrap();
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.message == "Too many values for a page-break property."),
        "{report:?}"
    );

    // In @page, too many values on page-break-* triggers a single warning per declaration list.
    let report = validate_css_text(
        "@page { page-break-before: always always; page-break-after: always always; }",
        &config,
    )
    .unwrap();
    let warnings = report
        .messages
        .iter()
        .filter(|m| m.message == "Too many values for a page-break property.")
        .count();
    assert_eq!(warnings, 1, "{report:?}");
}

#[test]
fn layer_allows_nested_qualified_rules() {
    let css = r#"@layer foo {
  div {
    color: red;
  }
}"#;
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert!(report.messages.is_empty(), "{report:?}");
}

#[test]
fn layer_statement_is_accepted() {
    let report = validate_css_text("@layer foo, bar;", &Config::default()).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert!(report.messages.is_empty(), "{report:?}");
}
