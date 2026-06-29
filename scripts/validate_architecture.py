#!/usr/bin/env python3
"""Validate Spanda platform architecture — layers, dependencies, and governance rules.

Checks:
  - Every workspace crate is classified in architecture-manifest.yaml
  - No new layer dependency violations (baseline waivers allowed)
  - No new circular dependencies (baseline waivers allowed)
  - Duplicate entity model types outside spanda-config
  - Orphaned crates (no dependents and not a leaf interface)

See docs/platform-architecture.md and docs/dependency-rules.md.
"""

from __future__ import annotations

import argparse
import re
import sys
from collections import defaultdict
from dataclasses import dataclass
from pathlib import Path

try:
    import yaml
except ImportError:
    yaml = None  # type: ignore

ROOT = Path(__file__).resolve().parent.parent
MANIFEST_YAML = Path(__file__).resolve().parent / "architecture-manifest.yaml"
MANIFEST_JSON = Path(__file__).resolve().parent / "architecture-manifest.json"
CRATES_DIR = ROOT / "crates"

RE_PATH_DEP = re.compile(r'spanda-[\w-]+\s*=\s*\{\s*path\s*=\s*"\.\./([^"]+)"', re.M)
RE_PACKAGE_NAME = re.compile(r'^name\s*=\s*"([^"]+)"', re.M)
RE_STRUCT = re.compile(r"^pub struct (Entity\w+|RobotRecord|DeviceRecord|MissionRecord|FleetRecord)\b", re.M)


@dataclass
class LayerInfo:
    id: str
    index: int
    title: str


@dataclass
class Violation:
    kind: str
    message: str


def load_manifest() -> dict:
    if MANIFEST_JSON.exists():
        import json

        return json.loads(MANIFEST_JSON.read_text(encoding="utf-8"))
    if yaml is not None and MANIFEST_YAML.exists():
        return yaml.safe_load(MANIFEST_YAML.read_text(encoding="utf-8"))
    if MANIFEST_YAML.exists():
        return _load_simple_yaml(MANIFEST_YAML.read_text(encoding="utf-8"))
    raise SystemExit("No architecture manifest found (yaml or json).")


def _load_simple_yaml(text: str) -> dict:
    """Minimal YAML loader for architecture-manifest.yaml (no external deps)."""
    import json

    # Normalize block-style manifest into JSON-compatible structure via line parser.
    lines = text.splitlines()
    root: dict = {}
    stack: list[tuple[int, dict | list]] = [(0, root)]
    current_key: str | None = None
    list_item: dict | None = None

    def current_container() -> dict | list:
        return stack[-1][1]

    for raw in lines:
        if not raw.strip() or raw.strip().startswith("#"):
            continue
        indent = len(raw) - len(raw.lstrip(" "))
        line = raw.strip()

        while stack and indent < stack[-1][0]:
            finished = stack.pop()
            if isinstance(finished[1], dict) and current_key and list_item is None:
                pass
            list_item = None

        if line.startswith("- "):
            item = line[2:].strip()
            container = current_container()
            if not isinstance(container, list):
                raise ValueError(f"Expected list context for {line}")
            if ":" in item:
                obj: dict = {}
                k, v = item.split(":", 1)
                obj[k.strip()] = _parse_scalar(v.strip())
                container.append(obj)
                list_item = obj
            else:
                container.append(_parse_scalar(item))
            continue

        if ":" not in line:
            continue
        key, val = line.split(":", 1)
        key = key.strip()
        val = val.strip()
        container = current_container()

        if val == "":
            new: dict | list = {}
            if isinstance(container, dict):
                container[key] = new
            elif isinstance(container, list) and list_item is not None:
                list_item[key] = new
            stack.append((indent + 2, new))
            current_key = key
            continue

        if val.startswith("{") and val.endswith("}"):
            parsed = _parse_inline_map(val)
            if isinstance(container, dict):
                container[key] = parsed
            elif isinstance(container, list) and list_item is not None:
                list_item[key] = parsed
            continue

        if isinstance(container, dict):
            container[key] = _parse_scalar(val)
        elif isinstance(container, list) and list_item is not None:
            list_item[key] = _parse_scalar(val)

    return root


