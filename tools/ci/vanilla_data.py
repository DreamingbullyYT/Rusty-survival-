#!/usr/bin/env python3
"""Fetch pinned Minecraft Java Edition inputs for an ephemeral CI parity run."""
from __future__ import annotations
import argparse
from concurrent.futures import ThreadPoolExecutor, as_completed
import hashlib
import json
from pathlib import Path, PurePosixPath
import re
import tempfile
from urllib.parse import urlsplit
import urllib.request

VERSION_MANIFEST = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json"
USER_AGENT = "Rusty-survival-vanilla-parity/1.0"
CHUNK_SIZE = 1024 * 1024


def require_https(url: str) -> str:
    parsed = urlsplit(url)
    if parsed.scheme != "https" or not parsed.netloc:
        raise ValueError("Mojang metadata contains a non-HTTPS or invalid download URL")
    return url


def safe_relative_path(value: str) -> Path:
    if not isinstance(value, str) or not value or "\\" in value:
        raise ValueError("Invalid relative path in Mojang metadata")
    path = PurePosixPath(value)
    if path.is_absolute() or any(part in ("", ".", "..") for part in path.parts):
        raise ValueError("Unsafe relative path in Mojang metadata")
    return Path(*path.parts)


def sha1_file(path: Path) -> str:
    digest = hashlib.sha1()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(CHUNK_SIZE), b""):
            digest.update(chunk)
    return digest.hexdigest()


def read_json_url(url: str) -> dict:
    request = urllib.request.Request(require_https(url), headers={"User-Agent": USER_AGENT})
    with urllib.request.urlopen(request, timeout=120) as response:
        data = response.read(64 * 1024 * 1024 + 1)
    if len(data) > 64 * 1024 * 1024:
        raise ValueError("Mojang metadata JSON exceeds the 64 MiB safety limit")
    value = json.loads(data)
    if not isinstance(value, dict):
        raise ValueError("Mojang metadata root must be a JSON object")
    return value


def download_verified(url: str, destination: Path, checksum: str, size: int | None = None) -> bool:
    """Download atomically and verify SHA-1 and optional byte length."""
    require_https(url)
    if not re.fullmatch(r"[0-9a-fA-F]{40}", str(checksum)):
        raise ValueError("Invalid SHA-1 in Mojang metadata")
    checksum = checksum.lower()
    destination.parent.mkdir(parents=True, exist_ok=True)
    if destination.is_file():
        if sha1_file(destination) == checksum and (size is None or destination.stat().st_size == size):
            return False
        destination.unlink()
    temporary = None
    digest = hashlib.sha1()
    total = 0
    try:
        request = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
        with tempfile.NamedTemporaryFile(dir=destination.parent, delete=False) as stream:
            temporary = Path(stream.name)
            with urllib.request.urlopen(request, timeout=120) as response:
                while True:
                    chunk = response.read(CHUNK_SIZE)
                    if not chunk:
                        break
                    stream.write(chunk)
                    digest.update(chunk)
                    total += len(chunk)
        if digest.hexdigest() != checksum:
            raise ValueError(f"SHA-1 mismatch for {destination.name}")
        if size is not None and total != size:
            raise ValueError(f"Size mismatch for {destination.name}")
        temporary.replace(destination)
        temporary = None
        return True
    finally:
        if temporary is not None:
            temporary.unlink(missing_ok=True)


def validate_version_json(version_data: dict, game_lock: dict, checksum: str) -> None:
    accepted = {game_lock["manifest_sha1"], *game_lock.get("accepted_manifest_sha1", [])}
    if checksum.lower() not in {value.lower() for value in accepted}:
        raise ValueError("Pinned version JSON SHA-1 is not accepted by research-lock.json")
    if version_data.get("id") != game_lock["version"]:
        raise ValueError("Mojang version JSON id differs from research-lock.json")
    downloads = version_data.get("downloads", {})
    for side in ("client", "server"):
        expected = game_lock[f"{side}_sha1"].lower()
        actual = downloads.get(side, {}).get("sha1", "").lower()
        if actual != expected or not downloads.get(side, {}).get("url"):
            raise ValueError(f"Mojang {side} download differs from research-lock.json")
    if version_data.get("javaVersion", {}).get("majorVersion") != game_lock["java_major"]:
        raise ValueError("Mojang Java major version differs from research-lock.json")


def _download_entry(entry):
    url, destination, checksum, size = entry
    return download_verified(url, destination, checksum, size)


def download_many(entries, workers: int, label: str) -> int:
    if not entries:
        return 0
    downloaded = 0
    with ThreadPoolExecutor(max_workers=workers) as pool:
        futures = [pool.submit(_download_entry, entry) for entry in entries]
        for index, future in enumerate(as_completed(futures), start=1):
            try:
                downloaded += int(future.result())
            except Exception as error:
                for pending in futures:
                    pending.cancel()
                raise RuntimeError(f"{label} download failed ({index}/{len(entries)})") from error
            if index % 100 == 0 or index == len(entries):
                print(f"{label}: verified {index}/{len(entries)}", flush=True)
    return downloaded


