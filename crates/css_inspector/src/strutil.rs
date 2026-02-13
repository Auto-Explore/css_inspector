use std::borrow::Cow;

pub(crate) fn ascii_lowercase_cow(s: &str) -> Cow<'_, str> {
    let Some(first_upper) = s.bytes().position(|b| b.is_ascii_uppercase()) else {
        return Cow::Borrowed(s);
    };

    // `first_upper` is always a UTF-8 boundary: ASCII uppercase bytes cannot appear as UTF-8
    // continuation bytes (which are 0x80..=0xBF).
    let mut out = s.to_owned();
    out[first_upper..].make_ascii_lowercase();
    Cow::Owned(out)
}

pub fn starts_with_ascii_ci(s: &str, prefix: &str) -> bool {
    let sb = s.as_bytes();
    let pb = prefix.as_bytes();
    sb.len() >= pb.len() && sb[..pb.len()].eq_ignore_ascii_case(pb)
}

pub(crate) fn ends_with_ascii_ci(s: &str, suffix: &str) -> bool {
    let sb = s.as_bytes();
    let suf = suffix.as_bytes();
    sb.len() >= suf.len() && sb[sb.len() - suf.len()..].eq_ignore_ascii_case(suf)
}

#[inline(always)]
pub(crate) fn scan_quoted_string_end(bytes: &[u8], quote: u8, start: usize) -> Option<usize> {
    let mut escape = false;
    let rest = bytes.get(start..)?;
    for (offset, &b) in rest.iter().enumerate() {
        if escape {
            escape = false;
            continue;
        }
        if b == b'\\' {
            escape = true;
            continue;
        }
        if b == quote {
            return Some(start + offset);
        }
    }
    None
}

#[cfg(test)]
mod scan_quoted_string_end_tests {
    use super::scan_quoted_string_end;

    #[test]
    fn finds_terminating_quote() {
        let bytes = br#""abc""#;
        assert_eq!(scan_quoted_string_end(bytes, b'"', 1), Some(4));
    }

    #[test]
    fn ignores_escaped_quotes() {
        let bytes = b"\"a\\\"b\"";
        assert_eq!(scan_quoted_string_end(bytes, b'"', 1), Some(5));
    }

    #[test]
    fn handles_escaped_backslashes() {
        let bytes = b"\"a\\\\\"";
        assert_eq!(scan_quoted_string_end(bytes, b'"', 1), Some(4));
    }

    #[test]
    fn returns_none_for_unclosed_string() {
        let bytes = b"\"a\\\"";
        assert_eq!(scan_quoted_string_end(bytes, b'"', 1), None);
    }

    #[test]
    fn returns_none_when_start_is_out_of_bounds() {
        let bytes = b"\"x\"";
        assert_eq!(scan_quoted_string_end(bytes, b'"', bytes.len() + 1), None);
    }
}

#[cfg(test)]
mod ascii_ci_prefix_suffix_tests {
    use super::{ends_with_ascii_ci, starts_with_ascii_ci};

    #[test]
    fn starts_with_ascii_ci_matches_or_rejects_without_panicking() {
        assert!(starts_with_ascii_ci("Hello", "he"));
        assert!(starts_with_ascii_ci("Hello", ""));
        assert!(!starts_with_ascii_ci("Hi", "hello"));
        assert!(!starts_with_ascii_ci("", "a"));
        assert!(starts_with_ascii_ci("", ""));
        assert!(!starts_with_ascii_ci("❤", "h"));
        assert!(!starts_with_ascii_ci("❤", "❤H"));
        assert!(starts_with_ascii_ci("🦀A", "🦀a"));
        assert!(!starts_with_ascii_ci("Ä", "ä"));
    }

    #[test]
    fn ends_with_ascii_ci_matches_or_rejects_without_panicking() {
        assert!(ends_with_ascii_ci("Hello", "LO"));
        assert!(ends_with_ascii_ci("Hello", ""));
        assert!(!ends_with_ascii_ci("Hi", "hello"));
        assert!(!ends_with_ascii_ci("", "a"));
        assert!(ends_with_ascii_ci("", ""));
        assert!(!ends_with_ascii_ci("❤", "h"));
        assert!(ends_with_ascii_ci("❤H", "h"));
        assert!(!ends_with_ascii_ci("❤", "❤H"));
        assert!(ends_with_ascii_ci("x🦀A", "🦀a"));
        assert!(!ends_with_ascii_ci("Ä", "ä"));
    }
}