def _parse_scalar(val: str):
    if val in ("true", "false"):
        return val == "true"
    if val.startswith('"') and val.endswith('"'):
        return val[1:-1]
    if val.isdigit():
        return int(val)
    return val


def _parse_inline_map(val: str) -> dict:
    inner = val[1:-1].strip()
    out: dict = {}
    for part in inner.split(","):
        if not part.strip():
            continue
        k, v = part.split(":", 1)
        out[k.strip()] = _parse_scalar(v.strip())
    return out


def crate_dir_to_package(crate_dir: str) -> str:
    cargo = CRATES_DIR / crate_dir / "Cargo.toml"
    if not cargo.exists():
        return crate_dir
    m = RE_PACKAGE_NAME.search(cargo.read_text(encoding="utf-8"))
    return m.group(1) if m else crate_dir


def discover_crates() -> dict[str, list[str]]:
    graph: dict[str, list[str]] = {}
    for cargo in sorted(CRATES_DIR.glob("*/Cargo.toml")):
        crate_dir = cargo.parent.name
        pkg = crate_dir_to_package(crate_dir)
        text = cargo.read_text(encoding="utf-8")
        deps = RE_PATH_DEP.findall(text)
        graph[pkg] = sorted({crate_dir_to_package(d) for d in deps})
    return graph


def build_layer_map(manifest: dict) -> dict[str, int]:
    layers: dict[str, int] = {}
    for pkg, meta in manifest.get("rust_crates", {}).items():
        layer_id = meta["layer"]
        idx = next(l["index"] for l in manifest["layers"] if l["id"] == layer_id)
        layers[pkg] = idx
    return layers


def layer_title(manifest: dict, layer_id: str) -> str:
    for layer in manifest["layers"]:
        if layer["id"] == layer_id:
            return layer["title"]
    return layer_id


def normalize_cycle(cycle: list[str]) -> tuple[str, ...]:
    if not cycle or cycle[0] != cycle[-1]:
        cycle = cycle + [cycle[0]]
    body = cycle[:-1]
    if not body:
        return tuple(cycle)
    rotations = [tuple(body[i:] + body[:i] + [body[i]]) for i in range(len(body))]
    return min(rotations)


def find_scc_cycles(graph: dict[str, list[str]]) -> list[frozenset[str]]:
    """Return strongly connected components with more than one node."""
    index = 0
    stack: list[str] = []
    on_stack: set[str] = set()
    indices: dict[str, int] = {}
    lowlink: dict[str, int] = {}
    sccs: list[frozenset[str]] = []

    def strongconnect(node: str) -> None:
        nonlocal index
        indices[node] = index
        lowlink[node] = index
        index += 1
        stack.append(node)
        on_stack.add(node)

        for dep in graph.get(node, []):
            if dep not in indices:
                strongconnect(dep)
                lowlink[node] = min(lowlink[node], lowlink[dep])
            elif dep in on_stack:
                lowlink[node] = min(lowlink[node], indices[dep])

        if lowlink[node] == indices[node]:
            component: list[str] = []
            while True:
                w = stack.pop()
                on_stack.remove(w)
                component.append(w)
                if w == node:
                    break
            if len(component) > 1:
                sccs.append(frozenset(component))

    for node in sorted(graph):
        if node not in indices:
            strongconnect(node)

    return sorted(sccs, key=lambda s: sorted(s)[0])


def find_cycles(graph: dict[str, list[str]]) -> list[tuple[str, ...]]:
    """Legacy path-based cycle finder — prefer find_scc_cycles for CI."""
    cycles: list[tuple[str, ...]] = []
    seen_normalized: set[tuple[str, ...]] = set()

    def dfs(node: str, stack: list[str], on_stack: set[str]) -> None:
        if node in on_stack:
            idx = stack.index(node)
            norm = normalize_cycle(stack[idx:] + [node])
            if norm not in seen_normalized:
                seen_normalized.add(norm)
                cycles.append(norm)
            return
        if node in stack:
            return
        stack.append(node)
        on_stack.add(node)
        for dep in graph.get(node, []):
            dfs(dep, stack, on_stack)
        stack.pop()
        on_stack.remove(node)

    for n in sorted(graph):
        dfs(n, [], set())
    return sorted(cycles)


