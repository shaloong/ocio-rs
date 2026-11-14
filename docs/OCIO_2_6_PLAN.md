# OpenColorIO 2.6 full-coverage plan

The objective is usable, correct Rust access to the public OpenColorIO API
through 2.6.0, not merely matching method names. The supported native range is
`>= 2.4.1, < 2.7`; later interfaces remain opt-in through their Cargo features.
Work is prioritized by missing capabilities, safety, and application impact.

## Work breakdown

1. **Inventory and traceability.** Compare the public 2.4.1, 2.5.2 and 2.6.0
   headers by class, signature, overload, enum, callback and free function;
   map each to the bridge, safe API, version gate, and behavior test. Account
   explicitly for constructors, ownership, exceptions and unsupported or
   intentionally unsafe native interop. Name-level parity is a starting point,
   not proof of signature or behavioral parity.
2. **Architecture and safety.** Resolve gaps including a genuinely customizable
   `GpuShaderCreator`; document and test callback lifetime, ownership, error and
   panic containment, reentrancy, thread safety, and resource cleanup. Keep OCIO
   semantics at the boundary and avoid duplicating native state without reason.
3. **Behavior by subsystem.** Audit configuration, colorspaces and views,
   contexts and files, transforms and metadata, processors and image layouts,
   GPU shader resources and dynamic properties, app helpers, and global APIs.
   Exercise both normal and boundary/error cases against real native libraries.
4. **Performance and compatibility.** Preserve batched FFI calls, borrowed
   buffers and zero-copy views where lifetimes allow; benchmark hot paths against
   direct C++ and run hardware GPU comparisons where applicable. Check version
   gates, feature combinations, static/shared builds and supported platforms.
5. **Documentation and release readiness.** Keep examples, compatibility and
   limitations current. Before any completeness or release claim, review the
   inventory evidence, native and cross-platform results, performance regressions,
   CI state, and release/package checks for the exact candidate commit.

The current inventory and runtime validation are incomplete; the status in
`OCIO_2_6.md` must not be read as a full-library coverage claim.

## Validation placement and cost

- **Every push / PR:** existing cross-platform stub suites, lint/docs/parity,
  and Fedora's installed 2.4 system-library regression suite. PRs additionally
  run representative 2.6 native tests and the existing Arch zlib-ng integration.
  The 2.6 PR selection exercises new-version behavior, app helpers and GPU
  descriptor behavior with the highest interface level, not all combinations.
- **Scheduled and manual:** exact minimum-version builds, full bundled platform
  matrix, all three interface levels against 2.6, and separate installed 2.6
  shared-library discovery/runtime. These are also appropriate release-candidate
  gates. Reuse existing workflow jobs instead of adding a matrix for each API.
- **Manual / relevant change / release candidate:** actual Vulkan hardware
  comparisons, direct-C++ performance baselines, sanitizer and packaging/offline
  release audits. Hardware and performance results depend on the machine and
  are not substituted by a software-rendered CI pass.

Tests should be placed in existing subsystem suites and have deterministic,
small fixtures. Escalate a check into PR CI when it covers a high-risk regression
that cheaper checks do not catch; move expensive cross-products to periodic or
release validation when representative PR coverage remains effective. Record
unrun gates and their scope rather than presenting partial results as complete.
