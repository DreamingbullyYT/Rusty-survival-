#!/usr/bin/env python3
"""Read-only duplicate and responsibility-overlap scanner for Rust engine code.

This is a heuristic audit tool, not a semantic proof system. It never edits source.
"""
from __future__ import annotations

import argparse
import difflib
import hashlib
import json
import re
import tomllib
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SKIP_DIRS = {".git", "target", "vendor", "build", "dist", "node_modules"}
RUST_KEYWORDS = {
    "as", "async", "await", "break", "const", "continue", "crate", "dyn",
    "else", "enum", "extern", "false", "fn", "for", "if", "impl", "in",
    "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while", "yield",
}
TOKEN_RE = re.compile(
    r'"(?:\\.|[^"\\])*"|\'(?:\\.|[^\'\\])*\'|'
    r'\b\d+(?:\.\d+)?\b|[A-Za-z_][A-Za-z0-9_]*|'
    r'::|->|=>|==|!=|<=|>=|&&|\|\||\.\.|[^\s]'
)
FN_RE = re.compile(r'\bfn\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(')


def rust_files(root: Path):
    for path in root.rglob("*.rs"):
        if any(part in SKIP_DIRS for part in path.parts):
            continue
        yield path


def strip_comments_and_extract_functions(source: str):
    """Find function bodies with a small lexer that skips comments and literals."""
    functions = []
    i, n = 0, len(source)
    clean = list(source)
    while i < n:
        if source.startswith("//", i):
            j = source.find("\n", i)
            j = n if j < 0 else j
            for k in range(i, j):
                clean[k] = " "
            i = j
        elif source.startswith("/*", i):
            depth, j = 1, i + 2
            while j < n and depth:
                if source.startswith("/*", j):
                    depth += 1
                    j += 2
                elif source.startswith("*/", j):
                    depth -= 1
                    j += 2
                else:
                    j += 1
            for k in range(i, j):
                if clean[k] != "\n":
                    clean[k] = " "
            i = j
        elif source[i] == '"':
            i += 1
            while i < n:
                if source[i] == "\\":
                    i += 2
                elif source[i] == '"':
                    i += 1
                    break
                else:
                    i += 1
        else:
            i += 1
    cleaned = "".join(clean)
    for match in FN_RE.finditer(cleaned):
        open_brace = cleaned.find("{", match.end())
        semicolon = cleaned.find(";", match.end())
        if open_brace < 0 or (semicolon >= 0 and semicolon < open_brace):
            continue
        depth, j = 1, open_brace + 1
        while j < n and depth:
            if cleaned[j] == "{":
                depth += 1
            elif cleaned[j] == "}":
                depth -= 1
            j += 1
        if depth == 0:
            functions.append((match.group(1), source[open_brace + 1:j - 1]))
    return functions


def normalized_tokens(code: str):
    tokens = TOKEN_RE.findall(code)
    result = []
    for token in tokens:
        if token.startswith('"') or token.startswith("'"):
            result.append("<LITERAL>")
        elif token[0].isdigit():
            result.append("<NUMBER>")
        elif re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", token) and token not in RUST_KEYWORDS:
            result.append("<IDENT>")
        else:
            result.append(token)
    return result


def shingles(tokens, width=5):
    if len(tokens) < width:
        return {tuple(tokens)} if tokens else set()
    return {tuple(tokens[i:i + width]) for i in range(len(tokens) - width + 1)}


def code_candidates(root: Path):
    funcs = []
    for path in rust_files(root):
        try:
            source = path.read_text(encoding="utf-8")
        except (OSError, UnicodeDecodeError):
            continue
        for name, body in strip_comments_and_extract_functions(source):
            tokens = normalized_tokens(body)
            if len(tokens) >= 25:
                funcs.append({
                    "path": path.relative_to(root).as_posix(),
                    "name": name,
                    "tokens": tokens,
                    "hash": hashlib.sha256(" ".join(tokens).encode()).hexdigest(),
                })
    exact = defaultdict(list)
    for fn in funcs:
        exact[fn["hash"]].append(fn)
    exact_groups = [
        [{"path": f["path"], "function": f["name"]} for f in group]
        for group in exact.values() if len(group) > 1
    ]

    similar = []
    for i, left in enumerate(funcs):
        left_set = shingles(left["tokens"])
        for right in funcs[i + 1:]:
            if left["path"] == right["path"] and left["name"] == right["name"]:
                continue
            a, b = len(left["tokens"]), len(right["tokens"])
            if min(a, b) / max(a, b) < 0.72:
                continue
            right_set = shingles(right["tokens"])
            union = left_set | right_set
            score = len(left_set & right_set) / len(union) if union else 0.0
            if score >= 0.72:
                similar.append({
                    "score": round(score, 3),
                    "left": {"path": left["path"], "function": left["name"]},
                    "right": {"path": right["path"], "function": right["name"]},
                })
    similar.sort(key=lambda item: item["score"], reverse=True)
    return exact_groups, similar, len(funcs)


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
    parser.add_argument("--json", type=Path, default=Path("reports/engine-audit.json"))
    parser.add_argument("--markdown", type=Path, default=Path("reports/engine-audit.md"))
    args = parser.parse_args()
    root = args.root.resolve()

    exact, similar, function_count = code_candidates(root)
    concepts = concept_candidates(root)
    report = {
        "scope": "read-only heuristic scan; no source files modified",
        "rust_functions_scanned": function_count,
        "exact_duplicate_groups": exact,
        "structural_similarity_candidates": similar,
        "conceptual_overlap_candidates": concepts,
    }
    json_path = root / args.json
    md_path = root / args.markdown
    json_path.parent.mkdir(parents=True, exist_ok=True)
    md_path.parent.mkdir(parents=True, exist_ok=True)
    json_path.write_text(json.dumps(report, indent=2), encoding="utf-8")

    lines = [
        "# Engine Duplicate Audit",
        "",
        f"- Rust functions scanned: **{function_count}**",
        f"- Exact duplicate groups: **{len(exact)}**",
        f"- Structural similarity candidates: **{len(similar)}**",
        f"- Conceptual overlap candidates: **{len(concepts)}**",
        "",
        "> Heuristic findings only. Similarity is not proof of duplication. No source code was changed.",
        "",
        "## Exact duplicates",
    ]
    for group in exact:
        lines.append("- " + "; ".join(f"`{x['path']}::{x['function']}`" for x in group))
    if not exact:
        lines.append("- None detected.")
    lines += ["", "## Structural similarity"]
    for item in similar:
        lines.append(
            f"- **{item['score']:.1%}** — `{item['left']['path']}::{item['left']['function']}` "
            f"↔ `{item['right']['path']}::{item['right']['function']}`"
        )
    if not similar:
        lines.append("- None detected.")
    lines += ["", "## Conceptual overlap"]
    for item in concepts:
        if "error" in item:
            lines.append(f"- ERROR: {item['error']}")
        else:
            lines.append(
                f"- `{item['left']}` ↔ `{item['right']}`; "
                f"shared tags: {', '.join(item['shared_tags']) or 'none'}; "
                f"shared terms: {', '.join(item['shared_responsibility_terms']) or 'none'}."
            )
    if not concepts:
        lines.append("- None detected (or registry has no systems yet).")
    md_path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"Scanned {function_count} Rust functions.")
    print(f"Exact groups: {len(exact)} | structural candidates: {len(similar)} | conceptual candidates: {len(concepts)}")
    print(f"Reports: {json_path} and {md_path}")


if __name__ == "__main__":
    main()
