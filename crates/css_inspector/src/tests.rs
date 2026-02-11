use super::{
    AttrConstraint, AttrOp, Config, Fetcher, RuleBlockKind, StdFetcher, ascii_lowercase_cow,
    at_rule_name, border_side_component_flags, constraints_pair_conflict, contains_ascii_ci,
    contains_invalid_top_level_chars, contains_unknown_at_rule, count_brace_balance_errors,
    dash_match_prefix, ends_with_ascii_ci, find_double_crlf, for_each_affected_border_longhand,
    is_css_wide_keyword, is_css_wide_keywordish_token, is_known_at_rule_name, iter_rule_blocks,
    iter_top_level_import_urls, memchr_crlf, parse_http_response, parse_properties_file,
    starts_with_ascii_ci, strip_css_comments, validate_css_declarations_text, validate_css_text,
    validate_css_text_with_fetcher, validate_css_uri_with_fetcher,
};
use std::borrow::Cow;
use std::cell::Cell;

struct MapFetcher(std::collections::HashMap<String, Vec<u8>>);

impl Fetcher for MapFetcher {
    fn fetch(&self, uri: &str) -> Result<Vec<u8>, super::ValidatorError> {
        self.0
            .get(uri)
            .cloned()
            .ok_or_else(|| super::ValidatorError::InvalidInput(format!("missing: {uri}")))
    }
}

struct CountingFetcher {
    map: std::collections::HashMap<String, Vec<u8>>,
    calls: Cell<usize>,
}

impl CountingFetcher {
    fn new(map: std::collections::HashMap<String, Vec<u8>>) -> Self {
        Self {
            map,
            calls: Cell::new(0),
        }
    }
}

impl Fetcher for CountingFetcher {
    fn fetch(&self, uri: &str) -> Result<Vec<u8>, super::ValidatorError> {
        self.calls.set(self.calls.get() + 1);
        self.map
            .get(uri)
            .cloned()
            .ok_or_else(|| super::ValidatorError::InvalidInput(format!("missing: {uri}")))
    }
}

fn collect_affected_border_longhands(prop: &str, tokens: &[&str]) -> Option<Vec<&'static str>> {
    let mut out = Vec::new();
    let has_any = for_each_affected_border_longhand(prop, tokens, |p| out.push(p));
    has_any.then_some(out)
}

#[test]
fn parse_properties_file_trims_ignores_comments_and_normalizes_ascii_case() {
    let props = parse_properties_file(
        "# comment\n\n  Color  : x\nMARGIN: y\nno-colon-here\n\tfont-size: z  \n",
    );
    assert!(props.contains("color"));
    assert!(props.contains("margin"));
    assert!(props.contains("font-size"));
    assert!(!props.contains("Color"));
    assert!(!props.contains("MARGIN"));
}

#[test]
fn css4_phase1_properties_file_matches_w3c_level4_diff() {
    #[derive(serde::Deserialize)]
    struct W3cPropertyRow {
        #[serde(default)]
        property: String,
        #[serde(default)]
        title: String,
    }

    fn parse_properties_file_names_in_order(s: &str) -> Vec<String> {
        let mut out = Vec::new();
        for raw in s.lines() {
            let line = raw.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let name = if let Some((name, _)) = line.split_once(':') {
                name.trim()
            } else {
                let mut parts = line.split_whitespace();
                let Some(first) = parts.next() else {
                    continue;
                };
                if parts.next().is_none() {
                    continue;
                }
                first
            };
            if !name.is_empty() {
                out.push(name.to_ascii_lowercase());
            }
        }
        out
    }

    fn parse_properties_file_names_set(s: &str) -> std::collections::BTreeSet<String> {
        parse_properties_file_names_in_order(s)
            .into_iter()
            .collect()
    }

    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");

    let w3c_path = root.join("data/w3c/all-properties.en.json");
    let css3_path = root.join("data/css_properties/CSS3Properties.properties");
    let css4_path = root.join("data/css_properties/CSS4Properties.properties");

    let w3c_text = std::fs::read_to_string(&w3c_path).expect("read all-properties.en.json");
    let rows: Vec<W3cPropertyRow> =
        serde_json::from_str(&w3c_text).expect("parse all-properties.en.json");

    let mut level4 = std::collections::BTreeSet::new();
    for r in rows {
        if !r.title.contains("Level 4") {
            continue;
        }
        let prop = r.property.trim().to_ascii_lowercase();
        if prop.is_empty() || prop == "--*" {
            continue;
        }
        level4.insert(prop);
    }

    let css3_text = std::fs::read_to_string(&css3_path).expect("read CSS3Properties.properties");
    let mut css3 = parse_properties_file_names_set(&css3_text);
    css3.insert("color-profile".to_string());

    let expected_css4: std::collections::BTreeSet<String> =
        level4.difference(&css3).cloned().collect();

    let css4_text = std::fs::read_to_string(&css4_path).expect("read CSS4Properties.properties");
    let css4_in_order = parse_properties_file_names_in_order(&css4_text);
    let css4_set = parse_properties_file_names_set(&css4_text);

    assert_eq!(css4_set, expected_css4);

    let mut sorted = css4_in_order.clone();
    sorted.sort();
    sorted.dedup();
    assert_eq!(
        css4_in_order, sorted,
        "CSS4Properties.properties should be sorted and de-duplicated; regenerate with `python3 scripts/generate_css4_properties_data.py`"
    );
}

#[test]
fn css4_phase1_profile_accepts_all_css4_supplement_property_names() {
    fn css4_props() -> Vec<String> {
        let s = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../data/css_properties/CSS4Properties.properties"
        ));
        let mut out: Vec<String> = parse_properties_file(s)
            .into_iter()
            .map(|p| p.into_owned())
            .collect();
        out.sort();
        out
    }

    let props = css4_props();
    assert!(!props.is_empty(), "expected css4 supplement properties");

    let mut css = String::from("a{\n");
    for p in &props {
        css.push_str(p);
        css.push_str(": inherit;\n");
    }
    css.push_str("}\n");

    let config = Config {
        profile: Some("css4".to_string()),
        ..Config::default()
    };
    let report = validate_css_text(&css, &config).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
}

#[test]
fn css4_phase1_default_profile_rejects_css4_supplement_properties_as_unknown() {
    fn css4_props() -> Vec<String> {
        let s = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../data/css_properties/CSS4Properties.properties"
        ));
        let mut out: Vec<String> = parse_properties_file(s)
            .into_iter()
            .map(|p| p.into_owned())
            .collect();
        out.sort();
        out
    }

    let props = css4_props();
    assert!(!props.is_empty(), "expected css4 supplement properties");

    let mut css = String::from("a{\n");
    for p in &props {
        css.push_str(p);
        css.push_str(": inherit;\n");
    }
    css.push_str("}\n");

    let report = validate_css_text(&css, &Config::default()).unwrap();
    assert_eq!(report.warnings, 0, "{report:?}");
    assert_eq!(report.errors, props.len(), "{report:?}");
    assert_eq!(report.messages.len(), props.len(), "{report:?}");

    let expected: std::collections::BTreeSet<String> = props
        .iter()
        .map(|p| format!("Unknown property “{p}”."))
        .collect();
    let got: std::collections::BTreeSet<String> =
        report.messages.iter().map(|m| m.message.clone()).collect();
    assert_eq!(got, expected);
    assert!(
        report
            .messages
            .iter()
            .all(|m| matches!(m.severity, super::Severity::Error))
    );
}

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
fn default_profile_rejects_css4_only_cursor_values() {
    let css = r#"a { cursor: grab; }"#;
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(report.messages.len(), 1, "{report:?}");
    assert_eq!(
        report.messages[0].message,
        "Invalid value for property “cursor”."
    );
}

#[test]
fn default_profile_rejects_modern_background_filter_and_content_values() {
    let css = r#"
a {
  background-image: linear-gradient(red, blue);
  filter: blur(5px);
  content: var(--txt);
}
"#;
    let report = validate_css_text(css, &Config::default()).unwrap();
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
fn nested_rules_in_style_blocks_are_validated() {
    let css = r#"
a {
  color: red;
  .child { no-such-prop: 1; }
  @media (min-width: 1px) { no-such-prop2: 2; }
}
"#;
    let report = validate_css_text(css, &Config::default()).unwrap();
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
    let report = validate_css_text(css, &Config::default()).unwrap();
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
fn font_face_allows_src_descriptor_and_page_warns_once_for_page_break_too_many_values() {
    // `src` is not a normal CSS property, but is allowed inside @font-face.
    let report = validate_css_text("a { src: url(x); }", &Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.message == "Unknown property “src”."),
        "{report:?}"
    );

    let report = validate_css_text("@FoNt-FaCe { src: url(x); }", &Config::default()).unwrap();
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.message == "Unknown property “src”."),
        "{report:?}"
    );

    let report = validate_css_text("@page { src: url(x); }", &Config::default()).unwrap();
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.message == "Unknown property “src”."),
        "{report:?}"
    );

    // Page-break warnings should only be emitted inside @page.
    let report = validate_css_text(
        "@font-face { page-break-before: always always; }",
        &Config::default(),
    )
    .unwrap();
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
        &Config::default(),
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
fn at_rule_name_parses_leading_at_sign_and_trims() {
    assert_eq!(at_rule_name("@media screen"), Some("media"));
    assert_eq!(at_rule_name(" @supports(display: grid)"), Some("supports"));
    assert_eq!(at_rule_name("@font-face {"), Some("font-face"));
}

#[test]
fn at_rule_name_splits_on_other_whitespace() {
    assert_eq!(at_rule_name("@media\nscreen"), Some("media"));
    assert_eq!(at_rule_name("@media\tprint"), Some("media"));
}

#[test]
fn strip_css_comments_borrows_when_no_comments_present() {
    let (out, ok) = strip_css_comments("a{color:red}");
    assert!(ok);
    assert!(matches!(out, Cow::Borrowed(_)));
    assert_eq!(out.as_ref(), "a{color:red}");
}

#[test]
fn strip_css_comments_removes_comments_and_preserves_utf8() {
    let (out, ok) = strip_css_comments("a/*x*/bé");
    assert!(ok);
    assert!(matches!(out, Cow::Owned(_)));
    assert_eq!(out.as_ref(), "a bé");

    let (out2, ok2) = strip_css_comments("a/*");
    assert!(!ok2);
    assert!(matches!(out2, Cow::Borrowed(_)));
    assert_eq!(out2.as_ref(), "a");
}

#[test]
fn strip_css_comments_returns_prefix_when_unclosed_after_another_comment() {
    let (out, ok) = strip_css_comments("a/*x*/b/*");
    assert!(!ok);
    assert!(matches!(out, Cow::Owned(_)));
    assert_eq!(out.as_ref(), "a b");
}

#[test]
fn iter_top_level_import_urls_keeps_scanning_past_stray_braces() {
    let css = "@import \"a.css\"}; @import \"b.css\";";
    let urls: Vec<_> = iter_top_level_import_urls(css).collect();
    assert_eq!(urls, vec!["a.css", "b.css"]);
}

