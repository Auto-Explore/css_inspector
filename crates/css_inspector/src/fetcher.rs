use std::collections::HashSet;
use std::time::Duration;

use crate::config::Config;
use crate::errors::ValidatorError;
use crate::imports::iter_top_level_import_urls;
use crate::report::{Report, push_error};
use crate::strutil::{contains_ascii_ci, starts_with_ascii_ci};
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

    // Base URL parsing is intentionally strict and case-sensitive (e.g., `parse_http_url` and
    // `file_url_to_path` both require lowercase schemes), so avoid implying support for mixed-case
    // schemes here.
    if let Some((scheme_host, path)) = split_http_base(base) {
        let dir = path.rsplit_once('/').map_or("", |(d, _)| d);
        return format!("{scheme_host}{dir}/{rel}");
    }
    if base.starts_with("file://") {
        if let Ok(path) = file_url_to_path(base) {
            let dir = path.rsplit_once('/').map_or("", |(d, _)| d);
            return format!("file://{dir}/{rel}");
        }
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
    let path = file_url_to_path(uri)?;
    let mut data = std::fs::read(path)
        .map_err(|e| ValidatorError::InvalidInput(format!("file read failed: {e}")))?;
    data.truncate(max_bytes);
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

    let mut data = body.to_vec();
    data.truncate(fetcher.max_bytes);
    Ok(data)
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

    fn with_fake_curl(script_body: &str, f: impl FnOnce() -> ()) {
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

fn fetch_http_url(fetcher: &StdFetcher, uri: &str) -> Result<Vec<u8>, ValidatorError> {
    let mut current = uri.to_owned();
    for _ in 0..=fetcher.max_redirects {
        let (host, port, path) = parse_http_url(&current)?;
        let data = http_get_bytes(fetcher, host, port, path)?;
        let (status, headers, body) = parse_http_response(&data)?;
        if matches!(status, 301 | 302 | 303 | 307 | 308) {
            if let Some(loc) = header_value_ascii_ci(&headers, "location") {
                current = resolve_relative_uri(Some(&current), loc);
                continue;
            }
        }
        if !(200..300).contains(&status) {
            return Err(ValidatorError::InvalidInput(format!(
                "HTTP status {status} for {current}"
            )));
        }
        return Ok(body);
    }
    Err(ValidatorError::InvalidInput("too many redirects".into()))
}

#[inline]
fn header_value_ascii_ci<'a>(headers: &'a [(String, String)], name: &str) -> Option<&'a str> {
    headers
        .iter()
        .find_map(|(k, v)| k.eq_ignore_ascii_case(name).then_some(v.as_str()))
}

#[cfg(test)]
mod header_value_ascii_ci_tests {
    use super::header_value_ascii_ci;

    #[test]
    fn finds_first_matching_header_value_case_insensitively() {
        let headers = vec![
            ("Location".to_string(), "a".to_string()),
            ("location".to_string(), "b".to_string()),
        ];
        assert_eq!(header_value_ascii_ci(&headers, "location"), Some("a"));
    }

    #[test]
    fn returns_some_for_empty_header_value() {
        let headers = vec![("location".to_string(), "".to_string())];
        assert_eq!(header_value_ascii_ci(&headers, "Location"), Some(""));
    }

    #[test]
    fn returns_first_value_even_when_empty() {
        let headers = vec![
            ("Location".to_string(), "".to_string()),
            ("location".to_string(), "b".to_string()),
        ];
        assert_eq!(header_value_ascii_ci(&headers, "location"), Some(""));
    }

    #[test]
    fn returns_none_when_missing() {
        let headers = vec![("x".to_string(), "y".to_string())];
        assert_eq!(header_value_ascii_ci(&headers, "location"), None);
    }
}

