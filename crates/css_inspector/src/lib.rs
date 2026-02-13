//! Rust-based, suite-driven CSS validator.
//!
//! This validator is intentionally conservative and focuses on catching issues that are likely to
//! break real-world behavior, rather than aiming to be fully spec-complete.
//!
//! # Example
//! ```
//! use css_inspector::{Config, validate_css_text};
//!
//! let report = validate_css_text("a { color: red; }", &Config::default()).unwrap();
//! assert!(report.valid());
//! ```

mod config;
mod declarations;
mod errors;
mod fetcher;
mod imports;
mod known_properties;
mod parser;
mod report;
mod selectors;
mod strutil;
mod url;
mod validator;

pub use config::Config;
pub use errors::{UrlDecodePlusError, ValidatorError};
pub use fetcher::{
    Fetcher, StdFetcher, validate_css_text_with_fetcher, validate_css_uri_with_fetcher,
};
pub use report::{Message, Report, Severity};
pub use strutil::starts_with_ascii_ci;
pub use url::url_decode_plus;
pub use validator::{validate_css_declarations_text, validate_css_text};

#[cfg(test)]
pub(crate) use config::{css1_escapes_from_config, warning_level_from_config};
#[cfg(test)]
pub(crate) use declarations::{
    border_side_component_flags, for_each_affected_border_longhand, is_css_wide_keyword,
    is_css_wide_keywordish_token, is_valid_url_function_token,
};
#[cfg(test)]
pub(crate) use fetcher::{file_url_to_path, resolve_relative_uri, split_http_base};
#[cfg(test)]
pub(crate) use imports::{iter_top_level_import_urls, unquote};
#[cfg(test)]
pub(crate) use known_properties::parse_properties_file;
#[cfg(test)]
pub(crate) use parser::{
    RuleBlockKind, at_rule_name, contains_invalid_top_level_chars, contains_unknown_at_rule,
    count_brace_balance_errors, count_stray_top_level_declaration_errors, is_known_at_rule_name,
    iter_rule_blocks, strip_css_comments,
};
#[cfg(test)]
pub(crate) use selectors::{
    AttrConstraint, AttrOp, attribute_constraints_conflict, constraint_allows_value,
    constraints_pair_conflict, dash_match_prefix, parse_attr_selector,
};
#[cfg(test)]
pub(crate) use strutil::{ascii_lowercase_cow, ends_with_ascii_ci};

#[cfg(test)]
mod tests;