#[test]
fn iter_top_level_import_urls_skips_leading_whitespace() {
    let css = "  \n\t @import \"a.css\"; @import \"b.css\";";
    let urls: Vec<_> = iter_top_level_import_urls(css).collect();
    assert_eq!(urls, vec!["a.css", "b.css"]);
}

#[test]
fn iter_top_level_import_urls_parses_url_function_and_strips_quotes() {
    let css = "@import url(a.css); @import url(\"b.css\"); @import url( 'c.css' );";
    let urls: Vec<_> = iter_top_level_import_urls(css).collect();
    assert_eq!(urls, vec!["a.css", "b.css", "c.css"]);
}

#[test]
fn iter_top_level_import_urls_supports_nested_parentheses_in_url_function() {
    let css = "@import url(a(b).css);";
    let urls: Vec<_> = iter_top_level_import_urls(css).collect();
    assert_eq!(urls, vec!["a(b).css"]);
}

#[test]
fn iter_top_level_import_urls_ignores_parentheses_inside_quoted_url_function_args() {
    let css = "@import url(\"a(b).css\");";
    let urls: Vec<_> = iter_top_level_import_urls(css).collect();
    assert_eq!(urls, vec!["a(b).css"]);
}

#[test]
fn iter_top_level_import_urls_matches_import_and_url_case_insensitively() {
    let css = "@IMPORT URL(A.CSS); @import url(b.css);";
    let urls: Vec<_> = iter_top_level_import_urls(css).collect();
    assert_eq!(urls, vec!["A.CSS", "b.css"]);
}

#[test]
fn iter_top_level_import_urls_stops_on_unsupported_import_syntax() {
    let css = "@import (\"a.css\"); @import \"b.css\";";
    let urls: Vec<_> = iter_top_level_import_urls(css).collect();
    assert!(urls.is_empty());
}

#[test]
fn iter_top_level_import_urls_requires_whitespace_after_import_keyword() {
    let css = "@importurl(a.css); @import \"b.css\";";
    let urls: Vec<_> = iter_top_level_import_urls(css).collect();
    assert!(urls.is_empty());
}

#[test]
fn iter_top_level_import_urls_stops_after_first_non_import_statement() {
    let css = "@import \"a.css\"; body { color: red; } @import \"b.css\";";
    let urls: Vec<_> = iter_top_level_import_urls(css).collect();
    assert_eq!(urls, vec!["a.css"]);
}

#[test]
fn iter_top_level_import_urls_skips_empty_urls_and_continues() {
    let css = "@import \"\"; @import url(); @import url( ); @import \"b.css\";";
    let urls: Vec<_> = iter_top_level_import_urls(css).collect();
    assert_eq!(urls, vec!["b.css"]);
}

#[test]
fn iter_top_level_import_urls_stops_on_unterminated_url_function_after_yielding_previous_urls() {
    let css = "@import \"a.css\"; @import url(b.css; @import \"c.css\";";
    let urls: Vec<_> = iter_top_level_import_urls(css).collect();
    assert_eq!(urls, vec!["a.css"]);
}

#[test]
fn iter_top_level_import_urls_stops_on_unterminated_string_after_yielding_previous_urls() {
    let css = "@import \"a.css\"; @import 'b.css; @import \"c.css\";";
    let urls: Vec<_> = iter_top_level_import_urls(css).collect();
    assert_eq!(urls, vec!["a.css"]);
}

