use super::*;

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
fn at_rule_name_rejects_empty_or_missing_names() {
    assert_eq!(at_rule_name("@"), None);
    assert_eq!(at_rule_name("@   "), None);
    assert_eq!(at_rule_name("media screen"), None);
}
