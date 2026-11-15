import hashlib
import io
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

from vanilla_data import download_verified, safe_relative_path, validate_version_json


class Response(io.BytesIO):
    def __enter__(self):
        return self
    def __exit__(self, *args):
        self.close()


class VanillaDataTests(unittest.TestCase):
    def test_safe_relative_path(self):
        self.assertEqual(safe_relative_path("org/example/lib/1.0/lib.jar").as_posix(),
                         "org/example/lib/1.0/lib.jar")
        for value in ("../escape.jar", "/absolute.jar", "a/../../b", r"a\b.jar", ""):
            with self.subTest(value=value), self.assertRaises(ValueError):
                safe_relative_path(value)

    def test_version_json_is_checked_against_lock(self):
        data = {
            "id": "26.3", "javaVersion": {"majorVersion": 25},
            "downloads": {
                "client": {"sha1": "a" * 40, "url": "https://example.test/client.jar"},
                "server": {"sha1": "b" * 40, "url": "https://example.test/server.jar"},
            },
        }
        lock = {
            "version": "26.3", "java_major": 25, "manifest_sha1": "c" * 40,
            "accepted_manifest_sha1": ["d" * 40], "client_sha1": "a" * 40, "server_sha1": "b" * 40,
        }
        validate_version_json(data, lock, "d" * 40)
        with self.assertRaises(ValueError):
            validate_version_json(data, lock, "e" * 40)

    def test_download_is_atomic_and_checksum_verified(self):
        payload = b"verified vanilla bytes"
        checksum = hashlib.sha1(payload).hexdigest()
        with tempfile.TemporaryDirectory() as folder:
            target = Path(folder) / "artifact.jar"
            with patch("vanilla_data.urllib.request.urlopen", return_value=Response(payload)):
                self.assertTrue(download_verified("https://example.test/file", target, checksum, len(payload)))
            self.assertEqual(target.read_bytes(), payload)
            with patch("vanilla_data.urllib.request.urlopen") as opener:
                self.assertFalse(download_verified("https://example.test/file", target, checksum, len(payload)))
                opener.assert_not_called()

    def test_bad_checksum_leaves_no_partial_file(self):
        with tempfile.TemporaryDirectory() as folder:
            target = Path(folder) / "artifact.jar"
            with patch("vanilla_data.urllib.request.urlopen", return_value=Response(b"wrong")):
                with self.assertRaises(ValueError):
                    download_verified("https://example.test/file", target, "0" * 40)
            self.assertFalse(target.exists())
            self.assertEqual(list(Path(folder).iterdir()), [])


if __name__ == "__main__":
    unittest.main()