#[test]
fn iter_top_level_import_urls_handles_escaped_quotes_in_strings() {
    let css = r#"@import "a\"b.css"; @import 'c\'d.css';"#;
    let urls: Vec<_> = iter_top_level_import_urls(css).collect();
    assert_eq!(urls, vec![r#"a\"b.css"#, r#"c\'d.css"#]);
}

#[test]
fn iter_top_level_import_urls_handles_escaped_quotes_inside_url_function_strings() {
    let css = r#"@import url("a\"b.css"); @import url('c\'d.css');"#;
    let urls: Vec<_> = iter_top_level_import_urls(css).collect();
    assert_eq!(urls, vec![r#"a\"b.css"#, r#"c\'d.css"#]);
}

#[test]
fn count_brace_balance_errors_counts_unbalanced_braces_and_strings() {
    assert_eq!(count_brace_balance_errors("a{b}"), 0);
    assert_eq!(count_brace_balance_errors("a{b"), 1);
    assert_eq!(count_brace_balance_errors("a{{"), 2);
    assert_eq!(count_brace_balance_errors("}"), 1);
    assert_eq!(count_brace_balance_errors("a{\""), 1);
    assert_eq!(count_brace_balance_errors("a{{\""), 1);
    assert_eq!(count_brace_balance_errors("}{\\\""), 2);
    assert_eq!(count_brace_balance_errors(r#"a{"{"}"#), 0);
    assert_eq!(count_brace_balance_errors(r#"a{"}"}"#), 0);
    assert_eq!(count_brace_balance_errors(r#"a{"\""}"#), 0);
}

#[test]
fn contains_invalid_top_level_chars_ignores_strings() {
    assert!(contains_invalid_top_level_chars("<"));
    assert!(contains_invalid_top_level_chars("a<b"));
    assert!(!contains_invalid_top_level_chars(r#"a{"<"}"#));
    assert!(contains_invalid_top_level_chars(r#"a{"<"}<"#));
}

#[test]
fn count_stray_declaration_errors_does_not_split_on_semicolons_in_strings() {
    assert_eq!(
        super::count_stray_top_level_declaration_errors(r#"x:"a;b";"#),
        2
    );
    assert_eq!(
        super::count_stray_top_level_declaration_errors(r#"x:"a\";b";"#),
        2
    );
}

#[test]
fn reports_stray_declarations_and_unbalanced_braces() {
    let config = Config::default();

    let report = validate_css_text("bodytext-align:center;", &config).unwrap();
    assert_eq!(report.errors, 2, "{:?}", report.messages);
    assert_eq!(
        report
            .messages
            .iter()
            .filter(|m| m.message == "Stray declaration outside a rule.")
            .count(),
        2
    );

    let report = validate_css_text("bodytext-align:center; }", &config).unwrap();
    assert_eq!(report.errors, 3, "{:?}", report.messages);
    assert_eq!(
        report
            .messages
            .iter()
            .filter(|m| m.message == "Stray declaration outside a rule.")
            .count(),
        2
    );
    assert_eq!(
        report
            .messages
            .iter()
            .filter(|m| m.message == "Unbalanced braces.")
            .count(),
        1
    );
}

#[test]
fn strip_css_comments_inserts_whitespace_for_comments_at_boundaries() {
    let (out, ok) = strip_css_comments("/*x*/a/*y*/b");
    assert!(ok);
    assert_eq!(out.as_ref(), " a b");
}

#[test]
fn strip_css_comments_inserts_whitespace_for_adjacent_comments() {
    let (out, ok) = strip_css_comments("a/*x*//*y*/b");
    assert!(ok);
    assert_eq!(out.as_ref(), "a  b");
}

#[test]
fn at_rule_name_rejects_empty_or_missing_names() {
    assert_eq!(at_rule_name("@"), None);
    assert_eq!(at_rule_name("@   "), None);
    assert_eq!(at_rule_name("media screen"), None);
}

#[test]
fn clip_rect_allows_whitespace_or_commas_between_four_args() {
    let config = Config::default();

    let report = validate_css_text("a{clip:rect(1  2 3 4)}", &config).unwrap();
    assert_eq!(report.errors, 0, "{:?}", report.messages);

    let report = validate_css_text("a{clip:rect(1, 2, 3, 4)}", &config).unwrap();
    assert_eq!(report.errors, 0, "{:?}", report.messages);
}

#[test]
fn clip_rect_rejects_too_many_args() {
    let config = Config::default();
    let report = validate_css_text("a{clip:rect(1, 2, 3, 4, 5)}", &config).unwrap();
    assert_eq!(report.errors, 1, "{:?}", report.messages);
}

#[test]
fn ascii_ci_prefix_and_suffix_helpers_match_case_insensitively() {
    assert!(starts_with_ascii_ci("File://x", "file://"));
    assert!(!starts_with_ascii_ci("fi", "file://"));
    assert!(starts_with_ascii_ci("http://x", ""));
    assert!(!starts_with_ascii_ci("🦀", "a"));
    assert!(starts_with_ascii_ci("🦀a", "🦀"));

    assert!(ends_with_ascii_ci("Foo.BAR", "bar"));
    assert!(!ends_with_ascii_ci("bar", "bars"));
    assert!(ends_with_ascii_ci("x", ""));
    assert!(!ends_with_ascii_ci("🦀", "a"));
    assert!(ends_with_ascii_ci("a🦀", "🦀"));

    assert!(contains_ascii_ci("xCHUNKEDy", "chunked"));
    assert!(!contains_ascii_ci("xchunky", "chunked"));
    assert!(contains_ascii_ci("abc", ""));
    assert!(!contains_ascii_ci("ab", "abc"));
}

#[test]
fn ascii_lowercase_cow_borrows_without_ascii_uppercase() {
    let out = ascii_lowercase_cow("abcÖ");
    assert!(matches!(out, Cow::Borrowed(_)));
    assert_eq!(out.as_ref(), "abcÖ");
}

#[test]
fn ascii_lowercase_cow_borrows_empty_string() {
    let out = ascii_lowercase_cow("");
    assert!(matches!(out, Cow::Borrowed("")));
}

#[test]
fn ascii_lowercase_cow_lowercases_ascii_and_preserves_utf8() {
    let out = ascii_lowercase_cow("aÖB");
    assert!(matches!(out, Cow::Owned(_)));
    assert_eq!(out.as_ref(), "aÖb");
}

#[test]
fn ascii_lowercase_cow_handles_multibyte_prefix_before_ascii_uppercase() {
    let out = ascii_lowercase_cow("🦀A");
    assert!(matches!(out, Cow::Owned(_)));
    assert_eq!(out.as_ref(), "🦀a");
}

#[test]
fn ascii_lowercase_cow_lowercases_multiple_ascii_bytes() {
    let out = ascii_lowercase_cow("ABC");
    assert!(matches!(out, Cow::Owned(_)));
    assert_eq!(out.as_ref(), "abc");
}

#[test]
fn unknown_at_rule_detection_ignores_known_rules_and_strings() {
    assert!(!contains_unknown_at_rule(
        "@media screen { p { color: red } }"
    ));
    assert!(!contains_unknown_at_rule(
        "@layer base { p { color: red } }"
    ));
    assert!(!contains_unknown_at_rule(
        "@font-feature-values Fira Code { @character-variant { alt-a: 1; } }"
    ));
    assert!(!contains_unknown_at_rule(
        "@font-feature-values Fira Code { @stylistic { alt-a: 1; } }"
    ));
    assert!(!contains_unknown_at_rule(
        "@container test (max-width: 300px) { p { color: red } }"
    ));
    assert!(!contains_unknown_at_rule(
        "@keyframes spin { from { color: red } to { color: blue } }"
    ));
    assert!(!contains_unknown_at_rule(
        "@property --x { syntax: \"<length>\"; inherits: true; initial-value: 0px; }"
    ));
    assert!(!contains_unknown_at_rule(
        "@page { @bottom-left { content: \"x\"; } }"
    ));
    assert!(!contains_unknown_at_rule("a{content:\"@three-dee\"}"));
    assert!(contains_unknown_at_rule("@three-dee { x: y }"));
}

#[test]
fn unknown_at_rule_detection_ignores_at_signs_without_valid_names() {
    assert!(!contains_unknown_at_rule("@ { x: y }"));
    assert!(!contains_unknown_at_rule("@\t( x )"));
    assert!(!contains_unknown_at_rule("@1 { x: y }"));
}

#[test]
fn unknown_at_rule_detection_matches_known_names_without_whitespace() {
    assert!(!contains_unknown_at_rule("@MEDIA{a{color:red}}"));
    assert!(contains_unknown_at_rule("@THREE-DEE{a{color:red}}"));
}

#[test]
fn known_at_rule_names_match_case_insensitively() {
    assert!(is_known_at_rule_name("MEDIA"));
    assert!(is_known_at_rule_name("media"));
    assert!(is_known_at_rule_name("import"));
    assert!(is_known_at_rule_name("layer"));
    assert!(is_known_at_rule_name("LAYER"));
    assert!(is_known_at_rule_name("font-face"));
    assert!(is_known_at_rule_name("top-left-corner"));
    assert!(is_known_at_rule_name("TOP-LEFT-CORNER"));
    assert!(is_known_at_rule_name("top-left"));
    assert!(is_known_at_rule_name("TOP-LEFT"));
    assert!(is_known_at_rule_name("bottom-left"));
    assert!(is_known_at_rule_name("BOTTOM-LEFT"));
    assert!(is_known_at_rule_name("bottom-right-corner"));
    assert!(is_known_at_rule_name("BOTTOM-RIGHT-CORNER"));
    assert!(is_known_at_rule_name("left-top"));
    assert!(is_known_at_rule_name("LEFT-TOP"));
    assert!(is_known_at_rule_name("right-bottom"));
    assert!(is_known_at_rule_name("RIGHT-BOTTOM"));
    assert!(is_known_at_rule_name("font-feature-values"));
    assert!(is_known_at_rule_name("FONT-FEATURE-VALUES"));
    assert!(is_known_at_rule_name("stylistic"));
    assert!(is_known_at_rule_name("STYLISTIC"));
    assert!(is_known_at_rule_name("styleset"));
    assert!(is_known_at_rule_name("STYLESET"));
    assert!(is_known_at_rule_name("font-palette-values"));
    assert!(is_known_at_rule_name("FONT-PALETTE-VALUES"));
    assert!(is_known_at_rule_name("character-variant"));
    assert!(is_known_at_rule_name("CHARACTER-VARIANT"));
    assert!(is_known_at_rule_name("swash"));
    assert!(is_known_at_rule_name("SWASH"));
    assert!(is_known_at_rule_name("ornaments"));
    assert!(is_known_at_rule_name("ORNAMENTS"));
    assert!(is_known_at_rule_name("annotation"));
    assert!(is_known_at_rule_name("ANNOTATION"));
    assert!(is_known_at_rule_name("container"));
    assert!(is_known_at_rule_name("CONTAINER"));
    assert!(is_known_at_rule_name("keyframes"));
    assert!(is_known_at_rule_name("KEYFRAMES"));
    assert!(is_known_at_rule_name("counter-style"));
    assert!(is_known_at_rule_name("COUNTER-STYLE"));
    assert!(is_known_at_rule_name("property"));
    assert!(is_known_at_rule_name("PROPERTY"));
    assert!(is_known_at_rule_name("color-profile"));
    assert!(is_known_at_rule_name("COLOR-PROFILE"));
    assert!(is_known_at_rule_name("scope"));
    assert!(is_known_at_rule_name("SCOPE"));
    assert!(is_known_at_rule_name("starting-style"));
    assert!(is_known_at_rule_name("STARTING-STYLE"));
    assert!(is_known_at_rule_name("custom-media"));
    assert!(is_known_at_rule_name("CUSTOM-MEDIA"));
    assert!(is_known_at_rule_name("custom-selector"));
    assert!(is_known_at_rule_name("CUSTOM-SELECTOR"));
    assert!(is_known_at_rule_name("nest"));
    assert!(is_known_at_rule_name("NEST"));
    assert!(is_known_at_rule_name("view-transition"));
    assert!(is_known_at_rule_name("VIEW-TRANSITION"));
    assert!(!is_known_at_rule_name("three-dee"));
}

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
    let report = validate_css_text(css, &Config::default()).unwrap();
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
    let report = validate_css_text(css, &Config::default()).unwrap();
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
    let report = validate_css_text(css, &Config::default()).unwrap();
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
    let report = validate_css_text(css, &Config::default()).unwrap();
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
    let report = validate_css_text(css, &Config::default()).unwrap();
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
    let report = validate_css_text(css, &Config::default()).unwrap();
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
    assert_eq!(report.errors, 8, "{report:?}");
    assert_eq!(report.messages.len(), 8, "{report:?}");

    let expected: std::collections::BTreeSet<String> = [
        "Unknown at-rule.",
        "Invalid selector.",
        "Invalid value for property “inherits”.",
        "Invalid value for property “color”.",
        "Invalid value for property “outline-width”.",
        "Unknown property “no-such-descriptor”.",
        "Unknown property “no-such-descriptor”.",
        "Unknown property “no-such-prop”.",
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
            .all(|m| matches!(m.severity, super::Severity::Error))
    );
}

#[test]
fn url_functions_allow_unknown_protocol_schemes() {
    let css = r#".foo { background: url("foo://example.com/image.svg"); }"#;
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert!(report.messages.is_empty(), "{report:?}");
}

#[test]
fn zoom_property_is_allowed_with_common_values() {
    let css = r#"
div {
/* <percentage> values */
zoom: 50%;
zoom: 200%;

/* <number> values */
zoom: 1.1;
zoom: 0.7;

/* Non-standard keyword values */
zoom: normal;
zoom: reset;

/* Global values */
zoom: inherit;
zoom: initial;
zoom: revert;
zoom: revert-layer;
zoom: unset;
}
"#;
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert!(report.messages.is_empty(), "{report:?}");
}

#[test]
fn highlight_pseudo_element_is_accepted() {
    let css = r#"
::highlight(a),
::highlight(b),
::highlight(c) {
  background-color: yellow;
  color: black;
}
"#;
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert!(report.messages.is_empty(), "{report:?}");
}

#[test]
fn oklch_colors_with_from_and_var_are_accepted() {
    let css = r#"
div {
  width: 100px;
  height: 100px;
  background-color: red;
  color: red;
  background-color: oklch(from currentColor l c h);
}

:root {
  --lightness: 50%;
}

html {
  color: oklch(var(--lightness) 50% 270);
}
"#;
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert!(report.messages.is_empty(), "{report:?}");
}

#[test]
fn oklch_color_function_accepts_common_valid_forms() {
    for value in [
        // Absolute values.
        "oklch(40.1% 0.123 21.57)",
        "oklch(59.69% 0.156 49.77)",
        "oklch(59.69% 0.156 49.77 / .5)",
        // Relative values.
        "oklch(from green l c h / 0.5)",
        "oklch(from #123456 calc(l + 0.1) c h)",
        "oklch(from hsl(180 100% 50%) calc(l - 0.1) c h)",
        "oklch(from var(--color) l c h / calc(alpha - 0.1))",
        "oklch(from hsl(0 100% 50%) l c h)",
        "oklch(from hsl(0 100% 50%) 42.1% 0.25 328.363)",
        "oklch(from hsl(0 100% 50%) 0.8 0.4 h)",
        "oklch(from hsl(0 100% 50% / 0.8) l c h / alpha)",
        "oklch(from hsl(0 100% 50% / 0.8) l c h / 0.5)",
        "oklch(from hsl(0 100% 50%) calc(l + 0.2) calc(c + 0.1) calc(h - 20) / calc(alpha - 0.1))",
    ] {
        let css = format!("a{{color:{value};}}");
        let report = validate_css_text(&css, &Config::default()).unwrap();
        assert_eq!(report.errors, 0, "value={value} report={report:?}");
        assert_eq!(report.warnings, 0, "value={value} report={report:?}");
        assert!(
            report.messages.is_empty(),
            "value={value} report={report:?}"
        );
    }
}

#[test]
fn css_color4_color_function_tokens_are_accepted() {
    for value in [
        "hwb(180 10% 10%)",
        "lab(29.2345% 39.3825 20.0664)",
        "lch(29.2345% 44.2 27)",
        "oklab(0.7 0.1 0.2 / 0.5)",
        "color(display-p3 1 0 0)",
        "color-mix(in srgb, red 50%, blue)",
        "device-cmyk(0 81% 81% 30%)",
        "light-dark(black, white)",
        "color-contrast(black vs white)",
    ] {
        let css = format!("a{{color:{value};}}");
        let report = validate_css_text(&css, &Config::default()).unwrap();
        assert_eq!(report.errors, 0, "value={value} report={report:?}");
        assert_eq!(report.warnings, 0, "value={value} report={report:?}");
        assert!(
            report.messages.is_empty(),
            "value={value} report={report:?}"
        );
    }
}

#[test]
fn css_color_relative_rgb_and_hsl_syntax_is_accepted() {
    for value in [
        "rgb(from red r g b / 50%)",
        "rgba(from red r g b / 0.5)",
        "hsl(from red h s l / 50%)",
        "hsla(from red h s l / 0.5)",
    ] {
        let css = format!("a{{color:{value};}}");
        let report = validate_css_text(&css, &Config::default()).unwrap();
        assert_eq!(report.errors, 0, "value={value} report={report:?}");
        assert_eq!(report.warnings, 0, "value={value} report={report:?}");
        assert!(
            report.messages.is_empty(),
            "value={value} report={report:?}"
        );
    }
}

#[test]
fn css4_profile_accepts_additional_css4_selector_pseudos() {
    let css = r#"
a:user-valid { color: red; }
details:open > summary { color: red; }
dialog:modal { color: red; }
video:picture-in-picture { color: red; }
button:popover-open { color: red; }

::view-transition { color: red; }
::view-transition-group(foo) { color: red; }
.x:state(foo) { color: red; }
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
fn default_profile_rejects_css4_only_selector_pseudos() {
    let css = r#"a:user-valid { color: red; }"#;
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert_eq!(report.messages.len(), 1, "{report:?}");
    assert_eq!(report.messages[0].message, "Invalid selector.");
}

#[test]
fn css_wide_keyword_helpers_are_case_insensitive_and_accept_slash_forms() {
    assert!(is_css_wide_keyword(" InHeRiT "));
    assert!(is_css_wide_keyword("revert-layer"));
    assert!(!is_css_wide_keyword("inheritance"));

    assert!(is_css_wide_keywordish_token("inherit/20%"));
    assert!(is_css_wide_keywordish_token("REVERT/1"));
    assert!(!is_css_wide_keywordish_token("inheritance/1"));
}

#[test]
fn conflicting_attribute_selectors_produce_warning() {
    let css = r#"span[hello="Cleveland"][hello="Columbus"]{color:red}"#;
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 0);
    assert_eq!(report.warnings, 1);
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.message == "Conflicting attribute selector constraints.")
    );
}

#[test]
fn non_conflicting_attribute_selectors_do_not_warn() {
    let css = r#"span[hello="Cleveland"][hello="Cleveland"]{color:red}"#;
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 0);
    assert_eq!(report.warnings, 0);
}

#[test]
fn attribute_selectors_on_different_attributes_do_not_warn() {
    let css = r#"span[hello="Cleveland"][world="Columbus"]{color:red}"#;
    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 0);
    assert_eq!(report.warnings, 0);
}

#[test]
fn includes_conflict_checks_match_expected_behavior() {
    let a = AttrConstraint {
        op: AttrOp::Includes,
        value: Some("foo".to_string()),
    };
    let b = AttrConstraint {
        op: AttrOp::Includes,
        value: Some("bar".to_string()),
    };
    assert!(constraints_pair_conflict(&a, &b));

    let a2 = AttrConstraint {
        op: AttrOp::Includes,
        value: Some("foo bar".to_string()),
    };
    let b2 = AttrConstraint {
        op: AttrOp::Includes,
        value: Some("bar baz".to_string()),
    };
    assert!(!constraints_pair_conflict(&a2, &b2));

    let a3 = AttrConstraint {
        op: AttrOp::Includes,
        value: Some("foo bar baz".to_string()),
    };
    let b3 = AttrConstraint {
        op: AttrOp::Includes,
        value: Some("xxx yyy baz".to_string()),
    };
    assert!(!constraints_pair_conflict(&a3, &b3));

    let dash = AttrConstraint {
        op: AttrOp::DashMatch,
        value: Some("foo".to_string()),
    };
    assert!(!constraints_pair_conflict(
        &dash,
        &AttrConstraint {
            op: AttrOp::Includes,
            value: Some("foo bar".to_string()),
        }
    ));
    assert!(constraints_pair_conflict(
        &dash,
        &AttrConstraint {
            op: AttrOp::Includes,
            value: Some("bar baz".to_string()),
        }
    ));

    let a_ws = AttrConstraint {
        op: AttrOp::Includes,
        value: Some("  foo  ".to_string()),
    };
    let b_ws = AttrConstraint {
        op: AttrOp::Includes,
        value: Some("bar foo baz".to_string()),
    };
    assert!(!constraints_pair_conflict(&a_ws, &b_ws));
}

#[test]
fn constraints_pair_conflict_is_conservative_on_missing_or_empty_values() {
    let pref_missing = AttrConstraint {
        op: AttrOp::Prefix,
        value: None,
    };
    let pref = AttrConstraint {
        op: AttrOp::Prefix,
        value: Some("foo".to_string()),
    };
    assert!(!constraints_pair_conflict(&pref_missing, &pref));
    assert!(!constraints_pair_conflict(&pref, &pref_missing));

    let dash = AttrConstraint {
        op: AttrOp::DashMatch,
        value: Some("foo".to_string()),
    };
    let inc_empty = AttrConstraint {
        op: AttrOp::Includes,
        value: Some("   ".to_string()),
    };
    assert!(!constraints_pair_conflict(&dash, &inc_empty));
    assert!(!constraints_pair_conflict(&inc_empty, &dash));

    let inc_foo = AttrConstraint {
        op: AttrOp::Includes,
        value: Some("foo".to_string()),
    };
    assert!(!constraints_pair_conflict(&inc_empty, &inc_foo));
    assert!(!constraints_pair_conflict(&inc_foo, &inc_empty));
}

#[test]
fn dash_match_and_prefix_conflicts_match_expected_behavior() {
    let dash = AttrConstraint {
        op: AttrOp::DashMatch,
        value: Some("en".to_string()),
    };
    let pref_same = AttrConstraint {
        op: AttrOp::Prefix,
        value: Some("en".to_string()),
    };
    assert!(!constraints_pair_conflict(&dash, &pref_same));
    assert!(!constraints_pair_conflict(&pref_same, &dash));

    let pref_ok = AttrConstraint {
        op: AttrOp::Prefix,
        value: Some("en-".to_string()),
    };
    assert!(!constraints_pair_conflict(&dash, &pref_ok));
    assert!(!constraints_pair_conflict(&pref_ok, &dash));

    let pref_short = AttrConstraint {
        op: AttrOp::Prefix,
        value: Some("e".to_string()),
    };
    assert!(!constraints_pair_conflict(&dash, &pref_short));

    let pref_conflict = AttrConstraint {
        op: AttrOp::Prefix,
        value: Some("enx".to_string()),
    };
    assert!(constraints_pair_conflict(&dash, &pref_conflict));
    assert!(constraints_pair_conflict(&pref_conflict, &dash));
}

#[test]
fn constraint_allows_value_is_false_without_selector_value() {
    let exact = AttrConstraint {
        op: AttrOp::Exact,
        value: None,
    };
    assert!(!super::constraint_allows_value(&exact, "x"));

    let prefix = AttrConstraint {
        op: AttrOp::Prefix,
        value: None,
    };
    assert!(!super::constraint_allows_value(&prefix, "x"));
}

#[test]
fn constraint_allows_value_behaves_for_includes_and_dash_match() {
    let includes = AttrConstraint {
        op: AttrOp::Includes,
        value: Some("foo bar  baz".to_string()),
    };
    assert!(super::constraint_allows_value(&includes, "bar"));
    assert!(!super::constraint_allows_value(&includes, "qux"));

    let dash = AttrConstraint {
        op: AttrOp::DashMatch,
        value: Some("en".to_string()),
    };
    assert!(super::constraint_allows_value(&dash, "en"));
    assert!(super::constraint_allows_value(&dash, "en-US"));
    assert!(!super::constraint_allows_value(&dash, "fr"));
}

#[test]
fn includes_conflicts_require_a_non_empty_token_list() {
    let includes_empty = AttrConstraint {
        op: AttrOp::Includes,
        value: Some("   \t  ".to_string()),
    };

    let dash = AttrConstraint {
        op: AttrOp::DashMatch,
        value: Some("foo".to_string()),
    };
    assert!(!constraints_pair_conflict(&dash, &includes_empty));
    assert!(!constraints_pair_conflict(
        &dash,
        &AttrConstraint {
            op: AttrOp::Includes,
            value: Some("bar foo-baz".to_string()),
        }
    ));

    let pref = AttrConstraint {
        op: AttrOp::Prefix,
        value: Some("foo".to_string()),
    };
    assert!(!constraints_pair_conflict(&pref, &includes_empty));
    assert!(!constraints_pair_conflict(
        &pref,
        &AttrConstraint {
            op: AttrOp::Includes,
            value: Some("bar foobar".to_string()),
        }
    ));
    assert!(constraints_pair_conflict(
        &pref,
        &AttrConstraint {
            op: AttrOp::Includes,
            value: Some("bar baz".to_string()),
        }
    ));

    let suf = AttrConstraint {
        op: AttrOp::Suffix,
        value: Some("foo".to_string()),
    };
    assert!(!constraints_pair_conflict(&suf, &includes_empty));
    assert!(!constraints_pair_conflict(
        &suf,
        &AttrConstraint {
            op: AttrOp::Includes,
            value: Some("bar bazfoo".to_string()),
        }
    ));
    assert!(constraints_pair_conflict(
        &suf,
        &AttrConstraint {
            op: AttrOp::Includes,
            value: Some("bar baz".to_string()),
        }
    ));

    let sub = AttrConstraint {
        op: AttrOp::Substring,
        value: Some("foo".to_string()),
    };
    assert!(!constraints_pair_conflict(&sub, &includes_empty));
    assert!(!constraints_pair_conflict(
        &sub,
        &AttrConstraint {
            op: AttrOp::Includes,
            value: Some("bar xxfooyy".to_string()),
        }
    ));
    assert!(constraints_pair_conflict(
        &sub,
        &AttrConstraint {
            op: AttrOp::Includes,
            value: Some("bar baz".to_string()),
        }
    ));
}

#[test]
fn includes_conflict_checks_handle_single_token_lists() {
    let pref = AttrConstraint {
        op: AttrOp::Prefix,
        value: Some("foo".to_string()),
    };
    let includes_one_bad = AttrConstraint {
        op: AttrOp::Includes,
        value: Some("bar".to_string()),
    };
    let includes_one_ok = AttrConstraint {
        op: AttrOp::Includes,
        value: Some("foobar".to_string()),
    };

    assert!(constraints_pair_conflict(&pref, &includes_one_bad));
    assert!(!constraints_pair_conflict(&pref, &includes_one_ok));
}

#[test]
fn includes_includes_conflict_is_conservative_on_empty_token_lists() {
    let empty = AttrConstraint {
        op: AttrOp::Includes,
        value: Some("   \t  ".to_string()),
    };
    let empty_str = AttrConstraint {
        op: AttrOp::Includes,
        value: Some("".to_string()),
    };
    let non_empty = AttrConstraint {
        op: AttrOp::Includes,
        value: Some("foo".to_string()),
    };
    assert!(!constraints_pair_conflict(&empty, &non_empty));
    assert!(!constraints_pair_conflict(&non_empty, &empty));
    assert!(!constraints_pair_conflict(&empty, &empty));
    assert!(!constraints_pair_conflict(&empty_str, &non_empty));
    assert!(!constraints_pair_conflict(&non_empty, &empty_str));
    assert!(!constraints_pair_conflict(&empty_str, &empty_str));
    assert!(!constraints_pair_conflict(&empty, &empty_str));
    assert!(!constraints_pair_conflict(&empty_str, &empty));
}

#[test]
fn exact_constraints_must_satisfy_includes_constraints() {
    let ok = vec![
        AttrConstraint {
            op: AttrOp::Exact,
            value: Some("foo".to_string()),
        },
        AttrConstraint {
            op: AttrOp::Includes,
            value: Some("foo bar".to_string()),
        },
    ];
    assert!(!super::attribute_constraints_conflict(&ok));

    let bad = vec![
        AttrConstraint {
            op: AttrOp::Exact,
            value: Some("baz".to_string()),
        },
        AttrConstraint {
            op: AttrOp::Includes,
            value: Some("foo bar".to_string()),
        },
    ];
    assert!(super::attribute_constraints_conflict(&bad));
}

#[test]
fn parse_attr_selector_parses_name_operator_and_value() {
    let sel = super::parse_attr_selector(r#"data-foo^="bar""#).unwrap();
    assert_eq!(sel.name, "data-foo");
    assert_eq!(sel.op, AttrOp::Prefix);
    assert_eq!(sel.value.as_deref(), Some("bar"));

    let sel_ws = super::parse_attr_selector("  tst  ~=foo").unwrap();
    assert_eq!(sel_ws.name, "tst");
    assert_eq!(sel_ws.op, AttrOp::Includes);
    assert_eq!(sel_ws.value.as_deref(), Some("foo"));

    let sel2 = super::parse_attr_selector("tst = 'X'").unwrap();
    assert_eq!(sel2.name, "tst");
    assert_eq!(sel2.op, AttrOp::Exact);
    assert_eq!(sel2.value.as_deref(), Some("X"));

    let sel3 = super::parse_attr_selector("tst~=foo").unwrap();
    assert_eq!(sel3.op, AttrOp::Includes);
    assert_eq!(sel3.value.as_deref(), Some("foo"));

    let sel4 = super::parse_attr_selector("tst|=foo").unwrap();
    assert_eq!(sel4.op, AttrOp::DashMatch);
    assert_eq!(sel4.value.as_deref(), Some("foo"));

    let sel5 = super::parse_attr_selector("tst$=foo").unwrap();
    assert_eq!(sel5.op, AttrOp::Suffix);
    assert_eq!(sel5.value.as_deref(), Some("foo"));

    let sel6 = super::parse_attr_selector("tst*=foo").unwrap();
    assert_eq!(sel6.op, AttrOp::Substring);
    assert_eq!(sel6.value.as_deref(), Some("foo"));

    let sel7 = super::parse_attr_selector("tst").unwrap();
    assert_eq!(sel7.op, AttrOp::Exists);
    assert!(sel7.value.is_none());

    let sel8 = super::parse_attr_selector("tst foo").unwrap();
    assert_eq!(sel8.op, AttrOp::Exists);
    assert!(sel8.value.is_none());

    assert!(super::parse_attr_selector("=foo").is_none());
    assert!(super::parse_attr_selector("~=foo").is_none());
    assert!(super::parse_attr_selector("tst=").is_none());
    assert!(super::parse_attr_selector("tst^=").is_none());
    assert!(super::parse_attr_selector("tst~=   ").is_none());
    assert!(super::parse_attr_selector("tst=\"unterminated").is_none());
}

#[test]
fn parse_attr_selector_unquoted_values_take_first_token() {
    let sel = super::parse_attr_selector("tst~=foo bar").unwrap();
    assert_eq!(sel.name, "tst");
    assert_eq!(sel.op, AttrOp::Includes);
    assert_eq!(sel.value.as_deref(), Some("foo"));
}

#[test]
fn parse_attr_selector_handles_utf8_in_attribute_name() {
    let sel = super::parse_attr_selector("🦀~=foo").unwrap();
    assert_eq!(sel.name, "🦀");
    assert_eq!(sel.op, AttrOp::Includes);
    assert_eq!(sel.value.as_deref(), Some("foo"));

    assert!(super::parse_attr_selector("🦀=\"unterminated").is_none());
}

#[test]
fn parse_attr_selector_quoted_values_respect_escapes() {
    let sel = super::parse_attr_selector(r#"tst="a\"b""#).unwrap();
    assert_eq!(sel.name, "tst");
    assert_eq!(sel.op, AttrOp::Exact);
    assert_eq!(sel.value.as_deref(), Some(r#"a\"b"#));
}

#[test]
fn attribute_constraints_conflict_flags_mismatched_exact_values() {
    let sels = vec![
        AttrConstraint {
            op: AttrOp::Exact,
            value: Some("a".to_string()),
        },
        AttrConstraint {
            op: AttrOp::Exact,
            value: Some("b".to_string()),
        },
    ];
    assert!(super::attribute_constraints_conflict(&sels));
}

#[test]
fn attribute_constraints_conflict_allows_matching_exact_values() {
    let sels = vec![
        AttrConstraint {
            op: AttrOp::Exact,
            value: Some("a".to_string()),
        },
        AttrConstraint {
            op: AttrOp::Exact,
            value: Some("a".to_string()),
        },
    ];
    assert!(!super::attribute_constraints_conflict(&sels));
}

#[test]
fn attribute_constraints_conflict_treats_exact_without_value_as_conflicting() {
    let sels = vec![
        AttrConstraint {
            op: AttrOp::Exact,
            value: None,
        },
        AttrConstraint {
            op: AttrOp::Exact,
            value: Some("a".to_string()),
        },
    ];
    assert!(super::attribute_constraints_conflict(&sels));
}

#[test]
fn attribute_constraints_conflict_detects_exact_value_regardless_of_position() {
    let sels = vec![
        AttrConstraint {
            op: AttrOp::Includes,
            value: Some("foo".to_string()),
        },
        AttrConstraint {
            op: AttrOp::Exact,
            value: Some("bar".to_string()),
        },
    ];
    assert!(super::attribute_constraints_conflict(&sels));
}

#[test]
fn attribute_constraints_conflict_allows_exact_value_satisfying_prefix() {
    let sels = vec![
        AttrConstraint {
            op: AttrOp::Prefix,
            value: Some("fo".to_string()),
        },
        AttrConstraint {
            op: AttrOp::Exact,
            value: Some("foo".to_string()),
        },
    ];
    assert!(!super::attribute_constraints_conflict(&sels));
}

#[test]
fn attribute_constraints_conflict_checks_all_pairs() {
    let sels = vec![
        AttrConstraint {
            op: AttrOp::Includes,
            value: Some("foo".to_string()),
        },
        AttrConstraint {
            op: AttrOp::Exists,
            value: None,
        },
        AttrConstraint {
            op: AttrOp::Includes,
            value: Some("bar".to_string()),
        },
    ];
    assert!(super::attribute_constraints_conflict(&sels));
}

#[test]
fn parse_http_response_decodes_chunked_body_when_indicated() {
    let resp = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n4\r\nWiki\r\n5\r\npedia\r\n0\r\n\r\n";
    let (code, _headers, body) = parse_http_response(resp).unwrap();
    assert_eq!(code, 200);
    assert_eq!(body, b"Wikipedia");
}

#[test]
fn parse_http_response_accepts_chunk_extensions() {
    let resp = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n4;ext=1\r\nWiki\r\n0\r\n\r\n";
    let (_code, _headers, body) = parse_http_response(resp).unwrap();
    assert_eq!(body, b"Wiki");
}

#[test]
fn parse_http_response_returns_raw_body_when_not_chunked() {
    let resp = b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello";
    let (code, _headers, body) = parse_http_response(resp).unwrap();
    assert_eq!(code, 200);
    assert_eq!(body, b"Hello");
}

#[test]
fn parse_http_response_applies_utf8_lossy_to_body() {
    // Preserve existing behavior: the body is interpreted as UTF-8 and invalid sequences are
    // replaced with U+FFFD (encoded as 0xEF 0xBF 0xBD).
    let resp = b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\n\xFF";
    let (_code, _headers, body) = parse_http_response(resp).unwrap();
    assert_eq!(body, b"\xEF\xBF\xBD");
}

#[test]
fn parse_http_response_matches_transfer_encoding_case_insensitively() {
    let resp = b"HTTP/1.1 200 OK\r\ntRaNsFeR-EnCoDiNg: gzip, CHUNKED\r\n\r\n4\r\nWiki\r\n0\r\n\r\n";
    let (_code, _headers, body) = parse_http_response(resp).unwrap();
    assert_eq!(body, b"Wiki");
}

#[test]
fn parse_http_response_detects_chunked_in_any_transfer_encoding_header() {
    let resp =
            b"HTTP/1.1 200 OK\r\nTransfer-Encoding: gzip\r\nTransfer-Encoding: chunked\r\n\r\n0\r\n\r\n";
    let (_code, _headers, body) = parse_http_response(resp).unwrap();
    assert_eq!(body, b"");
}

#[test]
fn parse_http_response_trims_header_whitespace() {
    let resp = b"HTTP/1.1 200 OK\r\nTransfer-Encoding :  chunked  \r\n\r\n0\r\n\r\n";
    let (_code, headers, body) = parse_http_response(resp).unwrap();
    assert_eq!(body, b"");
    assert!(
        headers
            .iter()
            .any(|(k, v)| k == "Transfer-Encoding" && v == "chunked")
    );
}

#[test]
fn parse_http_response_ignores_malformed_header_lines() {
    let resp = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\nBadHeader\r\n\r\n0\r\n\r\n";
    let (_code, headers, body) = parse_http_response(resp).unwrap();
    assert!(
        headers
            .iter()
            .any(|(k, v)| k == "Transfer-Encoding" && v == "chunked")
    );
    assert_eq!(body, b"");
}

#[test]
fn parse_http_response_rejects_missing_double_crlf() {
    let resp = b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nHello";
    let err = parse_http_response(resp).unwrap_err();
    assert!(matches!(
        err,
        super::ValidatorError::InvalidInput(ref s) if s == "invalid HTTP response"
    ));
}

#[test]
fn find_double_crlf_finds_first_header_body_split() {
    assert_eq!(find_double_crlf(b"abc\r\n\r\nxyz"), Some(3));
    assert_eq!(find_double_crlf(b"\r\n\r\n"), Some(0));
    assert_eq!(find_double_crlf(b"nope"), None);
    assert_eq!(find_double_crlf(b"\r\n\r"), None);
}

#[test]
fn memchr_crlf_respects_start_offset() {
    let data = b"a\r\nb\r\n";
    assert_eq!(memchr_crlf(data, 0), Some(1));
    assert_eq!(memchr_crlf(data, 2), Some(4));
    assert_eq!(memchr_crlf(data, data.len()), None);
    assert_eq!(memchr_crlf(data, data.len().saturating_sub(1)), None);
}

#[test]
fn border_side_component_flags_classifies_tokens_conservatively() {
    assert_eq!(border_side_component_flags(&[]), (false, false, false));
    assert_eq!(
        border_side_component_flags(&["1px", "2px"]),
        (true, false, false)
    );
    assert_eq!(
        border_side_component_flags(&["solid", "dotted"]),
        (false, true, false)
    );
    assert_eq!(
        border_side_component_flags(&["solid", "red"]),
        (false, true, true)
    );
    assert_eq!(border_side_component_flags(&["foo"]), (false, false, true));
}

#[test]
fn affected_border_longhands_expands_border_shorthand_in_order() {
    let tokens = ["1px", "solid", "red"];
    let affected = collect_affected_border_longhands("border", &tokens).unwrap();
    assert_eq!(
        affected,
        vec![
            "border-top-width",
            "border-top-style",
            "border-top-color",
            "border-right-width",
            "border-right-style",
            "border-right-color",
            "border-bottom-width",
            "border-bottom-style",
            "border-bottom-color",
            "border-left-width",
            "border-left-style",
            "border-left-color",
        ]
    );
}

#[test]
fn affected_border_longhands_respects_token_count_for_aggregates() {
    assert_eq!(
        collect_affected_border_longhands("border-color", &[]).unwrap(),
        vec!["border-top-color"]
    );
    assert_eq!(
        collect_affected_border_longhands("border-color", &["red", "green", "blue"]).unwrap(),
        vec![
            "border-top-color",
            "border-right-color",
            "border-bottom-color"
        ]
    );
    assert_eq!(
        collect_affected_border_longhands("border-width", &["1px", "2px"]).unwrap(),
        vec!["border-top-width", "border-right-width"]
    );
    assert_eq!(
        collect_affected_border_longhands(
            "border-style",
            &["solid", "dotted", "double", "groove", "ridge"]
        )
        .unwrap(),
        vec![
            "border-top-style",
            "border-right-style",
            "border-bottom-style",
            "border-left-style",
        ]
    );
}

#[test]
fn affected_border_longhands_handles_side_shorthands_and_longhands_case_insensitively() {
    assert_eq!(
        collect_affected_border_longhands("BoRdEr-ToP", &["1px", "red"]).unwrap(),
        vec!["border-top-width", "border-top-color"]
    );
    assert_eq!(
        collect_affected_border_longhands("BORDER-RIGHT-WIDTH", &["1px"]).unwrap(),
        vec!["border-right-width"]
    );
}

#[test]
fn affected_border_longhands_rejects_unknown_border_side_kinds() {
    assert_eq!(
        collect_affected_border_longhands("border-top-foo", &["1px"]),
        None
    );
    assert_eq!(
        collect_affected_border_longhands("border-middle-width", &["1px"]),
        None
    );
}

#[test]
fn affected_border_longhands_returns_none_for_unrelated_properties() {
    assert_eq!(collect_affected_border_longhands("color", &["red"]), None);
    assert_eq!(
        collect_affected_border_longhands("background", &["red"]),
        None
    );
}

#[test]
fn warns_on_border_redefinition_per_affected_longhand_at_level_2() {
    let mut cfg = Config::default();
    cfg.warning = Some("2".to_string());
    let report = validate_css_text(
        "a { border-top: 1px solid red; border-top: 2px solid red; }",
        &cfg,
    )
    .unwrap();
    assert_eq!(report.errors, 0);
    assert_eq!(report.warnings, 3);
    assert_eq!(
        report
            .messages
            .iter()
            .filter(|m| m.message == "Property redefined.")
            .count(),
        3
    );
}

#[test]
fn warns_on_top_level_import_in_text_mode() {
    let report = validate_css_text("@import url(x); a{color:red}", &Config::default()).unwrap();
    assert_eq!(report.errors, 0);
    assert_eq!(report.warnings, 1);
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.message == "Imported style sheets are not checked.")
    );
}

#[test]
fn does_not_warn_on_nested_import_in_text_mode() {
    let report = validate_css_text(
        "@media screen { @import url(x); a{color:red} }",
        &Config::default(),
    )
    .unwrap();
    assert_eq!(report.errors, 0);
    assert_eq!(report.warnings, 0);
}

#[test]
fn warns_on_media_rules_that_do_not_match_user_medium() {
    let mut cfg = Config::default();
    cfg.medium = Some("screen".to_string());
    let report = validate_css_text("@media print { a{color:red} }", &cfg).unwrap();
    assert_eq!(report.errors, 0);
    assert_eq!(report.warnings, 1);
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.message == "Properties for other media might not work for usermedium.")
    );
}

#[test]
fn does_not_warn_on_media_rules_for_all_or_matching_user_medium() {
    let mut cfg = Config::default();
    cfg.medium = Some("all".to_string());
    let report = validate_css_text("@media print { a{color:red} }", &cfg).unwrap();
    assert_eq!(report.errors, 0);
    assert_eq!(report.warnings, 0);

    let mut cfg = Config::default();
    cfg.medium = Some("print".to_string());
    let report = validate_css_text("@media print { a{color:red} }", &cfg).unwrap();
    assert_eq!(report.errors, 0);
    assert_eq!(report.warnings, 0);

    let mut cfg = Config::default();
    cfg.medium = Some("print,screen".to_string());
    let report = validate_css_text("@media print { a{color:red} }", &cfg).unwrap();
    assert_eq!(report.errors, 0);
    assert_eq!(report.warnings, 0);
}

#[test]
fn validates_imported_stylesheets_via_fetcher() {
    let base = "http://example.com/dir/main.css";
    let import_uri = "http://example.com/dir/a.css";
    let mut map = std::collections::HashMap::new();
    map.insert(import_uri.to_string(), b"c { color: red;".to_vec());
    let fetcher = MapFetcher(map);

    let report = validate_css_text_with_fetcher(
        "@import \"a.css\"; b { color: red; }",
        Some(base),
        &Config::default(),
        &fetcher,
    )
    .unwrap();
    assert_eq!(report.errors, 1);
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.message == "Unbalanced braces.")
    );
}

#[test]
fn imported_stylesheet_messages_come_before_main_sheet_messages() {
    let base = "http://example.com/dir/main.css";
    let import_uri = "http://example.com/dir/a.css";
    let mut map = std::collections::HashMap::new();
    map.insert(import_uri.to_string(), b"<".to_vec());
    let fetcher = MapFetcher(map);

    let report = validate_css_text_with_fetcher(
        "@import \"a.css\"; b { color: red;",
        Some(base),
        &Config::default(),
        &fetcher,
    )
    .unwrap();

    let invalid_pos = report
        .messages
        .iter()
        .position(|m| m.message == "Invalid input.")
        .expect("import should produce Invalid input");
    let unbalanced_pos = report
        .messages
        .iter()
        .position(|m| m.message == "Unbalanced braces.")
        .expect("main sheet should produce Unbalanced braces");
    assert!(invalid_pos < unbalanced_pos, "{:?}", report.messages);
}

#[test]
fn reports_import_loops_as_errors() {
    let base = "http://example.com/dir/main.css";
    let import_uri = "http://example.com/dir/a.css";
    let mut map = std::collections::HashMap::new();
    map.insert(
        import_uri.to_string(),
        b"@import \"main.css\"; c { color: red; }".to_vec(),
    );
    let fetcher = MapFetcher(map);

    let report = validate_css_text_with_fetcher(
        "@import \"a.css\"; b { color: red; }",
        Some(base),
        &Config::default(),
        &fetcher,
    )
    .unwrap();

    assert_eq!(report.errors, 1);
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.message == "Import loop detected.")
    );
}

#[test]
fn validate_css_uri_with_fetcher_uses_uri_as_base_for_imports_and_loop_detection() {
    let main_uri = "http://example.com/dir/main.css";
    let import_uri = "http://example.com/dir/a.css";
    let mut map = std::collections::HashMap::new();
    map.insert(main_uri.to_string(), b"@import \"a.css\";".to_vec());
    map.insert(import_uri.to_string(), b"@import \"main.css\";".to_vec());
    let fetcher = MapFetcher(map);

    let report = validate_css_uri_with_fetcher(main_uri, &Config::default(), &fetcher).unwrap();

    assert_eq!(report.errors, 1);
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.message == "Import loop detected.")
    );
}

#[test]
fn reports_duplicate_imports_as_errors() {
    let base = "http://example.com/dir/main.css";
    let import_uri = "http://example.com/dir/a.css";
    let mut map = std::collections::HashMap::new();
    map.insert(import_uri.to_string(), b"c { color: red; }".to_vec());
    let fetcher = MapFetcher(map);

    let report = validate_css_text_with_fetcher(
        "@import \"a.css\"; @import \"a.css\"; b { color: red; }",
        Some(base),
        &Config::default(),
        &fetcher,
    )
    .unwrap();

    assert_eq!(report.errors, 1);
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.message == "Import loop detected.")
    );
}

#[test]
fn ignores_empty_import_urls() {
    let base = "http://example.com/dir/main.css";
    let fetcher = CountingFetcher::new(std::collections::HashMap::new());

    let report = validate_css_text_with_fetcher(
        "@import \"\"; b { color: red; }",
        Some(base),
        &Config::default(),
        &fetcher,
    )
    .unwrap();

    assert_eq!(fetcher.calls.get(), 0);
    assert_eq!(report.errors, 0, "{:?}", report.messages);
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.message.starts_with("Failed to fetch @import:"))
    );
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.message == "Import loop detected.")
    );
}

