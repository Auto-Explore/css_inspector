use css_inspector::Config;
use css_inspector_suite::{load_manifest, workspace_root};

#[test]
fn full_autotest_suite_matches_expected_counts() {
    let manifest = workspace_root()
        .join("test_results")
        .join("autotest_manifest.jsonl");
    let cases = load_manifest(&manifest).expect("load manifest");

    for c in cases
        .iter()
        .filter(|c| c.status == "ok" && c.input.kind == "text")
    {
        let css = c.input.text.as_deref().unwrap_or("");
        let expected = c.expected.as_ref().expect("missing expected");
        let report = css_inspector::validate_css_text(
            css,
            &Config {
                profile: c.config.profile.clone(),
                medium: c.config.medium.clone(),
                warning: c.config.warning.clone(),
            },
        )
        .expect("validate_css_text");

        assert_eq!(
            report.valid(),
            expected.valid,
            "validity mismatch for {}",
            c.id
        );
        assert_eq!(
            report.errors as i64, expected.errors,
            "error count mismatch for {}",
            c.id
        );
        assert_eq!(
            report.warnings as i64, expected.warnings,
            "warning count mismatch for {}",
            c.id
        );
    }
}
