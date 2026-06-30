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

## 0.2.0

- Targets OpenColorIO 2.5.2.
- Experimental release line for real OCIO bridge validation.
