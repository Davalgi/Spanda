#!/usr/bin/env python3
"""Copy repository markdown into docs-site/src for mdBook builds."""

from __future__ import annotations

import re
import shutil
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DOCS_SITE = ROOT / "docs-site"
SUMMARY = DOCS_SITE / "src" / "SUMMARY.md"
DEST_ROOT = DOCS_SITE / "src" / "repo-docs"

LINK_RE = re.compile(r"\]\((?:\./)?repo-docs/([^)]+)\)")


def sync_sources() -> int:
    if not SUMMARY.is_file():
        print(f"missing {SUMMARY}", file=sys.stderr)
        return 1

    summary = SUMMARY.read_text(encoding="utf-8")
    rel_paths = sorted({match.group(1) for match in LINK_RE.finditer(summary)})
    if not rel_paths:
        print("no external markdown links found in SUMMARY.md")
        return 0

    if DEST_ROOT.exists():
        shutil.rmtree(DEST_ROOT)

    copied = 0
    for rel in rel_paths:
        src = ROOT / rel
        if not src.is_file():
            print(f"missing source for mdBook: {rel}", file=sys.stderr)
            return 1
        dest = DEST_ROOT / rel
        dest.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(src, dest)
        copied += 1

    print(f"synced {copied} markdown file(s) into {DEST_ROOT.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    sys.exit(sync_sources())
