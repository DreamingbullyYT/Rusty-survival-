#!/usr/bin/env python3
"""Validate PMCS structure and require memory updates alongside implementation changes."""
from __future__ import annotations

import argparse
import pathlib
import re
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
MEMORY = pathlib.Path("docs/project-memory")
REQUIRED_FILES = [
    "AI_HANDOFF.md", "PROJECT_MEMORY.md", "CURRENT_STATE.md",
    "ARCHITECTURE.md", "DECISIONS.md", "ISSUES.md", "ROADMAP.md",
    "COMPATIBILITY.md", "PARITY_MATRIX.md", "REUSE_MATRIX.md",
    "CHANGELOG.md", "SESSION_LOG.md", "CHANGE_TEMPLATE.md",
]
REQUIRED_TEXT = {
    "AI_HANDOFF.md": ["Mandatory Entry Point", "Horizon", "SESSION_LOG.md", "IMPLEMENTED is not VERIFIED"],
    "PROJECT_MEMORY.md": ["Mission", "Ground rules", "Continuity system"],
    "CURRENT_STATE.md": ["Implemented", "Not implemented / not verified", "Next actions"],
    "DECISIONS.md": ["D-005", "D-006"],
    "ROADMAP.md": ["Phase 0", "Phase 4", "Exit criteria before engine implementation"],
    "SESSION_LOG.md": ["append-only", "## S-"],
    "CHANGE_TEMPLATE.md": ["Tests/commands", "Exact results", "Not tested"],
}
MEMORY_REQUIRED_FOR_CODE = {
    "docs/project-memory/CHANGELOG.md",
    "docs/project-memory/CURRENT_STATE.md",
    "docs/project-memory/ISSUES.md",
    "docs/project-memory/ROADMAP.md",
    "docs/project-memory/SESSION_LOG.md",
}
EXCLUDED_PREFIXES = (
    "docs/", ".github/", "scripts/", "tests/", ".",
)
EXCLUDED_FILES = {"README.md", "LICENSE", "LICENSE.md", ".gitignore"}


def git(*args: str) -> str:
    result = subprocess.run(["git", *args], cwd=ROOT, text=True, capture_output=True)
    if result.returncode:
        raise RuntimeError(result.stderr.strip() or f"git {' '.join(args)} failed")
    return result.stdout


def changed_paths(base: str) -> list[str]:
    return [p for p in git("diff", "--name-only", f"{base}...HEAD").splitlines() if p]


def is_implementation_path(path: str) -> bool:
    if path in EXCLUDED_FILES or path.startswith(EXCLUDED_PREFIXES):
        return False
    return True


def validate_session_log(base: str | None, errors: list[str]) -> None:
    path = ROOT / MEMORY / "SESSION_LOG.md"
    content = path.read_text(encoding="utf-8")
    ids = re.findall(r"^## (S-[^ ]+)", content, flags=re.MULTILINE)
    if not ids:
        errors.append("SESSION_LOG.md must contain at least one session entry.")
    if base:
        try:
            base_content = git("show", f"{base}:{MEMORY / 'SESSION_LOG.md'}")
            base_ids = set(re.findall(r"^## (S-[^ ]+)", base_content, flags=re.MULTILINE))
        except RuntimeError:
            base_ids = set()
        diff = git("diff", "--unified=0", f"{base}...HEAD", "--", str(MEMORY / "SESSION_LOG.md"))
        diff_lines = diff.splitlines()
        removed = [line[1:] for line in diff_lines if line.startswith("-") and not line.startswith("---")]
        added = [line[1:] for line in diff_lines if line.startswith("+") and not line.startswith("+++")]
        new_ids = re.findall(r"^## (S-[^ ]+)", "\n".join(added), flags=re.MULTILINE)
        # Preserve legacy duplicate IDs, but require every newly appended ID to be
        # unique against the full prior history and within this update.
        if len(new_ids) != len(set(new_ids)) or any(session_id in base_ids for session_id in new_ids):
            errors.append("SESSION_LOG.md adds a duplicate or previously used session ID.")
        # Appending to a legacy file without a final newline makes Git represent the
        # unchanged former last line as delete+add. Treat identical lines as preserved.
        from collections import Counter
        removed_counts = Counter(removed)
        added_counts = Counter(added)
        changed = any(count > added_counts[line] for line, count in removed_counts.items())
        if changed:
            errors.append("SESSION_LOG.md is append-only: existing lines were removed or edited.")
    elif ids and ids[-1] in ids[:-1]:
        errors.append("SESSION_LOG.md latest session ID duplicates an earlier entry.")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--base", help="Base commit SHA for freshness and append-only checks.")
    args = parser.parse_args()
    errors: list[str] = []

    for name in REQUIRED_FILES:
        path = ROOT / MEMORY / name
        if not path.is_file():
            errors.append(f"Missing required memory file: {MEMORY / name}")
            continue
        if path.stat().st_size == 0:
            errors.append(f"Memory file is empty: {MEMORY / name}")

    for name, needles in REQUIRED_TEXT.items():
        path = ROOT / MEMORY / name
        if path.is_file():
            content = path.read_text(encoding="utf-8")
            for needle in needles:
                if needle not in content:
                    errors.append(f"{MEMORY / name} is missing required marker: {needle}")

    session_path = ROOT / MEMORY / "SESSION_LOG.md"
    if session_path.is_file():
        validate_session_log(args.base, errors)

    if args.base:
        try:
            paths = changed_paths(args.base)
            implementation = [p for p in paths if is_implementation_path(p)]
            changed = set(paths)
            if implementation:
                missing = sorted(MEMORY_REQUIRED_FOR_CODE - changed)
                if missing:
                    errors.append(
                        "Implementation files changed without required memory updates: "
                        + ", ".join(missing)
                    )
        except RuntimeError as exc:
            errors.append(f"Could not inspect base diff: {exc}")

    if errors:
        print("PMCS validation FAILED:")
        for error in errors:
            print(f" - {error}")
        return 1

    print("PMCS validation PASSED.")
    if args.base:
        print("Base-diff freshness and append-only checks completed.")
    else:
        print("Structural checks completed; base-diff checks were not requested.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
