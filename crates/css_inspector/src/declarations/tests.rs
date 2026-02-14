#[allow(unused_imports)]
use super::*;

mod iter_declaration_statements_skipping_nested_blocks_tests {
    use super::{KnownProperties, iter_declaration_statements_skipping_nested_blocks};

    fn collect_trimmed_statements(block: &str) -> Vec<&str> {
        let known_properties = KnownProperties::default();
        iter_declaration_statements_skipping_nested_blocks(block, &known_properties)
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .collect()
    }

    #[test]
    fn yields_declarations_before_and_after_nested_rules() {
        let block = r#"
color: red;
.child { no-such-prop: 1; }
width: 1px;
"#;
        assert_eq!(
            collect_trimmed_statements(block),
            vec!["color: red", "width: 1px"]
        );
    }

    #[test]
    fn skips_nested_blocks_with_internal_semicolons_and_nested_braces() {
        let block = r#"
color: red;
@media screen {
  .a { color: blue; }
  @supports (display: grid) { b { margin: 0; } }
}
height: 2px;
"#;
        assert_eq!(
            collect_trimmed_statements(block),
            vec!["color: red", "height: 2px"]
        );
    }

    #[test]
    fn does_not_treat_braces_inside_strings_as_nested_blocks() {
        let block = r#"content: "a{b}c"; color: red;"#;
        assert_eq!(
            collect_trimmed_statements(block),
            vec![r#"content: "a{b}c""#, "color: red"]
        );
    }

    #[test]
    fn returns_trailing_statement_without_semicolon() {
        assert_eq!(collect_trimmed_statements("color: red"), vec!["color: red"]);
    }

    #[test]
    fn handles_multiple_nested_blocks_back_to_back() {
        let block = r#"
a { color: red; }
@media screen { b { color: blue; } }
width: 1px;
"#;
        assert_eq!(collect_trimmed_statements(block), vec!["width: 1px"]);
    }

    #[test]
    fn does_not_treat_value_blocks_as_nested_rules() {
        let mut known_properties = KnownProperties::default();
        known_properties.insert("color".into());
        known_properties.insert("background-color".into());

        let block = r#"
color:{var(--x)};
color:A { var(--x) };
background-color:red;
"#;
        let statements: Vec<_> =
            iter_declaration_statements_skipping_nested_blocks(block, &known_properties)
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .collect();
        assert_eq!(
            statements,
            vec![
                "color:{var(--x)}",
                "color:A { var(--x) }",
                "background-color:red"
            ]
        );
    }

    #[test]
    fn does_not_split_on_semicolons_inside_brackets() {
        let block = r#"
color: [;] var(--a);
background-color: red;
"#;
        assert_eq!(
            collect_trimmed_statements(block),
            vec!["color: [;] var(--a)", "background-color: red"]
        );
    }
}

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
    fn value_syntax_validation_rejects_unbalanced_brackets_for_known_properties() {
        let report =
            validate_css_declarations_text("width: [1px; height: 1px;", &Config::default())
                .unwrap();
        assert_eq!(report.errors, 1, "{report:?}");
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.message == "Invalid value for property “width”."),
            "{report:?}"
        );
    }

    #[test]
    fn value_syntax_validation_is_skipped_for_custom_properties() {
        let report =
            validate_css_declarations_text("--x: calc(1px; width: 1px;", &Config::default())
                .unwrap();
        assert_eq!(report.errors, 0, "{report:?}");
        assert_eq!(report.warnings, 0, "{report:?}");
        assert!(report.messages.is_empty(), "{report:?}");
    }

    #[test]
    fn unknown_property_is_reported_at_most_once_per_block() {
        let config = Config {
            profile: Some("css3".to_string()),
            ..Config::default()
        };
        let report = validate_css_declarations_text("foo: 1; foo: 2", &config).unwrap();
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
        let config = Config {
            profile: Some("css3".to_string()),
            ..Config::default()
        };
        let report =
            validate_css_declarations_text("-webkit-foo: 1; _bar: 2; zoom: 3", &config).unwrap();
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
        let cfg = Config {
            warning: Some("-1".to_string()),
            ..Config::default()
        };
        let report =
            validate_css_declarations_text("-webkit-foo: 1; _bar: 2; zoom: 3", &cfg).unwrap();
        assert_eq!(report.errors, 0, "{report:?}");
        assert_eq!(report.warnings, 0, "{report:?}");
        assert!(report.messages.is_empty(), "{report:?}");
    }

    #[test]
    fn overflow_clip_margin_is_accepted_but_warns_about_safari_support() {
        let config = Config {
            profile: Some("css3".to_string()),
            ..Config::default()
        };
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
            let report = validate_css_declarations_text(css, &config).unwrap();
            assert_eq!(report.errors, 0, "css={css:?} report={report:?}");
            assert_eq!(report.warnings, 1, "css={css:?} report={report:?}");
            assert_eq!(report.messages.len(), 1, "css={css:?} report={report:?}");
            assert_eq!(
                report.messages[0].message, "“overflow-clip-margin” is not supported by Safari.",
                "css={css:?} report={report:?}"
            );
        }
    }

    #[test]
    fn grid_template_rows_and_columns_accept_modern_track_syntax_examples() {
        let css = r#"
/* Keyword value */
grid-template-rows: none;

/* <track-list> values */
grid-template-rows: 100px 1fr;
grid-template-rows: [line-name] 100px;
grid-template-rows: [line-name1] 100px [line-name2 line-name3];
grid-template-rows: minmax(100px, 1fr);
grid-template-rows: fit-content(40%);
grid-template-rows: repeat(3, 200px);
grid-template-rows: subgrid;
grid-template-rows: masonry;

/* <auto-track-list> values */
grid-template-rows: 200px repeat(auto-fill, 100px) 300px;
grid-template-rows:
  minmax(100px, max-content)
  repeat(auto-fill, 200px) 20%;
grid-template-rows:
  [line-name1] 100px [line-name2]
  repeat(auto-fit, [line-name3 line-name4] 300px)
  100px;
grid-template-rows:
  [line-name1 line-name2] 100px
  repeat(auto-fit, [line-name1] 300px) [line-name3];

/* Global values */
grid-template-rows: inherit;
grid-template-rows: initial;
grid-template-rows: revert;
grid-template-rows: revert-layer;
grid-template-rows: unset;

/* Keyword value */
grid-template-columns: none;

/* <track-list> values */
grid-template-columns: 100px 1fr;
grid-template-columns: [line-name] 100px;
grid-template-columns: [line-name1] 100px [line-name2 line-name3];
grid-template-columns: minmax(100px, 1fr);
grid-template-columns: fit-content(40%);
grid-template-columns: repeat(3, 200px);
grid-template-columns: subgrid;
grid-template-columns: masonry;

/* <auto-track-list> values */
grid-template-columns: 200px repeat(auto-fill, 100px) 300px;
grid-template-columns:
  minmax(100px, max-content)
  repeat(auto-fill, 200px) 20%;
grid-template-columns:
  [line-name1] 100px [line-name2]
  repeat(auto-fit, [line-name3 line-name4] 300px)
  100px;
grid-template-columns:
  [line-name1 line-name2] 100px
  repeat(auto-fit, [line-name1] 300px) [line-name3];

/* Global values */
grid-template-columns: inherit;
grid-template-columns: initial;
grid-template-columns: revert;
grid-template-columns: revert-layer;
grid-template-columns: unset;
"#;

        let report = validate_css_declarations_text(css, &Config::default()).unwrap();
        assert_eq!(report.errors, 0, "{report:?}");
        assert_eq!(report.warnings, 0, "{report:?}");
        assert!(report.messages.is_empty(), "{report:?}");
    }

    #[test]
    fn grid_template_rows_and_columns_reject_invalid_syntax() {
        let css = r#"
grid-template-rows: repeat(3 200px);
grid-template-columns: minmax(100px 1fr);
grid-template-rows: 100px, 1fr;
grid-template-columns: fit-content();
grid-template-rows: foo;
grid-template-columns: [[]] 100px;
"#;

        let report = validate_css_declarations_text(css, &Config::default()).unwrap();
        assert_eq!(report.errors, 6, "{report:?}");
        assert_eq!(report.warnings, 0, "{report:?}");
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.message == "Invalid value for property “grid-template-rows”."),
            "{report:?}"
        );
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.message == "Invalid value for property “grid-template-columns”."),
            "{report:?}"
        );
    }

    #[test]
    fn aspect_ratio_accepts_number_and_ratio_values() {
        let report = validate_css_declarations_text("aspect-ratio:1;", &Config::default()).unwrap();
        assert_eq!(report.errors, 0, "{report:?}");
        assert_eq!(report.warnings, 0, "{report:?}");
        assert!(report.messages.is_empty(), "{report:?}");

        let report = validate_css_declarations_text(
            "aspect-ratio:1.5;aspect-ratio: 3000 / 3000;aspect-ratio:3e3/3e3;",
            &Config::default(),
        )
        .unwrap();
        assert_eq!(report.errors, 0, "{report:?}");
        assert_eq!(report.warnings, 0, "{report:?}");
        assert!(report.messages.is_empty(), "{report:?}");
    }

    #[test]
    fn aspect_ratio_rejects_invalid_css_number_syntax() {
        for css in [
            "aspect-ratio:12.;",
            "aspect-ratio:+-12.2;",
            "aspect-ratio:12.1.1;",
        ] {
            let report = validate_css_declarations_text(css, &Config::default()).unwrap();
            assert_eq!(report.errors, 1, "css={css:?} report={report:?}");
            assert_eq!(report.warnings, 0, "css={css:?} report={report:?}");
            assert!(
                report
                    .messages
                    .iter()
                    .any(|m| m.message == "Invalid value for property “aspect-ratio”."),
                "css={css:?} report={report:?}"
            );
        }
    }
}

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
    fn does_not_split_on_hex_escape_whitespace_terminators() {
        assert_eq!(
            split_top_level_value_tokens(r"\67 \72 \65 \65 \6E"),
            vec![r"\67 \72 \65 \65 \6E"]
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

mod css_value_syntax_tests {
    use super::is_valid_css_value_syntax;

    #[test]
    fn accepts_balanced_parentheses_and_brackets() {
        for value in [
            "calc(1px + 2px)",
            "rgb(0 0 0 / 50%)",
            r#"url("a{b}c")"#,
            r#"[a=b] (x)"#,
        ] {
            assert!(is_valid_css_value_syntax(value, false), "{value:?}");
        }
    }

    #[test]
    fn rejects_unbalanced_delimiters_and_strings() {
        for value in [
            "calc(1px",
            "calc(1px))",
            "rgb(0 0 0 / 50%]]",
            r#""unterminated"#,
        ] {
            assert!(!is_valid_css_value_syntax(value, false), "{value:?}");
        }
    }

    #[test]
    fn rejects_curly_braces_outside_strings() {
        for value in ["{", "}", "calc({})", "var(--x, {a})"] {
            assert!(!is_valid_css_value_syntax(value, false), "{value:?}");
        }
    }

    #[test]
    fn allows_balanced_curly_blocks_when_enabled() {
        for value in ["{}", "calc({})", "var(--x, {a})"] {
            assert!(is_valid_css_value_syntax(value, true), "{value:?}");
        }
        assert!(!is_valid_css_value_syntax("{", true));
        assert!(!is_valid_css_value_syntax("}", true));
        assert!(!is_valid_css_value_syntax("{}}", true));
    }
}

mod is_valid_property_name_tests {
    use super::is_valid_property_name;

    #[test]
    fn property_name_validation_allows_css_escapes_and_non_ascii() {
        for name in [
            "color",
            "-webkit-color",
            "--foo",
            "_x",
            "-1",
            "a_b-c9",
            "a©b",
            r"--\fffd",
            r"--a-\27 d",
            r"abc\{\}oops",
        ] {
            assert!(is_valid_property_name(name), "{name}");
        }

        for name in ["", "1abc", "a b", "a\tb", "a{b", "a/b", "a\\"] {
            assert!(!is_valid_property_name(name), "{name}");
        }
    }
}

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
        assert_eq!(strip_important("red ! important"), "red");
        assert_eq!(strip_important("red !  important"), "red");
        assert_eq!(strip_important("red!\timportant"), "red");
        assert_eq!(strip_important("red !IMPORTANT"), "red");
        assert_eq!(strip_important("red !important   "), "red");
    }

    #[test]
    fn handles_value_that_is_only_important() {
        assert_eq!(strip_important("!important"), "");
        assert_eq!(strip_important("  !important  "), "");
        assert_eq!(strip_important("! important"), "");
    }

    #[test]
    fn preserves_non_ascii_prefix() {
        assert_eq!(strip_important("❤ !important"), "❤");
    }
}

