#!/usr/bin/env python3
"""Fetch one official Minecraft Java server jar into an ephemeral directory.

This tool deliberately has no third-party dependencies. It resolves the exact
release through Mojang's version manifest, verifies both metadata and artifact
SHA-1/size, rejects non-official hosts (including redirects), and never caches
or publishes the downloaded file.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import sys
from urllib.error import HTTPError, URLError
from urllib.parse import urlsplit
from urllib.request import HTTPRedirectHandler, Request, build_opener

MANIFEST_URL = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json"
METADATA_HOSTS = frozenset({"piston-meta.mojang.com"})
ARTIFACT_HOSTS = frozenset({"piston-data.mojang.com", "launcher.mojang.com"})
MAX_METADATA_BYTES = 8 * 1024 * 1024
MAX_ARTIFACT_BYTES = 512 * 1024 * 1024


def validate_https_url(url: str, allowed_hosts: frozenset[str]) -> None:
    parsed = urlsplit(url)
    if (
        parsed.scheme != "https"
        or parsed.hostname not in allowed_hosts
        or parsed.username is not None
        or parsed.password is not None
        or parsed.port not in (None, 443)
    ):
        raise ValueError("URL is not on an approved official Mojang HTTPS host")


class OfficialRedirectHandler(HTTPRedirectHandler):
    """Validate every redirect target before urllib follows it."""

    def __init__(self, allowed_hosts: frozenset[str]):
        super().__init__()
        self.allowed_hosts = allowed_hosts

    def redirect_request(self, request, response, code, message, headers, new_url):
        validate_https_url(new_url, self.allowed_hosts)
        return super().redirect_request(
            request, response, code, message, headers, new_url
        )


def read_official_json(url: str, allowed_hosts: frozenset[str]) -> bytes:
    validate_https_url(url, allowed_hosts)
    opener = build_opener(OfficialRedirectHandler(allowed_hosts))
    request = Request(url, headers={"User-Agent": "RustySurvival-ReferenceTests/1.0"})
    with opener.open(request, timeout=30) as response:
        validate_https_url(response.geturl(), allowed_hosts)
        payload = response.read(MAX_METADATA_BYTES + 1)
    if len(payload) > MAX_METADATA_BYTES:
        raise ValueError("Mojang metadata exceeds the configured size limit")
    return payload


def verify_payload(payload: bytes, expected_size: int, expected_sha1: str) -> None:
    if expected_size <= 0 or expected_size > MAX_ARTIFACT_BYTES:
        raise ValueError("artifact size is outside the permitted range")
    if len(payload) != expected_size:
        raise ValueError(
            f"artifact size mismatch: expected {expected_size}, received {len(payload)}"
        )
    actual_sha1 = hashlib.sha1(payload).hexdigest()
    if actual_sha1.lower() != expected_sha1.lower():
        raise ValueError("artifact SHA-1 mismatch")


def resolve_server(version: str) -> dict:
    manifest_bytes = read_official_json(MANIFEST_URL, METADATA_HOSTS)
    manifest = json.loads(manifest_bytes)
    entry = next(
        (
            item
            for item in manifest.get("versions", [])
            if item.get("id") == version and item.get("type") == "release"
        ),
        None,
    )
    if entry is None:
        raise ValueError(f"Mojang manifest does not list {version} as a release")

    metadata_url = entry.get("url", "")
    validate_https_url(metadata_url, METADATA_HOSTS)
    expected_metadata_sha1 = entry.get("sha1", "")
    if len(expected_metadata_sha1) != 40:
        raise ValueError("version manifest entry has no valid metadata SHA-1")

    metadata_bytes = read_official_json(metadata_url, METADATA_HOSTS)
    if hashlib.sha1(metadata_bytes).hexdigest() != expected_metadata_sha1.lower():
        raise ValueError("version metadata SHA-1 mismatch")

    metadata = json.loads(metadata_bytes)
    if metadata.get("id") != version:
        raise ValueError("version metadata ID does not match requested release")
    server = metadata.get("downloads", {}).get("server")
    if not isinstance(server, dict):
        raise ValueError("official version metadata has no server artifact")
    validate_https_url(server.get("url", ""), ARTIFACT_HOSTS)

    sha1 = server.get("sha1", "")
    size = server.get("size")
    if (
        not isinstance(sha1, str)
        or len(sha1) != 40
        or any(char not in "0123456789abcdefABCDEF" for char in sha1)
        or not isinstance(size, int)
    ):
        raise ValueError("server artifact metadata has invalid SHA-1 or size")
    if size <= 0 or size > MAX_ARTIFACT_BYTES:
        raise ValueError("server artifact size is outside the permitted range")
    return {"version": version, "url": server["url"], "sha1": sha1, "size": size}


def download_verified(artifact: dict, output_dir: Path) -> Path:
    output_dir.mkdir(parents=True, exist_ok=False)
    destination = output_dir / "server.jar"
    partial = output_dir / "server.jar.part"
    validate_https_url(artifact["url"], ARTIFACT_HOSTS)
    opener = build_opener(OfficialRedirectHandler(ARTIFACT_HOSTS))
    request = Request(
        artifact["url"], headers={"User-Agent": "RustySurvival-ReferenceTests/1.0"}
    )
    digest = hashlib.sha1()
    received = 0
    try:
        with opener.open(request, timeout=60) as response, partial.open("xb") as target:
            validate_https_url(response.geturl(), ARTIFACT_HOSTS)
            declared_length = response.headers.get("Content-Length")
            if declared_length is not None and int(declared_length) != artifact["size"]:
                raise ValueError("HTTP Content-Length does not match Mojang metadata")
            while True:
                chunk = response.read(1024 * 1024)
                if not chunk:
                    break
                received += len(chunk)
                if received > artifact["size"]:
                    raise ValueError("download exceeded Mojang's declared artifact size")
                digest.update(chunk)
                target.write(chunk)
        if received != artifact["size"]:
            raise ValueError(
                f"artifact size mismatch: expected {artifact['size']}, received {received}"
            )
        if digest.hexdigest() != artifact["sha1"].lower():
            raise ValueError("artifact SHA-1 mismatch")
        os.replace(partial, destination)
        return destination
    except Exception:
        partial.unlink(missing_ok=True)
        destination.unlink(missing_ok=True)
        raise


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--version", default="26.3")
    parser.add_argument("--output-dir", required=True, type=Path)
    args = parser.parse_args()

    try:
        artifact = resolve_server(args.version)
        destination = download_verified(artifact, args.output_dir)
    except (HTTPError, URLError, TimeoutError, OSError, ValueError, json.JSONDecodeError) as error:
        print(f"Reference artifact acquisition failed: {error}", file=sys.stderr)
        return 1

    print(
        "Verified official Minecraft Java server artifact: "
        f"version={artifact['version']} bytes={artifact['size']} sha1={artifact['sha1']}"
    )
    print(f"Temporary path: {destination}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
