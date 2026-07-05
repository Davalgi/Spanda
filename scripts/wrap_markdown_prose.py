#!/usr/bin/env python3
"""Wrap markdown prose to a maximum line length for readable raw diffs."""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

from markdown_line_utils import DEFAULT_MAX_LINE, iter_markdown_files, should_skip_path, wrap_markdown_text

ROOT = Path(__file__).resolve().parents[1]

DEFAULT_TARGETS = [
    ROOT / "README.md",
    ROOT / "CHANGELOG.md",
    ROOT / "ROADMAP.md",
    ROOT / "CONTRIBUTING.md",
    ROOT / "CODE_OF_CONDUCT.md",
    ROOT / "docs",
    ROOT / "examples" / "README.md",
    ROOT / "website" / "README.md",
    ROOT / "docs-site" / "src",
]


def collect_paths(targets: list[str], all_docs: bool) -> list[Path]:
    if all_docs:
        return iter_markdown_files(ROOT, ROOT)

    paths: list[Path] = []
    for target in targets:
        path = Path(target)
        if not path.is_absolute():
            path = ROOT / path
        if path.is_dir():
            paths.extend(iter_markdown_files(path, ROOT))
        elif path.is_file():
            if not should_skip_path(path, ROOT):
                paths.append(path)
    if not targets and not all_docs:
        for default in DEFAULT_TARGETS:
            if default.is_dir():
                paths.extend(iter_markdown_files(default, ROOT))
            elif default.is_file():
                if not should_skip_path(default, ROOT):
                    paths.append(default)
    return sorted(set(paths))


def main() -> int:
    parser = argparse.ArgumentParser(description="Wrap markdown prose to a maximum line length.")
    parser.add_argument("targets", nargs="*", help="Files or directories (default: key docs)")
    parser.add_argument("--width", type=int, default=DEFAULT_MAX_LINE, help="Maximum line length")
    parser.add_argument("--all-docs", action="store_true", help="Process every markdown file in the repo")
    parser.add_argument("--check", action="store_true", help="Exit 1 when changes would be made")
    args = parser.parse_args()

    changed = 0
    for path in collect_paths(args.targets, args.all_docs):
        original = path.read_text(encoding="utf-8")
        wrapped, fixes = wrap_markdown_text(original, width=args.width)
        if wrapped == original:
            continue
        rel = path.relative_to(ROOT)
        if args.check:
            print(f"would wrap: {rel} ({fixes} long line(s))")
            changed += 1
            continue
        path.write_text(wrapped, encoding="utf-8")
        print(f"wrapped: {rel} ({fixes} long line(s))")
        changed += 1

    if args.check and changed:
        print(f"\n{changed} file(s) need wrapping — run: python3 scripts/wrap_markdown_prose.py")
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