#[test]
fn duplicate_imports_do_not_fetch_second_time() {
    let base = "http://example.com/dir/main.css";
    let import_uri = "http://example.com/dir/a.css";
    let mut map = std::collections::HashMap::new();
    map.insert(import_uri.to_string(), b"c { color: red; }".to_vec());
    let fetcher = CountingFetcher::new(map);

    let report = validate_css_text_with_fetcher(
        "@import \"a.css\"; @import \"a.css\"; b { color: red; }",
        Some(base),
        &Config::default(),
        &fetcher,
    )
    .unwrap();

    assert_eq!(report.errors, 1);
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.message == "Import loop detected.")
    );
    assert_eq!(fetcher.calls.get(), 1);
}

#[test]
fn reports_failed_import_fetches_as_errors() {
    let base = "http://example.com/dir/main.css";
    let fetcher = MapFetcher(std::collections::HashMap::new());

    let report = validate_css_text_with_fetcher(
        "@import \"missing.css\"; b { color: red; }",
        Some(base),
        &Config::default(),
        &fetcher,
    )
    .unwrap();

    assert_eq!(report.errors, 1);
    assert!(report.messages.iter().any(|m| {
        m.message.starts_with("Failed to fetch @import:")
            && m.message
                .contains("missing: http://example.com/dir/missing.css")
    }));
}

