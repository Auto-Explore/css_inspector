use std::borrow::Cow;
use std::collections::HashSet;
use std::sync::OnceLock;

use crate::config::Config;
use crate::strutil::ascii_lowercase_cow;

#[cfg(test)]
use crate::validate_css_text;

pub(crate) type KnownProperties = HashSet<Cow<'static, str>>;

pub(crate) fn known_properties_for_config(config: &Config) -> &'static KnownProperties {
    match config.profile.as_deref() {
        Some(p) if p.eq_ignore_ascii_case("css1") => known_properties_css1(),
        Some(p) if p.eq_ignore_ascii_case("css2") => known_properties_css2(),
        Some(p) if p.eq_ignore_ascii_case("css21") => known_properties_css21(),
        Some(p) if p.eq_ignore_ascii_case("css3svg") => known_properties_css3svg(),
        Some(p) if p.eq_ignore_ascii_case("svg") => known_properties_svg(),
        Some(p) if p.eq_ignore_ascii_case("svgbasic") => known_properties_svg_basic(),
        Some(p) if p.eq_ignore_ascii_case("svgtiny") => known_properties_svg_tiny(),
        _ => known_properties_css3(),
    }
}

fn parse_properties_file_into(s: &'static str, set: &mut KnownProperties) {
    for line in s.lines() {
        let line = line.trim();
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
            set.insert(ascii_lowercase_cow(name));
        }
    }
}

pub(crate) fn parse_properties_file(s: &'static str) -> KnownProperties {
    let mut set = HashSet::new();
    parse_properties_file_into(s, &mut set);
    set
}

#[cfg(test)]
mod parse_properties_file_tests {
    use std::borrow::Cow;
    use std::collections::HashSet;

    use super::{parse_properties_file, parse_properties_file_into};

    #[test]
    fn parses_property_names_ignoring_comments_and_garbage_lines() {
        let set = parse_properties_file(
            r#"
# comment

  color: anything
  FONT-SIZE: 12px
  foo : bar
  not-a-prop
  : bad
"#,
        );

        assert!(set.contains("color"));
        assert!(set.contains("font-size"));
        assert!(set.contains("foo"));
        assert!(!set.contains("not-a-prop"));
        assert!(!set.contains(""));
    }

    #[test]
    fn stores_borrowed_entries_when_no_lowercasing_needed() {
        let set = parse_properties_file("color: ok\n");
        let only = set.iter().next().expect("one entry");
        assert!(matches!(only, Cow::Borrowed("color")));
    }

    #[test]
    fn parse_properties_file_into_appends_to_existing_set() {
        let mut set = HashSet::new();
        parse_properties_file_into("color: ok\n", &mut set);
        parse_properties_file_into("fill-opacity: ok\n", &mut set);

        assert!(matches!(set.get("color"), Some(Cow::Borrowed("color"))));
        assert!(matches!(
            set.get("fill-opacity"),
            Some(Cow::Borrowed("fill-opacity"))
        ));
    }
}

#[cfg(test)]
mod known_properties_for_config_tests {
    use super::{
        Config, KnownProperties, known_properties_css1, known_properties_css2,
        known_properties_css3, known_properties_css3svg, known_properties_css21,
        known_properties_for_config, known_properties_svg, known_properties_svg_basic,
        known_properties_svg_tiny,
    };

    #[test]
    fn selects_known_properties_set_by_profile_case_insensitively() {
        type SetFn = fn() -> &'static KnownProperties;
        let cases = [
            ("css1", known_properties_css1 as SetFn),
            ("CSS2", known_properties_css2 as SetFn),
            ("cSs21", known_properties_css21 as SetFn),
            ("CSS3", known_properties_css3 as SetFn),
            ("", known_properties_css3 as SetFn),
            ("css", known_properties_css3 as SetFn),
            ("CSS", known_properties_css3 as SetFn),
            ("css-2015", known_properties_css3 as SetFn),
            ("cSs-2015", known_properties_css3 as SetFn),
            ("CSS3SVG", known_properties_css3svg as SetFn),
            ("SvG", known_properties_svg as SetFn),
            ("SvGBasic", known_properties_svg_basic as SetFn),
            ("SvGTiny", known_properties_svg_tiny as SetFn),
            ("unknown-profile", known_properties_css3 as SetFn),
        ];

        for (profile, expected) in cases {
            let config = Config {
                profile: Some(profile.to_string()),
                ..Config::default()
            };
            assert!(std::ptr::eq(
                known_properties_for_config(&config),
                expected()
            ));
        }
    }

    #[test]
    fn defaults_to_css3_when_profile_is_none() {
        let config = Config::default();
        assert!(std::ptr::eq(
            known_properties_for_config(&config),
            known_properties_css3()
        ));
    }

