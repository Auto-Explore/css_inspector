use super::*;

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
