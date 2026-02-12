use super::*;

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