    #[test]
    fn defaults_to_css3_when_profile_is_empty_string() {
        let config = Config {
            profile: Some(String::new()),
            ..Config::default()
        };
        assert!(std::ptr::eq(
            known_properties_for_config(&config),
            known_properties_css3()
        ));
    }

    #[test]
    fn defaults_to_css3_when_profile_is_not_trimmed() {
        let config = Config {
            profile: Some(" css1 ".to_string()),
            ..Config::default()
        };
        assert!(std::ptr::eq(
            known_properties_for_config(&config),
            known_properties_css3()
        ));
    }

    #[test]
    fn defaults_to_css3_when_profile_has_non_space_whitespace_prefix() {
        let config = Config {
            profile: Some("\tcss1".to_string()),
            ..Config::default()
        };
        assert!(std::ptr::eq(
            known_properties_for_config(&config),
            known_properties_css3()
        ));
    }

    #[test]
    fn known_properties_sets_include_expected_entries() {
        assert!(known_properties_css1().contains("font-style"));
        assert!(known_properties_css2().contains("font-stretch"));
        assert!(known_properties_css21().contains("background-color"));
        assert!(known_properties_css3svg().contains("font-style"));
        assert!(known_properties_css3svg().contains("fill-opacity"));
        assert!(known_properties_svg().contains("alignment-baseline"));
        assert!(known_properties_svg_basic().contains("fill"));
        assert!(known_properties_svg_tiny().contains("stroke"));
    }
}

#[cfg(test)]
mod svg_attribute_property_tests {
    use super::{Config, validate_css_text};

    #[test]
    fn svg_attributes_are_accepted_as_properties_in_svg_profiles() {
        let css = r#"
rect {   x:  1px;
         y:  2px;
         rx: 3px;
         ry: 4px;
}

circle { r:  5px;
         cx: 6px;
         cy: 7px;
}

path {   d:  path("M 1 2 z");
}
"#;

        for profile in ["svg", "css3svg"] {
            let config = Config {
                profile: Some(profile.to_string()),
                ..Config::default()
            };
            let report = validate_css_text(css, &config).unwrap();
            assert_eq!(report.errors, 0, "profile={profile} report={report:?}");
            assert_eq!(report.warnings, 0, "profile={profile} report={report:?}");
            assert!(
                report.messages.is_empty(),
                "profile={profile} report={report:?}"
            );
        }
    }
}

#[cfg(test)]
mod font_feature_values_tests {
    use super::{Config, validate_css_text};

    #[test]
    fn font_feature_values_at_rule_is_accepted() {
        let css = r#"
@font-feature-values Fira Code {
    @character-variant {
        alt-a: 1;
        alt-g: 2;
        alt-i-1: 3;
    }
}
"#;
        let report = validate_css_text(css, &Config::default()).unwrap();
        assert_eq!(report.errors, 0, "{report:?}");
        assert_eq!(report.warnings, 0, "{report:?}");
        assert!(report.messages.is_empty(), "{report:?}");
    }
}

#[cfg(test)]
mod container_query_tests {
    use super::{Config, validate_css_text};

    #[test]
    fn container_queries_are_accepted() {
        let css = r#"
.container {
  container-name: test-container;
  container-type: inline-size;
}

p {
  font-size: 1rem;
}

@container test-container (max-width: 300px) {
  p {
    font-size: .5rem;
  }
}
"#;
        let report = validate_css_text(css, &Config::default()).unwrap();
        assert_eq!(report.errors, 0, "{report:?}");
        assert_eq!(report.warnings, 0, "{report:?}");
        assert!(report.messages.is_empty(), "{report:?}");
    }

    #[test]
    fn container_query_range_syntax_with_grid_template_is_accepted() {
        let css = r#"
@container my-layout (inline-size > 45em) {
  .media-object {
    grid-template: 'img content' auto / auto 1fr;
  }
}
"#;
        let report = validate_css_text(css, &Config::default()).unwrap();
        assert_eq!(report.errors, 0, "{report:?}");
        assert_eq!(report.warnings, 0, "{report:?}");
        assert!(report.messages.is_empty(), "{report:?}");
    }
}

#[cfg(test)]
mod scrollbar_gutter_tests {
    use super::{Config, validate_css_text};

    #[test]
    fn scrollbar_gutter_auto_is_accepted() {
        let css = r#"
#mydiv {
    scrollbar-gutter: auto;
}
"#;
        let report = validate_css_text(css, &Config::default()).unwrap();
        assert_eq!(report.errors, 0, "{report:?}");
        assert_eq!(report.warnings, 0, "{report:?}");
        assert!(report.messages.is_empty(), "{report:?}");
    }
}