def prepare(lock_path: Path, game_dir: Path, workers: int = 12) -> dict:
    lock = json.loads(lock_path.read_text(encoding="utf-8"))
    game = lock["minecraft"]
    version = game["version"]
    if not re.fullmatch(r"[A-Za-z0-9_.-]+", version) or version in (".", ".."):
        raise ValueError("Unsafe Minecraft version in research-lock.json")
    if workers < 1 or workers > 32:
        raise ValueError("--workers must be between 1 and 32")

    global_manifest = read_json_url(VERSION_MANIFEST)
    entry = next((item for item in global_manifest.get("versions", []) if item.get("id") == version), None)
    if entry is None:
        raise ValueError(f"Mojang manifest does not list pinned version {version}")
    version_url = require_https(entry["url"])
    version_checksum = entry.get("sha1", "").lower()
    if not re.fullmatch(r"[0-9a-f]{40}", version_checksum):
        raise ValueError("Mojang manifest has no valid SHA-1 for the pinned version")

    version_dir = game_dir / "versions" / version
    version_json_path = version_dir / f"{version}.json"
    download_verified(version_url, version_json_path, version_checksum, entry.get("size"))
    version_data = json.loads(version_json_path.read_text(encoding="utf-8"))
    validate_version_json(version_data, game, version_checksum)
    print(f"Minecraft {version}: version JSON SHA-1 verified", flush=True)

    client, server = version_data["downloads"]["client"], version_data["downloads"]["server"]
    download_many([
        (client["url"], version_dir / f"{version}.jar", game["client_sha1"], client.get("size")),
        (server["url"], version_dir / f"{version}-server.jar", game["server_sha1"], server.get("size")),
    ], workers, "Game JARs")
    print("Client and server JAR SHA-1 verified", flush=True)

    libraries, seen_paths = [], {}
    for library in version_data.get("libraries", []):
        artifact = library.get("downloads", {}).get("artifact")
        if not artifact:
            continue
        relative = safe_relative_path(artifact.get("path"))
        checksum = str(artifact.get("sha1", "")).lower()
        url = require_https(artifact.get("url", ""))
        key = relative.as_posix()
        if key in seen_paths:
            if seen_paths[key] != checksum:
                raise ValueError(f"Conflicting checksums for Maven path {key}")
            continue
        seen_paths[key] = checksum
        libraries.append((url, game_dir / "libraries" / relative, checksum, artifact.get("size")))
    count = download_many(libraries, workers, "Client libraries")
    print(f"Client library artifacts verified: {len(libraries)} ({count} downloaded)", flush=True)

    asset_index = version_data.get("assetIndex")
    if not isinstance(asset_index, dict):
        raise ValueError("Pinned version JSON has no assetIndex")
    index_id = asset_index.get("id")
    if not isinstance(index_id, str) or not re.fullmatch(r"[A-Za-z0-9_.-]+", index_id):
        raise ValueError("Invalid asset index id")
    index_path = game_dir / "assets" / "indexes" / f"{index_id}.json"
    download_verified(asset_index["url"], index_path, asset_index["sha1"], asset_index.get("size"))
    index = json.loads(index_path.read_text(encoding="utf-8"))
    objects = index.get("objects")
    if not isinstance(objects, dict):
        raise ValueError("Asset index has no objects map")

    unique_objects = {}
    for item in objects.values():
        checksum = str(item.get("hash", "")).lower()
        if not re.fullmatch(r"[0-9a-f]{40}", checksum):
            raise ValueError("Asset index contains an invalid object hash")
        size = item.get("size")
        if size is not None and (not isinstance(size, int) or size < 0):
            raise ValueError("Asset index contains an invalid object size")
        if checksum in unique_objects and unique_objects[checksum] != size:
            raise ValueError("Asset index has conflicting sizes for one object hash")
        unique_objects[checksum] = size

    asset_entries = [
        (f"https://resources.download.minecraft.net/{checksum[:2]}/{checksum}",
         game_dir / "assets" / "objects" / checksum[:2] / checksum, checksum, size)
        for checksum, size in unique_objects.items()
    ]
    count = download_many(asset_entries, workers, "Asset objects")
    print(f"Asset index {index_id} verified; objects: {len(asset_entries)} ({count} downloaded)", flush=True)
    return {
        "version": version, "version_json_sha1": version_checksum,
        "client_sha1": game["client_sha1"], "server_sha1": game["server_sha1"],
        "library_count": len(libraries), "asset_index": index_id,
        "asset_object_count": len(asset_entries),
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--lock", type=Path, default=Path(__file__).resolve().parents[2] / "research-lock.json")
    parser.add_argument("--game-dir", type=Path, required=True)
    parser.add_argument("--workers", type=int, default=12)
    args = parser.parse_args()
    try:
        summary = prepare(args.lock, args.game_dir, args.workers)
    except Exception as error:
        import sys
        print(f"vanilla-data: {type(error).__name__}: {error}", file=sys.stderr)
        return 1
    print("Verified pinned vanilla inputs: " + json.dumps(summary, sort_keys=True), flush=True)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
