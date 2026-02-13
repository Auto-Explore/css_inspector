use std::collections::HashSet;
use std::time::Duration;

use crate::config::Config;
use crate::errors::ValidatorError;
use crate::imports::iter_top_level_import_urls;
use crate::report::{Report, push_error};
use crate::strutil::starts_with_ascii_ci;
use crate::validator::{
    strip_comments_or_push_error, strip_comments_or_push_error_with, validate_css_text_stripped,
};

#[cfg(test)]
static CURL_PROGRAM_OVERRIDE: std::sync::Mutex<Option<std::ffi::OsString>> =
    std::sync::Mutex::new(None);

pub trait Fetcher {
    fn fetch(&self, uri: &str) -> Result<Vec<u8>, ValidatorError>;
}

#[derive(Clone, Debug)]
pub struct StdFetcher {
    pub allow_network: bool,
    pub max_bytes: usize,
    pub max_redirects: usize,
    pub connect_timeout: Duration,
    pub read_timeout: Duration,
}

impl Default for StdFetcher {
    fn default() -> Self {
        Self {
            allow_network: false,
            max_bytes: 2 * 1024 * 1024,
            max_redirects: 8,
            connect_timeout: Duration::from_secs(5),
            read_timeout: Duration::from_secs(30),
        }
    }
}

impl Fetcher for StdFetcher {
    fn fetch(&self, uri: &str) -> Result<Vec<u8>, ValidatorError> {
        if starts_with_ascii_ci(uri, "file://") {
            fetch_file_url(uri, self.max_bytes)
        } else if starts_with_ascii_ci(uri, "http://") || starts_with_ascii_ci(uri, "https://") {
            if !self.allow_network {
                Err(ValidatorError::InvalidInput(
                    "network fetching is disabled".into(),
                ))
            } else {
                fetch_http_url_with_curl(self, uri)
            }
        } else {
            Err(ValidatorError::InvalidInput(format!(
                "unsupported URI scheme: {uri}"
            )))
        }
    }
}

#[cfg(test)]
mod std_fetcher_tests {
    use super::{Fetcher, StdFetcher, ValidatorError};
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn fetch_rejects_network_when_disabled() {
        let fetcher = StdFetcher::default();
        let err = fetcher.fetch("http://example.com/a.css").unwrap_err();
        assert!(matches!(
            err,
            ValidatorError::InvalidInput(ref s) if s == "network fetching is disabled"
        ));

        let err = fetcher.fetch("HTTP://example.com/a.css").unwrap_err();
        assert!(matches!(
            err,
            ValidatorError::InvalidInput(ref s) if s == "network fetching is disabled"
        ));

        let err = fetcher.fetch("https://example.com/a.css").unwrap_err();
        assert!(matches!(
            err,
            ValidatorError::InvalidInput(ref s) if s == "network fetching is disabled"
        ));
    }

    #[test]
    fn fetch_rejects_unknown_schemes() {
        let fetcher = StdFetcher::default();
        let err = fetcher.fetch("ftp://example.com/a.css").unwrap_err();
        assert!(matches!(
            err,
            ValidatorError::InvalidInput(ref s) if s == "unsupported URI scheme: ftp://example.com/a.css"
        ));
    }

    #[test]
    fn fetch_reads_file_urls_and_truncates_to_max_bytes() {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("ae-css-std-fetcher-{nanos}.css"));
        fs::write(&path, b"abcdef").expect("write temp css file");

        let uri = format!("file://{}", path.display());
        let fetcher = StdFetcher {
            max_bytes: 3,
            ..StdFetcher::default()
        };
        let data = fetcher.fetch(&uri).expect("fetch file:// URL");
        assert_eq!(data, b"abc");

        let _ = fs::remove_file(path);
    }

    #[test]
    fn fetch_reads_file_urls_without_truncation_when_below_limit() {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("ae-css-std-fetcher-no-trunc-{nanos}.css"));
        fs::write(&path, b"abcdef").expect("write temp css file");

        let uri = format!("file://{}", path.display());
        let fetcher = StdFetcher {
            max_bytes: 64,
            ..StdFetcher::default()
        };
        let data = fetcher.fetch(&uri).expect("fetch file:// URL");
        assert_eq!(data, b"abcdef");

        let _ = fs::remove_file(path);
    }
}

