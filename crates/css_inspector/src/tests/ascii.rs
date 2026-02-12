use super::*;

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
