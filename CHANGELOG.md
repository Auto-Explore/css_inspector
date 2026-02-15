# Changelog

All notable changes to this project will be documented in this file.

## [0.2.1] - Unreleased

### Added

- Validate the `inset` shorthand (length/percentage/auto; `css4` profile also allows `anchor()` / `anchor-size()` tokens).
- Accept a broader set of `cursor` keywords in the `css4` profile (e.g. `grab`, `grabbing`, `zoom-in`, `zoom-out`, `none`).

### Changed

- `cursor: url(...)` now requires a keyword fallback (e.g. `cursor: url(foo.png) 2 2, pointer;`).

### Fixed

- Treat `inset` as a multi-token property in the single-value heuristic to avoid false positives.

## [0.2.0] - 2026-02-14

### Changed

- Use `rustc-hash` (`FxHashMap`/`FxHashSet`) internally for improved performance.

## [0.1.0] - 2026-02-13

Initial public release.

### Added

- Suite-driven, conservative CSS validation via `validate_css_text` and `validate_css_declarations_text`.
- Optional recursive `@import` validation via `validate_css_text_with_fetcher` / `validate_css_uri_with_fetcher`.
