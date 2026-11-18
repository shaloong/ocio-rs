# Context signature and ownership audit

Scope: `Context` in the installed OpenColorIO 2.4.1 and 2.6.0 public headers.
The declared methods and overloads examined below exist at both ends of that
range. This is a class-level audit, not full-library coverage.

| Native operation | Rust surface | Notes and evidence |
| --- | --- | --- |
| `Create`, `createEditableCopy`, deleted copy/assignment, destructor | `Context::create`, `create_editable_copy`; `ConstContext::create_editable_copy` | Handles retain native shared ownership and destroy their own references. Copies are editable and independent. |
| Cache ID, search paths and working directory | `try_cache_id`, search-path and working-directory accessors; corresponding `ConstContext` reads | Fallible variants preserve native errors. Text is copied because context mutation may invalidate native pointers. |
| String variable name/value, indexed access, add/update/remove/clear/merge | `set_string_var`, `remove_string_var`, `try_*` accessors, `try_add_string_vars` and `try_add_string_vars_from_const` | Removal uses native null-value semantics. Native missing values use empty strings; no separate existence bit is inferred. |
| Environment mode and `loadEnvironment` | `set_environment_mode`, `try_environment_mode`, `load_environment`; `ConstContext::environment_mode` | The genuine native `Unknown` enum remains distinct from an unrecognized value. The legacy getter falls back to `Unknown` on error. Environment loading is process-state dependent. |
| Both `resolveStringVar` and `resolveFileLocation` overloads | Fallible simple and `*_with_used_context` methods on editable and readonly contexts | These preserve native errors and return an owned result plus the requested used-variable context. Native file resolution may account for variables in search paths even if only one path was needed. |
| Config IO proxy get/set | Typed proxy get/set/clear and readonly getter | Proxy handles retain shared ownership. The bridge invalidates context-local resolution caches when changing proxy; process-global file hash cache management remains explicit. |
| Stream insertion | `serialize` on both context types | Diagnostic stream output, not a stable interchange format; may contain sensitive variable values. |

The existing `context_behavior_tests` suite passed on Windows native static
2.4.1 and 2.6.0 after the environment-mode change, and conversion boundary
tests exercise known and unknown discriminants. The suite covers representative
paths and variables, not every environment, filesystem, proxy or concurrent
native interop configuration. Existing CI already runs this suite; no additional
job is needed.
Nested string-variable resolution additionally checks that both editable and
shared contexts populate the caller's used-context on a cache hit, including
recursive variables and preservation of existing entries.
