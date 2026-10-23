# Releasing ocio-rs

Both workspace crates use the same version. The 0.3 line supports OpenColorIO
`>= 2.4.1, < 2.6`, with `v2_5` raising the minimum to 2.5.1. Release from
`main`, the default branch, after promoting the tested `develop` candidate.

## Prepare a candidate

1. Fetch both branches and inspect `git log --left-right main...develop`.
   Merge outstanding `main` history into a branch based on `develop`; preserve
   contributor commits and do not reset or force-push shared branches.
2. Update both package versions, the `ocio-sys` dependency requirement, and
   `Cargo.lock`. Update the changelog, both READMEs, both migration guides,
   status, contribution, development, security, and release documentation.
   Keep historical changelog entries intact.
3. Open the release-preparation PR against `develop`, using English
   Conventional Commit subjects. Merge it with its original history after
   validation. Open the promotion PR with **head develop, base main**.

## Candidate validation

Use current stable Cargo with workspace packaging support. Start with a clean
checkout, a C++17 compiler, and CMake; initialize the OpenColorIO submodule for
the generator/parity tooling. The bundled builder normally uses the source
under `ocio-sys/vendor/OpenColorIO` before the submodule fallback.

```bash
cargo fetch --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --no-default-features -- -D warnings
cargo clippy --workspace --all-targets --no-default-features --features v2_5 -- -D warnings
cargo test --workspace --no-default-features
cargo test --workspace --no-default-features --features v2_5
cargo test --examples --no-default-features
cargo test --examples --no-default-features --features v2_5
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --no-default-features
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --no-default-features --features v2_5
cargo run --bin check_parity --quiet -- --check-l3
cargo package --workspace
```

Workspace packaging stages the matching local `ocio-sys` archive while
verifying `ocio-rs`, so a new version can be checked before publication. Do not
convert an unresolved registry dependency or other packaging failure into a
passing release check. Confirm the generated `.crate` payloads contain the
native sources, build helpers, license, and correct dependency metadata.

Run the repository audit with PowerShell 7.4 or newer:

```powershell
pwsh -NoProfile -File tools/release_audit.ps1 -IncludeBundled -IncludeTopLevelPackage -Offline
```

Run `cargo fetch --locked` first to populate Cargo's dependency cache. The
script checks both interface levels, packages both crates when requested,
builds the extracted `ocio-sys` archive with `bundled` and `bundled,v2_5`, and
runs both native interface suites with `-IncludeBundled`. `-Offline` applies
to package/native payload verification; preliminary Rust checks may populate
Cargo's cache. It does not claim a clean machine can build Rust dependencies
without first downloading them. Clear backend/linker environment overrides
before running the audit; a pure bundled payload check must use its own source
tree, not `OCIO_SOURCE_DIR` or an installed-library override.

### Required native coverage

- Installed OpenColorIO 2.4.x with `system`, shared linking, and real CPU tests;
  `system,v2_5` must reject this installation.
- Exact 2.4.1 baseline and exact 2.5.1 with `v2_5` (source-build CI).
- Bundled baseline and `bundled,v2_5`, including the optimization-flag regression.
- Linux, macOS, and Windows bundled `v2_5` tests.
- Arch Linux with system zlib-ng: bundled minizip-ng must still use bundled zlib.
- Extracted-package offline native builds, plus a registry-only consumer after
  publication. Stub results alone cannot satisfy these gates.

`CI` automatically runs cross-platform stub tests, Ubuntu audit, and Fedora
system tests on pushes/PRs. Arch/zlib-ng runs on PRs and weekly/manual events.
Weekly/manual `CI` adds bundled tests on three platforms and exact native minima.
`Release Audit` is a manual Ubuntu packaging/native audit; `Sanitizers` is a
manual Linux nightly AddressSanitizer run for the stub bridge, using an explicit
Linux target so host proc macros are not instrumented. Dispatch these
workflows on the candidate ref, record their completed runs in the promotion
PR, and ensure they tested the candidate being merged.

## Promote and keep branch history consistent

Merge the passing `develop` -> `main` PR with a merge commit, then synchronize
the resulting `main` commit back into `develop` through a **main -> develop**
PR. A release merge creates a new commit on `main`, even if its tree matches
`develop`; without synchronization GitHub will show `develop` as behind.
This synchronization PR may have no file changes; it still carries necessary
ancestry. Merge it without squashing or rebasing so the `main` merge commit is
retained. If there is concurrent development, resolve its conflicts in a
temporary branch based on `develop` that merges `main`, then target `develop`.

After synchronization, fetch and verify:

```bash
git fetch origin
git merge-base --is-ancestor origin/main origin/develop
git rev-list --left-right --count origin/main...origin/develop
```

The first command must succeed; the left-hand count must be **0**. The
right-hand count may be positive because of development and synchronization
merge commits. Do not try to force identical histories by discarding commits.
Remove merged temporary release/synchronization branches. Keep `main` the
default branch; ordinary PRs must explicitly select `develop`.

## Publish 0.3.0

From the clean, validated `main` release commit, publish the low-level crate
first, then wait for its registry index entry before publishing the wrapper:

```bash
cargo publish -p ocio-sys --registry crates-io
cargo publish -p ocio-rs --registry crates-io --dry-run
cargo publish -p ocio-rs --registry crates-io
```

Use authenticated Cargo publishing (or a separately configured trusted
publishing workflow). Do not put credentials in the repository. Verify both
0.3.0 registry entries and docs.rs, then test a separate application using
`ocio-rs = { version = "0.3", features = ["system"] }` with
`OCIO_RS_LINK=dynamic`. Exercise a non-identity CPU transform and confirm it
uses real OCIO. Test the registry bundled backend as well.

Create the `v0.3.0` tag on that `main` release commit and publish GitHub release
notes with the breaking changes and migration links. Only mark a release as
published, or close a release-request issue, once both crates are available.
If a Cargo mirror is configured, verify against crates.io itself to avoid
mistaking a stale mirror for a successful publication.

Source-replacement mirrors may also bypass Cargo's temporary workspace-package
registry during pre-release verification. If the wrapper cannot resolve its
staged sibling despite `cargo package --workspace`, rerun in a clean Cargo home
without source replacement, populate that cache, and rerun the checks. Do not
weaken the dependency requirement to an old published version to make it pass.