#[cfg(test)]
mod starts_with_ascii_ci_tests {
    use super::starts_with_ascii_ci;

    #[test]
    fn empty_prefix_matches() {
        assert!(starts_with_ascii_ci("abc", ""));
    }

    #[test]
    fn longer_prefix_does_not_match() {
        assert!(!starts_with_ascii_ci("ab", "abc"));
    }

    #[test]
    fn matches_case_insensitively() {
        assert!(starts_with_ascii_ci("Hello", "heL"));
    }

    #[test]
    fn non_match_returns_false() {
        assert!(!starts_with_ascii_ci("abc", "b"));
    }
}

#[cfg(test)]
mod ends_with_ascii_ci_tests {
    use super::ends_with_ascii_ci;

    #[test]
    fn empty_suffix_matches() {
        assert!(ends_with_ascii_ci("abc", ""));
    }

    #[test]
    fn longer_suffix_does_not_match() {
        assert!(!ends_with_ascii_ci("ab", "abc"));
    }

    #[test]
    fn matches_case_insensitively() {
        assert!(ends_with_ascii_ci("Hello", "LLo"));
    }

    #[test]
    fn non_match_returns_false() {
        assert!(!ends_with_ascii_ci("abc", "b"));
    }
}

#[inline(always)]
pub(crate) fn step_string_state(b: u8, in_string: &mut Option<u8>, escape: &mut bool) -> bool {
    match *in_string {
        Some(q) => {
            if *escape {
                *escape = false;
            } else if b == b'\\' {
                *escape = true;
            } else if b == q {
                *in_string = None;
            }
            true
        }
        None if matches!(b, b'"' | b'\'') => {
            *in_string = Some(b);
            true
        }
        None => false,
    }
}

pub(crate) fn skip_css_escape(bytes: &[u8], i: &mut usize) {
    // CSS escapes:
    // - `\\` followed by a newline (line continuation, consumed)
    // - `\\` followed by 1-6 hex digits (optionally followed by a single whitespace terminator)
    // - `\\` followed by any single character
    //
    // `*i` is expected to point at the backslash.
    debug_assert_eq!(bytes.get(*i), Some(&b'\\'));

    *i += 1;
    let Some(&b2) = bytes.get(*i) else {
        return;
    };

    // Line continuation escape (`\\\n` etc).
    if matches!(b2, b'\n' | b'\r' | b'\x0C') {
        if b2 == b'\r' && bytes.get(*i + 1) == Some(&b'\n') {
            *i += 2;
        } else {
            *i += 1;
        }
        return;
    }

    // Hex escape.
    if b2.is_ascii_hexdigit() {
        let mut digits = 0usize;
        while *i < bytes.len() && digits < 6 && bytes[*i].is_ascii_hexdigit() {
            *i += 1;
            digits += 1;
        }
        if *i < bytes.len() && bytes[*i].is_ascii_whitespace() {
            if bytes[*i] == b'\r' && bytes.get(*i + 1) == Some(&b'\n') {
                *i += 2;
            } else {
                *i += 1;
            }
        }
        return;
    }

    // Single-character escape.
    *i += 1;
}

#[cfg(test)]
mod step_string_state_tests {
    use super::step_string_state;

    #[test]
    fn starts_and_ends_strings_and_handles_escapes() {
        let mut in_string: Option<u8> = None;
        let mut escape = false;

        assert!(step_string_state(b'"', &mut in_string, &mut escape));
        assert_eq!(in_string, Some(b'"'));
        assert!(!escape);

        assert!(step_string_state(b'\\', &mut in_string, &mut escape));
        assert!(escape);

        // Escaped quotes do not terminate the string.
        assert!(step_string_state(b'"', &mut in_string, &mut escape));
        assert_eq!(in_string, Some(b'"'));
        assert!(!escape);

        assert!(step_string_state(b'"', &mut in_string, &mut escape));
        assert_eq!(in_string, None);
    }

