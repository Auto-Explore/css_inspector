use crate::strutil::{scan_quoted_string_end, step_string_state};

pub(crate) fn has_top_level_import_url(css: &str) -> bool {
    iter_top_level_import_urls(css).next().is_some()
}

#[inline(always)]
fn skip_ascii_whitespace(bytes: &[u8], i: &mut usize) {
    while *i < bytes.len() && bytes[*i].is_ascii_whitespace() {
        *i += 1;
    }
}

#[inline(always)]
fn parse_import_url<'a>(css: &'a str, bytes: &[u8], j: usize) -> Option<(&'a str, usize)> {
    const URL_PREFIX: &[u8] = b"url(";
    if bytes
        .get(j..j + URL_PREFIX.len())
        .is_some_and(|b| b.eq_ignore_ascii_case(URL_PREFIX))
    {
        let mut k = j + URL_PREFIX.len();
        let mut paren_depth = 1i32;
        let mut in_string: Option<u8> = None;
        let mut escape = false;
        let start = k;
        while k < bytes.len() {
            let b = bytes[k];
            if step_string_state(b, &mut in_string, &mut escape) {
                k += 1;
                continue;
            }
            if b == b'(' {
                paren_depth += 1;
            } else if b == b')' {
                paren_depth -= 1;
                if paren_depth == 0 {
                    let raw = css[start..k].trim();
                    return Some((unquote(raw), k + 1));
                }
            }
            k += 1;
        }
        None
    } else if matches!(bytes.get(j).copied(), Some(b'"' | b'\'')) {
        let q = bytes[j];
        let end = scan_quoted_string_end(bytes, q, j + 1)?;
        Some((&css[j + 1..end], end + 1))
    } else {
        None
    }
}

#[inline(always)]
fn advance_past_at_import_tail(bytes: &[u8], depth: &mut usize, mut j: usize) -> usize {
    while j < bytes.len() {
        let b = bytes[j];
        if b == b'{' {
            *depth += 1;
        } else if b == b'}' {
            *depth = depth.saturating_sub(1);
        } else if *depth == 0 && b == b';' {
            return j + 1;
        }
        j += 1;
    }
    j
}

pub(crate) fn iter_top_level_import_urls<'a>(css: &'a str) -> impl Iterator<Item = &'a str> + 'a {
    let bytes = css.as_bytes();
    let mut i = 0usize;
    let mut depth: usize = 0;
    std::iter::from_fn(move || next_top_level_import_url(css, bytes, &mut i, &mut depth))
        .filter(|url| !url.is_empty())
}

#[inline(always)]
fn next_top_level_import_url<'a>(
    css: &'a str,
    bytes: &[u8],
    i: &mut usize,
    depth: &mut usize,
) -> Option<&'a str> {
    const AT_IMPORT: &[u8] = b"@import";
    skip_ascii_whitespace(bytes, i);
    if *i >= bytes.len() || *depth != 0 || bytes[*i] != b'@' {
        return None;
    }

    let at_import = bytes.get(*i..*i + AT_IMPORT.len())?;
    if !at_import.eq_ignore_ascii_case(AT_IMPORT) {
        return None;
    }

    let mut j = *i + AT_IMPORT.len();
    if bytes.get(j).is_some_and(|b| !b.is_ascii_whitespace()) {
        return None;
    }
    skip_ascii_whitespace(bytes, &mut j);
    if j >= bytes.len() {
        return None;
    }

    let (url, j) = parse_import_url(css, bytes, j)?;
    *i = advance_past_at_import_tail(bytes, depth, j);
    Some(url.trim())
}

pub(crate) fn unquote(s: &str) -> &str {
    let s = s.trim();
    match s.as_bytes() {
        [b'"', .., b'"'] | [b'\'', .., b'\''] => &s[1..s.len() - 1],
        _ => s,
    }
}

#[cfg(test)]
mod unquote_tests {
    use super::unquote;

    #[test]
    fn unquote_strips_matching_double_quotes() {
        assert_eq!(unquote(r#""x""#), "x");
        assert_eq!(unquote(r#""x y""#), "x y");
        assert_eq!(unquote(" \"x\" "), "x");
        assert_eq!(unquote("\n\"x\"\t"), "x");
    }

    #[test]
    fn unquote_strips_matching_single_quotes() {
        assert_eq!(unquote("'x'"), "x");
        assert_eq!(unquote("'x y'"), "x y");
        assert_eq!(unquote(" 'x' "), "x");
    }

    #[test]
    fn unquote_strips_empty_quoted_strings() {
        assert_eq!(unquote(r#""""#), "");
        assert_eq!(unquote("''"), "");
    }

    #[test]
    fn unquote_strips_empty_quoted_strings_even_with_whitespace() {
        assert_eq!(unquote("  \"\"  "), "");
        assert_eq!(unquote("\n\t''\t"), "");
    }

    #[test]
    fn unquote_handles_utf8_between_quotes() {
        assert_eq!(unquote(r#""🦀""#), "🦀");
        assert_eq!(unquote("'🦀'"), "🦀");
    }

    #[test]
    fn unquote_strips_single_quotes_around_double_quote_char() {
        assert_eq!(unquote("'\"'"), "\"");
    }

    #[test]
    fn unquote_leaves_unterminated_or_mismatched_quotes() {
        assert_eq!(unquote("\"x"), "\"x");
        assert_eq!(unquote("x\""), "x\"");
        assert_eq!(unquote("\"x'"), "\"x'");
        assert_eq!(unquote("'x\""), "'x\"");
        assert_eq!(unquote("\"\"x\"\""), "\"x\"");
    }

    #[test]
    fn unquote_trims_before_handling_mismatched_quotes() {
        assert_eq!(unquote("  \"x'  "), "\"x'");
        assert_eq!(unquote("\n\"x\" y\t"), "\"x\" y");
    }

    #[test]
    fn unquote_trims_even_without_quotes() {
        assert_eq!(unquote("  x  "), "x");
        assert_eq!(unquote("\n\tx\t"), "x");
    }

    #[test]
    fn unquote_leaves_single_quote_char_unchanged() {
        assert_eq!(unquote("'"), "'");
        assert_eq!(unquote("\""), "\"");
    }
}
