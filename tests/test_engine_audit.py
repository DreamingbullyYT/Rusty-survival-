import importlib.util
from pathlib import Path

MODULE_PATH = Path(__file__).parents[1] / "tools" / "engine_audit.py"
SPEC = importlib.util.spec_from_file_location("engine_audit", MODULE_PATH)
audit = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(audit)


def test_normalization_ignores_identifier_names():
    a = audit.normalized_tokens("fn alpha() { let count = 1; count + 2 }")
    b = audit.normalized_tokens("fn beta() { let total = 9; total + 4 }")
    assert a == b


def test_shingles_detect_shared_structure():
    a = audit.shingles(["a", "b", "c", "d", "e", "f"])
    b = audit.shingles(["a", "b", "c", "d", "e", "x"])
    assert len(a & b) > 0


def test_extracts_function_body():
    source = "fn example() { let x = 1; if x > 0 { return; } }"
    found = audit.strip_comments_and_extract_functions(source)
    assert len(found) == 1
    assert found[0][0] == "example"
    assert "return" in found[0][1]