pub(crate) fn parse_http_url(uri: &str) -> Result<(&str, u16, &str), ValidatorError> {
    let rest = uri
        .strip_prefix("http://")
        .ok_or_else(|| ValidatorError::InvalidInput("not an http:// URL".into()))?;

    let (host_port, path) = rest
        .find('/')
        // The index always points at the ASCII `/` byte, which is a UTF-8 boundary.
        .map_or((rest, "/"), |i| rest.split_at(i));

    let (host, port) = match host_port.rsplit_once(':') {
        Some((h, p)) => {
            let port = p
                .parse::<u16>()
                .map_err(|_| ValidatorError::InvalidInput(format!("invalid port in URL: {uri}")))?;
            (h, port)
        }
        None => (host_port, 80u16),
    };

    Ok((host, port, path))
}

#[cfg(test)]
mod parse_http_url_tests {
    use super::{ValidatorError, parse_http_url};

    #[test]
    fn parses_default_port_and_root_path() {
        let (host, port, path) = parse_http_url("http://example.com").unwrap();
        assert_eq!(host, "example.com");
        assert_eq!(port, 80);
        assert_eq!(path, "/");
    }

    #[test]
    fn parses_explicit_port_and_path() {
        let (host, port, path) = parse_http_url("http://example.com:8080/a/b").unwrap();
        assert_eq!(host, "example.com");
        assert_eq!(port, 8080);
        assert_eq!(path, "/a/b");
    }

    #[test]
    fn rejects_non_http_schemes_or_mixed_case() {
        let err = parse_http_url("https://example.com/").unwrap_err();
        assert!(matches!(
            err,
            ValidatorError::InvalidInput(ref s) if s == "not an http:// URL"
        ));

        let err = parse_http_url("HTTP://example.com/").unwrap_err();
        assert!(matches!(
            err,
            ValidatorError::InvalidInput(ref s) if s == "not an http:// URL"
        ));
    }

    #[test]
    fn rejects_invalid_ports() {
        let err = parse_http_url("http://example.com:nope/").unwrap_err();
        assert!(matches!(
            err,
            ValidatorError::InvalidInput(ref s) if s == "invalid port in URL: http://example.com:nope/"
        ));
    }
}

fn http_get_bytes(
    fetcher: &StdFetcher,
    host: &str,
    port: u16,
    path: &str,
) -> Result<Vec<u8>, ValidatorError> {
    use std::io::{Read, Write};
    use std::net::TcpStream;

    let addr = format!("{host}:{port}");
    let mut stream = TcpStream::connect(addr)
        .map_err(|e| ValidatorError::InvalidInput(format!("connect failed: {e}")))?;
    let _ = stream.set_read_timeout(Some(fetcher.read_timeout));
    let _ = stream.set_write_timeout(Some(fetcher.connect_timeout));

    let req = format!(
        "GET {path} HTTP/1.1\r\nHost: {host}\r\nUser-Agent: css_inspector\r\nAccept: */*\r\nConnection: close\r\n\r\n"
    );
    stream
        .write_all(req.as_bytes())
        .map_err(|e| ValidatorError::InvalidInput(format!("write failed: {e}")))?;

    let mut buf = Vec::new();
    let mut tmp = [0u8; 8192];
    loop {
        let n = stream
            .read(&mut tmp)
            .map_err(|e| ValidatorError::InvalidInput(format!("read failed: {e}")))?;
        if n == 0 {
            break;
        }
        buf.extend_from_slice(&tmp[..n]);
        if buf.len() > fetcher.max_bytes {
            break;
        }
    }
    Ok(buf)
}

#[inline]
pub(crate) fn find_double_crlf(data: &[u8]) -> Option<usize> {
    data.windows(4).position(|w| w == b"\r\n\r\n")
}

#[cfg(test)]
mod find_double_crlf_tests {
    use super::find_double_crlf;

    #[test]
    fn finds_first_double_crlf_sequence() {
        assert_eq!(find_double_crlf(b"\r\n\r\n"), Some(0));
        assert_eq!(find_double_crlf(b"a\r\n\r\nb"), Some(1));
        assert_eq!(find_double_crlf(b"a\r\nb\r\n\r\nc"), Some(4));
    }

