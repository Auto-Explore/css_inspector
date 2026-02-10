# CSS Inspector

Rust-based CSS validator.

Project goal: prioritize validation findings by severity, so the highest severity is reserved for issues that are likely to break real-world behavior as opposed to being strictly spec compliant.

This project is written in Rust to integrate easily with the existing Rust ecosystem.

## What it validates

This validator is intentionally conservative and mostly “suite-driven”: it implements
the checks needed to match the upstream W3C autotest fixtures, not a full CSS parser.

High-level checks (errors unless noted):

- Comment handling: strips `/* ... */`; unterminated comments report `Unclosed comment.`.
- Top-level structure: unbalanced `{}`; stray trailing `\\`; top-level `<` (as a cheap “HTML in CSS” guard).
- At-rule names: unknown `@...` names are rejected (name-only check; does not fully parse each at-rule).
- Stray top-level declarations: `prop: value;` outside a rule is rejected.
- Selectors (minimal): basic sanity checks + pseudo validation against a profile-dependent allowlist;
  detects obviously invalid selectors (unbalanced `[]`/`()`/strings, unknown pseudo names, etc).
- Attribute selector conflicts (warning): e.g. `[hello="A"][hello="B"]` yields
  `Conflicting attribute selector constraints.`.
- Declarations:
  - Syntax checks (`prop: value`, property-name validity, missing `;` recovery).
  - Unknown properties: rejected based on a profile-dependent “known properties” list
    (custom properties `--foo` are allowed).
  - Value checks:
    - Missing values.
    - CSS-wide keywords sanity (rejects mixing wide keywords with other tokens).
    - A conservative “too many tokens” check for many single-valued properties.
    - Suite-driven value validation for a limited set of properties (e.g. `color`, `background*`,
      `border*`, `outline*`, `font`, `cursor`, `content`, `counter-*`, `quotes`, `filter`, etc).
- Optional `@import` handling:
  - In “text mode”, presence of top-level `@import url(...)` yields a warning:
    `Imported style sheets are not checked.`
  - In “fetching mode”, top-level `@import` rules are resolved/fetched recursively and validated
    (with loop detection).

Configuration knobs:

- `profile`: selects the “known properties” set from `data/css_properties/*.properties`.
- `medium`: if set, warns if an `@media ... {` block doesn’t match the configured medium.
- `warning`: numeric warning threshold; `0` is the default, and `-1` suppresses all warnings.

## Vendored data files

The validator uses "known CSS properties" lists per profile, vendored under:

`data/css_properties/*.properties`

Update them with:

`python3 scripts/update_css_properties_data.py`

## Regenerate manifest

`python3 scripts/extract_autotest_manifest.py > test_results/autotest_manifest.jsonl`

## Run manifest checks (port completeness + decoding)

`cargo test -p css_inspector_suite`



# Prior work

This project builds on prior work by W3C. https://github.com/w3c/css-validator

W3C SOFTWARE NOTICE AND LICENSE

This work (and included software, documentation such as READMEs, or other related items) is being provided by the copyright holders under the following license.

License

By obtaining, using and/or copying this work, you (the licensee) agree that you have read, understood, and will comply with the following terms and conditions.

Permission to copy, modify, and distribute this software and its documentation, with or without modification, for any purpose and without fee or royalty is hereby granted, provided that you include the following on ALL copies of the software and documentation or portions thereof, including modifications:
The full text of this NOTICE in a location viewable to users of the redistributed or derivative work.
Any pre-existing intellectual property disclaimers, notices, or terms and conditions. If none exist, the W3C Software Short Notice should be included (hypertext is preferred, text is permitted) within the body of any redistributed or derivative code.
Notice of any changes or modifications to the files, including the date changes were made. (We recommend you provide URIs to the location from which the code is derived.)

Disclaimers

THIS SOFTWARE AND DOCUMENTATION IS PROVIDED "AS IS," AND COPYRIGHT HOLDERS MAKE NO REPRESENTATIONS OR WARRANTIES, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO, WARRANTIES OF MERCHANTABILITY OR FITNESS FOR ANY PARTICULAR PURPOSE OR THAT THE USE OF THE SOFTWARE OR DOCUMENTATION WILL NOT INFRINGE ANY THIRD PARTY PATENTS, COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS.

COPYRIGHT HOLDERS WILL NOT BE LIABLE FOR ANY DIRECT, INDIRECT, SPECIAL OR CONSEQUENTIAL DAMAGES ARISING OUT OF ANY USE OF THE SOFTWARE OR DOCUMENTATION.

The name and trademarks of copyright holders may NOT be used in advertising or publicity pertaining to the software without specific, written prior permission. Title to copyright in this software and any associated documentation will at all times remain with copyright holders.

Notes
This version: http://www.w3.org/Consortium/Legal/2002/copyright-software-20021231

This formulation of W3C's notice and license became active on December 31 2002. This version removes the copyright ownership notice such that this license can be used with materials other than those owned by the W3C, reflects that ERCIM is now a host of the W3C, includes references to this specific dated version of the license, and removes the ambiguous grant of "use". Otherwise, this version is the same as the previous version and is written so as to preserve the Free Software Foundation's assessment of GPL compatibility and OSI's certification under the Open Source Definition.
