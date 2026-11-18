# Config callable surface

Scope: the public `Config` declarations in OpenColorIO 2.4.1, 2.5.2,
and 2.6.0. The table groups equivalent overloads and ownership choices.

| Native family | Rust coverage |
| --- | --- |
| Creation, loading, copying, validation, version and identity | Native empty/raw, environment, file, YAML, built-in and I/O proxy loaders; editable copy, version query/update/upgrade, validation, name, family separator, and description. The separator only accepts native printable ASCII or NUL. |
| Serialization, archive and context | Native YAML and OCIOZ writers, archivable status, context and environment declarations, search paths and working directory. Cache and processor-context overloads are mapped in the [processor audit](OCIO_CONFIG_PROCESSOR_AUDIT.md); proxy retention/detachment in the [proxy audit](OCIO_CONFIG_IO_PROXY_AUDIT.md). |
| Color spaces and roles | Native category sets; default and filtered enumeration; name/index/canonical lookup; add/remove/clear; used/inactive/linearity queries. Roles expose name/index lookups and editing. `remove_role` passes the native null pointer; an empty string is not a removal. |
| Display/view creation and lookup | Native default and source-color-space-filtered queries, shared and display-defined enumeration, virtual displays, view fields, monitor/ICC instantiation, and add/remove/clear. The typed queries take `ViewType`, not a reference-space filter. Empty display queries global shared views. |
| Active and temporary displays/views | Native string and indexed access, add/remove/clear where available, all-display indices and temporary-display state. The baseline clear path uses empty native selection strings with the same reset behavior. |
| Rules, looks and named/view transforms | Native rule installation and both filepath matching overloads, default-only matching, deprecated parsing, luma and strict parsing; native look, view-transform and named-transform operations with independent editable copies or retained read-only handles. |
| Processors and cache controls | Both default and explicit contexts and all processor-construction families are covered in the [overload audit](OCIO_CONFIG_PROCESSOR_AUDIT.md). |

Between 2.4.1 and 2.5.2, the Config additions cover shared/virtual view
presence and equality, clearing shared views, incremental active lists,
temporary-display mutation, and named-transform removal. Between 2.5.2
and 2.6.0, Config adds built-in matching, interop ID lookup/generation,
alias-aware authoring and queries, canonical names, resolved view color
spaces, and display descriptions. These use the existing version-gated
paths; the HDR compatibility check is an app-helper namespace operation.

The old Rust methods named `*_by_reference_space` and the old virtual-view
queries remain compatible for the two valid legacy discriminants:
`Scene` selects shared views and `Display` selects display-defined views.
`All` is not a native view type; a checked legacy query rejects it except
when the empty display already selects global shared views. Raw view queries
also reject invalid enum values before converting to C++.

The callable binding need not reproduce private constructors, C++ stream
operator spellings, or an independent read-only Rust wrapper for every
operation already available through the editable handle or native bridge.

Windows native config, config core, display management, and runtime-settings
suites passed on 2.4.1 (35 tests), 2.5.2 (45), and 2.6.0 (45). The added
regressions cover both view origins on physical and virtual displays, native
role removal, and rejecting a Unicode separator without changing the config.
