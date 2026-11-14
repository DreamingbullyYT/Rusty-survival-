#!/usr/bin/env python3
"""Read-only heuristic duplicate and responsibility-overlap scanner for Rust."""
from __future__ import annotations

import argparse
import hashlib
import json
import re
import tomllib
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SKIP_DIRS = {".git", "target", "vendor", "third_party", "build", "dist", "node_modules", "launcher", "xtask"}
KEYWORDS = set("""
as async await break const continue crate dyn else enum extern false fn for if impl in
let loop match mod move mut pub ref return self Self static struct super trait true type
unsafe use where while yield
""".split())
TOKEN_RE = re.compile(
    r'"(?:\\.|[^"\\])*"|\'(?:\\.|[^\'\\])*\'|'
    r'\b\d+(?:\.\d+)?\b|[A-Za-z_][A-Za-z0-9_]*|'
    r'::|->|=>|==|!=|<=|>=|&&|\|\||\.\.|[^\s]'
)
FN_RE = re.compile(r'\bfn\s+([A-Za-z_][A-Za-z0-9_]*)\s*(?:<[^;{}]*?>)?\s*\(')


def rust_files(root: Path, scope: str = "engine"):
    """Yield production Rust files in the requested scope.

    Engine scope is driven by the upstream tools/layers.json registry. It includes
    each engine crate's src/ tree and crate-level build.rs, but excludes standalone
    tests, examples, benches and fixtures. Inline #[cfg(test)] modules are still
    part of their source file and are therefore a documented scanner limitation.
    """
    if scope == "all":
        for path in root.rglob("*.rs"):
            if not any(part in SKIP_DIRS for part in path.parts):
                yield path
        return

    config_path = root / "tools" / "layers.json"
    try:
        config = json.loads(config_path.read_text(encoding="utf-8"))
        engine_crates = sorted(
            name for name, layer in config["crates"].items() if layer == "engine"
        )
    except (OSError, json.JSONDecodeError, KeyError, TypeError) as exc:
        raise ValueError(f"Cannot load engine crate scope from {config_path}: {exc}") from exc

    excluded = {"tests", "benches", "examples", "fixtures", "testdata"}
    for crate in engine_crates:
        crate_root = root / "crates" / crate
        if not crate_root.is_dir():
            continue
        for path in crate_root.rglob("*.rs"):
            relative = path.relative_to(crate_root)
            if any(part in excluded or part in SKIP_DIRS for part in relative.parts):
                continue
            if relative.parts[0] == "src" or relative.as_posix() == "build.rs":
                yield path


def mask_rust(source: str, mask_literals: bool):
    """Mask comments (including nested block comments) and optionally literals."""
    out = list(source)
    i, n = 0, len(source)

    def blank(start, end):
        for k in range(start, end):
            if out[k] != "\n":
                out[k] = " "

    while i < n:
        # Raw strings: r"...", r#"..."#, br#"..."#
        raw = re.match(r'(?:br|r)(#*)"', source[i:])
        if raw:
            hashes = raw.group(1)
            start = i
            terminator = '"' + hashes
            end_at = source.find(terminator, i + raw.end())
            i = n if end_at < 0 else end_at + len(terminator)
            if mask_literals:
                blank(start, i)
            continue
        if source.startswith("//", i):
            end = source.find("\n", i)
            end = n if end < 0 else end
            blank(i, end)
            i = end
            continue
        if source.startswith("/*", i):
            start, depth = i, 1
            i += 2
            while i < n and depth:
                if source.startswith("/*", i):
                    depth += 1
                    i += 2
                elif source.startswith("*/", i):
                    depth -= 1
                    i += 2
                else:
                    i += 1
            blank(start, i)
            continue
        if source[i] == "'":
            char_literal = re.match(r"'(?:\\.|[^'\\])'", source[i:])
            if char_literal:
                start = i
                i += len(char_literal.group(0))
                if mask_literals:
                    blank(start, i)
                continue
        if source[i] == '"':
            start = i
            i += 1
            while i < n:
                if source[i] == "\\":
                    i += 2
                elif source[i] == '"':
                    i += 1
                    break
                else:
                    i += 1
            if mask_literals:
                blank(start, i)
            continue
        i += 1
    return "".join(out)