#[test]
fn validates_nested_imports_recursively() {
    let base = "http://example.com/dir/main.css";
    let a_uri = "http://example.com/dir/a.css";
    let b_uri = "http://example.com/dir/b.css";
    let mut map = std::collections::HashMap::new();
    map.insert(a_uri.to_string(), b"@import \"b.css\"; a {".to_vec());
    map.insert(b_uri.to_string(), b"b { color: red;".to_vec());
    let fetcher = CountingFetcher::new(map);

    let report = validate_css_text_with_fetcher(
        "@import \"a.css\"; main { color: red; }",
        Some(base),
        &Config::default(),
        &fetcher,
    )
    .unwrap();

    // a.css has unbalanced braces, b.css has unbalanced braces, main sheet is valid.
    assert_eq!(report.errors, 2, "{:?}", report.messages);
    assert_eq!(fetcher.calls.get(), 2);
    let unbalanced = report
        .messages
        .iter()
        .filter(|m| m.severity == super::Severity::Error && m.message == "Unbalanced braces.")
        .count();
    assert_eq!(unbalanced, 2, "{:?}", report.messages);
}

#[test]
fn nested_imports_use_imported_uri_as_base() {
    let base = "http://example.com/main.css";
    let a_uri = "http://example.com/dir/a.css";
    let b_uri = "http://example.com/dir/b.css";
    let mut map = std::collections::HashMap::new();
    map.insert(a_uri.to_string(), b"@import \"b.css\"; a {".to_vec());
    map.insert(b_uri.to_string(), b"b {".to_vec());
    let fetcher = CountingFetcher::new(map);

    let report = validate_css_text_with_fetcher(
        "@import \"dir/a.css\"; main { color: red; }",
        Some(base),
        &Config::default(),
        &fetcher,
    )
    .unwrap();

    // a.css has unbalanced braces, b.css has unbalanced braces, main sheet is valid.
    assert_eq!(report.errors, 2, "{:?}", report.messages);
    assert_eq!(fetcher.calls.get(), 2);
    assert!(
        !report
            .messages
            .iter()
            .any(|m| m.message.starts_with("Failed to fetch @import:"))
    );
}