mod validate_simple_property_value_tests {
    use super::{
        Report, validate_background_color, validate_border_side_color, validate_color,
        validate_float, validate_outline_color, validate_outline_style, validate_outline_width,
    };

    #[test]
    fn validate_float_accepts_known_keywords_and_rejects_wrong_arity() {
        let mut report = Report::default();
        validate_float(&["left"], false, &mut report);
        assert_eq!(report.errors, 0);

        validate_float(&["left", "right"], false, &mut report);
        assert_eq!(report.errors, 1);
        assert_eq!(
            report.messages.last().unwrap().message,
            "Invalid value for property “float”."
        );
    }

    #[test]
    fn validate_color_like_properties_require_single_token_and_accept_basic_color_names() {
        let mut report = Report::default();
        validate_color(&["red"], false, false, &mut report);
        validate_background_color(&["red"], false, false, &mut report);
        validate_border_side_color(&["red"], false, false, &mut report);
        assert_eq!(report.errors, 0);

        validate_color(&["red", "blue"], false, false, &mut report);
        assert_eq!(report.errors, 1);
        assert_eq!(
            report.messages.last().unwrap().message,
            "Invalid value for property “color”."
        );
    }

    #[test]
    fn validate_outline_family_properties_accept_expected_keywords() {
        let mut report = Report::default();

        validate_outline_color(&["invert"], false, false, false, &mut report);
        validate_outline_style(&["SOLID"], false, &mut report);
        validate_outline_width(&["THIN"], &mut report);
        validate_outline_width(&["0"], &mut report);
        validate_outline_width(&["1px"], &mut report);
        assert_eq!(report.errors, 0);

        validate_outline_style(&["bad"], false, &mut report);
        assert_eq!(report.errors, 1);
        assert_eq!(
            report.messages.last().unwrap().message,
            "Invalid value for property “outline-style”."
        );
    }

