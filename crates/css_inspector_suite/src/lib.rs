use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

mod wpt_style;

pub use wpt_style::{
    WPT_CSS_STYLE_RESULTS_FORMAT_VERSION, WPT_CSS_STYLE_RESULTS_META_FILE, WptCssStyleBlockResult,
    WptCssStyleCheckOptions, WptCssStyleCheckSummary, WptCssStyleFailure, WptCssStyleFailureKind,
    WptCssStyleFileResults, WptCssStyleResultsMeta, WptCssStyleWriteSummary,
    check_wpt_css_style_results_tree, extract_style_blocks, git_head_commit,
    load_wpt_css_style_results_meta, wpt_css_style_results_file_path,
    wpt_css_style_results_meta_path, write_wpt_css_style_file_results_atomic,
    write_wpt_css_style_results_meta_atomic, write_wpt_css_style_results_tree,
};

#[derive(Debug, Error)]
pub enum SuiteError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("invalid manifest: {0}")]
    InvalidManifest(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Expected {
    pub valid: bool,
    pub errors: i64,
    pub warnings: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Input {
    pub kind: String, // text | uri | unknown
    pub text: Option<String>,
    pub uri: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub profile: Option<String>,
    pub medium: Option<String>,
    pub warning: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Source {
    pub file: String,
    pub index: usize,
    pub section: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Case {
    pub id: String,
    pub source: Source,
    pub validator_url: String,
    pub input: Input,
    pub config: Config,
    pub expected: Option<Expected>,
    pub observed: Option<Expected>,
    pub status: String, // ok | mismatch | error
    pub note: Option<String>,
}

pub fn load_manifest(path: &Path) -> Result<Vec<Case>, SuiteError> {
    let s = fs::read_to_string(path)?;
    let mut cases = Vec::new();
    for (idx, line) in s.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || line.starts_with('#') {
            continue;
        }
        let c: Case = serde_json::from_str(trimmed).map_err(|e| {
            SuiteError::InvalidManifest(format!(
                "{path}:{line_no}: {e}",
                path = path.display(),
                line_no = idx + 1
            ))
        })?;
        cases.push(c);
    }
    Ok(cases)
}

pub fn workspace_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for dir in manifest_dir.ancestors() {
        let cargo_toml = dir.join("Cargo.toml");
        if !cargo_toml.exists() {
            continue;
        }
        let Ok(s) = fs::read_to_string(&cargo_toml) else {
            continue;
        };
        if s.contains("[workspace]") {
            return dir.to_path_buf();
        }
    }
    panic!(
        "workspace root not found from CARGO_MANIFEST_DIR={}",
        manifest_dir.display()
    );
}

pub fn url_decode_plus(input: &str) -> Result<String, SuiteError> {
    css_inspector::url_decode_plus(input).map_err(|e| SuiteError::InvalidManifest(e.to_string()))
}

pub fn parse_validator_query(url: &str) -> Result<HashMap<String, String>, SuiteError> {
    let mut out = HashMap::new();
    let Some((_, qs)) = url.split_once('?') else {
        return Ok(out);
    };
    for part in qs.split('&') {
        if part.is_empty() {
            continue;
        }
        let (k, v) = part.split_once('=').unwrap_or((part, ""));
        out.insert(url_decode_plus(k)?, url_decode_plus(v)?);
    }
    Ok(out)
}

pub fn validate_manifest_invariants(cases: &[Case]) -> Result<(), SuiteError> {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    struct CaseKey<'a> {
        id: &'a str,
        kind: &'a str,
        profile: Option<&'a str>,
        medium: Option<&'a str>,
        warning: Option<&'a str>,
    }

    let mut seen: HashSet<CaseKey<'_>> = HashSet::with_capacity(cases.len());
    for c in cases {
        let key = CaseKey {
            id: &c.id,
            kind: &c.input.kind,
            profile: c.config.profile.as_deref(),
            medium: c.config.medium.as_deref(),
            warning: c.config.warning.as_deref(),
        };
        if !seen.insert(key) {
            return Err(SuiteError::InvalidManifest(format!(
                "duplicate case key for id={}",
                c.id
            )));
        }
        if c.status != "ok" {
            continue;
        }

        let (Some(expected), Some(observed)) = (c.expected.as_ref(), c.observed.as_ref()) else {
            return Err(SuiteError::InvalidManifest(format!(
                "ok case missing expected/observed: {}",
                c.id
            )));
        };
        if expected != observed {
            return Err(SuiteError::InvalidManifest(format!(
                "ok case has expected != observed: {}",
                c.id
            )));
        }

        match c.input.kind.as_str() {
            "text" if c.input.text.as_deref().unwrap_or("").is_empty() => {
                return Err(SuiteError::InvalidManifest(format!(
                    "text case missing input.text: {}",
                    c.id
                )));
            }
            "uri" if c.input.uri.as_deref().unwrap_or("").is_empty() => {
                return Err(SuiteError::InvalidManifest(format!(
                    "uri case missing input.uri: {}",
                    c.id
                )));
            }
            _ => {}
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::{
        Case, Config, Input, Source, load_manifest, parse_validator_query, url_decode_plus,
        validate_manifest_invariants, workspace_root,
    };

    #[test]
    fn parse_validator_query_handles_missing_query_string() {
        let q = parse_validator_query("http://example.com/path").unwrap();
        assert!(q.is_empty());
    }

    #[test]
    fn parse_validator_query_handles_empty_query_string() {
        let q = parse_validator_query("http://example.com/?").unwrap();
        assert!(q.is_empty());
    }

    #[test]
    fn parse_validator_query_decodes_plus_and_percent_escapes() {
        let q = parse_validator_query("http://example.com/?a=b+c&x=%2F").unwrap();
        assert_eq!(q.get("a").map(String::as_str), Some("b c"));
        assert_eq!(q.get("x").map(String::as_str), Some("/"));
    }

    #[test]
    fn parse_validator_query_handles_parts_without_equals() {
        let q = parse_validator_query("http://example.com/?a&b=&c=d").unwrap();
        assert_eq!(q.get("a").map(String::as_str), Some(""));
        assert_eq!(q.get("b").map(String::as_str), Some(""));
        assert_eq!(q.get("c").map(String::as_str), Some("d"));
    }

    #[test]
    fn parse_validator_query_ignores_empty_parts() {
        let q = parse_validator_query("http://example.com/?a=b&&c=d&").unwrap();
        assert_eq!(q.get("a").map(String::as_str), Some("b"));
        assert_eq!(q.get("c").map(String::as_str), Some("d"));
        assert_eq!(q.len(), 2);
    }

    #[test]
    fn parse_validator_query_rejects_invalid_escapes() {
        let err = parse_validator_query("http://example.com/?bad=%ZZ").unwrap_err();
        assert!(matches!(err, super::SuiteError::InvalidManifest(_)));
        assert!(err.to_string().contains("invalid hex digit"));
    }

    #[test]
    fn parse_validator_query_last_value_wins_for_duplicate_keys() {
        let q = parse_validator_query("http://example.com/?a=1&a=2").unwrap();
        assert_eq!(q.get("a").map(String::as_str), Some("2"));
    }

    #[test]
    fn url_decode_plus_returns_input_when_no_escapes() {
        assert_eq!(url_decode_plus("abcDEF123-_.*").unwrap(), "abcDEF123-_.*");
    }

    #[test]
    fn url_decode_plus_decodes_plus_and_percent_escapes() {
        assert_eq!(url_decode_plus("a+b").unwrap(), "a b");
        assert_eq!(url_decode_plus("abc%2Fdef").unwrap(), "abc/def");
    }

    #[test]
    fn url_decode_plus_rejects_incomplete_percent_escapes() {
        let err = url_decode_plus("%").unwrap_err();
        assert!(err.to_string().contains("incomplete percent-escape"));
        let err = url_decode_plus("%A").unwrap_err();
        assert!(err.to_string().contains("incomplete percent-escape"));
    }

    #[test]
    fn url_decode_plus_rejects_invalid_hex_digits() {
        let err = url_decode_plus("%GG").unwrap_err();
        assert!(err.to_string().contains("invalid hex digit"));
    }

    #[test]
    fn url_decode_plus_rejects_decoded_invalid_utf8() {
        let err = url_decode_plus("%FF").unwrap_err();
        assert!(err.to_string().contains("invalid utf-8 in decoded string"));
    }

    fn base_case(id: &str) -> Case {
        Case {
            id: id.to_string(),
            source: Source {
                file: "f".to_string(),
                index: 0,
                section: None,
            },
            validator_url: "http://example.com/".to_string(),
            input: Input {
                kind: "text".to_string(),
                text: Some("a{}".to_string()),
                uri: None,
            },
            config: Config {
                profile: None,
                medium: None,
                warning: None,
            },
            expected: Some(super::Expected {
                valid: true,
                errors: 0,
                warnings: 0,
            }),
            observed: Some(super::Expected {
                valid: true,
                errors: 0,
                warnings: 0,
            }),
            status: "ok".to_string(),
            note: None,
        }
    }

    #[test]
    fn validate_manifest_invariants_rejects_duplicate_key() {
        let mut a = base_case("a");
        let b = base_case("a");
        // Make sure the cases are not byte-for-byte identical: only the key matters.
        a.source.index = 1;
        let err = validate_manifest_invariants(&[a, b]).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("duplicate case key"));
    }

    #[test]
    fn validate_manifest_invariants_allows_same_id_with_different_config() {
        let mut a = base_case("a");
        let mut b = base_case("a");
        a.config.warning = Some("1".to_string());
        b.config.warning = Some("2".to_string());
        validate_manifest_invariants(&[a, b]).unwrap();
    }

    #[test]
    fn validate_manifest_invariants_rejects_ok_text_case_missing_input_text() {
        let mut c = base_case("a");
        c.input.text = None;
        let err = validate_manifest_invariants(&[c]).unwrap_err();
        assert!(err.to_string().contains("text case missing input.text"));
    }

    #[test]
    fn validate_manifest_invariants_rejects_ok_uri_case_missing_input_uri() {
        let mut c = base_case("a");
        c.input.kind = "uri".to_string();
        c.input.text = None;
        c.input.uri = None;
        let err = validate_manifest_invariants(&[c]).unwrap_err();
        assert!(err.to_string().contains("uri case missing input.uri"));
    }

    #[test]
    fn load_manifest_skips_blank_and_hash_comment_lines() {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);

        let mut path = std::env::temp_dir();
        path.push(format!("css_inspector_load_manifest_{n}.jsonl"));

        let json = r#"{"id":"1","source":{"file":"f","index":0,"section":null},"validator_url":"http://example.com/","input":{"kind":"text","text":"x","uri":null},"config":{"profile":null,"medium":null,"warning":null},"expected":null,"observed":null,"status":"error","note":null}"#;
        let contents = format!("\n  \n# comment\n{json}\n# trailing comment\n");
        std::fs::write(&path, contents).unwrap();

        let cases = load_manifest(&path).unwrap();
        assert_eq!(cases.len(), 1);
        assert_eq!(cases[0].id, "1");

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn load_manifest_accepts_json_lines_wrapped_in_whitespace() {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);

        let mut path = std::env::temp_dir();
        path.push(format!(
            "css_inspector_load_manifest_whitespace_json_{n}.jsonl"
        ));

        let json = r#"{"id":"1","source":{"file":"f","index":0,"section":null},"validator_url":"http://example.com/","input":{"kind":"text","text":"x","uri":null},"config":{"profile":null,"medium":null,"warning":null},"expected":null,"observed":null,"status":"error","note":null}"#;
        std::fs::write(&path, format!("  {json}  \n")).unwrap();

        let cases = load_manifest(&path).unwrap();
        assert_eq!(cases.len(), 1);
        assert_eq!(cases[0].id, "1");

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn load_manifest_does_not_treat_whitespace_prefixed_hash_as_comment() {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);

        let mut path = std::env::temp_dir();
        path.push(format!(
            "css_inspector_load_manifest_whitespace_hash_{n}.jsonl"
        ));

        let json = r#"{"id":"1","source":{"file":"f","index":0,"section":null},"validator_url":"http://example.com/","input":{"kind":"text","text":"x","uri":null},"config":{"profile":null,"medium":null,"warning":null},"expected":null,"observed":null,"status":"error","note":null}"#;
        let contents = format!("  # not a comment\n{json}\n");
        std::fs::write(&path, contents).unwrap();

        let err = load_manifest(&path).unwrap_err();
        assert!(err.to_string().contains(":1:"), "{err}");

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn load_manifest_reports_line_number_for_invalid_json() {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let n = COUNTER.fetch_add(1, Ordering::Relaxed);

        let mut path = std::env::temp_dir();
        path.push(format!(
            "css_inspector_load_manifest_invalid_json_{n}.jsonl"
        ));

        std::fs::write(&path, "\n{not json}\n").unwrap();

        let err = load_manifest(&path).unwrap_err();
        assert!(
            err.to_string().contains(&format!("{}:2:", path.display())),
            "{err}"
        );

        let _ = std::fs::remove_file(&path);
    }
}