#[test]
fn failed_import_fetch_does_not_short_circuit_main_sheet_validation() {
    let base = "http://example.com/dir/main.css";
    let fetcher = MapFetcher(std::collections::HashMap::new());

    let report = validate_css_text_with_fetcher(
        "@import \"missing.css\"; b { color: red;",
        Some(base),
        &Config::default(),
        &fetcher,
    )
    .unwrap();

    assert_eq!(report.errors, 2, "{:?}", report.messages);
    assert!(report.messages.iter().any(|m| {
        m.message.starts_with("Failed to fetch @import:")
            && m.message
                .contains("missing: http://example.com/dir/missing.css")
    }));
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.message == "Unbalanced braces.")
    );
}

#[test]
fn reports_unclosed_comment_in_imported_stylesheet_as_error() {
    let base = "http://example.com/dir/main.css";
    let import_uri = "http://example.com/dir/a.css";
    let mut map = std::collections::HashMap::new();
    map.insert(import_uri.to_string(), b"c { color: red; /*".to_vec());
    let fetcher = MapFetcher(map);

    let report = validate_css_text_with_fetcher(
        "@import \"a.css\"; b { color: red; }",
        Some(base),
        &Config::default(),
        &fetcher,
    )
    .unwrap();

    assert_eq!(report.errors, 1);
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.message == "Unclosed comment in @import.")
    );
}