    #[test]
    fn escaped_non_quote_does_not_end_string() {
        let mut in_string: Option<u8> = None;
        let mut escape = false;

        assert!(step_string_state(b'"', &mut in_string, &mut escape));
        assert!(step_string_state(b'\\', &mut in_string, &mut escape));
        assert!(escape);

        assert!(step_string_state(b'a', &mut in_string, &mut escape));
        assert_eq!(in_string, Some(b'"'));
        assert!(!escape);

        assert!(step_string_state(b'"', &mut in_string, &mut escape));
        assert_eq!(in_string, None);
    }

    #[test]
    fn returns_false_for_non_string_bytes() {
        let mut in_string: Option<u8> = None;
        let mut escape = false;

        assert!(!step_string_state(b'a', &mut in_string, &mut escape));
        assert_eq!(in_string, None);
        assert!(!escape);

        assert!(!step_string_state(b'\\', &mut in_string, &mut escape));
        assert_eq!(in_string, None);
        assert!(!escape);
    }

    #[test]
    fn supports_single_quoted_strings() {
        let mut in_string: Option<u8> = None;
        let mut escape = false;

        assert!(step_string_state(b'\'', &mut in_string, &mut escape));
        assert_eq!(in_string, Some(b'\''));

        assert!(step_string_state(b'a', &mut in_string, &mut escape));
        assert_eq!(in_string, Some(b'\''));

        assert!(step_string_state(b'\\', &mut in_string, &mut escape));
        assert!(escape);

        // Escaped quotes do not terminate the string.
        assert!(step_string_state(b'\'', &mut in_string, &mut escape));
        assert_eq!(in_string, Some(b'\''));
        assert!(!escape);

        assert!(step_string_state(b'\'', &mut in_string, &mut escape));
        assert_eq!(in_string, None);
        assert!(!escape);
    }
}

pub(crate) fn split_top_level_commas<'a>(s: &'a str) -> impl Iterator<Item = &'a str> + 'a {
    let bytes = s.as_bytes();
    let mut start = 0usize;
    let mut i = 0usize;
    let mut in_string: Option<u8> = None;
    let mut escape = false;
    let mut paren_depth: u32 = 0;
    let mut bracket_depth: u32 = 0;

    std::iter::from_fn(move || {
        if start > bytes.len() {
            return None;
        }

        while i < bytes.len() {
            let b = bytes[i];
            if step_string_state(b, &mut in_string, &mut escape) {
                i += 1;
                continue;
            }
            match b {
                b'(' => paren_depth += 1,
                b')' => paren_depth = paren_depth.saturating_sub(1),
                b'[' => bracket_depth += 1,
                b']' => bracket_depth = bracket_depth.saturating_sub(1),
                b',' if paren_depth == 0 && bracket_depth == 0 => {
                    let out = &s[start..i];
                    i += 1;
                    start = i;
                    return Some(out);
                }
                _ => {}
            }
            i += 1;
        }

        let out = &s[start..];
        start = bytes.len() + 1;
        Some(out)
    })
}

#[cfg(test)]
mod split_top_level_commas_tests {
    use super::split_top_level_commas;

    #[test]
    fn splits_commas_only_at_top_level() {
        let got: Vec<&str> = split_top_level_commas("a,b,c").collect();
        assert_eq!(got, vec!["a", "b", "c"]);

        let got: Vec<&str> = split_top_level_commas(",a,").collect();
        assert_eq!(got, vec!["", "a", ""]);

        let got: Vec<&str> = split_top_level_commas("a,func(b,c),d").collect();
        assert_eq!(got, vec!["a", "func(b,c)", "d"]);

        let got: Vec<&str> = split_top_level_commas("a,[b,c],d").collect();
        assert_eq!(got, vec!["a", "[b,c]", "d"]);
    }

    #[test]
    fn ignores_commas_inside_strings_and_escapes() {
        let got: Vec<&str> = split_top_level_commas(r#"a,"b,c",d"#).collect();
        assert_eq!(got, vec!["a", r#""b,c""#, "d"]);

        let got: Vec<&str> = split_top_level_commas("a,\"b\\\"c,d\",e").collect();
        assert_eq!(got, vec!["a", r#""b\"c,d""#, "e"]);
    }

    #[test]
    fn iterator_is_fused() {
        let mut it = split_top_level_commas("a,b");
        assert_eq!(it.next(), Some("a"));
        assert_eq!(it.next(), Some("b"));
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn yields_one_item_for_empty_string() {
        let got: Vec<&str> = split_top_level_commas("").collect();
        assert_eq!(got, vec![""]);
    }
}