    #[test]
    fn returns_none_when_missing() {
        assert_eq!(find_double_crlf(b""), None);
        assert_eq!(find_double_crlf(b"\r\n\r"), None);
        assert_eq!(find_double_crlf(b"\n\n\n\n"), None);
    }
}

pub(crate) type HttpHeaders = Vec<(String, String)>;
pub(crate) type ParsedHttpResponse = (u16, HttpHeaders, Vec<u8>);

pub(crate) fn parse_http_response(data: &[u8]) -> Result<ParsedHttpResponse, ValidatorError> {
    let Some(split) = find_double_crlf(data) else {
        return Err(ValidatorError::InvalidInput("invalid HTTP response".into()));
    };

    // Decode the header and body separately. This preserves the current behavior of treating the
    // body as UTF-8 lossily (because the validator expects text), while avoiding a full-response
    // allocation when the body contains invalid UTF-8.
    let head = String::from_utf8_lossy(&data[..split]);
    let body_raw = String::from_utf8_lossy(&data[split + 4..]);

    let mut lines = head.lines();
    let status_line = lines
        .next()
        .ok_or_else(|| ValidatorError::InvalidInput("missing status line".into()))?;
    let code = status_line
        .split_whitespace()
        .nth(1)
        .ok_or_else(|| ValidatorError::InvalidInput("missing status code".into()))?
        .parse::<u16>()
        .map_err(|_| ValidatorError::InvalidInput("invalid status code".into()))?;

    let mut headers: HttpHeaders = Vec::new();
    let mut is_chunked = false;
    for line in lines {
        let Some((k, v)) = line.split_once(':') else {
            continue;
        };
        let k = k.trim().to_owned();
        let v = v.trim().to_owned();
        if !is_chunked
            && k.eq_ignore_ascii_case("transfer-encoding")
            && contains_ascii_ci(&v, "chunked")
        {
            is_chunked = true;
        }
        headers.push((k, v));
    }
    let body = if is_chunked {
        decode_chunked(body_raw.as_bytes())?
    } else {
        body_raw.into_owned().into_bytes()
    };
    Ok((code, headers, body))
}

#[cfg(test)]
mod parse_http_response_tests {
    use super::{ValidatorError, parse_http_response};

    #[test]
    fn parses_simple_response_with_body() {
        let data = b"HTTP/1.1 200 OK\r\nContent-Type: text/css\r\n\r\nbody";
        let (status, headers, body) = parse_http_response(data).unwrap();
        assert_eq!(status, 200);
        assert_eq!(
            headers,
            vec![("Content-Type".to_string(), "text/css".to_string())]
        );
        assert_eq!(body, b"body");
    }

    #[test]
    fn parses_chunked_response_body_when_header_indicates_chunked() {
        let data =
            b"HTTP/1.1 200 OK\r\nTransfer-Encoding: gzip, Chunked\r\n\r\n4\r\nWiki\r\n0\r\n\r\n";
        let (status, headers, body) = parse_http_response(data).unwrap();
        assert_eq!(status, 200);
        assert_eq!(
            headers,
            vec![("Transfer-Encoding".to_string(), "gzip, Chunked".to_string())]
        );
        assert_eq!(body, b"Wiki");
    }

    #[test]
    fn errors_on_missing_delimiter() {
        let err = parse_http_response(b"HTTP/1.1 200 OK\r\n").unwrap_err();
        assert!(matches!(
            err,
            ValidatorError::InvalidInput(ref s) if s == "invalid HTTP response"
        ));
    }

