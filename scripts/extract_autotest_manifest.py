#!/usr/bin/env python3
import html as html_mod
import json
import re
import sys
from dataclasses import dataclass, asdict
from pathlib import Path
from typing import Optional, Dict, Any, List, Tuple
from urllib.parse import urlparse, parse_qs


def repo_root() -> Path:
    here = Path(__file__).resolve()
    for p in [here.parent, *here.parents]:
        if (p / "Cargo.toml").exists() and (p / "crates").exists():
            return p
    raise RuntimeError(f"could not find repo root from {here}")


ROOT = repo_root()


@dataclass
class Expected:
    valid: bool
    errors: int
    warnings: int


@dataclass
class Input:
    kind: str  # "text" | "uri"
    text: Optional[str] = None
    uri: Optional[str] = None


@dataclass
class Config:
    profile: Optional[str] = None
    medium: Optional[str] = None
    warning: Optional[str] = None


@dataclass
class Source:
    file: str
    index: int
    section: Optional[str] = None


@dataclass
class Case:
    id: str
    source: Source
    validator_url: str
    input: Input
    config: Config
    expected: Optional[Expected] = None
    observed: Optional[Expected] = None
    status: str = "ok"  # ok | mismatch | error
    note: Optional[str] = None


RE_H2 = re.compile(r"<h2>(.*?)</h2>", re.IGNORECASE | re.DOTALL)
RE_DIV = re.compile(r'<div class="(res|error)">(.*?)</div>', re.IGNORECASE | re.DOTALL)
RE_H3_LINK = re.compile(r"<h3>\s*<a[^>]*>(.*?)</a>\s*</h3>", re.IGNORECASE | re.DOTALL)
RE_VALIDATOR_LINK = re.compile(
    r'<a href="([^"]*?/css-validator/validator\?[^"]+)">\s*Go to the Validator page\s*</a>',
    re.IGNORECASE | re.DOTALL,
)
RE_DT = re.compile(r"<dt>\s*(.*?)\s*</dt>", re.IGNORECASE | re.DOTALL)
RE_DD = re.compile(r"<dd>\s*(.*?)\s*</dd>", re.IGNORECASE | re.DOTALL)


def strip_tags(s: str) -> str:
    s = re.sub(r"<br\\s*/?>", "\n", s, flags=re.IGNORECASE)
    s = re.sub(r"<[^>]+>", "", s)
    return html_mod.unescape(s).strip()


def parse_validity_word(word: str) -> Optional[bool]:
    w = word.strip().lower()
    if w == "valid":
        return True
    if w == "not valid":
        return False
    return None


def parse_counts(dds: List[str], start: int) -> Tuple[Optional[Expected], Optional[str]]:
    if start + 2 >= len(dds):
        return None, "missing dd entries for validity/errors/warnings"
    valid = parse_validity_word(dds[start])
    if valid is None:
        return None, f"unexpected validity string: {dds[start]!r}"
    m_err = re.match(r"Errors:\s*(\d+)\s*$", dds[start + 1])
    m_warn = re.match(r"Warnings:\s*(\d+)\s*$", dds[start + 2])
    if not m_err or not m_warn:
        return None, f"unexpected counts strings: {dds[start+1]!r}, {dds[start+2]!r}"
    return Expected(valid=valid, errors=int(m_err.group(1)), warnings=int(m_warn.group(1))), None


def parse_dl(block_html: str) -> Tuple[Optional[Expected], Optional[Expected], Optional[str]]:
    dts = [strip_tags(x) for x in RE_DT.findall(block_html)]
    dds = [strip_tags(x) for x in RE_DD.findall(block_html)]

    # The expected structure is:
    # dt "Awaited result" then 3 dd, dt "Result" then 3 dd.
    # Some blocks may not have a dl at all.
    if not dts and not dds:
        return None, None, "missing <dl> with awaited/result"

    # Locate dt positions by walking the original HTML, because dt/dd are not paired.
    # Approximate by: the first dt is awaited, second is result; dd order follows.
    awaited_pos = None
    result_pos = None
    for i, dt in enumerate(dts):
        if awaited_pos is None and dt.lower() == "awaited result":
            awaited_pos = i
        elif result_pos is None and dt.lower() == "result":
            result_pos = i

    # Fallback: assume order.
    if awaited_pos is None or result_pos is None:
        awaited_pos = 0
        result_pos = 1 if len(dts) > 1 else None

    if result_pos is None:
        return None, None, "missing Result section"

    # In these pages, dd list is exactly 6 entries.
    if len(dds) < 6:
        return None, None, f"expected at least 6 <dd> entries, got {len(dds)}"

    awaited, err1 = parse_counts(dds, 0)
    observed, err2 = parse_counts(dds, 3)
    if err1 or err2:
        return None, None, err1 or err2
    return awaited, observed, None