pub fn validate_css_uri_with_fetcher(
    uri: &str,
    config: &Config,
    fetcher: &dyn Fetcher,
) -> Result<Report, ValidatorError> {
    let bytes = fetcher.fetch(uri)?;
    let css = String::from_utf8_lossy(&bytes);
    validate_css_text_with_fetcher(css.as_ref(), Some(uri), config, fetcher)
}

pub fn validate_css_text_with_fetcher(
    css: &str,
    base_uri: Option<&str>,
    config: &Config,
    fetcher: &dyn Fetcher,
) -> Result<Report, ValidatorError> {
    // Resolve top-level @import rules and validate imported sheets first, then validate the
    // current sheet. This function is intentionally conservative and exists primarily for URI
    // parity rather than full CSS parsing fidelity.
    let mut report = Report::default();
    let Some(stripped) = strip_comments_or_push_error(css, &mut report) else {
        return Ok(report);
    };
    let stripped = stripped.as_ref();

    let mut seen = HashSet::new();
    if let Some(b) = base_uri {
        seen.insert(b.to_owned());
    }
    validate_imports_recursive(stripped, base_uri, config, fetcher, &mut seen, &mut report)?;
    validate_css_text_stripped(stripped, config, false, &mut report);
    Ok(report)
}

fn validate_imports_recursive(
    css: &str,
    base_uri: Option<&str>,
    config: &Config,
    fetcher: &dyn Fetcher,
    seen: &mut HashSet<String>,
    report: &mut Report,
) -> Result<(), ValidatorError> {
    for import_url in iter_top_level_import_urls(css) {
        let resolved = resolve_relative_uri(base_uri, import_url);
        if !seen.insert(resolved.clone()) {
            push_error(report, "Import loop detected.");
            continue;
        }

        let bytes = match fetcher.fetch(&resolved) {
            Ok(bytes) => bytes,
            Err(e) => {
                push_error(report, format!("Failed to fetch @import: {e}"));
                continue;
            }
        };

        let imported_css = String::from_utf8_lossy(&bytes);
        let Some(imported_stripped) = strip_comments_or_push_error_with(
            imported_css.as_ref(),
            report,
            "Unclosed comment in @import.",
        ) else {
            continue;
        };
        let imported_stripped = imported_stripped.as_ref();

        validate_imports_recursive(
            imported_stripped,
            Some(resolved.as_str()),
            config,
            fetcher,
            seen,
            report,
        )?;
        validate_css_text_stripped(imported_stripped, config, true, report);
    }
    Ok(())
}

pub(crate) fn resolve_relative_uri(base: Option<&str>, rel: &str) -> String {
    let rel = rel.trim();
    if starts_with_ascii_ci(rel, "http://")
        || starts_with_ascii_ci(rel, "https://")
        || starts_with_ascii_ci(rel, "file://")
    {
        return rel.to_owned();
    }

    let Some(base) = base else {
        return rel.to_owned();
    };

    // Base URL parsing is intentionally strict and case-sensitive (e.g., `split_http_base` and
    // `file_url_to_path` both require lowercase schemes), so avoid implying support for mixed-case
    // schemes here.
    if let Some((scheme_host, path)) = split_http_base(base) {
        let dir = path.rsplit_once('/').map_or("", |(d, _)| d);
        return format!("{scheme_host}{dir}/{rel}");
    }
    if base.starts_with("file://")
        && let Ok(path) = file_url_to_path(base)
    {
        let dir = path.rsplit_once('/').map_or("", |(d, _)| d);
        return format!("file://{dir}/{rel}");
    }
    rel.to_owned()
}

pub(crate) fn split_http_base(base: &str) -> Option<(&str, &str)> {
    for prefix in ["http://", "https://"] {
        if let Some(rest) = base.strip_prefix(prefix) {
            let host_end = match rest.find('/') {
                Some(i) => i,
                None => {
                    // Avoid treating query-only or fragment-only origins as valid bases.
                    if rest.contains('?') || rest.contains('#') {
                        return None;
                    }
                    rest.len()
                }
            };
            let (scheme_host, path) = base.split_at(prefix.len() + host_end);
            let path = if path.is_empty() { "/" } else { path };
            return Some((scheme_host, path));
        }
    }
    None
}

