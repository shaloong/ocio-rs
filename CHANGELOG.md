# Changelog

## Unreleased

- Continue replacing generated stub symbols with real OpenColorIO bridge calls.
- Remove wrapper methods for APIs that are not present in OpenColorIO 2.5.
- Add project status, contribution, and security documentation.
- Expand CI coverage for formatting, linting, tests, docs, packaging, and manual bundled builds.
- Harden `Config` wrapper coverage for OCIO 2.5.1/2.5.2 entry points, including
  active display/view management, virtual displays, processor overloads, and
  built-in configs.
- Improve docs.rs build metadata and align parity reporting so compatibility
  aliases no longer show up as hard release blockers.
- Add a release-audit helper script that codifies the pre-publish checks and
  reports the known top-level package blocker separately from real failures.
- Add a manual GitHub Actions `Release Audit` workflow that runs the repository
  audit, bundled tests, and offline packaging checks from a recursive checkout.
- Promote the parity checker into both CI and the release-audit flow so wrapper
  and bridge drift show up as first-class release failures.
- Make `Config::serialize()` and `Config::archive()` return real OCIO text in
  non-stub builds, and deprecate `Processor::optimized_legacy_gpu_processor()`
  so OCIO 2.5 callers prefer the main GPU processor path.
- Fix `Baker::bake()` so it no longer routes a filesystem path through an
  `ostream*` ABI slot; baked output now goes through `bake_to_string()` first
  and is written to disk from Rust.
- Add `GroupTransform::write_to_string(&Config, format)` so callers do not need
  to pass raw `ostream*` pointers through the OCIO ABI for serialized transform
  output.

## 0.2.0

- Targets OpenColorIO 2.5.2.
- Experimental release line for real OCIO bridge validation.
