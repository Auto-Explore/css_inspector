#!/usr/bin/env python3
"""
Update vendored CSS property lists used by the Rust CSS validator.

The Rust implementation needs a list of "known properties" per profile. Upstream
those come from the W3C css-validator project as *.properties files.

This script can either:
  - copy the files from a local checkout (preferred when available), or
  - download the files from the upstream GitHub repo at a specific ref (commit/tag/branch).

It writes into:
  css_inspector/data/css_properties/
"""

from __future__ import annotations

import argparse
import re
import shutil
import sys
import urllib.request
from pathlib import Path

EXPECTED_FILES = [
    "CSS1Properties.properties",
    "CSS2Properties.properties",
    "CSS21Properties.properties",
    "CSS3Properties.properties",
    "CSS3SVGProperties.properties",
    "SVGProperties.properties",
    "SVGBasicProperties.properties",
    "SVGTinyProperties.properties",
]


def repo_root() -> Path:
    here = Path(__file__).resolve()
    for p in [here.parent, *here.parents]:
        if (p / "Cargo.toml").exists() and (p / "crates").exists():
            return p
    raise RuntimeError(f"could not find repo root from {here}")


def parse_gitsubtree_css_validator_defaults(root: Path) -> tuple[str | None, str | None]:
    """
    Best-effort parse of `.gitsubtrees.yaml` to find:
      - css-validator.remote_url
      - css-validator.current_commit
    """

    p = root / ".gitsubtrees.yaml"
    if not p.exists():
        return (None, None)

    in_block = False
    remote_url = None
    current_commit = None
    for raw in p.read_text(encoding="utf-8").splitlines():
        line = raw.rstrip("\n")
        if not line.strip():
            continue
        if re.match(r"^\S", line):
            in_block = line.strip() == "css-validator:"
            continue
        if not in_block:
            continue
        m = re.match(r"^\s*remote_url:\s*(\S+)\s*$", line)
        if m:
            remote_url = m.group(1)
            continue
        m = re.match(r"^\s*current_commit:\s*([0-9a-f]{7,40})\s*$", line)
        if m:
            current_commit = m.group(1)
            continue

    return (remote_url, current_commit)


def github_raw_base_from_remote(remote_url: str) -> str:
    """
    Convert `https://github.com/<org>/<repo>.git` into
    `https://raw.githubusercontent.com/<org>/<repo>`.
    """

    remote_url = remote_url.removesuffix(".git")
    m = re.match(r"^https?://github\.com/([^/]+)/([^/]+)$", remote_url)
    if not m:
        raise ValueError(f"unsupported remote_url format: {remote_url!r}")
    org, repo = m.group(1), m.group(2)
    return f"https://raw.githubusercontent.com/{org}/{repo}"


def local_source_candidates(root: Path) -> list[Path]:
    """
    Common local checkout locations for css-validator.

    The W3C repo contains the property lists under:
      org/w3c/css/properties/*.properties
    """

    rel = Path("org") / "w3c" / "css" / "properties"
    return [
        root / "css-validator" / rel,
        root / "vendor" / "css-validator" / rel,
        root / "w3c" / "css" / "properties",  # legacy/older layout
    ]


def resolve_local_source_dir(
    root: Path, requested: Path | None
) -> tuple[Path | None, str | None]:
    """
    Resolve local source dir.

    Returns (dir, reason). If dir is None, reason is a user-facing explanation.
    """

    if requested is not None:
        if requested.exists():
            return (requested, None)
        return (None, f"missing source dir: {requested}")

    for candidate in local_source_candidates(root):
        if candidate.exists():
            return (candidate, None)
    tried = "\n".join(f"  - {p}" for p in local_source_candidates(root))
    return (None, f"no local css-validator checkout found. Tried:\n{tried}")