fn fetch_file_url(uri: &str, max_bytes: usize) -> Result<Vec<u8>, ValidatorError> {
    use std::io::Read;

    let path = file_url_to_path(uri)?;
    let file = std::fs::File::open(&path)
        .map_err(|e| ValidatorError::InvalidInput(format!("file read failed: {e}")))?;
    let mut data = Vec::new();
    let mut limited = file.take(max_bytes as u64);
    limited
        .read_to_end(&mut data)
        .map_err(|e| ValidatorError::InvalidInput(format!("file read failed: {e}")))?;
    Ok(data)
}

pub(crate) fn file_url_to_path(uri: &str) -> Result<String, ValidatorError> {
    let rest = uri
        .strip_prefix("file://")
        .ok_or_else(|| ValidatorError::InvalidInput("not a file:// URL".into()))?;
    if rest.starts_with('/') {
        return Ok(rest.to_owned());
    }
    let (host, path) = rest
        .split_once('/')
        .ok_or_else(|| ValidatorError::InvalidInput(format!("invalid file URL: {uri}")))?;
    Ok(format!("//{host}/{path}"))
}

fn fetch_http_url_with_curl(fetcher: &StdFetcher, uri: &str) -> Result<Vec<u8>, ValidatorError> {
    use std::process::Command;

    let curl_bin: std::ffi::OsString = {
        #[cfg(test)]
        {
            CURL_PROGRAM_OVERRIDE
                .lock()
                .expect("curl override lock")
                .clone()
                .unwrap_or_else(|| "curl".into())
        }
        #[cfg(not(test))]
        {
            "curl".into()
        }
    };

    let connect_timeout = fetcher.connect_timeout.as_secs().max(1).to_string();
    let max_time = (fetcher.connect_timeout + fetcher.read_timeout)
        .as_secs()
        .max(1)
        .to_string();

    let output = Command::new(curl_bin)
        .arg("--location")
        .arg("--silent")
        .arg("--show-error")
        .arg("--max-redirs")
        .arg(fetcher.max_redirects.to_string())
        .arg("--connect-timeout")
        .arg(connect_timeout)
        .arg("--max-time")
        .arg(max_time)
        .arg("--user-agent")
        .arg("css_inspector")
        .arg("--output")
        .arg("-")
        .arg("--write-out")
        .arg("\n%{http_code}")
        .arg(uri)
        .output()
        .map_err(|e| ValidatorError::InvalidInput(format!("curl failed: {e}")))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let msg = stderr.trim();
        return Err(ValidatorError::InvalidInput(if msg.is_empty() {
            "curl failed".into()
        } else {
            format!("curl failed: {msg}")
        }));
    }

    let stdout = output.stdout;
    let Some(pos) = stdout.iter().rposition(|&b| b == b'\n') else {
        return Err(ValidatorError::InvalidInput(
            "curl did not provide an HTTP status code".into(),
        ));
    };
    let (body, status_bytes) = stdout.split_at(pos);
    let status_bytes = &status_bytes[1..];
    let status_str = std::str::from_utf8(status_bytes)
        .map_err(|_| ValidatorError::InvalidInput("curl returned a non-utf8 status code".into()))?;
    let status: u16 = status_str.trim().parse().map_err(|_| {
        ValidatorError::InvalidInput(format!("curl returned invalid status code: {status_str:?}"))
    })?;
    if !(200..300).contains(&status) {
        return Err(ValidatorError::InvalidInput(format!(
            "HTTP status {status} for {uri}"
        )));
    }

    let end = body.len().min(fetcher.max_bytes);
    Ok(body[..end].to_vec())
}

#[cfg(all(test, unix))]
mod fetch_http_url_with_curl_tests {
    use super::{Fetcher, StdFetcher, ValidatorError};
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::path::PathBuf;
    use std::sync::Mutex;
    use std::time::{SystemTime, UNIX_EPOCH};

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn unique_tmp_dir(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_nanos();
        std::env::temp_dir().join(format!("ae-css-fetcher-{name}-{nanos}"))
    }

