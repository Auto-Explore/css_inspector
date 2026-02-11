use std::borrow::Cow;

use crate::config::Config;
use crate::report::{Report, push_warning_level};
use crate::strutil::{split_top_level_commas, step_string_state};

pub(crate) fn at_rule_name(prelude: &str) -> Option<&str> {
    let at = prelude.trim_start().strip_prefix('@')?;
    at.trim_start()
        .split(|c: char| c.is_whitespace() || c == '(')
        .next()
        .filter(|name| !name.is_empty())
}

#[inline]
pub(crate) fn at_rule_decl_list_context_flags(
    kind: RuleBlockKind,
    prelude: &str,
) -> (bool, bool, bool, bool, bool, bool, bool, bool, bool) {
    if kind != RuleBlockKind::AtRuleDeclList {
        return (
            false, false, false, false, false, false, false, false, false,
        );
    }
    let Some(name) = at_rule_name(prelude) else {
        return (
            false, false, false, false, false, false, false, false, false,
        );
    };
    (
        name.eq_ignore_ascii_case("page"),
        name.eq_ignore_ascii_case("font-face"),
        name.eq_ignore_ascii_case("property"),
        name.eq_ignore_ascii_case("font-palette-values"),
        name.eq_ignore_ascii_case("counter-style"),
        name.eq_ignore_ascii_case("color-profile"),
        name.eq_ignore_ascii_case("view-transition"),
        name.eq_ignore_ascii_case("scroll-timeline"),
        name.eq_ignore_ascii_case("view-timeline"),
    )
}

#[cfg(test)]
mod at_rule_name_tests {
    use super::at_rule_name;

    #[test]
    fn parses_at_rule_names_conservatively() {
        assert_eq!(at_rule_name("@page"), Some("page"));
        assert_eq!(at_rule_name(" @font-face "), Some("font-face"));
        assert_eq!(at_rule_name("@media screen"), Some("media"));
        assert_eq!(at_rule_name("@supports (display: grid)"), Some("supports"));
        assert_eq!(at_rule_name(" @  page"), Some("page"));

        assert_eq!(at_rule_name("page"), None);
        assert_eq!(at_rule_name("@"), None);
        assert_eq!(at_rule_name("@ (x)"), None);
        assert_eq!(at_rule_name(" @   "), None);
    }
}

#[cfg(test)]
mod at_rule_decl_list_context_flags_tests {
    use super::{RuleBlockKind, at_rule_decl_list_context_flags};

    #[test]
    fn returns_false_outside_at_rule_decl_lists() {
        assert_eq!(
            at_rule_decl_list_context_flags(RuleBlockKind::QualifiedRule, "@page"),
            (
                false, false, false, false, false, false, false, false, false
            )
        );
    }

    #[test]
    fn detects_page_and_font_face_case_insensitively() {
        assert_eq!(
            at_rule_decl_list_context_flags(RuleBlockKind::AtRuleDeclList, "@page"),
            (true, false, false, false, false, false, false, false, false)
        );
        assert_eq!(
            at_rule_decl_list_context_flags(RuleBlockKind::AtRuleDeclList, " @FONT-FACE "),
            (false, true, false, false, false, false, false, false, false)
        );
        assert_eq!(
            at_rule_decl_list_context_flags(RuleBlockKind::AtRuleDeclList, "@property --x"),
            (false, false, true, false, false, false, false, false, false)
        );
        assert_eq!(
            at_rule_decl_list_context_flags(
                RuleBlockKind::AtRuleDeclList,
                "@font-palette-values --x"
            ),
            (false, false, false, true, false, false, false, false, false)
        );
        assert_eq!(
            at_rule_decl_list_context_flags(RuleBlockKind::AtRuleDeclList, "@counter-style x"),
            (false, false, false, false, true, false, false, false, false)
        );
        assert_eq!(
            at_rule_decl_list_context_flags(RuleBlockKind::AtRuleDeclList, "@color-profile --x"),
            (false, false, false, false, false, true, false, false, false)
        );
        assert_eq!(
            at_rule_decl_list_context_flags(RuleBlockKind::AtRuleDeclList, "@view-transition"),
            (false, false, false, false, false, false, true, false, false)
        );
        assert_eq!(
            at_rule_decl_list_context_flags(RuleBlockKind::AtRuleDeclList, "@scroll-timeline --x"),
            (false, false, false, false, false, false, false, true, false)
        );
        assert_eq!(
            at_rule_decl_list_context_flags(RuleBlockKind::AtRuleDeclList, "@view-timeline --x"),
            (false, false, false, false, false, false, false, false, true)
        );
        assert_eq!(
            at_rule_decl_list_context_flags(RuleBlockKind::AtRuleDeclList, "@media screen"),
            (
                false, false, false, false, false, false, false, false, false
            )
        );
    }