    #[test]
    fn css4_profile_accepts_outline_auto_keywords() {
        let mut report = Report::default();
        validate_outline_style(&["auto"], true, &mut report);
        validate_outline_color(&["auto"], false, true, false, &mut report);
        assert_eq!(report.errors, 0, "{report:?}");
    }
}

mod token_predicate_tests {
    use super::{is_css_identifier_token, is_integer_token};

    #[test]
    fn is_css_identifier_token_matches_tokens_conservatively() {
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
        assert!(is_css_identifier_token("aé"));
        assert!(is_css_identifier_token("\u{0081}\u{0082}\u{0083}"));
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

mod validate_voice_family_tests {
    use super::{Report, validate_voice_family};

    #[test]
    fn accepts_generic_voice_with_integer_index() {
        for tokens in [
            &["female", "1"][..],
            &["male", "2"][..],
            &["\"alice\"", "3"][..],
        ] {
            let mut report = Report::default();
            validate_voice_family(tokens, &mut report);
            assert_eq!(report.errors, 0, "{tokens:?}: {report:?}");
        }
    }

    #[test]
    fn rejects_unattached_integers() {
        for tokens in [&["1"][..], &["female", "1", "2"][..]] {
            let mut report = Report::default();
            validate_voice_family(tokens, &mut report);
            assert_eq!(report.errors, 1, "{tokens:?}: {report:?}");
            assert_eq!(
                report.messages.last().unwrap().message,
                "Invalid value for property “voice-family”."
            );
        }
    }
}

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
    fn accepts_auto_case_insensitively() {
        for tokens in [&["auto"][..], &[" AUTO "][..]] {
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
    fn accepts_multiple_string_pairs() {
        let mut report = Report::default();
        validate_quotes(&["\"<\"", "\">\"", "\"[\"", "\"]\""], &mut report);
        assert_eq!(report.errors, 0);
    }

    #[test]
    fn rejects_other_forms() {
        for tokens in [
            &[][..],
            &["\"a\""][..],
            &["a", "b"][..],
            &["\"a\"", "b"][..],
            &["\"a\"", "\"b\"", "\"c\""][..],
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

mod validate_counter_list_tests {
    use super::{Report, validate_counter_list};

    #[test]
    fn accepts_none_alone_case_insensitively() {
        for tokens in [&["none"][..], &[" NONE "][..]] {
            let mut report = Report::default();
            validate_counter_list(tokens, "counter-increment", false, &mut report);
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
            validate_counter_list(tokens, "counter-reset", false, &mut report);
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
            validate_counter_list(tokens, "counter-reset", false, &mut report);
            assert_eq!(report.errors, 1, "{tokens:?}: {report:?}");
            assert_eq!(report.messages.len(), 1);
            assert_eq!(
                report.messages[0].message,
                "Invalid value for property “counter-reset”."
            );
        }
    }
}

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

    #[test]
    fn url_function_token_accepts_escapes_in_unquoted_urls() {
        assert!(super::is_valid_url_function_token(
            r#"url(support/\'green\ block.png)"#
        ));
    }
}

mod system_color_keyword_tests {
    use super::is_valid_color_token;

    #[test]
    fn system_colors_are_only_accepted_in_lenient_mode() {
        assert!(!is_valid_color_token("Menu", false, false));
        assert!(is_valid_color_token("Menu", false, true));
    }
}

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

mod color_function_arg_tests {
    use super::{is_valid_hsl_like_function, is_valid_rgb_like_function};

    #[test]
    fn rgb_like_requires_exact_nonempty_argument_count_in_strict_mode() {
        assert!(is_valid_rgb_like_function("rgb(1,2,3)", false, false));
        assert!(is_valid_rgb_like_function("rgb(1 2 3)", false, false));
        assert!(is_valid_rgb_like_function("rgb(1 2 3 / 0.5)", false, false));
        assert!(!is_valid_rgb_like_function("rgb()", false, false));
        assert!(!is_valid_rgb_like_function("rgb(1,2,3", false, false));
        assert!(!is_valid_rgb_like_function("rgb(1,2)", false, false));
        assert!(!is_valid_rgb_like_function("rgb(1,2,3,4)", false, false));
        assert!(is_valid_rgb_like_function("rgba(1,2,3,0.5)", true, false));
        assert!(is_valid_rgb_like_function("rgba(1 2 3 / 0.5)", true, false));
        assert!(!is_valid_rgb_like_function("rgba()", true, false));
        assert!(!is_valid_rgb_like_function("rgba(1,2,3,0.5", true, false));
        assert!(!is_valid_rgb_like_function("rgba(1,2,3)", true, false));
        assert!(!is_valid_rgb_like_function("rgba(1 2 3)", true, false));
        assert!(!is_valid_rgb_like_function(
            "rgba(1,2,3,0.5,0.6)",
            true,
            false
        ));
    }

    #[test]
    fn hsl_like_requires_percent_saturation_and_lightness_in_strict_mode() {
        assert!(is_valid_hsl_like_function("hsl(0,0%,0%)", false, false));
        assert!(is_valid_hsl_like_function("hsl(0 0% 0%)", false, false));
        assert!(is_valid_hsl_like_function(
            "hsl(0 0% 0% / 50%)",
            false,
            false
        ));
        assert!(!is_valid_hsl_like_function("hsl()", false, false));
        assert!(!is_valid_hsl_like_function("hsl(0,0%,0%", false, false));
        assert!(!is_valid_hsl_like_function("hsl(0,0,0%)", false, false));
        assert!(!is_valid_hsl_like_function("hsl(0,0%,0)", false, false));
        assert!(is_valid_hsl_like_function("hsla(0,0%,0%,0.5)", true, false));
        assert!(is_valid_hsl_like_function(
            "hsla(0 0% 0% / 0.5)",
            true,
            false
        ));
        assert!(!is_valid_hsl_like_function("hsla()", true, false));
        assert!(!is_valid_hsl_like_function("hsla(0,0%,0%,0.5", true, false));
        assert!(!is_valid_hsl_like_function("hsla(0,0%,0%)", true, false));
        assert!(!is_valid_hsl_like_function("hsla(0 0% 0%)", true, false));
    }

    #[test]
    fn rgb_like_alpha_values_match_is_alpha_value_rules_in_strict_mode() {
        assert!(is_valid_rgb_like_function("rgba(1,2,3,0)", true, false));
        assert!(is_valid_rgb_like_function("rgba(1,2,3,1)", true, false));
        assert!(!is_valid_rgb_like_function("rgba(1,2,3,2)", true, false));
        assert!(is_valid_rgb_like_function("rgba(1,2,3,100%)", true, false));
        assert!(!is_valid_rgb_like_function("rgba(1,2,3,101%)", true, false));
    }

    #[test]
    fn hsl_like_alpha_values_match_is_alpha_value_rules_in_strict_mode() {
        assert!(is_valid_hsl_like_function("hsla(0,0%,0%,0)", true, false));
        assert!(is_valid_hsl_like_function("hsla(0,0%,0%,1)", true, false));
        assert!(!is_valid_hsl_like_function("hsla(0,0%,0%,2)", true, false));
        assert!(is_valid_hsl_like_function(
            "hsla(0,0%,0%,100%)",
            true,
            false
        ));
        assert!(!is_valid_hsl_like_function(
            "hsla(0,0%,0%,101%)",
            true,
            false
        ));
    }

    #[test]
    fn color_function_args_allow_whitespace_around_commas_and_alpha_in_strict_mode() {
        assert!(is_valid_rgb_like_function("rgb( 1 , 2 , 3 )", false, false));
        assert!(is_valid_rgb_like_function(
            "rgba(1, 2, 3, 0.5 )",
            true,
            false
        ));
        assert!(is_valid_hsl_like_function(
            "hsl( 0 , 0% , 0% )",
            false,
            false
        ));
        assert!(is_valid_hsl_like_function(
            "hsla(0, 0%, 0%, 50% )",
            true,
            false
        ));
    }

    #[test]
    fn color_functions_accept_balanced_calls_in_lenient_mode() {
        assert!(is_valid_rgb_like_function(
            "rgb(none 255 none)",
            false,
            true
        ));
        assert!(is_valid_hsl_like_function(
            "hsl(from red h s l)",
            false,
            true
        ));
    }
}

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
        assert_eq!(n, Some(100.0));
        assert_eq!(u, "px");

        let (n, u) = split_number_and_unit("1π");
        assert_eq!(n, Some(1.0));
        assert_eq!(u, "π");
    }

    #[test]
    fn rejects_inputs_without_a_number_prefix() {
        assert_eq!(split_number_and_unit(""), (None, ""));
        assert_eq!(split_number_and_unit("px"), (None, ""));
        assert_eq!(split_number_and_unit("π1"), (None, ""));
        assert_eq!(split_number_and_unit(".px"), (None, ""));
        assert_eq!(split_number_and_unit("+"), (None, ""));
        assert_eq!(split_number_and_unit("-."), (None, ""));
    }

    #[test]
    fn keeps_unit_even_when_number_is_not_finite() {
        assert_eq!(split_number_and_unit("1e999px"), (None, "px"));
    }

    #[test]
    fn handles_optional_sign_and_decimal_points() {
        assert_eq!(split_number_and_unit("-1.5em"), (Some(-1.5), "em"));
        assert_eq!(split_number_and_unit("+.5rem"), (Some(0.5), "rem"));
        assert_eq!(split_number_and_unit("-.px"), (None, ""));
    }
}

mod css_number_tests {
    use super::parse_css_number;

    fn assert_close(a: f64, b: f64) {
        let diff = (a - b).abs();
        assert!(
            diff <= 1e-12 || diff <= (a.abs().max(b.abs()) * 1e-12),
            "a={a} b={b} diff={diff}"
        );
    }

    #[test]
    fn accepts_valid_css_numbers() {
        assert_close(parse_css_number("12").unwrap(), 12.0);
        assert_close(parse_css_number("4.01").unwrap(), 4.01);
        assert_close(parse_css_number("-456.8").unwrap(), -456.8);
        assert_close(parse_css_number("0.0").unwrap(), 0.0);
        assert_close(parse_css_number("+0.0").unwrap(), 0.0);
        assert_close(parse_css_number("-0.0").unwrap(), 0.0);
        assert_close(parse_css_number(".60").unwrap(), 0.60);
        assert_close(parse_css_number("10e3").unwrap(), 10e3);
        assert_close(parse_css_number("-3.4e-2").unwrap(), -3.4e-2);
    }

    #[test]
    fn rejects_invalid_css_numbers() {
        for s in ["12.", "+-12.2", "12.1.1"] {
            assert!(parse_css_number(s).is_none(), "{s}");
        }
    }
}

mod is_length_token_tests {
    use super::is_length_token;

    #[test]
    fn accepts_common_modern_units() {
        for t in [
            "1px",
            "1q",
            "1em",
            "1rem",
            "1ch",
            "1lh",
            "1vw",
            "1svh",
            "1dvb",
            "1cqi",
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
        let config = Config {
            profile: Some("css3".to_string()),
            ..Config::default()
        };
        for css in [
            "list-style: inside outside;",
            "list-style: outside outside;",
        ] {
            let report = validate_css_declarations_text(css, &config).unwrap();
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

    #[test]
    fn css4_accepts_shorthand_with_style_color_and_thickness() {
        let report = validate_css_declarations_text(
            "text-decoration: wavy underline overline green 5px;",
            &Config {
                profile: Some("css4".to_string()),
                ..Config::default()
            },
        )
        .unwrap();
        assert_eq!(report.errors, 0, "{report:?}");
        assert_eq!(report.warnings, 0, "{report:?}");
        assert!(report.messages.is_empty(), "{report:?}");
    }
}

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
