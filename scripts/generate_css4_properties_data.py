#!/usr/bin/env python3
"""
Generate data/css_properties/CSS4Properties.properties for the `css4` profile.

This script is intentionally offline: it uses already-vendored reference data:
  - data/w3c/all-properties.en.json (see scripts/update_w3c_all_properties_data.py)
  - data/css_properties/CSS3Properties.properties (see scripts/update_css_properties_data.py)

The generated file contains property names that appear in “Level 4” module specs
but are not present in the css-validator CSS3 profile list.

Notes:
  - `css_inspector` also treats `color-profile` as present in the CSS3 profile via
    a code-level override, so this script subtracts it as well.
  - This is only name allowlisting; it does not imply value validation support.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path


def repo_root() -> Path:
    here = Path(__file__).resolve()
    for p in [here.parent, *here.parents]:
        if (p / "Cargo.toml").exists() and (p / "crates").exists():
            return p
    raise RuntimeError(f"could not find repo root from {here}")


def parse_css_validator_properties(path: Path) -> set[str]:
    """
    Parse css-validator-style *.properties files.

    This mirrors the parsing logic in crates/css_inspector/src/known_properties.rs.
    """

    out: set[str] = set()
    text = path.read_text(encoding="utf-8", errors="replace")
    for raw in text.splitlines():
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        if ":" in line:
            name = line.split(":", 1)[0].strip()
        else:
            parts = line.split()
            if len(parts) < 2:
                continue
            name = parts[0].strip()
        if name:
            out.add(name.lower())
    return out


def parse_level4_properties(all_properties_json: Path) -> set[str]:
    rows = json.loads(all_properties_json.read_text(encoding="utf-8"))
    if not isinstance(rows, list) or not rows:
        raise RuntimeError(f"unexpected JSON shape in {all_properties_json}: expected non-empty array")

    out: set[str] = set()
    for row in rows:
        if not isinstance(row, dict):
            continue
        title = str(row.get("title", ""))
        if "Level 4" not in title:
            continue
        prop = str(row.get("property", "")).strip().lower()
        if not prop or prop == "--*":
            continue
        out.add(prop)
    return out


HEADER = """# CSS4Properties.properties
#
# This file is a css_inspector-local supplement used by the `css4` profile.
#
# It is intentionally minimal: it contains only property *names* that appear in
# W3C “module Level 4” specs but are not present in the css-validator CSS3
# profile property list vendored in `CSS3Properties.properties`.
#
# Source reference data:
#   - data/w3c/all-properties.en.json
#
# Generation (conceptual):
#   1) Take all entries whose `title` contains “Level 4”
#   2) Extract unique `property` names (excluding `--*`)
#   3) Subtract the set from `CSS3Properties.properties` (plus `color-profile`)
#
# Notes:
# - This does not imply full syntax/value validation for these properties; it
#   only avoids “Unknown property …” errors when `--profile css4` is used.
# - Some entries come from early drafts (ED/WD/FPWD) and may change; this file
#   is expected to evolve.
#
"""


def main() -> int:
    root = repo_root()
    ap = argparse.ArgumentParser(
        description="Generate data/css_properties/CSS4Properties.properties for the css4 profile."
    )
    ap.add_argument(
        "--all-properties-json",
        type=Path,
        default=(root / "data" / "w3c" / "all-properties.en.json"),
        help="Path to W3C all-properties.en.json (default: data/w3c/all-properties.en.json).",
    )
    ap.add_argument(
        "--css3-properties",
        type=Path,
        default=(root / "data" / "css_properties" / "CSS3Properties.properties"),
        help="Path to CSS3Properties.properties (default: data/css_properties/CSS3Properties.properties).",
    )
    ap.add_argument(
        "--out",
        type=Path,
        default=(root / "data" / "css_properties" / "CSS4Properties.properties"),
        help="Output file path (default: data/css_properties/CSS4Properties.properties).",
    )
    args = ap.parse_args()

    css3 = parse_css_validator_properties(args.css3_properties)
    css3.add("color-profile")

    level4 = parse_level4_properties(args.all_properties_json)
    missing = sorted(level4 - css3)

    args.out.parent.mkdir(parents=True, exist_ok=True)
    lines = [HEADER, "\n"]
    for name in missing:
        lines.append(f"{name}:\n")
    args.out.write_text("".join(lines), encoding="utf-8")

    print(
        f"ok: wrote {args.out} (level4_props={len(level4)}, css3_props={len(css3)}, added={len(missing)})"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

