#!/usr/bin/env python3
"""Refresh SHA-256 checksums in registry/index.json from hosted tarballs."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
INDEX = ROOT / "registry" / "index.json"
PACKAGES = ROOT / "registry" / "packages"


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(8192), b""):
            digest.update(chunk)
    return digest.hexdigest()


def main() -> None:
    entries = json.loads(INDEX.read_text(encoding="utf-8"))
    updated = 0
    for entry in entries:
        name = entry["name"]
        checksums = entry.setdefault("version_checksums", {})
        for version in entry.get("versions", []):
            tarball = PACKAGES / name / version
            if not tarball.is_file():
                raise SystemExit(f"missing tarball: {tarball}")
            digest = sha256_file(tarball)
            if checksums.get(version) != digest:
                checksums[version] = digest
                updated += 1
            sidecar = tarball.parent / f"{version}.sha256"
            sidecar.write_text(f"{digest}\n", encoding="utf-8")
    INDEX.write_text(json.dumps(entries, indent=2) + "\n", encoding="utf-8")
    print(f"✓ updated {updated} checksum(s) in {INDEX.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