def parse_validator_url(raw_href: str) -> Tuple[str, Dict[str, str], Input]:
    href = html_mod.unescape(raw_href)
    parsed = urlparse(href)
    qs = parse_qs(parsed.query, keep_blank_values=True)
    params: Dict[str, str] = {k: v[-1] if v else "" for (k, v) in qs.items()}

    if "text" in params:
        # `parse_qs` already decodes percent-escapes and `+` into spaces once.
        # Do not decode twice: some test inputs intentionally contain sequences like `%23`.
        return href, params, Input(kind="text", text=params["text"])
    if "uri" in params:
        return href, params, Input(kind="uri", uri=params["uri"])
    return href, params, Input(kind="unknown")


def extract_cases(html_path: Path) -> List[Case]:
    raw = html_path.read_text(encoding="utf-8", errors="replace")
    cases: List[Case] = []
    current_section: Optional[str] = None

    # Track sections by scanning the file linearly.
    pos = 0
    index = 0
    while True:
        m_h2 = RE_H2.search(raw, pos)
        m_div = RE_DIV.search(raw, pos)
        if not m_div:
            break
        if m_h2 and m_h2.start() < m_div.start():
            current_section = strip_tags(m_h2.group(1))
            pos = m_h2.end()
            continue

        kind = m_div.group(1).lower()
        block = m_div.group(2)
        pos = m_div.end()

        # Only res blocks contribute a runnable case; error blocks are recorded for completeness.
        h3 = RE_H3_LINK.search(block)
        if not h3:
            continue
        case_id = strip_tags(h3.group(1))

        vlink = RE_VALIDATOR_LINK.search(block)
        if not vlink:
            # Still record the case for traceability.
            cases.append(
                Case(
                    id=case_id,
                    source=Source(file=str(html_path.relative_to(ROOT)), index=index, section=current_section),
                    validator_url="",
                    input=Input(kind="unknown"),
                    config=Config(),
                    expected=None,
                    observed=None,
                    status="error",
                    note="missing validator link",
                )
            )
            index += 1
            continue

        validator_url, params, inp = parse_validator_url(vlink.group(1))
        cfg = Config(
            profile=params.get("profile"),
            medium=params.get("medium"),
            warning=params.get("warning"),
        )

        if kind == "error":
            cases.append(
                Case(
                    id=case_id,
                    source=Source(file=str(html_path.relative_to(ROOT)), index=index, section=current_section),
                    validator_url=validator_url,
                    input=inp,
                    config=cfg,
                    expected=None,
                    observed=None,
                    status="error",
                    note="autotest error block (no awaited/result counts)",
                )
            )
            index += 1
            continue

        awaited, observed, err = parse_dl(block)
        status = "ok"
        note = None
        if err:
            status = "error"
            note = err
        elif awaited != observed:
            status = "mismatch"
            note = "awaited result differs from recorded result in HTML"

        cases.append(
            Case(
                id=case_id,
                source=Source(file=str(html_path.relative_to(ROOT)), index=index, section=current_section),
                validator_url=validator_url,
                input=inp,
                config=cfg,
                expected=awaited,
                observed=observed,
                status=status,
                note=note,
            )
        )
        index += 1

    return cases


def main() -> int:
    results_dir = ROOT / "test_results"
    html_files = sorted(results_dir.glob("*.html"))
    if not html_files:
        print(f"no html files found in {results_dir}", file=sys.stderr)
        return 2

    all_cases: List[Case] = []
    for f in html_files:
        all_cases.extend(extract_cases(f))

    # Emit JSONL on stdout.
    for c in all_cases:
        obj: Dict[str, Any] = asdict(c)
        print(json.dumps(obj, ensure_ascii=False))

    ok = sum(1 for c in all_cases if c.status == "ok")
    mismatch = sum(1 for c in all_cases if c.status == "mismatch")
    err = sum(1 for c in all_cases if c.status == "error")
    print(
        f"# summary: total={len(all_cases)} ok={ok} mismatch={mismatch} error={err}",
        file=sys.stderr,
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
