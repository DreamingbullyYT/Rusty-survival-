#!/usr/bin/env python3
"""Regression tests for the PMCS validator using isolated temporary Git repositories."""
from __future__ import annotations

import pathlib
import shutil
import subprocess
import tempfile
import unittest

SOURCE_ROOT = pathlib.Path(__file__).resolve().parents[1]


def run_git(root: pathlib.Path, *args: str) -> str:
    result = subprocess.run(["git", *args], cwd=root, text=True, capture_output=True)
    if result.returncode:
        raise RuntimeError(result.stderr or result.stdout)
    return result.stdout.strip()


class ValidatorRegressionTests(unittest.TestCase):
    def setUp(self) -> None:
        self.tmp = tempfile.TemporaryDirectory()
        self.root = pathlib.Path(self.tmp.name)
        shutil.copytree(SOURCE_ROOT / "docs", self.root / "docs")
        shutil.copytree(SOURCE_ROOT / "scripts", self.root / "scripts")
        run_git(self.root, "init", "-q")
        run_git(self.root, "config", "user.name", "PMCS Tests")
        run_git(self.root, "config", "user.email", "pmcs-tests@example.invalid")
        run_git(self.root, "add", ".")
        run_git(self.root, "commit", "-qm", "baseline")
        self.base = run_git(self.root, "rev-parse", "HEAD")

    def tearDown(self) -> None:
        self.tmp.cleanup()

    def validate(self, *args: str) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            ["python", "scripts/validate_project_memory.py", *args],
            cwd=self.root, text=True, capture_output=True,
        )

    def commit_all(self, message: str) -> None:
        run_git(self.root, "add", "-A")
        run_git(self.root, "commit", "-qm", message)

    def test_missing_required_document_fails(self) -> None:
        (self.root / "docs/project-memory/ARCHITECTURE.md").unlink()
        result = self.validate()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("Missing required memory file", result.stdout)

    def test_implementation_change_without_memory_update_fails(self) -> None:
        source = self.root / "engine/src/lib.rs"
        source.parent.mkdir(parents=True)
        source.write_text("// implementation change\n", encoding="utf-8")
        self.commit_all("add implementation without memory")
        result = self.validate("--base", self.base)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("without required memory updates", result.stdout)

    def test_existing_session_log_lines_are_append_only(self) -> None:
        path = self.root / "docs/project-memory/SESSION_LOG.md"
        content = path.read_text(encoding="utf-8")
        path.write_text(content.replace("This file is chronological and append-only. ", "", 1), encoding="utf-8")
        self.commit_all("rewrite existing session log line")
        result = self.validate("--base", self.base)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("append-only", result.stdout)

    def test_valid_implementation_change_with_memory_update_passes(self) -> None:
        source = self.root / "engine/src/lib.rs"
        source.parent.mkdir(parents=True)
        source.write_text("// implementation change\n", encoding="utf-8")
        memory = self.root / "docs/project-memory"
        for name in ("CHANGELOG.md", "CURRENT_STATE.md", "ISSUES.md", "ROADMAP.md"):
            path = memory / name
            path.write_text(path.read_text(encoding="utf-8") + "\n<!-- regression fixture -->\n", encoding="utf-8")
        log = memory / "SESSION_LOG.md"
        log.write_text(
            log.read_text(encoding="utf-8")
            + "\n## S-TEST-APPEND — Valid fixture\nDate: test\n",
            encoding="utf-8",
        )
        self.commit_all("add implementation and update PMCS")
        result = self.validate("--base", self.base)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)


if __name__ == "__main__":
    unittest.main(verbosity=2)
