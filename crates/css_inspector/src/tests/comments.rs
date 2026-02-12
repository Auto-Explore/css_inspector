use super::*;

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
fn css4_profile_strips_double_slash_line_comments() {
    let css = "a { color: red; // comment\n width: 1px; }";

    let strict = Config {
        profile: Some("css3".to_string()),
        ..Config::default()
    };
    let report = validate_css_text(css, &strict).unwrap();
    assert_eq!(report.errors, 1, "{report:?}");
    assert!(
        report
            .messages
            .iter()
            .any(|m| m.message == "Invalid property name in declaration."),
        "{report:?}"
    );

    let report = validate_css_text(css, &Config::default()).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
    assert_eq!(report.warnings, 0, "{report:?}");
    assert!(report.messages.is_empty(), "{report:?}");
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
