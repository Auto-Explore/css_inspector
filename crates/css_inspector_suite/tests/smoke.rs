use css_inspector::Config;
use css_inspector_suite::{load_manifest, workspace_root};

fn run_case(id: &str) {
    let manifest = workspace_root()
        .join("test_results")
        .join("autotest_manifest.jsonl");
    let cases = load_manifest(&manifest).expect("load manifest");
    let case = cases
        .iter()
        .find(|c| c.status == "ok" && c.id == id)
        .unwrap_or_else(|| panic!("missing ok case {id}"));
    let css = case.input.text.as_deref().expect("missing input.text");
    let expected = case.expected.as_ref().expect("missing expected");

    let report = css_inspector::validate_css_text(
        css,
        &Config {
            profile: case.config.profile.clone(),
            medium: case.config.medium.clone(),
            warning: case.config.warning.clone(),
        },
    )
    .expect("validate_css_text");

    assert_eq!(report.valid(), expected.valid, "validity mismatch for {id}");
    assert_eq!(
        report.errors as i64, expected.errors,
        "error count mismatch for {id}"
    );
    assert_eq!(
        report.warnings as i64, expected.warnings,
        "warning count mismatch for {id}"
    );
}

#[test]
fn smoke_corrected_bug_invalid_value1() {
    run_case("testsuite/general/bugs/corrected-bug_invalid-value1.css");
}

#[test]
fn smoke_corrected_bug_invalid_value2() {
    run_case("testsuite/general/bugs/corrected-bug_invalid-value2.css");
}

#[test]
fn smoke_multiple_attribute_selectors_warning() {
    run_case("testsuite/general/selectors/multipleattributes-002.css");
}

#[test]
fn smoke_border_shorthand_invalid_style() {
    run_case("testsuite/bugs/920.css");
}

#[test]
fn smoke_border_important_is_allowed() {
    run_case("testsuite/bugs/703.css");
}

#[test]
fn smoke_invalid_selector_unterminated_string() {
    run_case("testsuite/bugs/2800.css");
}

#[test]
fn smoke_invalid_garbage_input() {
    run_case("testsuite/bugs/2919.css");
}

#[test]
fn smoke_bug_233_border_redefinition_warning_count() {
    run_case("testsuite/bugs/233.css");
}

#[test]
fn smoke_bug_372_huge_error_count_suite() {
    run_case("testsuite/bugs/372.css");
}

#[test]
fn smoke_huge_value_validation_suite() {
    run_case("testsuite/properties/too-many-values/box.css");
}