    #[test]
    fn returns_false_when_at_rule_name_is_missing_and_accepts_paren_suffix() {
        assert_eq!(
            at_rule_decl_list_context_flags(RuleBlockKind::AtRuleDeclList, "page"),
            (
                false, false, false, false, false, false, false, false, false
            )
        );
        assert_eq!(
            at_rule_decl_list_context_flags(RuleBlockKind::AtRuleDeclList, "@page("),
            (true, false, false, false, false, false, false, false, false)
        );
    }
}

pub(crate) fn ends_with_stray_backslash(css: &str) -> bool {
    css.trim_end().ends_with('\\')
}

pub(crate) fn is_known_at_rule_name(name: &str) -> bool {
    const KNOWN: [&str; 47] = [
        "import",
        "media",
        "page",
        "top-left-corner",
        "top-left",
        "top-center",
        "top-right",
        "top-right-corner",
        "bottom-left-corner",
        "bottom-left",
        "bottom-center",
        "bottom-right",
        "bottom-right-corner",
        "left-top",
        "left-middle",
        "left-bottom",
        "right-top",
        "right-middle",
        "right-bottom",
        "font-face",
        "font-feature-values",
        "stylistic",
        "styleset",
        "character-variant",
        "swash",
        "ornaments",
        "annotation",
        "font-palette-values",
        "container",
        "charset",
        "namespace",
        "supports",
        "layer",
        "nest",
        "custom-media",
        "custom-selector",
        "document",
        "keyframes",
        "counter-style",
        "property",
        "color-profile",
        "scope",
        "starting-style",
        "view-transition",
        "scroll-timeline",
        "view-timeline",
        "timeline-scope",
        // Keep in sync with upstream expectations in `contains_unknown_at_rule`.
    ];
    if name.bytes().any(|b| b.is_ascii_uppercase()) {
        KNOWN.iter().any(|k| k.eq_ignore_ascii_case(name))
    } else {
        KNOWN.contains(&name)
    }
}

pub(crate) fn contains_unknown_at_rule(css: &str) -> bool {
    // Test-driven: the suite includes `@three-dee` / `@background-lighting` (invalid) and expects
    // a single error for the presence of unknown at-rules.
    let bytes = css.as_bytes();
    let mut i = 0usize;
    let mut in_string: Option<u8> = None;
    let mut escape = false;

    #[inline]
    fn scan_at_rule_name<'a>(
        css: &'a str,
        bytes: &[u8],
        at_pos: usize,
    ) -> Option<(&'a str, usize)> {
        debug_assert_eq!(bytes.get(at_pos), Some(&b'@'));
        let mut j = at_pos + 1;
        while j < bytes.len() && bytes[j].is_ascii_whitespace() {
            j += 1;
        }
        let start = j;
        while j < bytes.len() && (bytes[j].is_ascii_alphabetic() || bytes[j] == b'-') {
            j += 1;
        }
        (j != start).then(|| (&css[start..j], j))
    }

    while i < bytes.len() {
        let b = bytes[i];
        if step_string_state(b, &mut in_string, &mut escape) {
            i += 1;
            continue;
        }

        if b == b'@' {
            if let Some((name, next_i)) = scan_at_rule_name(css, bytes, i) {
                if !is_known_at_rule_name(name) {
                    return true;
                }
                i = next_i;
                continue;
            }
        }

        i += 1;
    }
    false
}

