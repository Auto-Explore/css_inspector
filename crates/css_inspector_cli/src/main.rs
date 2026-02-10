use std::env;
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
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
            println!("{}", serde_json::to_string_pretty(&report)?);
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

            eprintln!(
                "autotest summary: total={total} passed={passed} failed={failed} skipped={skipped}"
            );
            if !failures.is_empty() {
                eprintln!("failures (showing up to {print_failures}):");
                for (id, expected, actual) in failures.iter().take(print_failures) {
                    eprintln!(
                        "  {id}: expected valid={} errors={} warnings={} got valid={} errors={} warnings={}",
                        expected.0, expected.1, expected.2, actual.0, actual.1, actual.2
                    );
                }
            }
            if strict && failed != 0 {
                return Err(format!("autotest suite had {failed} failing case(s)").into());
            }
            Ok(())
        }
        _ => {
            eprintln!("usage:");
            eprintln!(
                "  css_inspector_cli file <path> [--profile NAME] [--medium NAME] [--warning LEVEL] [--with-imports] [--allow-network]"
            );
            eprintln!(
                "  css_inspector_cli autotest --manifest <path> [--strict] [--allow-network] [--max-failures N] [--id-contains STR] [--print-failures N] [--expected valid|invalid|any]"
            );
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{report_from_validator_result, starts_with_ascii_ci};
    use css_inspector::{Message, Report, Severity, ValidatorError};

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
}
