#!/usr/bin/env python3
"""Fail CI when markdown prose exceeds the configured line length."""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

from markdown_line_utils import (
    DEFAULT_MAX_LINE,
    PROSE_VALIDATION_MAX,
    iter_markdown_files,
    line_violations,
)

ROOT = Path(__file__).resolve().parents[1]

DEFAULT_SCOPE = [
    ROOT / "README.md",
    ROOT / "CHANGELOG.md",
    ROOT / "ROADMAP.md",
    ROOT / "CONTRIBUTING.md",
    ROOT / "docs",
]


def main() -> int:
    parser = argparse.ArgumentParser(description="Validate markdown line lengths.")
    parser.add_argument("--max-line", type=int, default=PROSE_VALIDATION_MAX, help="Maximum prose line length")
    parser.add_argument(
        "--all-docs",
        action="store_true",
        help="Check every markdown file instead of the default documentation scope",
    )
    args = parser.parse_args()

    paths = iter_markdown_files(ROOT, ROOT) if args.all_docs else []
    if not args.all_docs:
        for target in DEFAULT_SCOPE:
            if target.is_dir():
                paths.extend(iter_markdown_files(target, ROOT))
            else:
                paths.append(target)
        paths = sorted(set(paths))

    failures = 0
    for path in paths:
        text = path.read_text(encoding="utf-8")
        violations = line_violations(text, max_line=args.max_line)
        if not violations:
            continue
        rel = path.relative_to(ROOT)
        failures += len(violations)
        print(f"{rel}: {len(violations)} line(s) exceed {args.max_line} characters")
        for line_no, length, preview in violations[:5]:
            print(f"  L{line_no} ({length} chars): {preview}...")
        if len(violations) > 5:
            print(f"  ... and {len(violations) - 5} more")

    if failures:
        print(
            f"\n{failures} markdown line-length violation(s). "
            "Run: python3 scripts/wrap_markdown_prose.py"
        )
        return 1

    print(f"Markdown line-length check passed ({args.max_line} char limit)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