    fn with_fake_curl(script_body: &str, f: impl FnOnce()) {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let dir = unique_tmp_dir("curl");
        fs::create_dir_all(&dir).expect("create temp dir");
        let curl_path = dir.join("curl");
        fs::write(&curl_path, script_body).expect("write fake curl");
        let mut perms = fs::metadata(&curl_path)
            .expect("stat fake curl")
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&curl_path, perms).expect("chmod fake curl");

        let mut curl_override = super::CURL_PROGRAM_OVERRIDE
            .lock()
            .expect("curl override lock");
        *curl_override = Some(curl_path.clone().into_os_string());
        drop(curl_override);

        struct Guard {
            curl_path: PathBuf,
            dir: PathBuf,
        }

        impl Drop for Guard {
            fn drop(&mut self) {
                let mut curl_override = super::CURL_PROGRAM_OVERRIDE
                    .lock()
                    .expect("curl override lock");
                *curl_override = None;
                drop(curl_override);

                let _ = fs::remove_file(&self.curl_path);
                let _ = fs::remove_dir(&self.dir);
            }
        }

        let _guard = Guard {
            curl_path: curl_path.clone(),
            dir: dir.clone(),
        };

        f();
    }

    #[test]
    fn returns_error_when_curl_exits_nonzero_with_message() {
        with_fake_curl("#!/bin/sh\necho boom 1>&2\nexit 2\n", || {
            let fetcher = StdFetcher {
                allow_network: true,
                ..StdFetcher::default()
            };
            let err = fetcher.fetch("http://example.com/x").unwrap_err();
            assert!(matches!(
                err,
                ValidatorError::InvalidInput(ref s) if s == "curl failed: boom"
            ));
        });
    }

    #[test]
    fn returns_generic_error_when_curl_exits_nonzero_without_message() {
        with_fake_curl("#!/bin/sh\nexit 2\n", || {
            let fetcher = StdFetcher {
                allow_network: true,
                ..StdFetcher::default()
            };
            let err = fetcher.fetch("http://example.com/x").unwrap_err();
            assert!(matches!(
                err,
                ValidatorError::InvalidInput(ref s) if s == "curl failed"
            ));
        });
    }

    #[test]
    fn rejects_success_without_status_code_marker() {
        with_fake_curl("#!/bin/sh\nprintf 'body'\n", || {
            let fetcher = StdFetcher {
                allow_network: true,
                ..StdFetcher::default()
            };
            let err = fetcher.fetch("https://example.com/x").unwrap_err();
            assert!(matches!(
                err,
                ValidatorError::InvalidInput(ref s) if s == "curl did not provide an HTTP status code"
            ));
        });
    }

    #[test]
    fn rejects_invalid_status_codes_and_non_success_http_responses() {
        with_fake_curl("#!/bin/sh\nprintf 'body\\nabc'\n", || {
            let fetcher = StdFetcher {
                allow_network: true,
                ..StdFetcher::default()
            };
            let err = fetcher.fetch("http://example.com/x").unwrap_err();
            assert!(matches!(
                err,
                ValidatorError::InvalidInput(ref s) if s == "curl returned invalid status code: \"abc\""
            ));
        });

        with_fake_curl("#!/bin/sh\nprintf 'body\\n404'\n", || {
            let fetcher = StdFetcher {
                allow_network: true,
                ..StdFetcher::default()
            };
            let err = fetcher.fetch("http://example.com/x").unwrap_err();
            assert!(matches!(
                err,
                ValidatorError::InvalidInput(ref s) if s == "HTTP status 404 for http://example.com/x"
            ));
        });
    }

    #[test]
    fn truncates_body_when_over_max_bytes() {
        with_fake_curl("#!/bin/sh\nprintf 'abcdef\\n200'\n", || {
            let fetcher = StdFetcher {
                allow_network: true,
                max_bytes: 3,
                ..StdFetcher::default()
            };
            let bytes = fetcher.fetch("http://example.com/x").unwrap();
            assert_eq!(bytes, b"abc");
        });
    }
}
