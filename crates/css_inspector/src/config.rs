#[derive(Clone, Debug)]
pub struct Config {
    pub profile: Option<String>,
    pub medium: Option<String>,
    pub warning: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            profile: Some("css4".to_string()),
            medium: None,
            warning: None,
        }
    }
}

pub(crate) fn warning_level_from_config(config: &Config) -> i32 {
    config
        .warning
        .as_deref()
        .and_then(|s| s.parse().ok())
        .unwrap_or_default()
}

pub(crate) fn css1_escapes_from_config(config: &Config) -> bool {
    config
        .profile
        .as_deref()
        .is_some_and(|p| p.eq_ignore_ascii_case("css1"))
}
