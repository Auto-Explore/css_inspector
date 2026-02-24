# Changelog

All notable changes to this project will be documented in this file.

## [0.2.2]

### Added

- Validate additional UI-focused properties: `resize` (incl `block`/`inline` in `css4`), `user-select`, `appearance`, `caret`/`caret-*`, `accent-color`, `pointer-events`, `outline-offset`, and `nav-*` (currently `auto` only).
- Add error-detection unit tests.

### Changed

- Accept a broader set of `cursor` keywords across profiles (e.g. `grab`, `grabbing`, `zoom-in`, `zoom-out`).

### Fixed

- In the `css4` profile, `outline` / `outline-color` no longer accept `invert` (use `auto` or an explicit color instead).

## [0.2.1] - 2026-02-15

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
