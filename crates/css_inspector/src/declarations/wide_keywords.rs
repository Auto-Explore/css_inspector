use crate::strutil::starts_with_ascii_ci;

pub(super) fn has_css_wide_keyword_mixed(tokens: &[&str]) -> bool {
    // If any CSS-wide keyword appears, it must be the only component.
    tokens.len() != 1 && tokens.iter().any(|t| is_css_wide_keywordish_token(t))
}

const CSS_WIDE_KEYWORDS: [&str; 6] = [
    "inherit",
    "initial",
    "unset",
    "revert",
    "revert-layer",
    "revert-rule",
];

#[cfg(test)]
mod has_css_wide_keyword_mixed_tests {
    use super::has_css_wide_keyword_mixed;

    #[test]
    fn returns_false_for_empty_or_single_component() {
        assert!(!has_css_wide_keyword_mixed(&[]));
        assert!(!has_css_wide_keyword_mixed(&["inherit"]));
        assert!(!has_css_wide_keyword_mixed(&["inherit/20%"]));
        assert!(!has_css_wide_keyword_mixed(&["red"]));
    }

    #[test]
    fn returns_true_when_css_wide_keyword_appears_with_other_tokens() {
        assert!(has_css_wide_keyword_mixed(&["inherit", "20%"]));
        assert!(has_css_wide_keyword_mixed(&["red", "unset"]));
        assert!(has_css_wide_keyword_mixed(&["inherit/20%", "red"]));
    }

    #[test]
    fn returns_false_when_no_css_wide_keywordish_token_is_present() {
        assert!(!has_css_wide_keyword_mixed(&["red", "blue"]));
        assert!(!has_css_wide_keyword_mixed(&["1px", "solid"]));
    }
}

pub(crate) fn is_css_wide_keyword(token: &str) -> bool {
    let t = token.trim();
    CSS_WIDE_KEYWORDS
        .iter()
        .any(|&kw| t.eq_ignore_ascii_case(kw))
}

pub(crate) fn is_css_wide_keywordish_token(token: &str) -> bool {
    // Also treat `<css-wide-keyword>/...` as a css-wide keyword usage for mixed-value detection,
    // e.g. `inherit/20%` in shorthand values.
    let t = token.trim();
    if is_css_wide_keyword(t) {
        return true;
    }
    for kw in CSS_WIDE_KEYWORDS {
        if t.len() > kw.len()
            && starts_with_ascii_ci(t, kw)
            && t.as_bytes().get(kw.len()) == Some(&b'/')
        {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod is_css_wide_keyword_tests {
    use super::{is_css_wide_keyword, is_css_wide_keywordish_token};

    #[test]
    fn matches_case_insensitively_and_trims() {
        for token in ["inherit", " INHERIT ", "ReVeRt-LaYeR", "ReVeRt-RuLe"] {
            assert!(is_css_wide_keyword(token), "{token}");
        }
        assert!(!is_css_wide_keyword("inherit/20%"));
    }

    #[test]
    fn keywordish_tokens_include_slash_forms() {
        for token in ["inherit/20%", "inherit/", " InHeRiT/20% "] {
            assert!(is_css_wide_keywordish_token(token), "{token}");
        }
        assert!(!is_css_wide_keywordish_token("inherit /20%"));
    }
}