def waiver_key(from_pkg: str, to_pkg: str) -> tuple[str, str]:
    return (from_pkg, to_pkg)


def check_unclassified(manifest: dict, graph: dict[str, list[str]]) -> list[Violation]:
    classified = set(manifest.get("rust_crates", {}))
    violations: list[Violation] = []
    for pkg in graph:
        if pkg not in classified:
            violations.append(
                Violation("unclassified", f"Crate `{pkg}` is not classified in architecture-manifest.yaml")
            )
    return violations


def check_layer_violations(
    manifest: dict, graph: dict[str, list[str]], layers: dict[str, int]
) -> tuple[list[Violation], list[Violation]]:
    waivers = {
        waiver_key(w["from"], w["to"])
        for w in manifest.get("dependency_waivers", [])
    }
    violations: list[Violation] = []
    waived: list[Violation] = []

    for from_pkg, deps in sorted(graph.items()):
        from_layer = layers.get(from_pkg)
        if from_layer is None:
            continue
        for to_pkg in deps:
            to_layer = layers.get(to_pkg)
            if to_layer is None:
                continue
            if to_layer > from_layer:
                msg = (
                    f"Layer violation: `{from_pkg}` (layer {from_layer}) "
                    f"depends on `{to_pkg}` (layer {to_layer})"
                )
                if waiver_key(from_pkg, to_pkg) in waivers:
                    waived.append(Violation("layer_waiver", msg))
                else:
                    violations.append(Violation("layer_violation", msg))
    return violations, waived


def check_cycles(
    manifest: dict, graph: dict[str, list[str]]
) -> tuple[list[Violation], list[Violation]]:
    waived_cycles: set[frozenset[str]] = set()
    for w in manifest.get("circular_dependency_waivers", []):
        if "members" in w:
            waived_cycles.add(frozenset(w["members"]))
        elif "cycle" in w:
            cycle = w["cycle"]
            if cycle[0] == cycle[-1]:
                waived_cycles.add(frozenset(cycle[:-1]))
            else:
                waived_cycles.add(frozenset(cycle))

    violations: list[Violation] = []
    waived: list[Violation] = []

    for scc in find_scc_cycles(graph):
        members = ", ".join(sorted(scc))
        if scc in waived_cycles:
            waived.append(Violation("cycle_waiver", f"Known SCC (waived): {{{members}}}"))
        else:
            violations.append(
                Violation("cycle", f"Strongly connected component (cycle): {{{members}}}")
            )
    return violations, waived


def check_duplicate_entity_types(manifest: dict) -> list[Violation]:
    owner = manifest["canonical_entity_types"]["owner_crate"]
    forbidden = set(manifest["canonical_entity_types"].get("forbidden_duplicates", []))
    violations: list[Violation] = []

    for cargo in CRATES_DIR.glob("*/Cargo.toml"):
        pkg = crate_dir_to_package(cargo.parent.name)
        if pkg == owner:
            continue
        lib_rs = cargo.parent / "src" / "lib.rs"
        if not lib_rs.exists():
            continue
        text = lib_rs.read_text(encoding="utf-8", errors="replace")
        for match in RE_STRUCT.finditer(text):
            name = match.group(1)
            if name in forbidden or (name.startswith("Entity") and name not in {
                "EntityId",  # lightweight newtype aliases may exist
            }):
                violations.append(
                    Violation(
                        "duplicate_model",
                        f"`{pkg}` defines `{name}` — canonical entity types live in `{owner}`",
                    )
                )
    return violations