def extract_functions(source: str):
    structure = mask_rust(source, True)
    token_source = mask_rust(source, False)
    found = []
    for match in FN_RE.finditer(structure):
        open_brace = structure.find("{", match.end())
        semicolon = structure.find(";", match.end())
        if open_brace < 0 or (semicolon >= 0 and semicolon < open_brace):
            continue
        depth, pos = 1, open_brace + 1
        while pos < len(structure) and depth:
            if structure[pos] == "{":
                depth += 1
            elif structure[pos] == "}":
                depth -= 1
            pos += 1
        if depth == 0:
            found.append((match.group(1), token_source[open_brace + 1:pos - 1]))
    return found


def normalized_tokens(code: str):
    code = mask_rust(code, False)
    result = []
    for token in TOKEN_RE.findall(code):
        if token.startswith('"') or token.startswith("'"):
            result.append("<LITERAL>")
        elif token[0].isdigit():
            result.append("<NUMBER>")
        elif re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", token) and token not in KEYWORDS:
            result.append("<IDENT>")
        else:
            result.append(token)
    return result


def shingles(tokens, width=5):
    if len(tokens) < width:
        return {tuple(tokens)} if tokens else set()
    return {tuple(tokens[i:i + width]) for i in range(len(tokens) - width + 1)}


def simhash(features):
    vector = [0] * 64
    for feature in features:
        digest = int.from_bytes(hashlib.blake2b(repr(feature).encode(), digest_size=8).digest(), "big")
        for bit in range(64):
            vector[bit] += 1 if digest & (1 << bit) else -1
    value = 0
    for bit, weight in enumerate(vector):
        if weight >= 0:
            value |= 1 << bit
    return value


def code_candidates(root: Path, scope: str = "engine"):
    functions = []
    source_files = list(rust_files(root, scope))
    for path in source_files:
        try:
            source = path.read_text(encoding="utf-8")
        except (OSError, UnicodeDecodeError):
            continue
        for name, body in extract_functions(source):
            tokens = normalized_tokens(body)
            if len(tokens) >= 25:
                features = shingles(tokens)
                functions.append({
                    "path": path.relative_to(root).as_posix(),
                    "name": name,
                    "tokens": tokens,
                    "features": features,
                    "hash": hashlib.sha256(" ".join(tokens).encode()).hexdigest(),
                    "simhash": simhash(features),
                })

    exact = defaultdict(list)
    for fn in functions:
        exact[fn["hash"]].append(fn)
    exact_groups = [
        [{"path": f["path"], "function": f["name"]} for f in group]
        for group in exact.values() if len(group) > 1
    ]

    # LSH candidate generation avoids comparing every function with every other
    # function. Four 16-bit bands catch close SimHash neighbours efficiently.
    buckets = defaultdict(list)
    for index, fn in enumerate(functions):
        value = fn["simhash"]
        for band in range(4):
            buckets[(band, (value >> (band * 16)) & 0xFFFF)].append(index)
    pairs = set()
    for indexes in buckets.values():
        if len(indexes) > 200:
            continue  # overly common band: skip noisy bucket
        for offset, left in enumerate(indexes):
            for right in indexes[offset + 1:]:
                if left != right:
                    pairs.add((min(left, right), max(left, right)))

    similar = []
    for left_i, right_i in pairs:
        left, right = functions[left_i], functions[right_i]
        if left["path"] == right["path"] and left["name"] == right["name"]:
            continue
        a, b = len(left["tokens"]), len(right["tokens"])
        if min(a, b) / max(a, b) < 0.72:
            continue
        union = left["features"] | right["features"]
        score = len(left["features"] & right["features"]) / len(union) if union else 0.0
        if score >= 0.72:
            similar.append({
                "score": round(score, 3),
                "left": {"path": left["path"], "function": left["name"]},
                "right": {"path": right["path"], "function": right["name"]},
            })
    similar.sort(key=lambda item: item["score"], reverse=True)
    return exact_groups, similar, len(functions), len(source_files)


