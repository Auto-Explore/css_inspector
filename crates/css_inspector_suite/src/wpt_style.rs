use std::fs;
use std::io::Write;
use std::path::{Component, Path, PathBuf};
use std::process::Command;
use std::{cmp, io};

use css_inspector::{Config as ValidatorConfig, Message, Report, Severity, ValidatorError};
use rustc_hash::FxHashSet;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{Config as SuiteConfig, SuiteError};

pub const WPT_CSS_STYLE_RESULTS_FORMAT_VERSION: u32 = 3;
pub const WPT_CSS_STYLE_RESULTS_META_FILE: &str = "_meta.md";

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WptCssStyleResultsTotals {
    pub files_with_style_blocks: usize,
    pub style_blocks: usize,
    pub errors: usize,
    pub warnings: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WptCssStyleResultsMeta {
    pub format_version: u32,
    pub wpt_commit: String,
    pub config: SuiteConfig,
    #[serde(default)]
    pub totals: WptCssStyleResultsTotals,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WptCssStyleBlockResult {
    pub index: usize,
    pub css: String,
    pub report: Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WptCssStyleFileResults {
    pub format_version: u32,
    pub file: String,
    pub styles: Vec<WptCssStyleBlockResult>,
}

#[derive(Clone, Debug, Default)]
pub struct WptCssStyleCheckOptions {
    pub id_contains: Option<String>,
    pub max_failures: Option<usize>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WptCssStyleCheckSummary {
    pub files_total: usize,
    pub files_matched: usize,
    pub files_failed: usize,
    pub blocks_total: usize,
    pub blocks_matched: usize,
    pub blocks_failed: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WptCssStyleFailureKind {
    MissingResultsFile,
    UnexpectedResultsFile,
    InvalidResultsFile {
        message: String,
    },
    StyleBlockCountMismatch {
        wpt: usize,
        stored: usize,
    },
    StyleTextMismatch {
        expected_len: usize,
        actual_len: usize,
    },
    ReportMismatch {
        expected: Value,
        actual: Value,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct WptCssStyleFailure {
    pub id: String,
    pub file: String,
    pub index: Option<usize>,
    pub kind: WptCssStyleFailureKind,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WptCssStyleWriteSummary {
    pub files_written: usize,
    pub blocks_written: usize,
}

pub fn git_head_commit(dir: &Path) -> Result<String, SuiteError> {
    let output = Command::new("git")
        .arg("-C")
        .arg(dir)
        .arg("rev-parse")
        .arg("HEAD")
        .output()?;
    if !output.status.success() {
        return Err(SuiteError::InvalidManifest(format!(
            "git rev-parse HEAD failed for {}: {}",
            dir.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn is_xmlish_markup_rel(path: &str) -> bool {
    let Some((_, ext)) = path.rsplit_once('.') else {
        return false;
    };
    matches!(
        ext.to_ascii_lowercase().as_str(),
        "xhtml" | "xht" | "xml" | "svg"
    )
}

fn strip_xml_comments_if_applicable(css: &str) -> String {
    // In XML, comment nodes are not part of a style element’s text content, but our extractor
    // slices raw bytes from the markup. Strip `<!-- ... -->` blocks for XML-ish documents *unless*
    // the stylesheet appears to be wrapped in a CDATA section, where `<!--` is just literal text.
    if find_ascii_ci(css.as_bytes(), b"<![cdata[", 0).is_some() {
        return css.to_string();
    }

    let mut out = String::new();
    let mut i = 0usize;
    let bytes = css.as_bytes();
    while let Some(start) = find_ascii_ci(bytes, b"<!--", i) {
        out.push_str(&css[i..start]);
        let Some(end) = find_ascii_ci(bytes, b"-->", start + 4) else {
            // Unterminated comment: leave the rest as-is.
            out.push_str(&css[start..]);
            return out;
        };
        i = end + 3;
    }
    out.push_str(&css[i..]);
    out
}

pub fn wpt_css_style_results_meta_path(results_root: &Path) -> PathBuf {
    results_root.join(WPT_CSS_STYLE_RESULTS_META_FILE)
}

pub fn wpt_css_style_results_file_path(
    results_root: &Path,
    wpt_file: &str,
) -> Result<PathBuf, SuiteError> {
    let rel = safe_rel_path_from_slash(wpt_file)?;
    let mut out = results_root.to_path_buf();
    out.push(rel);

    let Some(file_name) = out.file_name().map(|s| s.to_string_lossy()) else {
        return Err(SuiteError::InvalidManifest(format!(
            "invalid wpt file path: {wpt_file}"
        )));
    };
    out.set_file_name(format!("{file_name}.md"));
    Ok(out)
}

pub fn load_wpt_css_style_results_meta(
    results_root: &Path,
) -> Result<WptCssStyleResultsMeta, SuiteError> {
    let path = wpt_css_style_results_meta_path(results_root);
    let s = fs::read_to_string(&path)?;
    parse_results_meta_markdown(&path, &s)
}

pub fn write_wpt_css_style_results_meta_atomic(
    results_root: &Path,
    meta: &WptCssStyleResultsMeta,
) -> Result<(), SuiteError> {
    let path = wpt_css_style_results_meta_path(results_root);
    let md = render_results_meta_markdown(meta)?;
    write_markdown_atomic(&path, &md)
}

pub fn write_wpt_css_style_file_results_atomic(
    path: &Path,
    results: &WptCssStyleFileResults,
) -> Result<(), SuiteError> {
    let md = render_file_results_markdown(results)?;
    write_markdown_atomic(path, &md)
}

pub fn write_wpt_css_style_results_tree(
    wpt_root: &Path,
    wpt_commit: &str,
    results_root: &Path,
    config: &ValidatorConfig,
) -> Result<WptCssStyleWriteSummary, SuiteError> {
    let css_root = wpt_root.join("css");
    if !css_root.is_dir() {
        return Err(SuiteError::InvalidManifest(format!(
            "WPT css root not found: {}",
            css_root.display()
        )));
    }

    // Track existing result files so we can remove stale ones after writing.
    let mut existing_results_files: FxHashSet<PathBuf> = FxHashSet::default();
    if results_root.is_dir() {
        let mut md_files: Vec<PathBuf> = Vec::new();
        collect_markdown_files_rec(results_root, &mut md_files)?;
        existing_results_files =
            FxHashSet::with_capacity_and_hasher(md_files.len(), Default::default());
        existing_results_files.extend(md_files);
    }
    let meta_path = wpt_css_style_results_meta_path(results_root);

    let mut files: Vec<(String, PathBuf)> = Vec::new();
    collect_markup_files_rec(&css_root, &mut files, wpt_root)?;
    files.sort_by(|a, b| a.0.cmp(&b.0));

    let mut totals = WptCssStyleResultsTotals::default();

    let mut written_results_files: FxHashSet<PathBuf> =
        FxHashSet::with_capacity_and_hasher(files.len(), Default::default());
    let mut summary = WptCssStyleWriteSummary::default();

    for (rel, path) in files {
        let bytes = fs::read(&path)?;
        let doc = String::from_utf8_lossy(&bytes);
        let mut blocks = extract_style_blocks(&doc);
        if is_xmlish_markup_rel(&rel) {
            for b in &mut blocks {
                *b = strip_xml_comments_if_applicable(b);
            }
        }
        for b in &mut blocks {
            ensure_trailing_newline(b);
        }
        if blocks.is_empty() {
            continue;
        }

        totals.files_with_style_blocks += 1;
        totals.style_blocks += blocks.len();

        let mut styles: Vec<WptCssStyleBlockResult> = Vec::with_capacity(blocks.len());
        for (idx, mut css) in blocks.into_iter().enumerate() {
            ensure_trailing_newline(&mut css);
            let report =
                report_from_validator_result(css_inspector::validate_css_text(&css, config));
            totals.errors += report.errors;
            totals.warnings += report.warnings;
            styles.push(WptCssStyleBlockResult {
                index: idx,
                css,
                report: serde_json::to_value(&report)?,
            });
        }

        let results = WptCssStyleFileResults {
            format_version: WPT_CSS_STYLE_RESULTS_FORMAT_VERSION,
            file: rel.clone(),
            styles,
        };
        let out_path = wpt_css_style_results_file_path(results_root, &rel)?;
        write_wpt_css_style_file_results_atomic(&out_path, &results)?;
        written_results_files.insert(out_path);

        summary.files_written += 1;
        summary.blocks_written += results.styles.len();
    }

    for path in existing_results_files {
        if path == meta_path || written_results_files.contains(&path) {
            continue;
        }
        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            fs::remove_file(&path)?;
        }
    }

    let meta = WptCssStyleResultsMeta {
        format_version: WPT_CSS_STYLE_RESULTS_FORMAT_VERSION,
        wpt_commit: wpt_commit.to_string(),
        config: suite_config_from_validator_config(config),
        totals,
    };
    write_wpt_css_style_results_meta_atomic(results_root, &meta)?;

    Ok(summary)
}

pub fn check_wpt_css_style_results_tree(
    wpt_root: &Path,
    wpt_commit: &str,
    results_root: &Path,
    config: &ValidatorConfig,
    options: WptCssStyleCheckOptions,
) -> Result<(WptCssStyleCheckSummary, Vec<WptCssStyleFailure>), SuiteError> {
    let meta = load_wpt_css_style_results_meta(results_root)?;
    if meta.format_version != WPT_CSS_STYLE_RESULTS_FORMAT_VERSION {
        return Err(SuiteError::InvalidManifest(format!(
            "unsupported wpt css style results format_version={}",
            meta.format_version
        )));
    }
    if meta.wpt_commit != wpt_commit {
        return Err(SuiteError::InvalidManifest(format!(
            "wpt commit mismatch: results meta has {}, but fixtures/wpt is {}",
            meta.wpt_commit, wpt_commit
        )));
    }
    let suite_config = suite_config_from_validator_config(config);
    if meta.config != suite_config {
        return Err(SuiteError::InvalidManifest(format!(
            "validator config mismatch: results meta has {:?}, but current run is {:?}",
            meta.config, suite_config
        )));
    }

    let css_root = wpt_root.join("css");
    if !css_root.is_dir() {
        return Err(SuiteError::InvalidManifest(format!(
            "WPT css root not found: {}",
            css_root.display()
        )));
    }

    let mut files: Vec<(String, PathBuf)> = Vec::new();
    collect_markup_files_rec(&css_root, &mut files, wpt_root)?;
    files.sort_by(|a, b| a.0.cmp(&b.0));

    let max_failures = options
        .max_failures
        .filter(|&n| n != 0)
        .unwrap_or(usize::MAX);
    let mut expected_results: FxHashSet<PathBuf> = if options.id_contains.is_none() {
        FxHashSet::with_capacity_and_hasher(files.len(), Default::default())
    } else {
        FxHashSet::default()
    };
    let mut summary = WptCssStyleCheckSummary::default();
    let mut failures: Vec<WptCssStyleFailure> = Vec::new();

    for (rel, path) in files {
        if failures.len() >= max_failures {
            break;
        }

        let bytes = fs::read(&path)?;
        let doc = String::from_utf8_lossy(&bytes);
        let mut blocks = extract_style_blocks(&doc);
        if is_xmlish_markup_rel(&rel) {
            for b in &mut blocks {
                *b = strip_xml_comments_if_applicable(b);
            }
        }
        for b in &mut blocks {
            ensure_trailing_newline(b);
        }
        if blocks.is_empty() {
            continue;
        }

        let mut matching_indexes: Vec<usize> = Vec::with_capacity(blocks.len());
        for idx in 0..blocks.len() {
            let id = wpt_style_id(&rel, idx);
            if options
                .id_contains
                .as_deref()
                .is_none_or(|needle| id.contains(needle))
            {
                matching_indexes.push(idx);
            }
        }
        if matching_indexes.is_empty() {
            continue;
        }

        summary.files_total += 1;

        let results_path = wpt_css_style_results_file_path(results_root, &rel)?;
        if options.id_contains.is_none() {
            expected_results.insert(results_path.clone());
        }

        let mut file_failed = false;
        let mut file_blocks_checked = 0usize;
        let mut file_blocks_failed = 0usize;

        if !results_path.exists() {
            file_failed = true;
            file_blocks_checked = matching_indexes.len();
            file_blocks_failed = matching_indexes.len();
            push_failure(
                &mut failures,
                max_failures,
                WptCssStyleFailure {
                    id: rel.clone(),
                    file: rel.clone(),
                    index: None,
                    kind: WptCssStyleFailureKind::MissingResultsFile,
                },
            );
        } else {
            let stored = match load_wpt_css_style_file_results(&results_path) {
                Ok(v) => v,
                Err(e) => {
                    file_failed = true;
                    file_blocks_checked = matching_indexes.len();
                    file_blocks_failed = matching_indexes.len();
                    push_failure(
                        &mut failures,
                        max_failures,
                        WptCssStyleFailure {
                            id: rel.clone(),
                            file: rel.clone(),
                            index: None,
                            kind: WptCssStyleFailureKind::InvalidResultsFile {
                                message: e.to_string(),
                            },
                        },
                    );
                    finalize_file_summary(
                        &mut summary,
                        file_failed,
                        file_blocks_checked,
                        file_blocks_failed,
                    );
                    continue;
                }
            };

            if stored.format_version != WPT_CSS_STYLE_RESULTS_FORMAT_VERSION {
                file_failed = true;
                file_blocks_checked = matching_indexes.len();
                file_blocks_failed = matching_indexes.len();
                push_failure(
                    &mut failures,
                    max_failures,
                    WptCssStyleFailure {
                        id: rel.clone(),
                        file: rel.clone(),
                        index: None,
                        kind: WptCssStyleFailureKind::InvalidResultsFile {
                            message: format!(
                                "unsupported format_version={}",
                                stored.format_version
                            ),
                        },
                    },
                );
            } else if stored.file != rel {
                file_failed = true;
                file_blocks_checked = matching_indexes.len();
                file_blocks_failed = matching_indexes.len();
                push_failure(
                    &mut failures,
                    max_failures,
                    WptCssStyleFailure {
                        id: rel.clone(),
                        file: rel.clone(),
                        index: None,
                        kind: WptCssStyleFailureKind::InvalidResultsFile {
                            message: format!("stored file mismatch: {}", stored.file),
                        },
                    },
                );
            } else if stored.styles.len() != blocks.len() {
                file_failed = true;
                file_blocks_checked = matching_indexes.len();
                file_blocks_failed = matching_indexes.len();
                push_failure(
                    &mut failures,
                    max_failures,
                    WptCssStyleFailure {
                        id: rel.clone(),
                        file: rel.clone(),
                        index: None,
                        kind: WptCssStyleFailureKind::StyleBlockCountMismatch {
                            wpt: blocks.len(),
                            stored: stored.styles.len(),
                        },
                    },
                );
            } else {
                for idx in matching_indexes {
                    if failures.len() >= max_failures {
                        break;
                    }
                    file_blocks_checked += 1;

                    let expected = stored
                        .styles
                        .get(idx)
                        .expect("checked stored.styles.len() == blocks.len()");
                    if expected.index != idx {
                        file_failed = true;
                        file_blocks_failed += 1;
                        push_failure(
                            &mut failures,
                            max_failures,
                            WptCssStyleFailure {
                                id: wpt_style_id(&rel, idx),
                                file: rel.clone(),
                                index: Some(idx),
                                kind: WptCssStyleFailureKind::InvalidResultsFile {
                                    message: format!(
                                        "stored style index mismatch: {}",
                                        expected.index
                                    ),
                                },
                            },
                        );
                        continue;
                    }

                    let actual_css = &blocks[idx];
                    if expected.css != *actual_css {
                        file_failed = true;
                        file_blocks_failed += 1;
                        push_failure(
                            &mut failures,
                            max_failures,
                            WptCssStyleFailure {
                                id: wpt_style_id(&rel, idx),
                                file: rel.clone(),
                                index: Some(idx),
                                kind: WptCssStyleFailureKind::StyleTextMismatch {
                                    expected_len: expected.css.len(),
                                    actual_len: actual_css.len(),
                                },
                            },
                        );
                        continue;
                    }

                    let report = report_from_validator_result(css_inspector::validate_css_text(
                        actual_css, config,
                    ));
                    let actual_report = serde_json::to_value(&report)?;
                    if actual_report != expected.report {
                        file_failed = true;
                        file_blocks_failed += 1;
                        push_failure(
                            &mut failures,
                            max_failures,
                            WptCssStyleFailure {
                                id: wpt_style_id(&rel, idx),
                                file: rel.clone(),
                                index: Some(idx),
                                kind: WptCssStyleFailureKind::ReportMismatch {
                                    expected: expected.report.clone(),
                                    actual: actual_report,
                                },
                            },
                        );
                        continue;
                    }
                }
            }
        }

        finalize_file_summary(
            &mut summary,
            file_failed,
            file_blocks_checked,
            file_blocks_failed,
        );
    }

    if options.id_contains.is_none() && results_root.is_dir() && failures.len() < max_failures {
        let mut result_files: Vec<PathBuf> = Vec::new();
        collect_markdown_files_rec(results_root, &mut result_files)?;
        result_files.sort_by(|a, b| cmp::Ord::cmp(&a.to_string_lossy(), &b.to_string_lossy()));

        for path in result_files {
            if failures.len() >= max_failures {
                break;
            }
            if path.file_name().and_then(|s| s.to_str()) == Some(WPT_CSS_STYLE_RESULTS_META_FILE) {
                continue;
            }
            if !expected_results.contains(&path) {
                let rel = path
                    .strip_prefix(results_root)
                    .unwrap_or(&path)
                    .to_path_buf();
                let rel_str = path_to_slash(&rel);
                push_failure(
                    &mut failures,
                    max_failures,
                    WptCssStyleFailure {
                        id: rel_str.clone(),
                        file: rel_str,
                        index: None,
                        kind: WptCssStyleFailureKind::UnexpectedResultsFile,
                    },
                );
            }
        }
    }

    Ok((summary, failures))
}

fn finalize_file_summary(
    summary: &mut WptCssStyleCheckSummary,
    file_failed: bool,
    blocks_checked: usize,
    blocks_failed: usize,
) {
    summary.blocks_total += blocks_checked;
    summary.blocks_failed += blocks_failed;
    summary.blocks_matched += blocks_checked.saturating_sub(blocks_failed);

    if file_failed || blocks_failed != 0 {
        summary.files_failed += 1;
    } else {
        summary.files_matched += 1;
    }
}

fn push_failure(out: &mut Vec<WptCssStyleFailure>, max: usize, f: WptCssStyleFailure) {
    if out.len() < max {
        out.push(f);
    }
}

fn wpt_style_id(file: &str, index: usize) -> String {
    format!("{file}#style[{index}]")
}

fn ensure_trailing_newline(s: &mut String) {
    if !s.ends_with('\n') {
        s.push('\n');
    }
}

fn write_markdown_atomic(path: &Path, markdown: &str) -> Result<(), SuiteError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let tmp_path = tmp_path_for(path);
    let file = fs::File::create(&tmp_path)?;
    let mut writer = io::BufWriter::new(file);
    writer.write_all(markdown.as_bytes())?;
    writer.flush()?;

    if path.exists() {
        fs::remove_file(path)?;
    }
    fs::rename(&tmp_path, path)?;
    Ok(())
}

#[derive(Clone, Debug)]
struct FencedCodeBlock {
    info: String,
    content: String,
}

fn parse_markdown_fenced_code_blocks(markdown: &str) -> Result<Vec<FencedCodeBlock>, SuiteError> {
    let mut out: Vec<FencedCodeBlock> = Vec::new();
    let mut open_fence_len: Option<usize> = None;
    let mut open_info = String::new();
    let mut content = String::new();

    for line in markdown.split_inclusive('\n') {
        let trimmed = line.trim_end_matches('\n');
        if let Some(fence_len) = open_fence_len {
            if is_fence_closer(trimmed, fence_len) {
                out.push(FencedCodeBlock {
                    info: std::mem::take(&mut open_info),
                    content: std::mem::take(&mut content),
                });
                open_fence_len = None;
            } else {
                content.push_str(line);
            }
            continue;
        }

        if let Some((fence_len, info)) = parse_fence_opener(trimmed) {
            open_fence_len = Some(fence_len);
            open_info = info;
            content.clear();
        }
    }

    if open_fence_len.is_some() {
        return Err(SuiteError::InvalidManifest(
            "unclosed fenced code block".to_string(),
        ));
    }

    Ok(out)
}

fn parse_fence_opener(line: &str) -> Option<(usize, String)> {
    let bytes = line.as_bytes();
    let mut n = 0usize;
    while n < bytes.len() && bytes[n] == b'`' {
        n += 1;
    }
    if n < 3 {
        return None;
    }
    let info = line[n..].trim().to_string();
    Some((n, info))
}

fn is_fence_closer(line: &str, fence_len: usize) -> bool {
    let bytes = line.as_bytes();
    let mut n = 0usize;
    while n < bytes.len() && bytes[n] == b'`' {
        n += 1;
    }
    n == fence_len && line[n..].trim().is_empty()
}

fn render_results_meta_markdown(meta: &WptCssStyleResultsMeta) -> Result<String, SuiteError> {
    let json = serde_json::to_string_pretty(meta)?;
    let mut out = String::new();
    out.push_str("# WPT CSS <style> results\n\n");
    out.push_str("Totals:\n");
    out.push_str(&format!(
        "- files_with_style_blocks: {}\n- style_blocks: {}\n- errors: {}\n- warnings: {}\n\n",
        meta.totals.files_with_style_blocks,
        meta.totals.style_blocks,
        meta.totals.errors,
        meta.totals.warnings
    ));
    out.push_str(
        "Machine-readable metadata for `css_inspector_cli wpt-style` lives in the JSON block below.\n\n",
    );
    out.push_str(&render_fenced_code_block("json", &json));
    Ok(out)
}

fn parse_results_meta_markdown(
    path: &Path,
    markdown: &str,
) -> Result<WptCssStyleResultsMeta, SuiteError> {
    let blocks = parse_markdown_fenced_code_blocks(markdown)?;
    let Some(json_block) = blocks
        .iter()
        .find(|b| block_lang(&b.info).eq_ignore_ascii_case("json"))
    else {
        return Err(SuiteError::InvalidManifest(format!(
            "{}: missing ```json meta block",
            path.display()
        )));
    };
    serde_json::from_str(&json_block.content).map_err(SuiteError::from)
}

fn load_wpt_css_style_file_results(path: &Path) -> Result<WptCssStyleFileResults, SuiteError> {
    let s = fs::read_to_string(path)?;
    parse_file_results_markdown(path, &s)
}

fn render_file_results_markdown(results: &WptCssStyleFileResults) -> Result<String, SuiteError> {
    #[derive(Serialize)]
    struct FileHeader<'a> {
        format_version: u32,
        file: &'a str,
    }

    let header = FileHeader {
        format_version: results.format_version,
        file: &results.file,
    };
    let header_json = serde_json::to_string_pretty(&header)?;

    let mut out = String::new();
    out.push_str("# ");
    out.push_str(&results.file);
    out.push_str("\n\n");
    out.push_str(&render_fenced_code_block("json", &header_json));

    for style in &results.styles {
        out.push_str("\n## style[");
        out.push_str(&style.index.to_string());
        out.push_str("]\n\n");
        out.push_str(&render_fenced_code_block("css", &style.css));
        out.push('\n');
        let report_json = serde_json::to_string_pretty(&style.report)?;
        out.push_str(&render_fenced_code_block("json", &report_json));
    }

    Ok(out)
}

fn parse_file_results_markdown(
    path: &Path,
    markdown: &str,
) -> Result<WptCssStyleFileResults, SuiteError> {
    #[derive(Deserialize)]
    struct FileHeader {
        format_version: u32,
        file: String,
    }

    let blocks = parse_markdown_fenced_code_blocks(markdown)?;
    if blocks.is_empty() {
        return Err(SuiteError::InvalidManifest(format!(
            "{}: empty results file",
            path.display()
        )));
    }

    if !block_lang(&blocks[0].info).eq_ignore_ascii_case("json") {
        return Err(SuiteError::InvalidManifest(format!(
            "{}: first fenced block must be ```json file header",
            path.display()
        )));
    }
    let header: FileHeader = serde_json::from_str(&blocks[0].content)?;

    let mut styles: Vec<WptCssStyleBlockResult> =
        Vec::with_capacity(blocks.len().saturating_sub(1) / 2);
    let mut i = 1usize;
    while i < blocks.len() {
        let css_block = blocks.get(i).ok_or_else(|| {
            SuiteError::InvalidManifest(format!(
                "{}: missing ```css block for style[{}]",
                path.display(),
                styles.len()
            ))
        })?;
        if !block_lang(&css_block.info).eq_ignore_ascii_case("css") {
            return Err(SuiteError::InvalidManifest(format!(
                "{}: expected ```css block for style[{}], got ```{}",
                path.display(),
                styles.len(),
                block_lang(&css_block.info)
            )));
        }
        i += 1;

        let report_block = blocks.get(i).ok_or_else(|| {
            SuiteError::InvalidManifest(format!(
                "{}: missing ```json report block for style[{}]",
                path.display(),
                styles.len()
            ))
        })?;
        if !block_lang(&report_block.info).eq_ignore_ascii_case("json") {
            return Err(SuiteError::InvalidManifest(format!(
                "{}: expected ```json report block for style[{}], got ```{}",
                path.display(),
                styles.len(),
                block_lang(&report_block.info)
            )));
        }
        i += 1;

        let mut css = css_block.content.clone();
        ensure_trailing_newline(&mut css);
        let report: Value = serde_json::from_str(&report_block.content)?;

        styles.push(WptCssStyleBlockResult {
            index: styles.len(),
            css,
            report,
        });
    }

    Ok(WptCssStyleFileResults {
        format_version: header.format_version,
        file: header.file,
        styles,
    })
}

fn block_lang(info: &str) -> &str {
    info.split_whitespace().next().unwrap_or("")
}

fn render_fenced_code_block(info: &str, content: &str) -> String {
    let mut out = String::new();
    let fence_len = cmp::max(3, max_backtick_run(content) + 1);
    let fence = "`".repeat(fence_len);

    out.push_str(&fence);
    out.push_str(info);
    out.push('\n');
    out.push_str(content);
    if !content.ends_with('\n') {
        out.push('\n');
    }
    out.push_str(&fence);
    out.push('\n');
    out
}

fn max_backtick_run(s: &str) -> usize {
    let mut max_run = 0usize;
    let mut run = 0usize;
    for b in s.bytes() {
        if b == b'`' {
            run += 1;
            max_run = cmp::max(max_run, run);
        } else {
            run = 0;
        }
    }
    max_run
}

fn collect_markdown_files_rec(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), SuiteError> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let ft = entry.file_type()?;
        if ft.is_dir() {
            collect_markdown_files_rec(&path, out)?;
            continue;
        }
        if !ft.is_file() {
            continue;
        }
        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            out.push(path);
        }
    }
    Ok(())
}

fn safe_rel_path_from_slash(path: &str) -> Result<PathBuf, SuiteError> {
    let rel = Path::new(path);
    let mut out = PathBuf::new();
    for c in rel.components() {
        match c {
            Component::Normal(part) => out.push(part),
            _ => {
                return Err(SuiteError::InvalidManifest(format!(
                    "invalid relative path: {path}"
                )));
            }
        }
    }
    Ok(out)
}

pub fn extract_style_blocks(document: &str) -> Vec<String> {
    fn is_tag_boundary(b: Option<u8>) -> bool {
        matches!(b, None | Some(b'>' | b'/' | b'\t' | b'\n' | b'\r' | b' '))
    }

    fn is_pure_template_placeholder(css: &str) -> bool {
        let t = css.trim();
        t.starts_with("${") && t.ends_with('}') && t.len() >= 3
    }

    fn starts_with_ascii_ci(bytes: &[u8], at: usize, needle: &[u8]) -> bool {
        bytes.get(at..at + needle.len()).is_some_and(|h| {
            h.iter()
                .zip(needle.iter())
                .all(|(&h, &n)| h.eq_ignore_ascii_case(&n))
        })
    }

    fn find_tag_end(bytes: &[u8], from: usize) -> Option<usize> {
        let mut i = from;
        let mut quote: Option<u8> = None;
        while i < bytes.len() {
            let b = bytes[i];
            if let Some(q) = quote {
                if b == q {
                    quote = None;
                }
                i += 1;
                continue;
            }
            if b == b'"' || b == b'\'' {
                quote = Some(b);
                i += 1;
                continue;
            }
            if b == b'>' {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    fn extract_style_blocks_from_raw_text(text: &str) -> Vec<String> {
        let bytes = text.as_bytes();
        let mut out = Vec::new();
        let mut pos = 0usize;

        while let Some(open_start) = find_ascii_ci(bytes, b"<style", pos) {
            if !is_tag_boundary(bytes.get(open_start + 6).copied()) {
                pos = open_start + 1;
                continue;
            }

            let Some(open_end) = find_tag_end(bytes, open_start + 6) else {
                break;
            };
            let content_start = open_end + 1;

            let Some(close_start) = find_ascii_ci(bytes, b"</style", content_start) else {
                break;
            };
            let content = &text[content_start..close_start];
            // When extracting `<style>` blocks embedded in `<script>` strings, ignore template
            // placeholders like `<style>${styleText}</style>`, where the actual stylesheet is
            // computed at runtime and not present in the static markup.
            if !is_pure_template_placeholder(content) {
                out.push(content.to_string());
            }

            let Some(close_end) = find_tag_end(bytes, close_start + 7) else {
                break;
            };
            pos = close_end + 1;
        }

        out
    }

    let bytes = document.as_bytes();
    let mut out = Vec::new();
    let mut pos = 0usize;

    while pos < bytes.len() {
        let Some(lt_rel) = bytes[pos..].iter().position(|&b| b == b'<') else {
            break;
        };
        let lt = pos + lt_rel;

        // Skip HTML comments (`<!-- ... -->`) so `<style` inside them doesn't confuse extraction.
        if matches!(bytes.get(lt..lt + 4), Some([b'<', b'!', b'-', b'-'])) {
            if let Some(end_rel) = find_ascii_ci(bytes, b"-->", lt + 4) {
                pos = end_rel + 3;
                continue;
            }
            // Malformed comment; don't give up on the whole document.
            pos = lt + 4;
            continue;
        }

        // For `<script>...</script>`, extract style tags that appear in literal JS string content
        // (e.g. Shadow DOM `innerHTML = "<style>...</style>"`), but do not treat tags inside the
        // script as markup for the purposes of matching the closing `</script>`.
        if starts_with_ascii_ci(bytes, lt, b"<script")
            && is_tag_boundary(bytes.get(lt + 7).copied())
        {
            let Some(open_end) = find_tag_end(bytes, lt + 7) else {
                break;
            };
            let content_start = open_end + 1;
            let Some(close_start) = find_ascii_ci(bytes, b"</script", content_start) else {
                break;
            };
            out.extend(extract_style_blocks_from_raw_text(
                &document[content_start..close_start],
            ));
            let Some(close_end) = find_tag_end(bytes, close_start + 8) else {
                break;
            };
            pos = close_end + 1;
            continue;
        }

        if !starts_with_ascii_ci(bytes, lt, b"<style")
            || !is_tag_boundary(bytes.get(lt + 6).copied())
        {
            pos = lt + 1;
            continue;
        }

        let Some(open_end) = find_tag_end(bytes, lt + 6) else {
            break;
        };
        let content_start = open_end + 1;

        let Some(close_start) = find_ascii_ci(bytes, b"</style", content_start) else {
            break;
        };
        out.push(document[content_start..close_start].to_string());

        let Some(close_end) = find_tag_end(bytes, close_start + 7) else {
            break;
        };
        pos = close_end + 1;
    }

    out
}

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

fn suite_config_from_validator_config(config: &ValidatorConfig) -> SuiteConfig {
    SuiteConfig {
        profile: config.profile.clone(),
        medium: config.medium.clone(),
        warning: config.warning.clone(),
    }
}

fn tmp_path_for(path: &Path) -> PathBuf {
    let file_name = path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();
    path.with_file_name(format!("{file_name}.tmp"))
}

fn collect_markup_files_rec(
    dir: &Path,
    out: &mut Vec<(String, PathBuf)>,
    wpt_root: &Path,
) -> Result<(), SuiteError> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let ft = entry.file_type()?;
        if ft.is_dir() {
            collect_markup_files_rec(&path, out, wpt_root)?;
            continue;
        }
        if !ft.is_file() || !is_markup_file(&path) {
            continue;
        }
        let rel = path.strip_prefix(wpt_root).unwrap_or(&path).to_path_buf();
        out.push((path_to_slash(&rel), path));
    }
    Ok(())
}

fn is_markup_file(path: &Path) -> bool {
    let Some(ext) = path.extension().and_then(|e| e.to_str()) else {
        return false;
    };
    matches!(
        ext.to_ascii_lowercase().as_str(),
        "html" | "htm" | "xhtml" | "xht" | "xml" | "svg"
    )
}

fn path_to_slash(path: &Path) -> String {
    let mut out = String::new();
    for c in path.components() {
        let Component::Normal(part) = c else {
            continue;
        };
        if !out.is_empty() {
            out.push('/');
        }
        out.push_str(&part.to_string_lossy());
    }
    out
}

fn find_ascii_ci(haystack: &[u8], needle: &[u8], from: usize) -> Option<usize> {
    if needle.is_empty() {
        return Some(from.min(haystack.len()));
    }
    if haystack.len() < needle.len() || from >= haystack.len() {
        return None;
    }

    let last = haystack.len() - needle.len();
    for i in from..=last {
        if haystack[i..i + needle.len()]
            .iter()
            .zip(needle.iter())
            .all(|(&h, &n)| h.eq_ignore_ascii_case(&n))
        {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{
        WptCssStyleBlockResult, WptCssStyleFileResults, WptCssStyleResultsMeta,
        ensure_trailing_newline, extract_style_blocks, find_ascii_ci, parse_file_results_markdown,
        parse_results_meta_markdown, render_file_results_markdown, render_results_meta_markdown,
        safe_rel_path_from_slash, suite_config_from_validator_config, wpt_style_id,
    };
    use css_inspector::Config as ValidatorConfig;
    use serde_json::json;

    #[test]
    fn find_ascii_ci_finds_case_insensitively() {
        assert_eq!(find_ascii_ci(b"Abc<STYLE>x</style>", b"<style", 0), Some(3));
    }

    #[test]
    fn extract_style_blocks_extracts_multiple_blocks() {
        let html = "<style>a{}</style><div></div><STYLE>b{}</STYLE>";
        assert_eq!(
            extract_style_blocks(html),
            vec!["a{}".to_string(), "b{}".to_string()]
        );
    }

    #[test]
    fn extract_style_blocks_ignores_missing_closing_tag() {
        let html = "<style>a{}";
        assert!(extract_style_blocks(html).is_empty());
    }

    #[test]
    fn extract_style_blocks_ignores_style_like_text_in_scripts() {
        let html = r#"<script>const x = "<stylesheet>";</script><style>a{}</style>"#;
        assert_eq!(extract_style_blocks(html), vec!["a{}".to_string()]);
    }

    #[test]
    fn extract_style_blocks_extracts_style_blocks_embedded_in_scripts() {
        let html = r#"<script>const x = `<style>a{}</style>`;</script>"#;
        assert_eq!(extract_style_blocks(html), vec!["a{}".to_string()]);
    }

    #[test]
    fn extract_style_blocks_ignores_placeholder_style_blocks_embedded_in_scripts() {
        let html = r#"<script>iframe.srcdoc = `<style>${styleText}</style>`;</script>"#;
        assert!(extract_style_blocks(html).is_empty());
    }

    #[test]
    fn extract_style_blocks_ignores_style_like_text_in_comments() {
        let html = r#"<!-- <style>a{}</style> --><style>b{}</style>"#;
        assert_eq!(extract_style_blocks(html), vec!["b{}".to_string()]);
    }

    #[test]
    fn extract_style_blocks_does_not_abort_on_malformed_html_comments() {
        let html = r#"<!-- oops --!><style>a{}</style>"#;
        assert_eq!(extract_style_blocks(html), vec!["a{}".to_string()]);
    }

    #[test]
    fn wpt_style_id_includes_index() {
        assert_eq!(wpt_style_id("css/x.html", 2), "css/x.html#style[2]");
    }

    #[test]
    fn safe_rel_path_from_slash_rejects_traversal() {
        assert!(safe_rel_path_from_slash("../x").is_err());
        assert!(safe_rel_path_from_slash("/abs").is_err());
    }

    #[test]
    fn results_meta_markdown_round_trips() {
        let config = ValidatorConfig {
            profile: Some("css3".to_string()),
            medium: None,
            warning: Some("0".to_string()),
        };
        let meta = WptCssStyleResultsMeta {
            format_version: super::WPT_CSS_STYLE_RESULTS_FORMAT_VERSION,
            wpt_commit: "abc123".to_string(),
            config: suite_config_from_validator_config(&config),
            totals: super::WptCssStyleResultsTotals {
                files_with_style_blocks: 1,
                style_blocks: 2,
                errors: 3,
                warnings: 4,
            },
        };
        let md = render_results_meta_markdown(&meta).unwrap();
        let parsed = parse_results_meta_markdown(std::path::Path::new("meta.md"), &md).unwrap();
        assert_eq!(parsed, meta);
    }

    #[test]
    fn file_results_markdown_round_trips_and_handles_backticks() {
        let mut css = "a{content:\"```\";}".to_string();
        ensure_trailing_newline(&mut css);

        let results = WptCssStyleFileResults {
            format_version: super::WPT_CSS_STYLE_RESULTS_FORMAT_VERSION,
            file: "css/x.html".to_string(),
            styles: vec![WptCssStyleBlockResult {
                index: 0,
                css,
                report: json!({"errors":0,"warnings":0,"messages":[]}),
            }],
        };

        let md = render_file_results_markdown(&results).unwrap();
        let parsed = parse_file_results_markdown(std::path::Path::new("x.md"), &md).unwrap();
        assert_eq!(parsed, results);
    }
}