#[cfg(test)]
mod font_optical_sizing_tests {
    use super::{Config, validate_css_text};

    #[test]
    fn font_optical_sizing_auto_and_none_are_accepted() {
        let css = r#"
#mydiv {
    font-optical-sizing: auto;
}

#mydiv2 {
    font-optical-sizing: none;
}
"#;
        let report = validate_css_text(css, &Config::default()).unwrap();
        assert_eq!(report.errors, 0, "{report:?}");
        assert_eq!(report.warnings, 0, "{report:?}");
        assert!(report.messages.is_empty(), "{report:?}");
    }

    #[test]
    fn font_optical_sizing_rejects_invalid_values() {
        let css = r#"
#mydiv {
    font-optical-sizing: no-such-value;
}
"#;
        let report = validate_css_text(css, &Config::default()).unwrap();
        assert_eq!(report.errors, 1, "{report:?}");
        assert_eq!(report.warnings, 0, "{report:?}");
        assert_eq!(report.messages.len(), 1, "{report:?}");
        assert_eq!(
            report.messages[0].message,
            "Invalid value for property “font-optical-sizing”."
        );
    }
}

#[cfg(test)]
mod clip_path_tests {
    use super::{Config, validate_css_text};

    #[test]
    fn clip_path_path_function_is_accepted() {
        let css = r#"
#mydiv {
    clip-path: path('M 0 200 L 0,75 A 5,5 0,0,1 150,75 L 200 200 z');
}
"#;
        let report = validate_css_text(css, &Config::default()).unwrap();
        assert_eq!(report.errors, 0, "{report:?}");
        assert_eq!(report.warnings, 0, "{report:?}");
        assert!(report.messages.is_empty(), "{report:?}");
    }

    #[test]
    fn clip_path_margin_box_polygon_is_accepted() {
        let css = r#"
#mydiv {
    clip-path: margin-box polygon(20% 0%, 100% 50%, 80% 100%, 0% 50%);
}
"#;
        let report = validate_css_text(css, &Config::default()).unwrap();
        assert_eq!(report.errors, 0, "{report:?}");
        assert_eq!(report.warnings, 0, "{report:?}");
        assert!(report.messages.is_empty(), "{report:?}");
    }
}

#[cfg(test)]
mod layer_at_rule_tests {
    use super::{Config, validate_css_text};

    #[test]
    fn layer_statement_is_accepted() {
        let css = r#"
@layer base, components, utilities;

#mydiv { color: red; }
"#;
        let report = validate_css_text(css, &Config::default()).unwrap();
        assert_eq!(report.errors, 0, "{report:?}");
        assert_eq!(report.warnings, 0, "{report:?}");
        assert!(report.messages.is_empty(), "{report:?}");
    }

    #[test]
    fn layer_block_is_accepted() {
        let css = r#"
@layer base {
    #mydiv { color: red; }
}
"#;
        let report = validate_css_text(css, &Config::default()).unwrap();
        assert_eq!(report.errors, 0, "{report:?}");
        assert_eq!(report.warnings, 0, "{report:?}");
        assert!(report.messages.is_empty(), "{report:?}");
    }
}

#[cfg(test)]
mod nested_at_rule_tests {
    use super::{Config, validate_css_text};

    #[test]
    fn media_with_nested_page_rule_is_accepted() {
        let css = r#"
@media print {
    @page { size: a4 }
    body { background: none }
}
"#;
        let report = validate_css_text(css, &Config::default()).unwrap();
        assert_eq!(report.errors, 0, "{report:?}");
        assert_eq!(report.warnings, 0, "{report:?}");
        assert!(report.messages.is_empty(), "{report:?}");
    }

    #[test]
    fn nested_media_queries_inside_style_rule_are_accepted() {
        let css = r#"
.foo {
    display: grid;
    @media (orientation: landscape) {
        grid-auto-flow: column;
        @media (width >= 1024px) {
            max-inline-size: 1024px;
        }
    }
}
"#;
        let report = validate_css_text(css, &Config::default()).unwrap();
        assert_eq!(report.errors, 0, "{report:?}");
        assert_eq!(report.warnings, 0, "{report:?}");
        assert!(report.messages.is_empty(), "{report:?}");
    }

    #[test]
    fn declarations_inside_nested_at_rules_are_validated() {
        let css = r#"
.foo {
    @media (orientation: landscape) {
        no-such-prop: 1;
    }
}
"#;
        let report = validate_css_text(css, &Config::default()).unwrap();
        assert_eq!(report.errors, 1, "{report:?}");
        assert_eq!(report.warnings, 0, "{report:?}");
        assert_eq!(report.messages.len(), 1, "{report:?}");
        assert_eq!(
            report.messages[0].message,
            "Unknown property “no-such-prop”."
        );
    }

