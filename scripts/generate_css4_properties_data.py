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
import collections
import difflib
import json
from pathlib import Path
from typing import Iterable, Optional


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


def parse_level4_property_rows(all_properties_json: Path) -> list[tuple[str, str, str]]:
    rows = json.loads(all_properties_json.read_text(encoding="utf-8"))
    if not isinstance(rows, list) or not rows:
        raise RuntimeError(f"unexpected JSON shape in {all_properties_json}: expected non-empty array")

    out: list[tuple[str, str, str]] = []
    for row in rows:
        if not isinstance(row, dict):
            continue
        title = str(row.get("title", ""))
        if "Level 4" not in title:
            continue
        status = str(row.get("status", "")).strip().upper()
        if not is_status_allowed(status):
            continue
        prop = str(row.get("property", "")).strip().lower()
        if not prop or prop == "--*":
            continue
        out.append((prop, title, status))
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
# Optional filters (script flags):
# - `--min-status WD` to ignore ED/FPWD entries, etc.
# - `--include-status WD --include-status CRD` to keep a fixed set.
#
# Notes:
# - This does not imply full syntax/value validation for these properties; it
#   only avoids “Unknown property …” errors when `--profile css4` is used.
# - Some entries come from early drafts (ED/WD/FPWD) and may change; this file
#   is expected to evolve.
#
"""


STATUS_ORDER = ["NOTE", "ED", "FPWD", "WD", "CRD", "CR", "REC"]
STATUS_RANK = {s: i for i, s in enumerate(STATUS_ORDER)}


def normalize_status(s: str) -> str:
    s = s.strip().upper()
    return s


def status_gte(a: str, b: str) -> bool:
    a = normalize_status(a)
    b = normalize_status(b)
    if a not in STATUS_RANK or b not in STATUS_RANK:
        return False
    return STATUS_RANK[a] >= STATUS_RANK[b]


def status_lt(a: str, b: str) -> bool:
    a = normalize_status(a)
    b = normalize_status(b)
    if a not in STATUS_RANK or b not in STATUS_RANK:
        return False
    return STATUS_RANK[a] < STATUS_RANK[b]


def is_status_allowed(status: str) -> bool:
    status = normalize_status(status)
    if not status:
        return True
    if status not in STATUS_RANK:
        return True

    if INCLUDE_STATUSES:
        return status in INCLUDE_STATUSES
    if EXCLUDE_STATUSES and status in EXCLUDE_STATUSES:
        return False
    if MIN_STATUS:
        return status_gte(status, MIN_STATUS)
    return True


INCLUDE_STATUSES: set[str] = set()
EXCLUDE_STATUSES: set[str] = set()
MIN_STATUS: Optional[str] = None


def build_output_text(names: Iterable[str]) -> str:
    lines = [HEADER, "\n"]
    for name in names:
        lines.append(f"{name}:\n")
    return "".join(lines)


def main() -> int:
    global INCLUDE_STATUSES, EXCLUDE_STATUSES, MIN_STATUS

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
    ap.add_argument(
        "--min-status",
        type=str,
        default=None,
        choices=STATUS_ORDER,
        help="Only include properties whose status is >= this stage (default: include all).",
    )
    ap.add_argument(
        "--include-status",
        type=str,
        action="append",
        default=[],
        choices=STATUS_ORDER,
        help="Explicitly include only these statuses (may be repeated). Overrides --min-status.",
    )
    ap.add_argument(
        "--exclude-status",
        type=str,
        action="append",
        default=[],
        choices=STATUS_ORDER,
        help="Exclude these statuses (may be repeated).",
    )
    ap.add_argument(
        "--check",
        action="store_true",
        help="Exit non-zero if the output file is not up to date (does not write).",
    )
    ap.add_argument(
        "--report",
        action="store_true",
        help="Print a small summary of the generated diff (counts by status/title).",
    )
    args = ap.parse_args()

    INCLUDE_STATUSES = {normalize_status(s) for s in args.include_status}
    EXCLUDE_STATUSES = {normalize_status(s) for s in args.exclude_status}
    MIN_STATUS = None if INCLUDE_STATUSES else args.min_status

    css3 = parse_css_validator_properties(args.css3_properties)
    css3.add("color-profile")

    level4_rows = parse_level4_property_rows(args.all_properties_json)
    level4 = {prop for (prop, _, _) in level4_rows}
    missing = sorted(level4 - css3)

    if args.report:
        first_meta: dict[str, tuple[str, str]] = {}
        for prop, title, status in level4_rows:
            first_meta.setdefault(prop, (title, status))

        status_counts: collections.Counter[str] = collections.Counter()
        title_counts: collections.Counter[str] = collections.Counter()
        for prop in missing:
            title, status = first_meta.get(prop, ("", ""))
            title_counts[title or "<missing title>"] += 1
            status_counts[normalize_status(status) or "<missing status>"] += 1

        print(
            f"report: level4_props={len(level4)} css3_props={len(css3)} added={len(missing)}",
        )
        print("report: added properties by status:")
        for s in [*STATUS_ORDER, "<missing status>"]:
            c = status_counts.get(s, 0)
            if c:
                print(f"  {s}: {c}")

        top_titles = title_counts.most_common(20)
        if top_titles:
            print("report: added properties by module title (top 20):")
            for title, c in top_titles:
                print(f"  {c}: {title}")

    out_text = build_output_text(missing)
    if args.check:
        on_disk = ""
        if args.out.exists():
            on_disk = args.out.read_text(encoding="utf-8", errors="replace")
        if on_disk == out_text:
            print(
                f"ok: {args.out} is up to date (level4_props={len(level4)}, css3_props={len(css3)}, added={len(missing)})"
            )
            return 0
        diff = difflib.unified_diff(
            on_disk.splitlines(keepends=True),
            out_text.splitlines(keepends=True),
            fromfile=str(args.out),
            tofile=str(args.out),
        )
        print("".join(diff))
        return 1

    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text(out_text, encoding="utf-8")

    print(
        f"ok: wrote {args.out} (level4_props={len(level4)}, css3_props={len(css3)}, added={len(missing)})"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