pub(crate) fn count_stray_top_level_declaration_errors(css: &str) -> usize {
    // When a `:` appears in a top-level, semicolon-terminated segment (not an at-rule), the
    // upstream parser reports multiple parse errors. The autotest suite expects:
    // - `bodytext-align:center;` => 2 errors
    // - `bodytext-align:center; }` => 3 errors (including an extra `}` error)
    let bytes = css.as_bytes();
    let mut i = 0usize;
    let mut depth: usize = 0;
    let mut in_string: Option<u8> = None;
    let mut escape = false;
    let mut seg_start = 0usize;
    let mut segments = 0usize;

    while i < bytes.len() {
        let b = bytes[i];
        if step_string_state(b, &mut in_string, &mut escape) {
            i += 1;
            continue;
        }
        match b {
            b'{' => {
                depth += 1;
            }
            b'}' => {
                depth = depth.saturating_sub(1);
            }
            b';' if depth == 0 => {
                let seg = css[seg_start..i].trim();
                if !seg.is_empty()
                    && !seg.starts_with('@')
                    && seg.contains(':')
                    && !seg.contains('{')
                    && !seg.contains('}')
                {
                    segments += 1;
                }
                seg_start = i + 1;
            }
            _ => {}
        }
        i += 1;
    }

    segments * 2
}

pub(crate) fn strip_css_comments(input: &str) -> (Cow<'_, str>, bool) {
    let bytes = input.as_bytes();
    let mut i = 0usize;
    let mut last = 0usize;
    let mut out: Option<String> = None;

    while i + 1 < bytes.len() {
        if bytes[i] == b'/' && bytes[i + 1] == b'*' {
            let mut j = i + 2;
            while j + 1 < bytes.len() && !(bytes[j] == b'*' && bytes[j + 1] == b'/') {
                j += 1;
            }
            if j + 1 >= bytes.len() {
                if let Some(mut out_buf) = out {
                    out_buf.push_str(&input[last..i]);
                    return (Cow::Owned(out_buf), false);
                }
                return (Cow::Borrowed(&input[..i]), false);
            }

            let out_buf = out.get_or_insert_with(|| String::with_capacity(input.len()));
            out_buf.push_str(&input[last..i]);

            i = j + 2;
            // Preserve token boundaries: comments behave like whitespace separators.
            out_buf.push(' ');
            last = i;
            continue;
        }
        i += 1;
    }

    if let Some(mut out_buf) = out {
        out_buf.push_str(&input[last..]);
        (Cow::Owned(out_buf), true)
    } else {
        (Cow::Borrowed(input), true)
    }
}

#[cfg(test)]
mod strip_css_comments_tests {
    use super::strip_css_comments;
    use std::borrow::Cow;

    #[test]
    fn strip_css_comments_returns_borrowed_when_no_comments() {
        let (out, ok) = strip_css_comments("a { color: red }");
        assert!(ok);
        assert!(matches!(out, Cow::Borrowed(_)));
        assert_eq!(out.as_ref(), "a { color: red }");
    }

    #[test]
    fn strip_css_comments_handles_empty_input() {
        let (out, ok) = strip_css_comments("");
        assert!(ok);
        assert!(matches!(out, Cow::Borrowed(_)));
        assert_eq!(out.as_ref(), "");
    }

