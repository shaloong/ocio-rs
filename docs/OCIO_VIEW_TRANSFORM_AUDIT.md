# ViewTransform signature audit

This inventory compares the public `ViewTransform` declarations in the exact
OpenColorIO 2.4.1, 2.5.2 and 2.6.0 headers with the current Rust wrappers.
It does not establish full behavioral, platform, or performance coverage.

| Native signature family | Rust mapping and ownership |
| --- | --- |
| `Create(ReferenceSpaceType)`, `createEditableCopy()` | `ViewTransform::create`, `create_editable_copy`; the latter is an independent editable copy. |
| `getName`, `getFamily`, `getDescription` (`const char *`, `noexcept`) and setters | Editable owned-string getters and fallible variants, plus setters. Editable borrowed reads require an exclusive handle borrow; `ConstViewTransform` also offers borrowed `CStr` reads. External native aliases must not mutate object-owned strings during a borrow. |
| `getInterchangeAttribute`, `setInterchangeAttribute`, `getInterchangeAttributes` | Editable and const access under `v2_5`; map reads are snapshots. Native 2.4.1 lacks these signatures; both 2.5.2 and 2.6.0 contain them. |
| `hasCategory`, `addCategory`, `removeCategory`, `getNumCategories`, `getCategory`, `clearCategories` | Editable membership, mutation, fallible count and indexed read; const membership and indexed borrowed reads. Native indexed out-of-range values are null in the tested implementations. |
| `getReferenceSpaceType() noexcept` | Editable `reference_space_type` and const reference-space query. |
| `getTransform(ViewTransformDirection) -> ConstTransformRcPtr` | `try_const_transform` preserves shared native ownership; editable `try_transform` returns an independent editable copy. |
| `setTransform(ConstTransformRcPtr, ViewTransformDirection)` | Editable and const-handle setters allow clearing with `None`. Native setters copy a non-null input in the inspected implementations. |
| Stream insertion | `serialize` produces diagnostic text, not a stable interchange format. |

The class has no public virtual customization interface and deletes copy
construction and assignment. The tested Windows 2.4.1 and 2.6.0 paths include
category boundaries and a retained const transform after its source view is
cleared and dropped. Borrowed editable text tests cover populated and empty
strings and a new borrow after mutation on both versions; a compile-fail test
checks mutation through the same handle during a live borrow. Errors thrown in native query paths and other platforms
remain untested; the mapping is not evidence for all config-level overloads.
