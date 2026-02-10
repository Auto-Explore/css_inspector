#!/usr/bin/env python3
"""
Fetch the W3C CSSWG "all properties" list (supplementary reference data).

Upstream URL:
  https://www.w3.org/Style/CSS/all-properties.en.json

This file is not a drop-in replacement for the W3C css-validator profile
property lists (see css_inspector/data/css_properties/*.properties),
but it can be useful as a supplementary "what properties exist in specs" input.
"""

from __future__ import annotations

import argparse
import json
import urllib.request
from pathlib import Path
from typing import Any


DEFAULT_URL = "https://www.w3.org/Style/CSS/all-properties.en.json"


def repo_root() -> Path:
    here = Path(__file__).resolve()
    for p in [here.parent, *here.parents]:
        if (p / "Cargo.toml").exists() and (p / "crates").exists():
            return p
    raise RuntimeError(f"could not find repo root from {here}")


def download(url: str) -> bytes:
    req = urllib.request.Request(
        url,
        headers={
            "User-Agent": "validator-rust-data-updater/1.0",
            "Accept": "application/json,*/*",
        },
    )
    with urllib.request.urlopen(req, timeout=60) as r:  # nosec - controlled URL
        data = r.read()
    if not data.strip():
        raise RuntimeError(f"downloaded empty response from {url}")
    return data


def validate_json(data: bytes, *, url: str) -> list[dict[str, Any]]:
    try:
        obj = json.loads(data)
    except Exception as e:
        raise RuntimeError(f"{url} did not return valid JSON: {e}") from e
    if not isinstance(obj, list) or not obj:
        raise RuntimeError(f"unexpected JSON shape from {url}: expected non-empty array")
    bad = 0
    for row in obj:
        if not isinstance(row, dict):
            bad += 1
            continue
        prop = row.get("property")
        if not isinstance(prop, str) or not prop.strip():
            bad += 1
            continue
    if bad:
        raise RuntimeError(f"unexpected entries in JSON from {url}: bad_entries={bad}")
    return obj


def count_unique_properties(rows: list[dict[str, Any]]) -> int:
    props = set()
    for r in rows:
        props.add(str(r.get("property", "")).strip().lower())
    props.discard("")
    return len(props)


def main() -> int:
    root = repo_root()
    default_out = (
        root / "data" / "w3c" / "all-properties.en.json"
    )

    ap = argparse.ArgumentParser(
        description="Download W3C CSSWG all-properties.en.json (supplementary reference data)."
    )
    ap.add_argument(
        "--url",
        default=DEFAULT_URL,
        help=f"Source URL (default: {DEFAULT_URL}).",
    )
    ap.add_argument(
        "--out",
        type=Path,
        default=default_out,
        help="Output path (default: css_inspector/data/w3c/all-properties.en.json).",
    )
    args = ap.parse_args()

    data = download(args.url)
    rows = validate_json(data, url=args.url)

    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_bytes(data)

    uniq = count_unique_properties(rows)
    print(f"ok: wrote {args.out} (bytes={len(data)}, entries={len(rows)}, unique_props={uniq})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