#[test]
fn unclosed_comment_in_import_does_not_short_circuit_main_sheet_validation() {
    let base = "http://example.com/dir/main.css";
    let import_uri = "http://example.com/dir/a.css";
    let mut map = std::collections::HashMap::new();
    map.insert(import_uri.to_string(), b"c { color: red; /*".to_vec());
    let fetcher = MapFetcher(map);

    let report = validate_css_text_with_fetcher(
        "@import \"a.css\"; b { color: red;",
        Some(base),
        &Config::default(),
        &fetcher,
    )
    .unwrap();

    assert_eq!(report.errors, 2, "{:?}", report.messages);
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.message == "Unclosed comment in @import.")
    );
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.message == "Unbalanced braces.")
    );
}

#[test]
fn strips_important_case_insensitively() {
    let report = super::validate_css_declarations_text(
        "float: LEFT !IMPORTANT; color: red !important",
        &Config::default(),
    )
    .unwrap();
    assert_eq!(report.errors, 0);
}

#[test]
fn outline_shorthand_accepts_case_insensitive_keywords_and_rejects_duplicates() {
    let report =
        super::validate_css_declarations_text("outline: THIN SoLiD red;", &Config::default())
            .unwrap();
    assert_eq!(report.errors, 0);

    let report =
        super::validate_css_declarations_text("outline: thin thick red;", &Config::default())
            .unwrap();
    assert_eq!(report.errors, 1);
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.message == "Invalid value for property “outline”.")
    );
}