    #[test]
    fn strip_css_comments_removes_comments_and_preserves_token_boundaries() {
        let (out, ok) = strip_css_comments("a/*x*/b");
        assert!(ok);
        assert!(matches!(out, Cow::Owned(_)));
        assert_eq!(out.as_ref(), "a b");

        let (out, ok) = strip_css_comments("/*x*/a");
        assert!(ok);
        assert!(matches!(out, Cow::Owned(_)));
        assert_eq!(out.as_ref(), " a");

        let (out, ok) = strip_css_comments("a/*x*/");
        assert!(ok);
        assert!(matches!(out, Cow::Owned(_)));
        assert_eq!(out.as_ref(), "a ");

        let (out, ok) = strip_css_comments("a/*x*/b/*y*/c");
        assert!(ok);
        assert!(matches!(out, Cow::Owned(_)));
        assert_eq!(out.as_ref(), "a b c");

        let (out, ok) = strip_css_comments("a/*x*/b/*y*/");
        assert!(ok);
        assert!(matches!(out, Cow::Owned(_)));
        assert_eq!(out.as_ref(), "a b ");
    }

    #[test]
    fn strip_css_comments_preserves_utf8_around_comments() {
        let (out, ok) = strip_css_comments("bé/*x*/c");
        assert!(ok);
        assert!(matches!(out, Cow::Owned(_)));
        assert_eq!(out.as_ref(), "bé c");
    }

    #[test]
    fn strip_css_comments_handles_empty_comments() {
        let (out, ok) = strip_css_comments("a/**/b");
        assert!(ok);
        assert!(matches!(out, Cow::Owned(_)));
        assert_eq!(out.as_ref(), "a b");
    }

    #[test]
    fn strip_css_comments_inserts_a_space_for_each_comment() {
        let (out, ok) = strip_css_comments("a/*x*//*y*/b");
        assert!(ok);
        assert!(matches!(out, Cow::Owned(_)));
        assert_eq!(out.as_ref(), "a  b");
    }

    #[test]
    fn strip_css_comments_reports_unclosed_comment() {
        let input = "a/*x";
        let (out, ok) = strip_css_comments(input);
        assert!(!ok);
        match &out {
            Cow::Borrowed(prefix) => assert!(std::ptr::eq(prefix.as_ptr(), input.as_ptr())),
            Cow::Owned(_) => panic!("expected borrowed prefix"),
        }
        assert_eq!(out.as_ref(), "a");
    }

    #[test]
    fn strip_css_comments_reports_unclosed_comment_without_body() {
        let (out, ok) = strip_css_comments("a/*");
        assert!(!ok);
        assert!(matches!(out, Cow::Borrowed(_)));
        assert_eq!(out.as_ref(), "a");
    }

    #[test]
    fn strip_css_comments_reports_unclosed_comment_at_start() {
        let (out, ok) = strip_css_comments("/*");
        assert!(!ok);
        assert!(matches!(out, Cow::Borrowed(_)));
        assert_eq!(out.as_ref(), "");
    }

    #[test]
    fn strip_css_comments_returns_prefix_on_unclosed_comment_after_a_previous_comment() {
        let (out, ok) = strip_css_comments("a/*x*/b/*y");
        assert!(!ok);
        assert!(matches!(out, Cow::Owned(_)));
        assert_eq!(out.as_ref(), "a b");
    }

    #[test]
    fn strip_css_comments_returns_prefix_on_unclosed_comment_after_a_previous_comment_without_tail()
    {
        let (out, ok) = strip_css_comments("a/*x*/b/*");
        assert!(!ok);
        assert!(matches!(out, Cow::Owned(_)));
        assert_eq!(out.as_ref(), "a b");
    }

    #[test]
    fn strip_css_comments_returns_prefix_on_unclosed_comment_preserving_tail_since_last_comment() {
        let (out, ok) = strip_css_comments("a/*x*/bcd/*");
        assert!(!ok);
        assert!(matches!(out, Cow::Owned(_)));
        assert_eq!(out.as_ref(), "a bcd");
    }