def concept_candidates(root: Path):
    registry = root / "project-memory" / "system_registry.toml"
    if not registry.exists():
        return []
    try:
        data = tomllib.loads(registry.read_text(encoding="utf-8"))
    except (OSError, tomllib.TOMLDecodeError) as exc:
        return [{"error": f"Cannot parse {registry}: {exc}"}]
    systems = data.get("system", [])
    candidates = []
    for i, left in enumerate(systems):
        left_tags = set(left.get("tags", []))
        left_words = set(re.findall(r"[a-z0-9_]+", left.get("responsibility", "").lower()))
        for right in systems[i + 1:]:
            shared_tags = sorted(left_tags & set(right.get("tags", [])))
            right_words = set(re.findall(r"[a-z0-9_]+", right.get("responsibility", "").lower()))
            shared_words = sorted((left_words & right_words) - {"the", "and", "for", "with", "from"})
            if len(shared_tags) >= 2 or len(shared_words) >= 3:
                candidates.append({
                    "left": left.get("id"),
                    "right": right.get("id"),
                    "shared_tags": shared_tags,
                    "shared_responsibility_terms": shared_words,
                    "note": "Possible conceptual overlap; human review required.",
                })
    return candidates


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=ROOT)
    parser.add_argument("--scope", choices=("engine", "all"), default="engine",
                        help="scan production files in engine crates (default), or every Rust file")
    parser.add_argument("--json", type=Path, default=Path("reports/engine-audit.json"))
    parser.add_argument("--markdown", type=Path, default=Path("reports/engine-audit.md"))
    args = parser.parse_args()
    root = args.root.resolve()
    exact, similar, count, files_count = code_candidates(root, args.scope)
    concepts = concept_candidates(root)
    report = {
        "scope": f"read-only {args.scope} scan; engine scope includes crate src/ and build.rs, excluding standalone tests/examples/benches/fixtures; no source files modified",
        "rust_files_scanned": files_count,
        "rust_functions_scanned": count,
        "exact_duplicate_groups": exact,
        "structural_similarity_candidates": similar,
        "conceptual_overlap_candidates": concepts,
    }
    json_path, md_path = root / args.json, root / args.markdown
    json_path.parent.mkdir(parents=True, exist_ok=True)
    md_path.parent.mkdir(parents=True, exist_ok=True)
    json_path.write_text(json.dumps(report, indent=2), encoding="utf-8")
    lines = [
        "# Engine Duplicate Audit", "",
        f"- Scope: **{args.scope}**",
        f"- Rust source files scanned: **{files_count}**",
        f"- Rust functions scanned: **{count}**",
        f"- Exact duplicate groups: **{len(exact)}**",
        f"- Structural similarity candidates: **{len(similar)}**",
        f"- Conceptual overlap candidates: **{len(concepts)}**", "",
        "> Heuristic findings only; similarity is not proof of duplication. Source files are never modified.", "",
        "## Exact duplicates",
    ]
    for group in exact:
        lines.append("- " + "; ".join(f"`{x['path']}::{x['function']}`" for x in group))
    if not exact:
        lines.append("- None detected.")
    lines += ["", "## Structural similarity"]
    for item in similar:
        lines.append(f"- **{item['score']:.1%}** — `{item['left']['path']}::{item['left']['function']}` ↔ `{item['right']['path']}::{item['right']['function']}`")
    if not similar:
        lines.append("- None detected.")
    lines += ["", "## Conceptual overlap"]
    for item in concepts:
        if "error" in item:
            lines.append(f"- ERROR: {item['error']}")
        else:
            lines.append(f"- `{item['left']}` ↔ `{item['right']}`; shared tags: {', '.join(item['shared_tags']) or 'none'}; shared terms: {', '.join(item['shared_responsibility_terms']) or 'none'}.")
    if not concepts:
        lines.append("- None detected (or registry has no systems yet).")
    md_path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"Scanned {files_count} Rust source files and {count} functions.")
    print(f"Exact groups: {len(exact)} | structural candidates: {len(similar)} | conceptual candidates: {len(concepts)}")
    print(f"Reports: {json_path} and {md_path}")


if __name__ == "__main__":
    main()
