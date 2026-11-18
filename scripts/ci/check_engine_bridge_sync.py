#!/usr/bin/env python3
"""Verify the HNMod-bundled coordinate module matches engine-core exactly."""
from pathlib import Path
import sys

root = Path(__file__).resolve().parents[2]
canonical = root / "crates/engine-core/src/coordinates.rs"
bundled = root / "mods/dreammod-core/src/engine_coordinates.rs"
if canonical.read_bytes() != bundled.read_bytes():
    print("ERROR: DreamMod engine_coordinates.rs drifted from engine-core")
    sys.exit(1)
print("OK: HNMod coordinate module is byte-identical to engine-core")