    #[test]
    fn strip_css_comments_returns_prefix_on_unclosed_comment_after_adjacent_comment() {
        let (out, ok) = strip_css_comments("a/*x*//*");
        assert!(!ok);
        assert!(matches!(out, Cow::Owned(_)));
        assert_eq!(out.as_ref(), "a ");
    }

    #[test]
    fn strip_css_comments_returns_prefix_on_unclosed_comment_after_leading_comment() {
        let (out, ok) = strip_css_comments("/*x*/a/*");
        assert!(!ok);
        assert!(matches!(out, Cow::Owned(_)));
        assert_eq!(out.as_ref(), " a");
    }

    #[test]
    fn strip_css_comments_strips_comment_only_input() {
        let (out, ok) = strip_css_comments("/*x*/");
        assert!(ok);
        assert!(matches!(out, Cow::Owned(_)));
        assert_eq!(out.as_ref(), " ");
    }

    #[test]
    fn strip_css_comments_reports_unclosed_comment_only_input() {
        for input in ["/*x", "/*"] {
            let (out, ok) = strip_css_comments(input);
            assert!(!ok, "{input}");
            assert!(matches!(out, Cow::Borrowed(_)), "{input}");
            assert_eq!(out.as_ref(), "", "{input}");
        }
    }

    #[test]
    fn strip_css_comments_reports_unclosed_comment_after_utf8_prefix() {
        let (out, ok) = strip_css_comments("bé/*");
        assert!(!ok);
        assert!(matches!(out, Cow::Borrowed(_)));
        assert_eq!(out.as_ref(), "bé");
    }

    #[test]
    fn strip_css_comments_reports_unclosed_comment_after_4byte_utf8_prefix() {
        let (out, ok) = strip_css_comments("🦀/*");
        assert!(!ok);
        assert!(matches!(out, Cow::Borrowed(_)));
        assert_eq!(out.as_ref(), "🦀");
    }

    #[test]
    fn strip_css_comments_reports_unclosed_comment_after_previous_comment_with_utf8_prefix() {
        let (out, ok) = strip_css_comments("a/*x*/bé/*");
        assert!(!ok);
        assert!(matches!(out, Cow::Owned(_)));
        assert_eq!(out.as_ref(), "a bé");
    }

    #[test]
    fn strip_css_comments_returns_prefix_on_unclosed_comment_after_leading_comment_with_utf8_tail()
    {
        let (out, ok) = strip_css_comments("/*x*/bé/*y");
        assert!(!ok);
        assert!(matches!(out, Cow::Owned(_)));
        assert_eq!(out.as_ref(), " bé");
    }

    #[test]
    fn strip_css_comments_returns_prefix_on_unclosed_comment_after_multiple_comments() {
        let (out, ok) = strip_css_comments("a/*x*/b/*y*/c/*");
        assert!(!ok);
        assert!(matches!(out, Cow::Owned(_)));
        assert_eq!(out.as_ref(), "a b c");
    }

    #[test]
    fn strip_css_comments_returns_prefix_on_unclosed_comment_including_intervening_text() {
        let (out, ok) = strip_css_comments("a/*x*/\n/*");
        assert!(!ok);
        assert!(matches!(out, Cow::Owned(_)));
        assert_eq!(out.as_ref(), "a \n");
    }

    #[test]
    fn strip_css_comments_does_not_treat_lone_slashes_as_comments() {
        for input in ["/", "a/", "/a", "a/b"] {
            let (out, ok) = strip_css_comments(input);
            assert!(ok, "{input}");
            assert!(matches!(out, Cow::Borrowed(_)), "{input}");
            assert_eq!(out.as_ref(), input);
        }
    }
}

