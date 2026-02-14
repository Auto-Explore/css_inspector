use super::*;

#[test]
fn parse_properties_file_trims_ignores_comments_and_normalizes_ascii_case() {
    let props = parse_properties_file(
        "# comment\n\n  Color  : x\nMARGIN: y\nno-colon-here\n\tfont-size: z  \n",
    );
    assert!(props.contains("color"));
    assert!(props.contains("margin"));
    assert!(props.contains("font-size"));
    assert!(!props.contains("Color"));
    assert!(!props.contains("MARGIN"));
}

#[test]
fn css4_phase1_properties_file_matches_w3c_level4_diff() {
    #[derive(serde::Deserialize)]
    struct W3cPropertyRow {
        #[serde(default)]
        property: String,
        #[serde(default)]
        title: String,
    }

    fn parse_properties_file_names_in_order(s: &str) -> Vec<String> {
        let mut out = Vec::with_capacity(s.lines().count());
        for raw in s.lines() {
            let line = raw.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let name = if let Some((name, _)) = line.split_once(':') {
                name.trim()
            } else {
                let mut parts = line.split_whitespace();
                let Some(first) = parts.next() else {
                    continue;
                };
                if parts.next().is_none() {
                    continue;
                }
                first
            };
            if !name.is_empty() {
                out.push(name.to_ascii_lowercase());
            }
        }
        out
    }

    fn parse_properties_file_names_set(s: &str) -> std::collections::BTreeSet<String> {
        parse_properties_file_names_in_order(s)
            .into_iter()
            .collect()
    }

    let crate_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = crate_root.join("../..");

    let w3c_path = workspace_root.join("data/w3c/all-properties.en.json");
    if !w3c_path.exists() {
        // This file is only present in the repo checkout, not in published crate source.
        eprintln!("skipping: missing {w3c_path:?}");
        return;
    }

    let css3_path = crate_root.join("data/css_properties/CSS3Properties.properties");
    let css4_path = crate_root.join("data/css_properties/CSS4Properties.properties");

    let w3c_text = std::fs::read_to_string(&w3c_path).expect("read all-properties.en.json");
    let rows: Vec<W3cPropertyRow> =
        serde_json::from_str(&w3c_text).expect("parse all-properties.en.json");

    let mut level4 = std::collections::BTreeSet::new();
    for r in rows {
        if !r.title.contains("Level 4") {
            continue;
        }
        let prop = r.property.trim().to_ascii_lowercase();
        if prop.is_empty() || prop == "--*" {
            continue;
        }
        level4.insert(prop);
    }

    let css3_text = std::fs::read_to_string(&css3_path).expect("read CSS3Properties.properties");
    let mut css3 = parse_properties_file_names_set(&css3_text);
    css3.insert("color-profile".to_string());

    let expected_css4: std::collections::BTreeSet<String> =
        level4.difference(&css3).cloned().collect();

    let css4_text = std::fs::read_to_string(&css4_path).expect("read CSS4Properties.properties");
    let css4_in_order = parse_properties_file_names_in_order(&css4_text);
    let css4_set = parse_properties_file_names_set(&css4_text);

    assert_eq!(css4_set, expected_css4);

    let mut sorted = css4_in_order.clone();
    sorted.sort();
    sorted.dedup();
    assert_eq!(
        css4_in_order, sorted,
        "CSS4Properties.properties should be sorted and de-duplicated; regenerate with `python3 scripts/generate_css4_properties_data.py`"
    );
}

#[test]
fn css4_phase1_profile_accepts_all_css4_supplement_property_names() {
    fn css4_props() -> Vec<String> {
        let s = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/data/css_properties/CSS4Properties.properties"
        ));
        let set = parse_properties_file(s);
        let mut out: Vec<String> = Vec::with_capacity(set.len());
        out.extend(set.into_iter().map(|p| p.into_owned()));
        out.sort();
        out
    }

    let props = css4_props();
    assert!(!props.is_empty(), "expected css4 supplement properties");

    let mut css = String::from("a{\n");
    for p in &props {
        css.push_str(p);
        css.push_str(": inherit;\n");
    }
    css.push_str("}\n");

    let config = Config {
        profile: Some("css4".to_string()),
        ..Config::default()
    };
    let report = validate_css_text(&css, &config).unwrap();
    assert_eq!(report.errors, 0, "{report:?}");
}

#[test]
fn css3_profile_rejects_css4_supplement_properties_as_unknown() {
    fn css4_props() -> Vec<String> {
        let s = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/data/css_properties/CSS4Properties.properties"
        ));
        let set = parse_properties_file(s);
        let mut out: Vec<String> = Vec::with_capacity(set.len());
        out.extend(set.into_iter().map(|p| p.into_owned()));
        out.sort();
        out
    }

    let props = css4_props();
    assert!(!props.is_empty(), "expected css4 supplement properties");

    let mut css = String::from("a{\n");
    for p in &props {
        css.push_str(p);
        css.push_str(": inherit;\n");
    }
    css.push_str("}\n");

    let config = Config {
        profile: Some("css3".to_string()),
        ..Config::default()
    };
    let report = validate_css_text(&css, &config).unwrap();
    assert_eq!(report.warnings, 0, "{report:?}");
    assert_eq!(report.errors, props.len(), "{report:?}");
    assert_eq!(report.messages.len(), props.len(), "{report:?}");

    let expected: std::collections::BTreeSet<String> = props
        .iter()
        .map(|p| format!("Unknown property “{p}”."))
        .collect();
    let got: std::collections::BTreeSet<String> =
        report.messages.iter().map(|m| m.message.clone()).collect();
    assert_eq!(got, expected);
    assert!(
        report
            .messages
            .iter()
            .all(|m| matches!(m.severity, Severity::Error))
    );
}
