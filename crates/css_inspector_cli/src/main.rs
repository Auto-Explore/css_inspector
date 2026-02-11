use std::env;
use std::io::Write;
use std::path::PathBuf;

use css_inspector::{
    Config, Message, Report, Severity, StdFetcher, ValidatorError, starts_with_ascii_ci,
};

type Failure = (String, (bool, i64, i64), (bool, i64, i64));

fn report_from_validator_result(result: Result<Report, ValidatorError>) -> Report {
    match result {
        Ok(report) => report,
        Err(ValidatorError::InvalidInput(message)) => Report {
            errors: 1,
            warnings: 0,
            messages: vec![Message {
                severity: Severity::Error,
                message,
            }],
        },
    }
}

fn run<I, WOut, WErr>(
    mut args: I,
    stdout: &mut WOut,
    stderr: &mut WErr,
) -> Result<(), Box<dyn std::error::Error>>
where
    I: Iterator<Item = String>,
    WOut: Write,
    WErr: Write,
{
    let cmd = args.next();

    match cmd.as_deref().unwrap_or("help") {
        "file" => {
            let path = args.next().ok_or("usage: css_inspector_cli file <path>")?;
            let mut config = Config::default();
            let mut use_fetcher = false;
            let mut allow_network = false;
            while let Some(arg) = args.next() {
                match arg.as_str() {
                    "--profile" => {
                        config.profile = Some(args.next().ok_or("missing value for --profile")?)
                    }
                    "--medium" => {
                        config.medium = Some(args.next().ok_or("missing value for --medium")?)
                    }
                    "--warning" => {
                        config.warning = Some(args.next().ok_or("missing value for --warning")?)
                    }
                    "--with-imports" => use_fetcher = true,
                    "--allow-network" => allow_network = true,
                    other => return Err(format!("unknown arg: {other}").into()),
                }
            }
            let css = std::fs::read_to_string(&path)?;
            let report = if use_fetcher {
                let fetcher = StdFetcher {
                    allow_network,
                    ..StdFetcher::default()
                };
                css_inspector::validate_css_text_with_fetcher(&css, None, &config, &fetcher)?
            } else {
                css_inspector::validate_css_text(&css, &config)?
            };
            writeln!(stdout, "{}", serde_json::to_string_pretty(&report)?)?;
            Ok(())
        }
        "autotest" => {
            let mut manifest: Option<PathBuf> = None;
            let mut strict = false;
            let mut allow_network = false;
            let mut max_failures: Option<usize> = None;
            let mut id_contains: Option<String> = None;
            let mut print_failures: usize = 20;
            let mut expected_filter: Option<bool> = None;
            while let Some(arg) = args.next() {
                match arg.as_str() {
                    "--manifest" => {
                        manifest = Some(PathBuf::from(
                            args.next().ok_or("missing value for --manifest")?,
                        ))
                    }
                    "--strict" => strict = true,
                    "--allow-network" => allow_network = true,
                    "--max-failures" => {
                        max_failures = Some(
                            args.next()
                                .ok_or("missing value for --max-failures")?
                                .parse()?,
                        )
                    }
                    "--id-contains" => {
                        id_contains = Some(args.next().ok_or("missing value for --id-contains")?)
                    }
                    "--print-failures" => {
                        print_failures = args
                            .next()
                            .ok_or("missing value for --print-failures")?
                            .parse()?
                    }
                    "--expected" => {
                        let v = args.next().ok_or("missing value for --expected")?;
                        expected_filter = match v.as_str() {
                            "valid" => Some(true),
                            "invalid" => Some(false),
                            "any" => None,
                            other => {
                                return Err(format!("unknown --expected value: {other}").into());
                            }
                        };
                    }
                    other => return Err(format!("unknown arg: {other}").into()),
                }
            }
            let manifest = manifest.ok_or(
                "usage: css_inspector_cli autotest --manifest <path> [--strict] [--allow-network] [--max-failures N] [--id-contains STR] [--print-failures N] [--expected valid|invalid|any]",
            )?;
            let cases = css_inspector_suite::load_manifest(&manifest)?;

            let mut total = 0usize;
            let mut passed = 0usize;
            let mut failed = 0usize;
            let mut skipped = 0usize;
            let mut failures: Vec<Failure> = Vec::new();

            let fetcher = StdFetcher {
                allow_network,
                ..StdFetcher::default()
            };

            for c in cases.iter().filter(|c| c.status == "ok") {
                if let Some(needle) = &id_contains {
                    if !c.id.contains(needle) {
                        continue;
                    }
                }
                let expected = c.expected.as_ref().expect("missing expected");
                if let Some(want_valid) = expected_filter {
                    if expected.valid != want_valid {
                        continue;
                    }
                }
                let config = Config {
                    profile: c.config.profile.clone(),
                    medium: c.config.medium.clone(),
                    warning: c.config.warning.clone(),
                };

                let report = report_from_validator_result(if c.input.kind == "text" {
                    let css = c.input.text.as_deref().unwrap_or("");
                    css_inspector::validate_css_text(css, &config)
                } else {
                    let uri = c.input.uri.as_deref().unwrap_or("");
                    if starts_with_ascii_ci(uri, "file://") || allow_network {
                        css_inspector::validate_css_uri_with_fetcher(uri, &config, &fetcher)
                    } else {
                        skipped += 1;
                        continue;
                    }
                });
                total += 1;

                let ok = report.valid() == expected.valid
                    && report.errors as i64 == expected.errors
                    && report.warnings as i64 == expected.warnings;
                if ok {
                    passed += 1;
                } else {
                    failed += 1;
                    failures.push((
                        c.id.clone(),
                        (expected.valid, expected.errors, expected.warnings),
                        (report.valid(), report.errors as i64, report.warnings as i64),
                    ));
                    if let Some(limit) = max_failures {
                        if failures.len() >= limit {
                            break;
                        }
                    }
                }
            }

            writeln!(
                stderr,
                "autotest summary: total={total} passed={passed} failed={failed} skipped={skipped}"
            )?;
            if !failures.is_empty() {
                writeln!(stderr, "failures (showing up to {print_failures}):")?;
                for (id, expected, actual) in failures.iter().take(print_failures) {
                    writeln!(
                        stderr,
                        "  {id}: expected valid={} errors={} warnings={} got valid={} errors={} warnings={}",
                        expected.0, expected.1, expected.2, actual.0, actual.1, actual.2
                    )?;
                }
            }
            if strict && failed != 0 {
                return Err(format!("autotest suite had {failed} failing case(s)").into());
            }
            Ok(())
        }
        _ => {
            writeln!(stderr, "usage:")?;
            writeln!(
                stderr,
                "  css_inspector_cli file <path> [--profile NAME] [--medium NAME] [--warning LEVEL] [--with-imports] [--allow-network]"
            )?;
            writeln!(
                stderr,
                "  css_inspector_cli autotest --manifest <path> [--strict] [--allow-network] [--max-failures N] [--id-contains STR] [--print-failures N] [--expected valid|invalid|any]"
            )?;
            Ok(())
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = std::io::stdout();
    let mut stderr = std::io::stderr();
    run(env::args().skip(1), &mut stdout, &mut stderr)
}

#[cfg(test)]
mod tests {
    use super::{report_from_validator_result, run, starts_with_ascii_ci};
    use css_inspector::{Message, Report, Severity, ValidatorError};
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn run_cli(args: Vec<String>) -> (Result<(), String>, String, String) {
        let mut out = Vec::new();
        let mut err = Vec::new();
        let result = run(args.into_iter(), &mut out, &mut err).map_err(|e| e.to_string());
        (
            result,
            String::from_utf8(out).expect("stdout utf8"),
            String::from_utf8(err).expect("stderr utf8"),
        )
    }

    fn unique_tmp_path(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_nanos();
        std::env::temp_dir().join(format!("ae-css-cli-{name}-{nanos}"))
    }

    #[test]
    fn empty_prefix_matches() {
        assert!(starts_with_ascii_ci("abc", ""));
    }

    #[test]
    fn longer_prefix_does_not_match() {
        assert!(!starts_with_ascii_ci("ab", "abc"));
    }

    #[test]
    fn matches_case_insensitively() {
        assert!(starts_with_ascii_ci("File://x", "file://"));
    }

    #[test]
    fn non_ascii_prefix_never_panics_or_matches_ascii_by_accident() {
        assert!(!starts_with_ascii_ci("❤", "h"));
        assert!(!starts_with_ascii_ci("❤", "❤H"));
    }

    #[test]
    fn report_from_validator_result_converts_invalid_input_to_single_error() {
        let report = report_from_validator_result(Err(ValidatorError::InvalidInput("x".into())));
        assert_eq!(
            report,
            Report {
                errors: 1,
                warnings: 0,
                messages: vec![Message {
                    severity: Severity::Error,
                    message: "x".into(),
                }],
            }
        );
    }

    #[test]
    fn report_from_validator_result_passes_through_ok_report() {
        let report = Report {
            errors: 0,
            warnings: 0,
            messages: vec![Message {
                severity: Severity::Warning,
                message: "w".into(),
            }],
        };
        assert_eq!(report_from_validator_result(Ok(report.clone())), report);
    }

    #[test]
    fn help_is_printed_for_unknown_or_missing_command() {
        let (result, stdout, stderr) = run_cli(Vec::new());
        assert!(result.is_ok());
        assert!(stdout.is_empty());
        assert!(stderr.contains("css_inspector_cli file <path>"));

        let (result, stdout, stderr) = run_cli(vec!["nope".into()]);
        assert!(result.is_ok());
        assert!(stdout.is_empty());
        assert!(stderr.contains("css_inspector_cli autotest --manifest"));
    }

    #[test]
    fn file_command_outputs_json_report() {
        let path = unique_tmp_path("file.css");
        fs::write(&path, "a { color: red; }").expect("write css");
        let path_arg = path.to_string_lossy().into_owned();

        let (result, stdout, stderr) = run_cli(vec!["file".into(), path_arg]);
        assert!(result.is_ok());
        assert!(stderr.is_empty());

        let value: serde_json::Value = serde_json::from_str(stdout.trim()).expect("valid json");
        assert_eq!(value["errors"].as_u64(), Some(0));

        let _ = fs::remove_file(path);
    }

    #[test]
    fn file_command_with_imports_uses_fetcher() {
        let imported_path = unique_tmp_path("imported.css");
        fs::write(&imported_path, "a { color: red; }").expect("write imported css");
        let imported_uri = format!("file://{}", imported_path.display());

        let path = unique_tmp_path("with-imports.css");
        fs::write(
            &path,
            format!("@import \"{imported_uri}\";\na{{color:blue;}}"),
        )
        .expect("write css");
        let path_arg = path.to_string_lossy().into_owned();

        let (result, stdout, stderr) = run_cli(vec![
            "file".into(),
            path_arg,
            "--with-imports".into(),
            "--allow-network".into(),
        ]);
        assert!(result.is_ok());
        assert!(stderr.is_empty());

        let value: serde_json::Value = serde_json::from_str(stdout.trim()).expect("valid json");
        assert_eq!(value["errors"].as_u64(), Some(0));

        let _ = fs::remove_file(path);
        let _ = fs::remove_file(imported_path);
    }

    #[test]
    fn file_command_rejects_unknown_args() {
        let path = unique_tmp_path("unknown-arg.css");
        fs::write(&path, "a { color: red; }").expect("write css");
        let path_arg = path.to_string_lossy().into_owned();

        let (result, stdout, _stderr) =
            run_cli(vec!["file".into(), path_arg, "--nope".into()]);
        assert!(result.is_err());
        assert!(stdout.is_empty());
        assert!(result.unwrap_err().contains("unknown arg: --nope"));

        let _ = fs::remove_file(path);
    }

    #[test]
    fn autotest_command_reports_failures_and_skips_network_uris() {
        let css_path = unique_tmp_path("autotest.css");
        fs::write(&css_path, "a { color: red; }").expect("write css");
        let css_uri = format!("file://{}", css_path.display());

        let manifest_path = unique_tmp_path("manifest.jsonl");
        let case_ok = serde_json::json!({
            "id": "t1-ok",
            "source": { "file": "t", "index": 0, "section": null },
            "validator_url": "http://example.com",
            "input": { "kind": "text", "text": "a { color: red; }", "uri": null },
            "config": { "profile": null, "medium": null, "warning": null },
            "expected": { "valid": true, "errors": 0, "warnings": 0 },
            "observed": null,
            "status": "ok",
            "note": null
        });
        let case_fail = serde_json::json!({
            "id": "t2-fail",
            "source": { "file": "t", "index": 1, "section": null },
            "validator_url": "http://example.com",
            "input": { "kind": "text", "text": "a { color: red; }", "uri": null },
            "config": { "profile": null, "medium": null, "warning": null },
            "expected": { "valid": false, "errors": 1, "warnings": 0 },
            "observed": null,
            "status": "ok",
            "note": null
        });
        let case_file_uri = serde_json::json!({
            "id": "t3-uri",
            "source": { "file": "t", "index": 2, "section": null },
            "validator_url": "http://example.com",
            "input": { "kind": "uri", "text": null, "uri": css_uri },
            "config": { "profile": null, "medium": null, "warning": null },
            "expected": { "valid": true, "errors": 0, "warnings": 0 },
            "observed": null,
            "status": "ok",
            "note": null
        });
        let case_network_uri = serde_json::json!({
            "id": "t4-skip",
            "source": { "file": "t", "index": 3, "section": null },
            "validator_url": "http://example.com",
            "input": { "kind": "uri", "text": null, "uri": "http://example.com/a.css" },
            "config": { "profile": null, "medium": null, "warning": null },
            "expected": { "valid": true, "errors": 0, "warnings": 0 },
            "observed": null,
            "status": "ok",
            "note": null
        });
        fs::write(
            &manifest_path,
            format!(
                "{}\n{}\n{}\n{}\n",
                serde_json::to_string(&case_ok).unwrap(),
                serde_json::to_string(&case_fail).unwrap(),
                serde_json::to_string(&case_file_uri).unwrap(),
                serde_json::to_string(&case_network_uri).unwrap(),
            ),
        )
        .expect("write manifest");
        let manifest_arg = manifest_path.to_string_lossy().into_owned();

        let (result, stdout, stderr) =
            run_cli(vec!["autotest".into(), "--manifest".into(), manifest_arg]);
        assert!(result.is_ok());
        assert!(stdout.is_empty());
        assert!(stderr.contains("autotest summary: total=3 passed=2 failed=1 skipped=1"));
        assert!(stderr.contains("t2-fail"));

        let _ = fs::remove_file(manifest_path);
        let _ = fs::remove_file(css_path);
    }

    #[test]
    fn autotest_strict_mode_errors_on_failure_and_honors_max_failures() {
        let manifest_path = unique_tmp_path("manifest-strict.jsonl");
        let case_ok = serde_json::json!({
            "id": "t1-ok",
            "source": { "file": "t", "index": 0, "section": null },
            "validator_url": "http://example.com",
            "input": { "kind": "text", "text": "a { color: red; }", "uri": null },
            "config": { "profile": null, "medium": null, "warning": null },
            "expected": { "valid": true, "errors": 0, "warnings": 0 },
            "observed": null,
            "status": "ok",
            "note": null
        });
        let case_fail = serde_json::json!({
            "id": "t2-fail",
            "source": { "file": "t", "index": 1, "section": null },
            "validator_url": "http://example.com",
            "input": { "kind": "text", "text": "a { color: red; }", "uri": null },
            "config": { "profile": null, "medium": null, "warning": null },
            "expected": { "valid": false, "errors": 1, "warnings": 0 },
            "observed": null,
            "status": "ok",
            "note": null
        });
        fs::write(
            &manifest_path,
            format!(
                "{}\n{}\n",
                serde_json::to_string(&case_ok).unwrap(),
                serde_json::to_string(&case_fail).unwrap(),
            ),
        )
        .expect("write manifest");
        let manifest_arg = manifest_path.to_string_lossy().into_owned();

        let (result, stdout, stderr) = run_cli(vec![
            "autotest".into(),
            "--manifest".into(),
            manifest_arg,
            "--strict".into(),
            "--max-failures".into(),
            "1".into(),
        ]);
        assert!(result.is_err());
        assert!(stdout.is_empty());
        assert!(stderr.contains("autotest summary: total=2 passed=1 failed=1 skipped=0"));
        assert!(result.unwrap_err().contains("autotest suite had 1 failing case(s)"));

        let _ = fs::remove_file(manifest_path);
    }

    #[test]
    fn autotest_expected_filter_rejects_unknown_values() {
        let manifest_path = unique_tmp_path("manifest-bad.jsonl");
        fs::write(&manifest_path, "").expect("write manifest");
        let manifest_arg = manifest_path.to_string_lossy().into_owned();

        let (result, _stdout, _stderr) = run_cli(vec![
            "autotest".into(),
            "--manifest".into(),
            manifest_arg,
            "--expected".into(),
            "nope".into(),
        ]);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("unknown --expected value: nope"));

        let _ = fs::remove_file(manifest_path);
    }
}