pub(crate) fn count_brace_balance_errors(css: &str) -> usize {
    let bytes = css.as_bytes();
    let mut depth: usize = 0;
    let mut errors: usize = 0;
    let mut in_string: Option<u8> = None;
    let mut escape = false;
    for &b in bytes {
        if step_string_state(b, &mut in_string, &mut escape) {
            continue;
        }
        match b {
            b'{' => depth += 1,
            b'}' => {
                if depth == 0 {
                    errors += 1;
                } else {
                    depth -= 1;
                }
            }
            _ => {}
        }
    }
    // Avoid double-counting: once we're inside an unterminated string, braces are ignored by the
    // tokenizer, so any remaining `depth` is likely a consequence of the same issue.
    errors + if in_string.is_some() { 1 } else { depth }
}

pub(crate) fn contains_invalid_top_level_chars(css: &str) -> bool {
    let bytes = css.as_bytes();
    let mut in_string: Option<u8> = None;
    let mut escape = false;
    for &b in bytes {
        if step_string_state(b, &mut in_string, &mut escape) {
            continue;
        }
        if b == b'<' {
            return true;
        }
    }
    false
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]

pub(crate) enum RuleBlockKind {
    QualifiedRule,
    AtRuleDeclList,
}

pub(crate) struct RuleBlock<'a> {
    pub(crate) kind: RuleBlockKind,
    pub(crate) prelude: &'a str,
    pub(crate) body: &'a str,
}

struct DeclBlock {
    kind: RuleBlockKind,
    body_start: usize,
}

struct Frame<'a> {
    decl: Option<DeclBlock>,
    prelude: &'a str,
}

pub(crate) fn iter_rule_blocks<'a>(css: &'a str) -> impl Iterator<Item = RuleBlock<'a>> + 'a {
    // Extract declaration-list blocks for:
    // - qualified rules: `selector { ... }`
    // - at-rules with declaration bodies: `@font-face { ... }`, `@page { ... }`, ...
    //
    // While descending through at-rules with rule-list bodies (e.g., `@media { ... }`),
    // this yields the nested qualified-rule declaration blocks.
    let mut i = 0usize;
    let mut prelude_start = 0usize;
    let mut stack: Vec<Frame<'a>> = Vec::new();

    let bytes = css.as_bytes();
    let mut in_string: Option<u8> = None;
    let mut escape = false;

    std::iter::from_fn(move || {
        while i < bytes.len() {
            let b = bytes[i];

            if step_string_state(b, &mut in_string, &mut escape) {
                i += 1;
                continue;
            }

            match b {
                b';' => {
                    prelude_start = i + 1;
                    i += 1;
                }
                b'{' => {
                    let prelude = css[prelude_start..i].trim();
                    let in_decl_context = stack.iter().any(|f| f.decl.is_some());
                    let decl = if prelude.starts_with('@') {
                        let is_decl_list = matches!(
                            at_rule_name(prelude),
                            Some(name)
                                if name.eq_ignore_ascii_case("font-face")
                                    || name.eq_ignore_ascii_case("page")
                                    || name.eq_ignore_ascii_case("property")
                                    || name.eq_ignore_ascii_case("font-palette-values")
                                    || name.eq_ignore_ascii_case("counter-style")
                                    || name.eq_ignore_ascii_case("color-profile")
                                    || name.eq_ignore_ascii_case("view-transition")
                                    || name.eq_ignore_ascii_case("scroll-timeline")
                                    || name.eq_ignore_ascii_case("view-timeline")
                        ) || in_decl_context;
                        is_decl_list.then_some(DeclBlock {
                            kind: RuleBlockKind::AtRuleDeclList,
                            body_start: i + 1,
                        })
                    } else {
                        Some(DeclBlock {
                            kind: RuleBlockKind::QualifiedRule,
                            body_start: i + 1,
                        })
                    };
                    let frame = Frame { decl, prelude };

                    stack.push(frame);
                    prelude_start = i + 1;
                    i += 1;
                }
                b'}' => {
                    let Some(frame) = stack.pop() else {
                        i += 1;
                        prelude_start = i;
                        continue;
                    };
                    let end = i;
                    i += 1;
                    prelude_start = i;

                    if let Some(decl) = frame.decl {
                        return Some(RuleBlock {
                            kind: decl.kind,
                            prelude: frame.prelude,
                            body: &css[decl.body_start..end],
                        });
                    }
                }
                _ => i += 1,
            }
        }
        None
    })
}
pub(crate) fn warn_on_other_media_rules(
    css: &str,
    config: &Config,
    warning_level: i32,
    report: &mut Report,
) {
    let Some(user_medium_raw) = config.medium.as_deref() else {
        return;
    };
    let user_media: Vec<String> = user_medium_raw
        .split(|c: char| c == ',' || c.is_ascii_whitespace())
        .filter(|t| !t.is_empty())
        .map(|t| t.to_ascii_lowercase())
        .collect();
    if user_media.is_empty() || user_media.iter().any(|m| m == "all") {
        return;
    }

    let bytes = css.as_bytes();
    let mut i = 0usize;
    let mut in_string: Option<u8> = None;
    let mut escape = false;

    while i < bytes.len() {
        let b = bytes[i];
        if step_string_state(b, &mut in_string, &mut escape) {
            i += 1;
            continue;
        }
        if b != b'@' {
            i += 1;
            continue;
        }

        let Some((name, after_name)) = scan_at_rule_name_at(css, bytes, i) else {
            i += 1;
            continue;
        };
        if !name.eq_ignore_ascii_case("media") {
            i = after_name;
            continue;
        }

        // Find the opening `{` for the @media prelude.
        let mut j = after_name;
        let mut in_string2: Option<u8> = None;
        let mut escape2 = false;
        let mut paren_depth: i64 = 0;
        while j < bytes.len() {
            let bj = bytes[j];
            if step_string_state(bj, &mut in_string2, &mut escape2) {
                j += 1;
                continue;
            }
            match bj {
                b'(' => paren_depth += 1,
                b')' => paren_depth -= 1,
                b'{' if paren_depth == 0 => break,
                _ => {}
            }
            j += 1;
        }
        if j >= bytes.len() {
            break;
        }

        let prelude = &css[after_name..j];
        if !media_prelude_matches_user_media(prelude, user_media.as_slice()) {
            push_warning_level(
                report,
                warning_level,
                0,
                "Properties for other media might not work for usermedium.".to_string(),
            );
            return;
        }
        i = j + 1;
    }
}

