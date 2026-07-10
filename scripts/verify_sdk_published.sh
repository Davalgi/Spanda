#!/usr/bin/env bash
# Verify official SDK 0.5.9+ packages are published on crates.io, PyPI, and npm.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

read_version() {
  grep '^version' "$ROOT/crates/spanda-sdk/Cargo.toml" | head -1 | sed 's/.*"\(.*\)".*/\1/'
}

VERSION="${1:-$(read_version)}"
CRATE="spanda-sdk"
PYPI="spanda-sdk"
NPM="@davalgi-spanda/sdk"

echo "== Verifying published SDK ${VERSION} =="

echo "== crates.io =="
CRATE_JSON="$(curl -fsSL -H 'User-Agent: spanda-sdk-verify/1.0' "https://crates.io/api/v1/crates/${CRATE}/${VERSION}")"
echo "$CRATE_JSON" | python3 -c 'import json,sys; d=json.load(sys.stdin); assert d["version"]["num"], d'
echo "crates.io: ${CRATE} ${VERSION} OK"

echo "== PyPI =="
PYPI_JSON="$(curl -fsSL "https://pypi.org/pypi/${PYPI}/json")"
echo "$PYPI_JSON" | python3 -c "import json,sys; v=sys.argv[1]; d=json.load(sys.stdin); assert v in d['releases'], d['info']['version']" "$VERSION"
echo "PyPI: ${PYPI} ${VERSION} OK"

echo "== npm =="
NPM_VERSION="$(npm view "${NPM}@${VERSION}" version 2>/dev/null || true)"
[[ "$NPM_VERSION" == "$VERSION" ]] || {
  echo "npm: expected ${VERSION}, got ${NPM_VERSION:-missing}" >&2
  exit 1
}
echo "npm: ${NPM} ${VERSION} OK"

echo "Published SDK ${VERSION} verified on all three registries."