    #[test]
    fn nested_layers_inside_style_rule_are_accepted() {
        let css = r#"
.foo {
    @layer base {
        block-size: 100%;
        @layer support {
            & .bar {
                min-block-size: 100%;
            }
        }
    }
}
"#;
        let report = validate_css_text(css, &Config::default()).unwrap();
        assert_eq!(report.errors, 0, "{report:?}");
        assert_eq!(report.warnings, 0, "{report:?}");
        assert!(report.messages.is_empty(), "{report:?}");
    }

    #[test]
    fn layer_blocks_with_dot_names_are_accepted() {
        let css = r#"
@layer base {
    .foo {
        block-size: 100%;
    }
}
@layer base.support {
    .foo .bar {
        min-block-size: 100%;
    }
}
"#;
        let report = validate_css_text(css, &Config::default()).unwrap();
        assert_eq!(report.errors, 0, "{report:?}");
        assert_eq!(report.warnings, 0, "{report:?}");
        assert!(report.messages.is_empty(), "{report:?}");
    }

    #[test]
    fn five_levels_of_nested_at_rules_reports_error_from_deepest_level() {
        let css = r#"
.foo {
    display: grid;
    @media (orientation: landscape) {
        grid-auto-flow: column;
        @supports (display: grid) {
            block-size: 100%;
            @container sidebar (min-width: 10px) {
                min-block-size: 0;
                @layer base {
                    max-inline-size: 100%;
                    @media (width >= 1024px) {
                        no-such-prop: 1;
                    }
                }
            }
        }
    }
}
"#;
        let report = validate_css_text(css, &Config::default()).unwrap();
        assert_eq!(report.errors, 1, "{report:?}");
        assert_eq!(report.warnings, 0, "{report:?}");
        assert_eq!(report.messages.len(), 1, "{report:?}");
        assert_eq!(
            report.messages[0].message,
            "Unknown property “no-such-prop”."
        );
    }

    #[test]
    fn six_levels_of_nested_at_rules_with_complex_valid_css_are_accepted() {
        let css = r#"
.foo, .bar {
    padding: 1rem 2rem;
    @layer base {
        --gap: 1rem;
        display: grid;
        gap: var(--gap);
        @media (orientation: landscape) {
            grid-auto-flow: column;
            @supports (display: grid) {
                @container main (min-width: 10px) {
                    @layer support {
                        @media (width >= 1024px) {
                            & .baz:is(.qux, [data-x="y"]):hover {
                                max-inline-size: calc(1024px - 2rem);
                                min-block-size: 100%;
                            }
                        }
                    }
                }
            }
        }
    }
}
"#;
        let report = validate_css_text(css, &Config::default()).unwrap();
        assert_eq!(report.errors, 0, "{report:?}");
        assert_eq!(report.warnings, 0, "{report:?}");
        assert!(report.messages.is_empty(), "{report:?}");
    }
}

macro_rules! css_properties_file {
    ($file:literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../data/css_properties/",
            $file
        ))
    };
}

macro_rules! known_properties_from_file {
    ($name:ident, $file:literal) => {
        fn $name() -> &'static KnownProperties {
            static ONCE: OnceLock<KnownProperties> = OnceLock::new();
            ONCE.get_or_init(|| parse_properties_file(css_properties_file!($file)))
        }
    };
}

known_properties_from_file!(known_properties_css1, "CSS1Properties.properties");
known_properties_from_file!(known_properties_css2, "CSS2Properties.properties");
known_properties_from_file!(known_properties_css21, "CSS21Properties.properties");

fn known_properties_css3() -> &'static KnownProperties {
    static ONCE: OnceLock<KnownProperties> = OnceLock::new();
    ONCE.get_or_init(|| {
        let mut set = parse_properties_file(css_properties_file!("CSS3Properties.properties"));
        // Upstream deploys have historically treated `color-profile` as a CSS3 property even when
        // validating with the `css3` profile (see autotest `propertiesCSS3.css`).
        set.insert(Cow::Borrowed("color-profile"));
        set
    })
}

fn known_properties_css3svg() -> &'static KnownProperties {
    static ONCE: OnceLock<KnownProperties> = OnceLock::new();
    ONCE.get_or_init(|| {
        let mut set = HashSet::new();
        parse_properties_file_into(css_properties_file!("CSS3Properties.properties"), &mut set);
        parse_properties_file_into(
            css_properties_file!("CSS3SVGProperties.properties"),
            &mut set,
        );
        set
    })
}

known_properties_from_file!(known_properties_svg, "SVGProperties.properties");
known_properties_from_file!(known_properties_svg_basic, "SVGBasicProperties.properties");
known_properties_from_file!(known_properties_svg_tiny, "SVGTinyProperties.properties");