fn scan_at_rule_name_at<'a>(css: &'a str, bytes: &[u8], at_pos: usize) -> Option<(&'a str, usize)> {
    debug_assert_eq!(bytes.get(at_pos), Some(&b'@'));
    let mut j = at_pos + 1;
    while j < bytes.len() && bytes[j].is_ascii_whitespace() {
        j += 1;
    }
    let start = j;
    while j < bytes.len() && (bytes[j].is_ascii_alphabetic() || bytes[j] == b'-') {
        j += 1;
    }
    (j != start).then(|| (&css[start..j], j))
}

fn media_prelude_matches_user_media(prelude: &str, user_media: &[String]) -> bool {
    for query in split_top_level_commas(prelude) {
        let query = query.trim();
        if query.is_empty() {
            continue;
        }

        let mut tokens = query.split_ascii_whitespace();
        let Some(mut first) = tokens.next() else {
            continue;
        };
        if first.eq_ignore_ascii_case("only") || first.eq_ignore_ascii_case("not") {
            first = tokens.next().unwrap_or("");
        }
        if first.is_empty() || first.starts_with('(') {
            // No explicit media type; defaults to `all`.
            return true;
        }

        let media = first.trim_end_matches(|c: char| c == '(');
        if media.eq_ignore_ascii_case("all") {
            return true;
        }
        if user_media.iter().any(|m| m.eq_ignore_ascii_case(media)) {
            return true;
        }
    }
    false
}
