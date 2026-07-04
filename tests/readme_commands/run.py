#!/usr/bin/env python3
"""README command smoke tests and golden-output comparison."""

from __future__ import annotations

import json
import os
import re
import subprocess
import sys
from pathlib import Path


def parse_manifest(path: Path) -> list[dict]:
    # Parse the lightweight TOML command manifest without external deps.
    text = path.read_text(encoding="utf-8")
    blocks = re.split(r"\n\[\[command\]\]\n", text)
    commands: list[dict] = []
    for block in blocks[1:]:
        mid = re.search(r'^id\s*=\s*"([^"]+)"', block, re.M)
        args_m = re.search(r"^args\s*=\s*(\[[^\]]+\])", block, re.M)
        markers_m = re.search(r"^markers\s*=\s*(\[[^\]]+\])", block, re.M)
        golden_m = re.search(r"^golden\s*=\s*(true|false)", block, re.M)
        if not mid or not args_m:
            continue
        commands.append(
            {
                "id": mid.group(1),
                "args": json.loads(args_m.group(1).replace("'", '"')),
                "markers": json.loads(markers_m.group(1).replace("'", '"'))
                if markers_m
                else [],
                "golden": golden_m.group(1) == "true" if golden_m else False,
            }
        )
    return commands


def normalize(text: str, root: str) -> str:
    # Replace absolute repo paths so goldens are portable across machines.
    text = text.replace(root, "<ROOT>")
    text = re.sub(
        r"/var/folders/[^\s]+/cursor-sandbox-cache/[^\s]+/cargo-target",
        "<TARGET>",
        text,
    )
    lines = text.splitlines()
    # Pull warning lines into a stable sorted trailer (install order varies).
    warnings = sorted(line for line in lines if "⚠" in line)
    body = [line for line in lines if "⚠" not in line]
    # Sort equally ranked recovery strategy lines for stable goldens.
    strategy_re = re.compile(r"^  .+ — \d+(?:\.\d+)?% success")
    strategies = sorted(line for line in body if strategy_re.match(line))
    if strategies:
        body = [line for line in body if not strategy_re.match(line)]
        # Re-insert strategies after the "Most effective strategies:" header when present.
        inserted = False
        rebuilt: list[str] = []
        for line in body:
            rebuilt.append(line)
            if line.strip() == "Most effective strategies:":
                rebuilt.extend(strategies)
                inserted = True
        if not inserted:
            rebuilt.extend(strategies)
        body = rebuilt
    if warnings:
        body.extend(warnings)
    # Sort contiguous JSON string-array items (order can vary across runs).
    body = _sort_json_string_array_blocks(body)
    text = "\n".join(body)
    text = re.sub(r"\n{3,}", "\n\n", text)
    return text.rstrip() + "\n"


def _sort_json_string_array_blocks(lines: list[str]) -> list[str]:
    # Stabilize JSON string arrays whose element order is non-deterministic.
    item_re = re.compile(r'^(\s*)"(.*)"(,)?$')
    result: list[str] = []
    index = 0
    while index < len(lines):
        match = item_re.match(lines[index])
        if not match:
            result.append(lines[index])
            index += 1
            continue
        indent = match.group(1)
        block: list[str] = []
        while index < len(lines):
            next_match = item_re.match(lines[index])
            if not next_match or next_match.group(1) != indent:
                break
            block.append(lines[index])
            index += 1
        if len(block) < 2:
            result.extend(block)
            continue
        contents = sorted(item_re.match(line).group(2) for line in block)
        for offset, content in enumerate(contents):
            comma = "," if offset < len(contents) - 1 else ""
            result.append(f'{indent}"{content}"{comma}')
    return result


def main() -> int:
    root = Path(os.environ["ROOT"])
    spanda = os.environ["SPANDA"]
    mode = os.environ.get("MODE", "smoke")
    update = os.environ.get("SPANDA_UPDATE_GOLDENS") == "1"
    manifest = root / "tests/readme_commands/commands.toml"
    golden_dir = root / "tests/readme_commands/golden"
    commands = parse_manifest(manifest)

    failed = 0
    for cmd in commands:
        cmd_id = cmd["id"]
        args = cmd["args"]
        print(f"== {cmd_id}: {Path(spanda).name} {' '.join(args)} ==")
        proc = subprocess.run(
            [spanda, *args],
            cwd=root,
            capture_output=True,
            text=True,
        )
        out = (proc.stdout or "") + (proc.stderr or "")
        if proc.returncode != 0:
            print(f"FAIL {cmd_id}: exit {proc.returncode}")
            print("\n".join(out.splitlines()[:40]))
            failed += 1
            continue
        missing = [m for m in cmd["markers"] if m not in out]
        if missing:
            print(f"FAIL {cmd_id}: missing marker(s) {missing}")
            print("\n".join(out.splitlines()[:40]))
            failed += 1
            continue

        if mode == "golden" and cmd["golden"]:
            golden_dir.mkdir(parents=True, exist_ok=True)
            golden_file = golden_dir / f"{cmd_id}.stdout"
            normalized = normalize(out, str(root))
            if update:
                golden_file.write_text(normalized, encoding="utf-8")
                print(f"updated {golden_file}")
            elif not golden_file.is_file():
                print(f"FAIL {cmd_id}: missing golden {golden_file}")
                failed += 1
            else:
                expected = golden_file.read_text(encoding="utf-8")
                if normalized != expected:
                    print(f"FAIL {cmd_id}: golden mismatch")
                    import difflib

                    for line in difflib.unified_diff(
                        expected.splitlines(keepends=True),
                        normalized.splitlines(keepends=True),
                        fromfile="golden",
                        tofile="actual",
                        n=2,
                    ):
                        sys.stdout.write(line)
                    failed += 1
                else:
                    print(f"ok {cmd_id} (golden)")
        else:
            print(f"ok {cmd_id}")

    total = len(commands)
    if failed:
        print(f"\nREADME command tests FAILED: {failed}/{total}")
        return 1
    print(f"\nREADME command tests OK: {total}/{total} ({mode})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
