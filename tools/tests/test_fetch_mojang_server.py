import unittest

from tools.fetch_mojang_server import (
    ARTIFACT_HOSTS,
    METADATA_HOSTS,
    validate_https_url,
    verify_payload,
)


class OfficialUrlTests(unittest.TestCase):
    def test_accepts_official_metadata_host(self):
        validate_https_url(
            "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json",
            METADATA_HOSTS,
        )

    def test_accepts_official_artifact_host(self):
        validate_https_url(
            "https://piston-data.mojang.com/v1/objects/example/server.jar",
            ARTIFACT_HOSTS,
        )

    def test_rejects_http(self):
        with self.assertRaises(ValueError):
            validate_https_url(
                "http://piston-meta.mojang.com/manifest.json", METADATA_HOSTS
            )

    def test_rejects_untrusted_host(self):
        with self.assertRaises(ValueError):
            validate_https_url("https://example.com/server.jar", ARTIFACT_HOSTS)

    def test_rejects_credentials_and_nonstandard_port(self):
        with self.assertRaises(ValueError):
            validate_https_url(
                "https://user@piston-meta.mojang.com/manifest.json",
                METADATA_HOSTS,
            )
        with self.assertRaises(ValueError):
            validate_https_url(
                "https://piston-meta.mojang.com:8443/manifest.json",
                METADATA_HOSTS,
            )


class ArtifactVerificationTests(unittest.TestCase):
    def test_accepts_exact_size_and_sha1(self):
        verify_payload(b"vanilla", 7, "7c9f4e9e2b1c7e7c1e8c8f7f7e6f7f8f9a0b1c2d")

    def test_rejects_size_mismatch(self):
        with self.assertRaises(ValueError):
            verify_payload(b"short", 6, "0" * 40)

    def test_rejects_hash_mismatch(self):
        with self.assertRaises(ValueError):
            verify_payload(b"vanilla", 7, "0" * 40)

    def test_rejects_unreasonable_size(self):
        with self.assertRaises(ValueError):
            verify_payload(b"x", 0, "0" * 40)


if __name__ == "__main__":
    unittest.main()