def copy_from_local(source_dir: Path, out_dir: Path) -> None:
    if not source_dir.exists():
        raise FileNotFoundError(f"missing source dir: {source_dir}")

    out_dir.mkdir(parents=True, exist_ok=True)
    for name in EXPECTED_FILES:
        src = source_dir / name
        if not src.exists():
            raise FileNotFoundError(f"missing source file: {src}")
        dst = out_dir / name
        shutil.copyfile(src, dst)


def download_from_upstream(raw_base: str, commit: str, out_dir: Path) -> None:
    out_dir.mkdir(parents=True, exist_ok=True)
    for name in EXPECTED_FILES:
        url = f"{raw_base}/{commit}/org/w3c/css/properties/{name}"
        dst = out_dir / name
        req = urllib.request.Request(
            url,
            headers={
                "User-Agent": "validator-rust-data-updater/1.0",
                "Accept": "text/plain,*/*",
            },
        )
        try:
            with urllib.request.urlopen(req, timeout=60) as r:  # nosec - controlled URLs
                data = r.read()
        except Exception as e:
            raise RuntimeError(
                f"failed to download {url}: {e}\n"
                "hint: if you're offline, clone https://github.com/w3c/css-validator and re-run with "
                "`--source-dir path/to/css-validator/org/w3c/css/properties`."
            ) from e
        if not data.strip():
            raise RuntimeError(f"downloaded empty file from {url}")
        dst.write_bytes(data)


def validate_out_dir(out_dir: Path) -> None:
    missing = [name for name in EXPECTED_FILES if not (out_dir / name).exists()]
    if missing:
        raise RuntimeError(f"missing output files: {missing}")
    for name in EXPECTED_FILES:
        p = out_dir / name
        if p.stat().st_size == 0:
            raise RuntimeError(f"empty output file: {p}")


def main() -> int:
    root = repo_root()
    default_out_dir = root / "data" / "css_properties"
    default_remote, default_commit = parse_gitsubtree_css_validator_defaults(root)

    ap = argparse.ArgumentParser(
        description="Update vendored CSS property lists used by the Rust CSS validator."
    )
    ap.add_argument(
        "--out-dir",
        type=Path,
        default=default_out_dir,
        help="Output directory for vendored properties (default: css_inspector/data/css_properties).",
    )
    ap.add_argument(
        "--source-dir",
        type=Path,
        default=None,
        help="Local source directory to copy from (default: auto-detect common css-validator checkouts).",
    )
    ap.add_argument(
        "--download",
        action="store_true",
        help="Download from upstream GitHub instead of copying from a local checkout.",
    )
    ap.add_argument(
        "--remote-url",
        default=default_remote or "https://github.com/w3c/css-validator.git",
        help="Upstream css-validator repo URL (default: from .gitsubtrees.yaml if available).",
    )
    ap.add_argument(
        "--commit",
        default=default_commit,
        help="Upstream ref (commit/tag/branch). Defaults to css-validator.current_commit from .gitsubtrees.yaml, else 'main'.",
    )

    args = ap.parse_args()
    out_dir = args.out_dir

    if args.download:
        ref = args.commit or "main"
        raw_base = github_raw_base_from_remote(args.remote_url)
        download_from_upstream(raw_base, ref, out_dir)
    else:
        source_dir, reason = resolve_local_source_dir(root, args.source_dir)
        if source_dir is not None:
            copy_from_local(source_dir, out_dir)
        else:
            # Keep `python3 scripts/update_css_properties_data.py` working out-of-the-box:
            # if there's no local checkout, fall back to downloading from upstream.
            ref = args.commit or "main"
            print(f"info: {reason}", file=sys.stderr)
            print(f"info: falling back to --download --commit {ref!r}", file=sys.stderr)
            raw_base = github_raw_base_from_remote(args.remote_url)
            download_from_upstream(raw_base, ref, out_dir)

    validate_out_dir(out_dir)
    print(f"ok: wrote {len(EXPECTED_FILES)} files into {out_dir}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
