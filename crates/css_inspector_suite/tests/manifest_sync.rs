use std::process::Command;

use css_inspector_suite::workspace_root;

#[test]
#[ignore]
fn manifest_is_in_sync_with_autotest_results_html() {
    let root = workspace_root();
    let script = root.join("scripts").join("extract_autotest_manifest.py");
    let manifest = workspace_root()
        .join("test_results")
        .join("autotest_manifest.jsonl");

    let out = Command::new("python3")
        .arg(&script)
        .current_dir(&root)
        .output()
        .expect("run extractor");
    assert!(out.status.success(), "extractor failed");

    let generated = String::from_utf8(out.stdout).expect("extractor output not utf-8");
    let committed = std::fs::read_to_string(&manifest).expect("read committed manifest");
    assert_eq!(
        generated, committed,
        "manifest is out of date; re-run extractor"
    );
}