#[test]
fn can_validate_file_url() {
    use std::time::{SystemTime, UNIX_EPOCH};
    let name = format!(
        "css_inspector_test_{}_{}.css",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let path = std::env::temp_dir().join(name);
    std::fs::write(&path, "a { color: red; }").unwrap();
    let uri = format!("file://{}", path.to_string_lossy());
    let report =
        validate_css_uri_with_fetcher(&uri, &Default::default(), &StdFetcher::default()).unwrap();
    assert_eq!(report.errors, 0);
}

#[test]
fn filter_reports_single_error_for_generic_values() {
    let report =
        super::validate_css_declarations_text("filter: blur(1px);", &Config::default()).unwrap();
    assert_eq!(report.errors, 1);
}

#[test]
fn filter_reports_single_error_for_empty_value() {
    let report = super::validate_css_declarations_text("filter: ;", &Config::default()).unwrap();
    assert_eq!(report.errors, 1);
}

#[test]
fn filter_reports_four_errors_for_progid() {
    let report = super::validate_css_declarations_text(
        "filter: progid:DXImageTransform.Microsoft.Blur();",
        &Config::default(),
    )
    .unwrap();
    assert_eq!(report.errors, 4);
}

#[test]
fn filter_reports_single_error_when_progid_prefix_is_split_by_whitespace() {
    let report = super::validate_css_declarations_text(
        "filter: progid:DXImageTransform. Microsoft.Blur();",
        &Config::default(),
    )
    .unwrap();
    assert_eq!(report.errors, 1);
}

#[test]
fn dash_match_prefix_accepts_exact_or_dash_suffix() {
    assert!(dash_match_prefix("en", "en"));
    assert!(dash_match_prefix("en-us", "en"));
    assert!(!dash_match_prefix("english", "en"));
    assert!(!dash_match_prefix("en", "en-us"));
}

#[test]
fn dash_match_prefix_handles_empty_dash_value() {
    assert!(dash_match_prefix("-", ""));
    assert!(dash_match_prefix("", ""));
    assert!(!dash_match_prefix("x", ""));
}

#[test]
fn unquote_strips_matching_quotes_after_trim() {
    assert_eq!(super::unquote(" \"foo\" "), "foo");
    assert_eq!(super::unquote("'bar'"), "bar");
    assert_eq!(super::unquote("''"), "");
}

#[test]
fn unquote_leaves_unmatched_quotes_but_trims() {
    assert_eq!(super::unquote(" \"foo"), "\"foo");
    assert_eq!(super::unquote("bar' "), "bar'");
    assert_eq!(super::unquote(" \"foo' "), "\"foo'");
}

#[test]
fn decode_chunked_decodes_basic_payload() {
    let decoded = super::decode_chunked(b"4\r\nWiki\r\n5\r\npedia\r\n0\r\n\r\n").unwrap();
    assert_eq!(decoded, b"Wikipedia");
}

#[test]
fn decode_chunked_tolerates_missing_crlf_after_chunk_data() {
    let decoded = super::decode_chunked(b"4\r\nWiki0\r\n\r\n").unwrap();
    assert_eq!(decoded, b"Wiki");
}

#[test]
fn decode_chunked_tolerates_input_truncated_after_chunk_data() {
    let decoded = super::decode_chunked(b"4\r\nWiki").unwrap();
    assert_eq!(decoded, b"Wiki");
}

#[test]
fn decode_chunked_ignores_chunk_extensions_and_detects_truncation() {
    let decoded = super::decode_chunked(b"4;ext=1\r\nWiki\r\n0\r\n\r\n").unwrap();
    assert_eq!(decoded, b"Wiki");

    let decoded = super::decode_chunked(b"4;ext=1;foo=bar\r\nWiki\r\n0\r\n\r\n").unwrap();
    assert_eq!(decoded, b"Wiki");

    let err = super::decode_chunked(b"4\r\nWi").unwrap_err();
    assert!(matches!(err, super::ValidatorError::InvalidInput(ref s) if s == "truncated chunk"));
}

#[test]
fn parse_http_url_parses_host_port_and_path() {
    let (host, port, path) = super::parse_http_url("http://example.com/a/b").unwrap();
    assert_eq!(host, "example.com");
    assert_eq!(port, 80);
    assert_eq!(path, "/a/b");

    let (host, port, path) = super::parse_http_url("http://example.com:8080/a").unwrap();
    assert_eq!(host, "example.com");
    assert_eq!(port, 8080);
    assert_eq!(path, "/a");
}

#[test]
fn parse_http_url_preserves_double_slashes_in_path() {
    let (host, port, path) = super::parse_http_url("http://example.com//a.css").unwrap();
    assert_eq!(host, "example.com");
    assert_eq!(port, 80);
    assert_eq!(path, "//a.css");

    let (host, port, path) = super::parse_http_url("http:////a.css").unwrap();
    assert_eq!(host, "");
    assert_eq!(port, 80);
    assert_eq!(path, "//a.css");
}

#[test]
fn parse_http_url_preserves_query_string_and_fragment_in_path() {
    let (host, port, path) = super::parse_http_url("http://example.com/a?b=c#frag").unwrap();
    assert_eq!(host, "example.com");
    assert_eq!(port, 80);
    assert_eq!(path, "/a?b=c#frag");
}

#[test]
fn parse_http_url_defaults_path_to_slash() {
    let (host, port, path) = super::parse_http_url("http://example.com").unwrap();
    assert_eq!(host, "example.com");
    assert_eq!(port, 80);
    assert_eq!(path, "/");

    let (host, port, path) = super::parse_http_url("http://example.com/").unwrap();
    assert_eq!(host, "example.com");
    assert_eq!(port, 80);
    assert_eq!(path, "/");

    let (host, port, path) = super::parse_http_url("http://example.com:8080").unwrap();
    assert_eq!(host, "example.com");
    assert_eq!(port, 8080);
    assert_eq!(path, "/");

    let (host, port, path) = super::parse_http_url("http://example.com:8080/").unwrap();
    assert_eq!(host, "example.com");
    assert_eq!(port, 8080);
    assert_eq!(path, "/");

    let (host, port, path) = super::parse_http_url("http://:8080").unwrap();
    assert_eq!(host, "");
    assert_eq!(port, 8080);
    assert_eq!(path, "/");
}

#[test]
fn parse_http_url_accepts_empty_host() {
    let (host, port, path) = super::parse_http_url("http:///a").unwrap();
    assert_eq!(host, "");
    assert_eq!(port, 80);
    assert_eq!(path, "/a");

    let (host, port, path) = super::parse_http_url("http:///").unwrap();
    assert_eq!(host, "");
    assert_eq!(port, 80);
    assert_eq!(path, "/");
}

#[test]
fn parse_http_url_accepts_empty_host_with_port() {
    let (host, port, path) = super::parse_http_url("http://:8080/a").unwrap();
    assert_eq!(host, "");
    assert_eq!(port, 8080);
    assert_eq!(path, "/a");
}

#[test]
fn split_http_base_splits_scheme_host_and_path() {
    let (scheme_host, path) = super::split_http_base("http://example.com/a/b.css").unwrap();
    assert_eq!(scheme_host, "http://example.com");
    assert_eq!(path, "/a/b.css");

    let (scheme_host, path) = super::split_http_base("http://example.com/a?b=c#frag").unwrap();
    assert_eq!(scheme_host, "http://example.com");
    assert_eq!(path, "/a?b=c#frag");

    let (scheme_host, path) = super::split_http_base("http://example.com//a.css").unwrap();
    assert_eq!(scheme_host, "http://example.com");
    assert_eq!(path, "//a.css");

    let (scheme_host, path) = super::split_http_base("http://example.com:8080/a.css").unwrap();
    assert_eq!(scheme_host, "http://example.com:8080");
    assert_eq!(path, "/a.css");

    let (scheme_host, path) = super::split_http_base("http://example.com/").unwrap();
    assert_eq!(scheme_host, "http://example.com");
    assert_eq!(path, "/");

    let (scheme_host, path) = super::split_http_base("http://example.com:8080/").unwrap();
    assert_eq!(scheme_host, "http://example.com:8080");
    assert_eq!(path, "/");

    let (scheme_host, path) = super::split_http_base("http://example.com").unwrap();
    assert_eq!(scheme_host, "http://example.com");
    assert_eq!(path, "/");
    assert!(super::split_http_base("http://example.com?x").is_none());
    assert!(super::split_http_base("http://example.com#frag").is_none());
    assert!(super::split_http_base("HTTP://example.com/a").is_none());
    let (scheme_host, path) = super::split_http_base("https://example.com/a").unwrap();
    assert_eq!(scheme_host, "https://example.com");
    assert_eq!(path, "/a");
    let (scheme_host, path) = super::split_http_base("https://example.com").unwrap();
    assert_eq!(scheme_host, "https://example.com");
    assert_eq!(path, "/");
}

#[test]
fn parse_http_url_and_split_http_base_handle_utf8_hosts() {
    let (host, port, path) = super::parse_http_url("http://❤/a").unwrap();
    assert_eq!(host, "❤");
    assert_eq!(port, 80);
    assert_eq!(path, "/a");

    let (host, port, path) = super::parse_http_url("http://❤:8080/a").unwrap();
    assert_eq!(host, "❤");
    assert_eq!(port, 8080);
    assert_eq!(path, "/a");

    let (host, port, path) = super::parse_http_url("http://❤:8080").unwrap();
    assert_eq!(host, "❤");
    assert_eq!(port, 8080);
    assert_eq!(path, "/");

    let (scheme_host, path) = super::split_http_base("http://❤/a.css").unwrap();
    assert_eq!(scheme_host, "http://❤");
    assert_eq!(path, "/a.css");
}

#[test]
fn parse_http_url_and_split_http_base_preserve_double_slashes_with_utf8_host() {
    let (host, port, path) = super::parse_http_url("http://❤//a.css").unwrap();
    assert_eq!(host, "❤");
    assert_eq!(port, 80);
    assert_eq!(path, "//a.css");

    let (scheme_host, path) = super::split_http_base("http://❤//a.css").unwrap();
    assert_eq!(scheme_host, "http://❤");
    assert_eq!(path, "//a.css");
}

#[test]
fn split_http_base_requires_a_path_separator() {
    assert!(super::split_http_base("http://example.com?x").is_none());
    assert!(super::split_http_base("http://example.com#frag").is_none());
}

#[test]
fn split_http_base_accepts_empty_host() {
    let (scheme_host, path) = super::split_http_base("http:///a/b").unwrap();
    assert_eq!(scheme_host, "http://");
    assert_eq!(path, "/a/b");

    let (scheme_host, path) = super::split_http_base("http:////a/b").unwrap();
    assert_eq!(scheme_host, "http://");
    assert_eq!(path, "//a/b");

    let (scheme_host, path) = super::split_http_base("http:///").unwrap();
    assert_eq!(scheme_host, "http://");
    assert_eq!(path, "/");
}

#[test]
fn parse_http_url_rejects_non_http_urls_and_invalid_ports() {
    let err = super::parse_http_url("https://example.com").unwrap_err();
    assert!(matches!(err, super::ValidatorError::InvalidInput(ref s) if s == "not an http:// URL"));

    let err = super::parse_http_url("http://example.com:nope/").unwrap_err();
    assert!(
        matches!(err, super::ValidatorError::InvalidInput(ref s) if s == "invalid port in URL: http://example.com:nope/")
    );

    let err = super::parse_http_url("http://example.com:/a").unwrap_err();
    assert!(
        matches!(err, super::ValidatorError::InvalidInput(ref s) if s == "invalid port in URL: http://example.com:/a")
    );

    let err = super::parse_http_url("http://example.com:").unwrap_err();
    assert!(
        matches!(err, super::ValidatorError::InvalidInput(ref s) if s == "invalid port in URL: http://example.com:")
    );
}

#[test]
fn file_url_to_path_accepts_absolute_and_host_paths() {
    assert_eq!(
        super::file_url_to_path("file:///tmp/a.css").unwrap(),
        "/tmp/a.css"
    );
    assert_eq!(
        super::file_url_to_path("file://localhost/tmp/a.css").unwrap(),
        "//localhost/tmp/a.css"
    );

    let err = super::file_url_to_path("http://example.com/a.css").unwrap_err();
    assert!(matches!(err, super::ValidatorError::InvalidInput(ref s) if s == "not a file:// URL"));

    let err = super::file_url_to_path("file://localhost").unwrap_err();
    assert!(
        matches!(err, super::ValidatorError::InvalidInput(ref s) if s == "invalid file URL: file://localhost")
    );
}

#[test]
fn resolve_relative_uri_resolves_against_http_and_file_bases() {
    assert_eq!(super::resolve_relative_uri(None, "c.css"), "c.css");
    assert_eq!(
        super::resolve_relative_uri(Some("http://example.com/a/b.css"), "  c.css  "),
        "http://example.com/a/c.css"
    );
    assert_eq!(
        super::resolve_relative_uri(Some("http://example.com/a/b.css"), "c.css"),
        "http://example.com/a/c.css"
    );
    assert_eq!(
        super::resolve_relative_uri(Some("http://example.com/a/"), "c.css"),
        "http://example.com/a/c.css"
    );
    assert_eq!(
        super::resolve_relative_uri(Some("file:///tmp/a/b.css"), "\n\tc.css\t"),
        "file:///tmp/a/c.css"
    );
    assert_eq!(
        super::resolve_relative_uri(Some("file:///tmp/a/b.css"), "c.css"),
        "file:///tmp/a/c.css"
    );
    assert_eq!(
        super::resolve_relative_uri(Some("file:///tmp/a/"), "c.css"),
        "file:///tmp/a/c.css"
    );
    assert_eq!(
        super::resolve_relative_uri(Some("file://localhost/tmp/a/b.css"), "c.css"),
        "file:////localhost/tmp/a/c.css"
    );
    assert_eq!(
        super::resolve_relative_uri(Some("HTTP://example.com/a/b.css"), "c.css"),
        "c.css"
    );
    assert_eq!(
        super::resolve_relative_uri(Some("FILE:///tmp/a/b.css"), "c.css"),
        "c.css"
    );
    assert_eq!(
        super::resolve_relative_uri(Some("http://example.com/a/b.css"), "http://cdn/x.css"),
        "http://cdn/x.css"
    );
    assert_eq!(
        super::resolve_relative_uri(Some("http://example.com/a/b.css"), "  HTTPS://cdn/x.css  "),
        "HTTPS://cdn/x.css"
    );
    assert_eq!(
        super::resolve_relative_uri(Some("http://example.com"), "c.css"),
        "http://example.com/c.css"
    );
    assert_eq!(
        super::resolve_relative_uri(Some("https://example.com/a/b.css"), "c.css"),
        "https://example.com/a/c.css"
    );
    assert_eq!(
        super::resolve_relative_uri(Some("https://example.com"), "c.css"),
        "https://example.com/c.css"
    );
}

#[test]
fn resolve_relative_uri_falls_back_when_file_base_is_invalid() {
    assert_eq!(
        super::resolve_relative_uri(Some("file://localhost"), "c.css"),
        "c.css"
    );
}

#[test]
fn url_function_token_accepts_mixed_case_url_prefix() {
    assert!(super::is_valid_url_function_token("URL(x)"));
    assert!(super::is_valid_url_function_token("uRl(\"x\")"));
    assert!(!super::is_valid_url_function_token("URl(x"));
}

#[test]
fn url_function_token_handles_quoted_and_unquoted_edge_cases() {
    assert!(super::is_valid_url_function_token("url(\"\")"));
    assert!(super::is_valid_url_function_token("url('x')"));
    assert!(!super::is_valid_url_function_token("url(')"));
    assert!(!super::is_valid_url_function_token("url(x\\\\)"));
    assert!(!super::is_valid_url_function_token("url(\"x\\\")"));
    assert!(super::is_valid_url_function_token("url(\"x\\\\\")"));
}

#[test]
fn azimuth_and_elevation_keywords_match_case_insensitively() {
    let cfg = Config::default();
    assert_eq!(
        validate_css_text("a{azimuth: LEFT}", &cfg).unwrap().errors,
        0
    );
    assert_eq!(
        validate_css_text("a{elevation: BELOW}", &cfg)
            .unwrap()
            .errors,
        0
    );
}

#[test]
fn ascii_lowercase_cow_borrows_when_no_uppercase() {
    let value = super::ascii_lowercase_cow("abc-123");
    assert!(matches!(value, Cow::Borrowed("abc-123")));
}

#[test]
fn ascii_lowercase_cow_owns_when_uppercase_present() {
    let value = super::ascii_lowercase_cow("AbC");
    assert!(matches!(value, Cow::Owned(ref s) if s == "abc"));
}

#[test]
fn warning_level_defaults_or_parses() {
    let mut cfg = Config::default();
    assert_eq!(super::warning_level_from_config(&cfg), 0);
    cfg.warning = Some("2".to_string());
    assert_eq!(super::warning_level_from_config(&cfg), 2);
    cfg.warning = Some("nope".to_string());
    assert_eq!(super::warning_level_from_config(&cfg), 0);
}

#[test]
fn css1_escapes_matches_profile_case_insensitively() {
    let mut cfg = Config::default();
    assert!(!super::css1_escapes_from_config(&cfg));
    cfg.profile = Some("css1".to_string());
    assert!(super::css1_escapes_from_config(&cfg));
    cfg.profile = Some("CSS1".to_string());
    assert!(super::css1_escapes_from_config(&cfg));
}

#[test]
fn font_shorthand_accepts_common_forms() {
    let cfg = Config::default();
    assert_eq!(
        validate_css_text("a{font: caption}", &cfg).unwrap().errors,
        0
    );
    assert_eq!(
        validate_css_text("a{font: 12px serif}", &cfg)
            .unwrap()
            .errors,
        0
    );
    assert_eq!(
        validate_css_text("a{font: italic 12px/14px serif}", &cfg)
            .unwrap()
            .errors,
        0
    );
}

#[test]
fn font_shorthand_accepts_numeric_weight_and_unitless_line_height() {
    let cfg = Config::default();
    assert_eq!(
        validate_css_text("a{font: 700 12px/1.2 serif}", &cfg)
            .unwrap()
            .errors,
        0
    );
}

#[test]
fn font_shorthand_rejects_invalid_numeric_weight() {
    let cfg = Config::default();
    assert_eq!(
        validate_css_text("a{font: 950 12px serif}", &cfg)
            .unwrap()
            .errors,
        1
    );
}

#[test]
fn font_shorthand_rejects_missing_family_and_bad_prefix() {
    let cfg = Config::default();
    assert_eq!(
        validate_css_text("a{font: italic 12px}", &cfg)
            .unwrap()
            .errors,
        1
    );
    assert_eq!(
        validate_css_text("a{font: foo 12px serif}", &cfg)
            .unwrap()
            .errors,
        1
    );
}

#[test]
fn border_shorthand_rejects_duplicate_components() {
    let cfg = Config::default();
    assert_eq!(
        validate_css_text("a{border: solid solid red}", &cfg)
            .unwrap()
            .errors,
        1
    );
    assert_eq!(
        validate_css_text("a{border: 1px 2px solid}", &cfg)
            .unwrap()
            .errors,
        1
    );
}

#[test]
fn border_shorthand_rejects_too_many_tokens() {
    let cfg = Config::default();
    assert_eq!(
        validate_css_text("a{border: 1px solid red extra}", &cfg)
            .unwrap()
            .errors,
        1
    );
}
