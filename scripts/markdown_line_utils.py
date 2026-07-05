#!/usr/bin/env python3
"""Shared helpers for markdown prose wrapping and line-length validation."""

from __future__ import annotations

import re
import textwrap
from pathlib import Path

DEFAULT_MAX_LINE = 100
TABLE_MAX_LINE = 400
URL_LINE_MAX = 200
PROSE_VALIDATION_MAX = 120

# Generated or machine-owned markdown — skip wrapping and strict line checks.
SKIP_PATH_PARTS = {
    ".git",
    "node_modules",
    "target",
    "dist",
    "build",
    "book",
    ".next",
    "coverage",
}
SKIP_RELATIVE_FILES = {
    "docs/api-reference.md",
    "docs/spanda-reference.md",
    "docs/documentation-coverage.md",
}
SKIP_PATH_PREFIXES = (
    "crates/spanda-cli/bundled-examples/",
    "docs-site/book/",
)

TABLE_ROW = re.compile(r"^\s*\|")
HTML_LINE = re.compile(r"^\s*<")
HEADING = re.compile(r"^\s{0,3}#{1,6}\s")
LIST_ITEM = re.compile(r"^(\s*)([-*+]|\d+\.)\s+(.*)$")
BLOCKQUOTE = re.compile(r"^(\s*(?:> ?)+)(.*)$")
HR = re.compile(r"^\s*([-*_])(\s*\1){2,}\s*$")


ROOT = Path(__file__).resolve().parents[1]


def should_skip_path(path: Path, repo_root: Path | None = None) -> bool:
    base = repo_root or ROOT
    rel = path.relative_to(base).as_posix()
    if rel in SKIP_RELATIVE_FILES:
        return True
    if any(rel.startswith(prefix) for prefix in SKIP_PATH_PREFIXES):
        return True
    return any(part in SKIP_PATH_PARTS for part in path.parts)


def iter_markdown_files(root: Path, repo_root: Path | None = None) -> list[Path]:
    base = repo_root or ROOT
    files: list[Path] = []
    for path in sorted(root.rglob("*.md")):
        if should_skip_path(path, base):
            continue
        files.append(path)
    return files


def is_fence_line(line: str) -> bool:
    return line.lstrip().startswith("```")


def should_leave_line_unwrapped(line: str, in_code: bool) -> bool:
    if in_code:
        return True
    stripped = line.strip()
    if not stripped:
        return True
    if is_fence_line(line):
        return True
    if HEADING.match(line):
        return True
    if HR.match(stripped):
        return True
    if TABLE_ROW.match(line):
        return True
    if HTML_LINE.match(line):
        return True
    return False


def wrap_plain_text(text: str, width: int, prefix: str = "", subsequent_prefix: str | None = None) -> list[str]:
    if subsequent_prefix is None:
        subsequent_prefix = prefix
    if len(text) <= width:
        return [prefix + text] if prefix else [text]
    wrapper = textwrap.TextWrapper(
        width=width,
        initial_indent=prefix,
        subsequent_indent=subsequent_prefix,
        break_long_words=False,
        break_on_hyphens=False,
    )
    return wrapper.wrap(text)


def wrap_markdown_line(line: str, width: int) -> list[str]:
    raw = line.rstrip("\n")
    if should_leave_line_unwrapped(raw, False):
        return [line]

    list_match = LIST_ITEM.match(raw)
    if list_match:
        indent, marker, content = list_match.groups()
        bullet_prefix = f"{indent}{marker} "
        cont_prefix = indent + " " * len(f"{marker} ")
        wrapped = wrap_plain_text(
            content,
            width=width,
            prefix=bullet_prefix,
            subsequent_prefix=cont_prefix,
        )
        return [part + "\n" for part in wrapped]

    quote_match = BLOCKQUOTE.match(raw)
    if quote_match:
        quote_prefix, content = quote_match.groups()
        wrapped = wrap_plain_text(
            content,
            width=width,
            prefix=quote_prefix,
            subsequent_prefix=quote_prefix,
        )
        return [part + "\n" for part in wrapped]

    wrapped = wrap_plain_text(raw, width=width)
    return [part + "\n" for part in wrapped]


def wrap_markdown_text(text: str, width: int = DEFAULT_MAX_LINE) -> tuple[str, int]:
    out: list[str] = []
    in_code = False
    fixes = 0

    for line in text.splitlines(keepends=True):
        raw = line.rstrip("\n")
        ending = "\n" if line.endswith("\n") else ""

        if is_fence_line(raw):
            in_code = not in_code
            out.append(line)
            continue
        if in_code or should_leave_line_unwrapped(raw, False):
            out.append(line)
            continue

        if len(raw) <= width:
            out.append(line)
            continue

        wrapped = wrap_markdown_line(raw, width)
        fixes += 1
        out.extend(wrapped if wrapped[-1].endswith("\n") else [wrapped[-1] + ending])
        if not wrapped[-1].endswith("\n") and ending:
            out[-1] = out[-1].rstrip("\n") + ending

    result = "".join(out)
    if text.endswith("\n") and not result.endswith("\n"):
        result += "\n"
    return result, fixes


def line_violations(
    text: str,
    max_line: int = DEFAULT_MAX_LINE,
    table_max_line: int = TABLE_MAX_LINE,
    url_max_line: int = URL_LINE_MAX,
) -> list[tuple[int, int, str]]:
    violations: list[tuple[int, int, str]] = []
    in_code = False
    for index, line in enumerate(text.splitlines(), start=1):
        raw = line.rstrip("\n")
        if is_fence_line(raw):
            in_code = not in_code
            continue
        if in_code:
            continue
        if not raw.strip():
            continue
        if HEADING.match(raw):
            if len(raw) > max_line:
                violations.append((index, len(raw), raw[:80]))
            continue
        if HR.match(raw.strip()) or HTML_LINE.match(raw):
            continue

        limit = max_line
        if TABLE_ROW.match(raw):
            limit = table_max_line
        elif "http://" in raw or "https://" in raw:
            limit = url_max_line

        if len(raw) > limit:
            violations.append((index, len(raw), raw[:80]))
    return violations
