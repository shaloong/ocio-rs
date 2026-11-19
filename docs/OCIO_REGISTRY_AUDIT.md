# Built-in registries and system monitor signatures

Scope: the public `SystemMonitors`, `BuiltinTransformRegistry`, and
`BuiltinConfigRegistry` declarations in OpenColorIO 2.4.1, 2.5.2, and 2.6.0.
The first two classes match across all three versions. The config registry
matches except for the deprecated 2.4.1 default-config-name getter,
removed by 2.5.2.

| Native operation | Rust path |
| --- | --- |
| `SystemMonitors::Get`, support and count | `SystemMonitors::get`, `is_supported`, `num_monitors`. The handle retains the native immutable snapshot and the count is full-width. |
| Monitor name and ICC profile path, both by `size_t` | Borrowed `CStr` results with native bounds errors. Preserve a valid empty path; an unexpected null pointer yields an error. Enumeration is native and depends on OS/display state. |
| `BuiltinTransformRegistry::Get`, count, style and description | The shared singleton facade exposes count and both indexed getters. The existing Rust string methods take signed indices for compatibility; the raw `ocio-sys` getters accept `usize`. |
| `BuiltinConfigRegistry::Get`, count, name, UI name and recommended flag | The native registry is a borrowed singleton. The Rust fallible count retains `size_t`; the existing signed indexed methods preserve compatibility, and raw indexed methods accept `usize`. |
| Built-in config YAML by `size_t` or by native name | `config_yaml_ref_by_index` and `config_yaml_ref_by_name` borrow the original immutable YAML. Both use existing FFI symbols; legacy owned methods remain available. |
| Deprecated 2.4.1 `getDefaultBuiltinConfigName` | Use the upstream replacement `resolve_config_path("ocio://default")`; the result is an OCIO URI, so strip `ocio://` only for callers needing a bare registry name. No separate legacy FFI is needed. |
| Protected constructors, deleted copying, virtual destruction | Rust obtains the native shared handle or singleton reference. It does not construct or delete a registry or monitor implementation. |

The individual 2.6 declarations were checked against the bridge: transform
registry `Get`, `getNumBuiltins`, `getBuiltinStyle(size_t)` and
`getBuiltinDescription(size_t)` retain the native shared registry; config
registry `Get` borrows the singleton and exposes count, indexed name/UI name,
indexed and named YAML, and indexed recommendation. The raw indices and
counts are `size_t`; signed convenience indices are checked before conversion.
Monitor `Get` retains a native shared snapshot, with support/count and the two
`size_t` text getters. The returned monitor text is tied to that snapshot.
Out-of-range handling may use `None` or an error rather than reproducing the
native exception in legacy convenience methods. This declaration mapping does
not establish monitor discovery on every operating system.

The registry checks compare borrowed YAML from both selectors to the same
native pointer and bytes, and check invalid indices and names. Monitor
checks cover the snapshot and full-width bounds errors. A successful
headless/single-machine run does not validate hardware ICC discovery.
The existing registry and app-helper suites passed on Windows: 12 tests
on native 2.4.1 and 14 each on 2.5.2 and 2.6.0.
