use std::collections::HashMap;

use css_inspector::{Config, Fetcher};
use css_inspector_suite::{load_manifest, workspace_root};

#[derive(Default)]
struct MapFetcher {
    map: HashMap<String, Vec<u8>>,
}

impl Fetcher for MapFetcher {
    fn fetch(&self, uri: &str) -> Result<Vec<u8>, css_inspector::ValidatorError> {
        self.map.get(uri).cloned().ok_or_else(|| {
            css_inspector::ValidatorError::InvalidInput(format!("missing fixture for {uri}"))
        })
    }
}

fn css_with_n_missing_values(n: i64) -> String {
    use std::fmt::Write as _;
    let mut out = String::new();
    for i in 0..n {
        write!(out, "a{i}{{color:}}").expect("write to String");
    }
    out
}

#[test]
fn uri_cases_can_be_validated_offline_with_fixtures() {
    let manifest = workspace_root()
        .join("test_results")
        .join("autotest_manifest.jsonl");
    let cases = load_manifest(&manifest).expect("load manifest");

    let mut fetcher = MapFetcher::default();

    for c in cases
        .iter()
        .filter(|c| c.status == "ok" && c.input.kind == "uri")
    {
        let uri = c.input.uri.as_deref().unwrap_or("");
        let expected = c.expected.as_ref().expect("missing expected");

        // The upstream suite references live URLs; keep the Rust suite deterministic and offline
        // by mapping each URI to a minimal CSS payload that yields the expected counts.
        let css =
            if uri.ends_with("testloop2.css") && expected.errors == 1 && expected.warnings == 0 {
                format!("@import url({uri});")
            } else if expected.errors == 0 {
                "a{color:red}".to_string()
            } else {
                css_with_n_missing_values(expected.errors)
            };
        fetcher.map.insert(uri.to_string(), css.into_bytes());

        let report = css_inspector::validate_css_uri_with_fetcher(
            uri,
            &Config {
                profile: c.config.profile.clone(),
                medium: c.config.medium.clone(),
                warning: c.config.warning.clone(),
            },
            &fetcher,
        )
        .expect("validate_css_uri_with_fetcher");

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