    #[test]
    fn errors_on_missing_or_invalid_status_code() {
        let err = parse_http_response(b"\r\n\r\nbody").unwrap_err();
        assert!(matches!(
            err,
            ValidatorError::InvalidInput(ref s) if s == "missing status line"
        ));

        let err = parse_http_response(b"HTTP/1.1\r\n\r\nbody").unwrap_err();
        assert!(matches!(
            err,
            ValidatorError::InvalidInput(ref s) if s == "missing status code"
        ));

        let err = parse_http_response(b"HTTP/1.1 x\r\n\r\nbody").unwrap_err();
        assert!(matches!(
            err,
            ValidatorError::InvalidInput(ref s) if s == "invalid status code"
        ));
    }
}

pub(crate) fn decode_chunked(data: &[u8]) -> Result<Vec<u8>, ValidatorError> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while let Some(pos) = memchr_crlf(data, i) {
        let line = std::str::from_utf8(&data[i..pos])
            .map_err(|_| ValidatorError::InvalidInput("invalid chunk header".into()))?;
        let size_str = line
            .split_once(';')
            .map_or(line, |(before, _)| before)
            .trim();
        let size = usize::from_str_radix(size_str, 16)
            .map_err(|_| ValidatorError::InvalidInput("invalid chunk size".into()))?;
        i = pos + 2;
        if size == 0 {
            break;
        }
        let chunk_end = i + size;
        if chunk_end > data.len() {
            return Err(ValidatorError::InvalidInput("truncated chunk".into()));
        }
        out.extend_from_slice(&data[i..chunk_end]);
        i = chunk_end;
        if data[i..].starts_with(b"\r\n") {
            i += 2;
        }
    }
    Ok(out)
}

pub(crate) fn memchr_crlf(data: &[u8], start: usize) -> Option<usize> {
    let mut i = start;
    while i + 1 < data.len() {
        if data[i] == b'\r' && data[i + 1] == b'\n' {
            return Some(i);
        }
        i += 1;
    }
    None
}

#[cfg(test)]
mod memchr_crlf_tests {
    use super::memchr_crlf;

    #[test]
    fn finds_crlf_at_or_after_start() {
        let data = b"a\r\nb\r\n";
        assert_eq!(memchr_crlf(data, 0), Some(1));
        assert_eq!(memchr_crlf(data, 1), Some(1));
        assert_eq!(memchr_crlf(data, 2), Some(4));
        assert_eq!(memchr_crlf(data, 4), Some(4));
    }

    #[test]
    fn returns_none_when_missing_or_past_end() {
        assert_eq!(memchr_crlf(b"", 0), None);
        assert_eq!(memchr_crlf(b"abc", 0), None);
        assert_eq!(memchr_crlf(b"\r", 0), None);
        assert_eq!(memchr_crlf(b"\r\n", 2), None);
    }
}

#[cfg(test)]
mod decode_chunked_tests {
    use super::decode_chunked;

    #[test]
    fn decodes_basic_chunked_body() {
        let body = b"4\r\nWiki\r\n5\r\npedia\r\n0\r\n\r\n";
        assert_eq!(decode_chunked(body).unwrap(), b"Wikipedia");
    }

    #[test]
    fn ignores_chunk_extensions() {
        let body = b"4;ext=1\r\nWiki\r\n0\r\n\r\n";
        assert_eq!(decode_chunked(body).unwrap(), b"Wiki");
    }

    #[test]
    fn accepts_missing_final_crlf_after_chunk_data() {
        let body = b"1\r\na";
        assert_eq!(decode_chunked(body).unwrap(), b"a");
    }

    #[test]
    fn errors_on_truncated_chunk() {
        let body = b"2\r\na";
        let err = decode_chunked(body).unwrap_err();
        assert!(matches!(
            err,
            super::ValidatorError::InvalidInput(ref s) if s == "truncated chunk"
        ));
    }

    #[test]
    fn errors_on_invalid_chunk_size() {
        let body = b"nope\r\nx\r\n";
        let err = decode_chunked(body).unwrap_err();
        assert!(matches!(
            err,
            super::ValidatorError::InvalidInput(ref s) if s == "invalid chunk size"
        ));
    }
}
