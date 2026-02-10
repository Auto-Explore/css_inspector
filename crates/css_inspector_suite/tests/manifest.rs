use std::path::PathBuf;

use css_inspector_suite::{
    load_manifest, parse_validator_query, validate_manifest_invariants, workspace_root,
};

fn manifest_path() -> PathBuf {
    workspace_root()
        .join("test_results")
        .join("autotest_manifest.jsonl")
}

#[test]
fn manifest_is_present_and_valid() {
    let cases = load_manifest(&manifest_path()).expect("load manifest");
    assert!(!cases.is_empty(), "manifest is empty");
    validate_manifest_invariants(&cases).expect("manifest invariants");
}

#[test]
fn manifest_has_all_autotest_res_blocks() {
    // Completeness check: count `class="res"` blocks across the source HTML files.
    let root = workspace_root();
    let results_dir = root.join("test_results");
    let mut expected_res_blocks = 0usize;
    for entry in std::fs::read_dir(&results_dir).expect("read results dir") {
        let entry = entry.expect("dir entry");
        if entry.path().extension().and_then(|s| s.to_str()) != Some("html") {
            continue;
        }
        let s = std::fs::read_to_string(entry.path()).expect("read html");
        expected_res_blocks += s.matches("class=\"res\"").count();
    }

    let cases = load_manifest(&manifest_path()).expect("load manifest");
    let manifest_res_blocks = cases
        .iter()
        .filter(|c| c.source.file.starts_with("test_results/"))
        .filter(|c| c.status != "error") // includes ok + mismatch (both are `res` blocks)
        .count();

    assert_eq!(
        manifest_res_blocks, expected_res_blocks,
        "manifest does not cover all <div class=\"res\"> blocks"
    );
}

#[test]
fn manifest_round_trips_text_param_decoding() {
    let cases = load_manifest(&manifest_path()).expect("load manifest");
    for c in cases
        .iter()
        .filter(|c| c.status == "ok" && c.input.kind == "text")
    {
        let params = parse_validator_query(&c.validator_url).expect("parse query");
        let decoded = params.get("text").expect("missing text param");
        assert_eq!(
            decoded,
            c.input.text.as_ref().expect("missing input.text"),
            "decoded text mismatch for {}",
            c.id
        );
        if let Some(profile) = &c.config.profile {
            assert_eq!(params.get("profile"), Some(profile));
        }
        if let Some(medium) = &c.config.medium {
            assert_eq!(params.get("medium"), Some(medium));
        }
        if let Some(warning) = &c.config.warning {
            assert_eq!(params.get("warning"), Some(warning));
        }
    }
}
