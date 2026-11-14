import importlib.util
import json
import tempfile
import unittest
from pathlib import Path

MODULE_PATH = Path(__file__).parents[1] / "tools" / "engine_audit.py"
SPEC = importlib.util.spec_from_file_location("engine_audit", MODULE_PATH)
audit = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(audit)


class EngineAuditTests(unittest.TestCase):
    def test_engine_scope_uses_upstream_layers_and_excludes_test_trees(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "tools").mkdir()
            (root / "tools" / "layers.json").write_text(json.dumps({
                "crates": {"horizon-core": "engine", "horizon-mods": "platform"}
            }), encoding="utf-8")
            paths = [
                "crates/horizon-core/src/lib.rs",
                "crates/horizon-core/src/tests/inline.rs",
                "crates/horizon-core/tests/integration.rs",
                "crates/horizon-core/examples/demo.rs",
                "crates/horizon-core/build.rs",
                "crates/horizon-mods/src/lib.rs",
            ]
            for name in paths:
                path = root / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text("fn sample() {}", encoding="utf-8")
            found = {p.relative_to(root).as_posix() for p in audit.rust_files(root)}
            self.assertEqual(found, {
                "crates/horizon-core/src/lib.rs",
                "crates/horizon-core/build.rs",
            })

    def test_all_scope_remains_available(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            path = root / "crates/horizon-mods/src/lib.rs"
            path.parent.mkdir(parents=True)
            path.write_text("fn sample() {}", encoding="utf-8")
            self.assertEqual(list(audit.rust_files(root, "all")), [path])

    def test_normalization_ignores_identifier_names(self):
        a = audit.normalized_tokens("fn alpha() { let count = 1; count + 2 }")
        b = audit.normalized_tokens("fn beta() { let total = 9; total + 4 }")
        self.assertEqual(a, b)

    def test_shingles_detect_shared_structure(self):
        a = audit.shingles(["a", "b", "c", "d", "e", "f"])
        b = audit.shingles(["a", "b", "c", "d", "e", "x"])
        self.assertTrue(a & b)

    def test_extracts_function_body_and_ignores_braces_in_strings(self):
        source = 'fn example() { let text = "} not a body"; if true { return; } }'
        found = audit.extract_functions(source)
        self.assertEqual(len(found), 1)
        self.assertEqual(found[0][0], "example")
        self.assertIn("return", found[0][1])

    def test_nested_comments_are_ignored(self):
        source = "fn example() { /* outer /* inner */ end */ let x = 1; }"
        found = audit.extract_functions(source)
        self.assertEqual(len(found), 1)
        self.assertIn("let x", found[0][1])

    def test_generic_functions_are_detected(self):
        source = "fn generic<T: Copy>(value: T) { let _copy = value; }"
        found = audit.extract_functions(source)
        self.assertEqual(len(found), 1)
        self.assertEqual(found[0][0], "generic")


if __name__ == "__main__":
    unittest.main()
