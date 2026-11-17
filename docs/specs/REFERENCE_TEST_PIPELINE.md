# Minecraft Java 26.3 Reference Test Pipeline

Status: DESIGN ONLY; downloader, fixture runner and workflow are not implemented.
Purpose: use a legitimate, version-pinned Java 26.3 installation as a temporary test oracle without publishing or committing Mojang files.

## 1. Two independent CI gates

### Engine Tests (ordinary Rust changes)
- Rust formatting.
- Unit/integration tests.
- Clippy with warnings denied.
- No Minecraft download for ordinary deterministic engine tests.

### Vanilla Reference Tests (separate gated workflow)
- Start with explicit `workflow_dispatch`; consider a schedule only after cost/runtime are measured.
- Use a clean GitHub-hosted runner and a unique directory under `$RUNNER_TEMP`.
- Resolve release 26.3 using Mojang's official version manifest and version JSON.
- Download only the artifact required by a test: server jar for server-side tests; client jar only for client-side tests.
- Verify exact size and SHA-1 against official metadata.
- Run a pinned Java runtime, a Java oracle and the Rust probe with identical fixture inputs.
- Compare normalized observable outputs and emit a concise pass/fail report.

Do not download assets/libraries/client files for a server-only test.

## 2. Temporary-file and non-publication policy

All game files live under a fresh `$RUNNER_TEMP` directory, never in the checkout, repository root, cache, artifact, release, or persistent runner home.

Required safeguards:
- Use fresh temporary directories per job.
- Set game home, working directory, logs, saves and caches to temporary paths.
- Never use actions/cache for game jars, assets, libraries or prepared game directories.
- Never upload Mojang files, derived binary bundles or crash dumps that may embed game data.
- Do not print credentials, access tokens, signed URLs or full environment dumps.
- Delete the temporary directory in an `always()` cleanup step, after both success and failure.
- Keep only authored fixture metadata and normalized outputs whose redistribution status has been reviewed.
- Never place Mojang jars/assets in Git, Git LFS, releases, package registries or public artifacts.

A temporary download is still a download and must use an authorized official source. We will not disguise the source, obfuscate workflow steps, bypass access controls, or hide the mechanism to evade copyright/licensing review. Workflow code in this public repository is visible. Protecting the binaries means not persisting or publishing them, not pretending the download did not happen.

## 3. Reference acquisition

- Resolve version `26.3` from Mojang's official version manifest and exact version JSON.
- Select the smallest artifact required for each test.
- Verify version identity, URL host, byte size and SHA-1 before use.
- Reject unexpected hosts, size mismatches and hash mismatches; no third-party mirror fallback.
- Do not store Minecraft account credentials or launcher tokens in Actions secrets.
- If a test ever requires authenticated user-owned data, stop and review a least-privilege method separately.

## 4. Fixture contract

Every fixture records:
- Unique ID and subsystem.
- Exact target release and artifact identity/hash.
- Seed, dimension, coordinates and settings where relevant.
- Initial state and ordered actions, including tick/frame counts.
- Expected observable output and normalization rules.
- Capture driver, Java runtime, timestamp and provenance.
- Tolerances only where exact equality is not the vanilla contract.

The Java oracle and Rust probe consume the same input and emit the same machine-readable schema. The comparator reports the first differing field and context. Raw game output remains in the temporary directory.

Synthetic tests may validate the harness itself, but must be labelled synthetic and never counted as vanilla parity fixtures. Expected results must never be auto-blessed from Rust output.

## 5. Failure behavior

- Missing/invalid official artifact: fail closed.
- Oracle or probe failure: fail the job.
- Output mismatch: fail; do not silently update expected output.
- Cleanup: unconditional.
- No binary artifact upload, cache or release.

## 6. Implementation sequence

1. Implement a small fixture schema and validator with synthetic self-tests.
2. Implement official metadata/artifact resolution and hash/size checks into `$RUNNER_TEMP`.
3. Add one small server-side Java oracle/Rust probe comparison.
4. Add a manual Actions workflow and test cleanup on success and deliberate failure.
5. Expand subsystem coverage only with captured reference evidence.

## 7. Compliance boundary

This pipeline is designed to prevent redistribution and repository contamination, not to conceal downloads. A public workflow can be inspected by anyone. Do not use misleading names, encoded shell commands, hidden mirrors or secret-based obfuscation. If public disclosure of the test process is unacceptable, do not run it in a public repository; use a private repository or privately controlled runner with properly licensed inputs.