def check_orphaned(
    manifest: dict, graph: dict[str, list[str]], layers: dict[str, int]
) -> list[Violation]:
    optional = {crate_dir_to_package("spanda-ros2-rclrs-native")}
    optional.update(manifest.get("optional_crates", []))
    reverse: dict[str, set[str]] = defaultdict(set)
    for src, deps in graph.items():
        for dep in deps:
            reverse[dep].add(src)

    leaves = {
        pkg
        for pkg, layer in layers.items()
        if layer <= 1  # foundation/compiler leaves are expected
    }
    interface_leaves = {
        pkg for pkg, layer in layers.items() if layer >= 5
    }

    violations: list[Violation] = []
    for pkg in graph:
        if pkg in optional:
            continue
        if pkg in leaves or pkg in interface_leaves:
            continue
        if not reverse.get(pkg) and not graph.get(pkg):
            violations.append(Violation("orphan", f"Crate `{pkg}` has no dependents and no dependencies"))
    return violations


def render_graph_dot(manifest: dict, graph: dict[str, list[str]], layers: dict[str, int]) -> str:
    layer_colors = {
        0: "#e8e8e8",
        1: "#d4e4ff",
        2: "#c8f0c8",
        3: "#fff0c8",
        4: "#ffd4d4",
        5: "#e8d4ff",
        6: "#ffe8f0",
    }
    lines = ["digraph spanda_arch {", '  rankdir=TB;', '  node [shape=box, fontsize=10];', ""]
    for pkg, layer_idx in sorted(layers.items(), key=lambda x: (x[1], x[0])):
        color = layer_colors.get(layer_idx, "#ffffff")
        lines.append(f'  "{pkg}" [style=filled, fillcolor="{color}"];')
    lines.append("")
    for src, deps in sorted(graph.items()):
        for dep in deps:
            lines.append(f'  "{src}" -> "{dep}";')
    lines.append("}")
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser(description="Validate Spanda platform architecture.")
    parser.add_argument("--verbose", action="store_true", help="Print waived violations.")
    parser.add_argument("--write-graph", type=Path, help="Write dependency graph DOT file.")
    parser.add_argument(
        "--warn-orphans",
        action="store_true",
        help="Treat orphaned crates as warnings instead of errors.",
    )
    args = parser.parse_args()

    manifest = load_manifest()
    graph = discover_crates()
    layers = build_layer_map(manifest)

    all_errors: list[Violation] = []
    all_warnings: list[Violation] = []

    unclassified = check_unclassified(manifest, graph)
    all_errors.extend(unclassified)

    layer_violations, layer_waived = check_layer_violations(manifest, graph, layers)
    all_errors.extend(layer_violations)

    cycle_violations, cycle_waived = check_cycles(manifest, graph)
    all_errors.extend(cycle_violations)

    duplicates = check_duplicate_entity_types(manifest)
    all_warnings.extend(duplicates)

    orphans = check_orphaned(manifest, graph, layers)
    if args.warn_orphans:
        all_warnings.extend(orphans)
    else:
        all_warnings.extend(orphans)

    if args.write_graph:
        args.write_graph.write_text(
            render_graph_dot(manifest, graph, layers), encoding="utf-8"
        )
        print(f"Wrote dependency graph: {args.write_graph}")

    print(f"Architecture manifest v{manifest.get('version', '?')}")
    print(f"Classified crates: {len(layers)} / {len(graph)} workspace members")
    print(f"Dependency edges: {sum(len(v) for v in graph.values())}")
    print(f"Layer violations (new): {len(layer_violations)}")
    print(f"Layer violations (waived): {len(layer_waived)}")
    print(f"Circular dependencies (new): {len(cycle_violations)}")
    print(f"Circular dependencies (waived): {len(cycle_waived)}")
    print(f"Unclassified crates: {len(unclassified)}")
    print(f"Duplicate model warnings: {len(duplicates)}")
    print(f"Orphan warnings: {len(orphans)}")

    if args.verbose:
        for v in layer_waived + cycle_waived:
            print(f"  [waived] {v.message}")

    for v in all_warnings:
        print(f"  WARNING: {v.message}")

    for v in all_errors:
        print(f"  ERROR: {v.message}")

    if all_errors:
        print("\nArchitecture validation FAILED.")
        return 1

    print("\nArchitecture validation passed.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
