# `css_inspector`

Rust CSS validator. This crate is primarily “suite-driven”: it implements the
checks needed to match the upstream W3C CSS Validator autotest fixtures, rather than a complete
CSS parser.

MSRV: Rust 1.93 (Edition 2024).

## Install

`cargo add css_inspector`

## Entry points

- `validate_css_text(css, &Config) -> Report`: validates a stylesheet as text.
- `validate_css_declarations_text(decls, &Config) -> Report`: validates a declaration list
  (e.g. an HTML `style=""` attribute).
- `validate_css_text_with_fetcher(css, base_uri, &Config, &dyn Fetcher) -> Report`: like
  `validate_css_text`, but recursively validates top-level `@import url(...)` sheets via a fetcher.
- `validate_css_uri_with_fetcher(uri, &Config, &dyn Fetcher) -> Report`: fetches CSS then validates
  with `validate_css_text_with_fetcher`.

## Configuration

`Config` currently supports:

- `profile`: selects the “known properties” set (defaults to CSS4) from `data/css_properties/*.properties`.
- `medium`: if set, warns when an `@media ... {` block doesn’t match the configured medium.
- `warning`: numeric warning threshold (default `0`); `-1` suppresses all warnings.

## Checks performed (today)

High-level checks (errors unless noted):

- Comment handling: strips `/* ... */`; unterminated comments report `Unclosed comment.`.
- Top-level structure:
  - Unbalanced `{}` (`Unbalanced braces.`).
  - Stray trailing `\\` (`Invalid escape at end of input.`).
  - Top-level `<` (`Invalid input.`) as a cheap “HTML in CSS” guard.
  - Stray top-level `prop: value;` segments (`Stray declaration outside a rule.`).
- At-rule name check: unknown `@...` names are rejected (`Unknown at-rule.`).
- `@media` vs `Config.medium` (warning): `Properties for other media might not work for usermedium.`
- Selector checks (minimal): basic sanity checks + pseudo validation against a profile-dependent allowlist.
- Attribute selector conflicts (warning): detects unsatisfiable combinations like `[a="x"][a="y"]`.
- Declarations:
  - Syntax checks (`prop: value`, property-name validity, missing `;` recovery).
  - Unknown properties: rejected based on the selected profile’s “known properties” list
    (custom properties `--foo` are allowed).
  - Vendor extension properties:
    - Usually reported as unknown-property errors.
    - When `warning = -1` (warnings suppressed), they are demoted to warnings, which are then suppressed,
      so they produce no messages (matching the suite’s expectations).
  - Values:
    - Missing values.
    - CSS-wide keyword sanity (rejects mixing wide keywords with other tokens).
    - “Too many tokens” for many single-valued properties.
    - Suite-driven value validation for a limited set of properties (e.g. `color`, `background*`,
      `border*`, `outline*`, `font`, `cursor`, `content`, `counter-*`, `quotes`, `filter`, etc).
- Optional top-level `@import` handling:
  - Text-only validation: if a top-level `@import url(...)` exists, warns once:
    `Imported style sheets are not checked.`
  - Fetching validation: recursively fetches and validates imported sheets first (with loop detection).

## Fetching

This crate defines `Fetcher` and a default `StdFetcher`:

- `file://...` is supported.
- `http://` and `https://` are supported only when `StdFetcher.allow_network = true`.
- Other schemes are rejected.

Note: `StdFetcher` is intended for test harnesses / controlled usage. If you validate untrusted CSS
with fetching enabled, treat the fetcher configuration as part of your security boundary.
