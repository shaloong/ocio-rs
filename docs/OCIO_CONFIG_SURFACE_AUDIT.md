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

The three `getNumViews` and `getView` overloads accept, respectively, a
display, a display plus source color space, or a view origin plus display.
The corresponding Rust queries preserve those distinctions and use native
signed indices. Rust `Option` queries translate OCIO's empty missing-view
sentinel to `None`, including virtual-display views. The color-space-filtered
overload otherwise keeps OCIO's own fallback behavior, which may return a
view even when no filtered view matched. A regression checks a zero filtered
count, the fallback view, and the missing-color-space error on native 2.4.1,
2.5.1, 2.5.2 and 2.6.0.

The `hasView` null-display form queries config-level shared views without
requiring attachment to a display. `try_has_shared_view` passes the native
null pointer directly; `has_view` addresses the display-name form. This avoids
depending on whether a future native implementation treats a non-null empty
display string the same way as the documented null argument.

The all-display and temporary-display indexed queries, as well as global
shared-view lookup, check the native count before calling OCIO: the 2.4.1
and 2.5.1 implementations use incorrect bounds conditions that can access
outside their lists. The raw bridge retains OCIO's empty
missing-name sentinel, while the Rust query returns `None`. Role-name
enumeration also maps OCIO's empty out-of-range sentinel to `None`; role
color-space values remain unchanged because an authored role may contain
an empty value. The config core, display management, and role behavior suites
passed on Windows against native 2.4.1, 2.5.1, 2.5.2, and 2.6.0. The bounds
check is a constant-time configuration query and does not affect pixel paths.

The short `addDisplayView` overload maps to `Config::add_display`: its
third and fourth arguments are the color-space name and looks expression,
not a view-transform name and viewing rule. The longer native overload maps
to `add_display_view_detailed`, which includes those additional fields.
The 2.6 alias-aware shared-view overload takes view, view transform, color
space, looks, rule, description, then a vector of aliases. The display-view
overload takes the display first and the same remaining arguments in that
order. `ViewWithAliases` forwards those fields through the corresponding bridge
calls; the existing 2.6 native test verifies distinct values for all fields,
two ordered aliases, and both shared and display-defined views.

The filtered color-space count and indexed-name overloads use
`try_num_color_spaces_filtered` and `try_color_space_name_by_index_filtered`;
their reference-space and visibility selectors are passed directly to OCIO.
The named-transform visibility overloads use
`try_num_named_transforms_filtered` and `try_named_transform_name_by_index_filtered`.
Indexed lookups convert OCIO's empty out-of-range result to `None` and retain
native errors separately. These paths do not construct a color-space set or
copy native configuration objects.

A focused declaration-name scan of the 2.6 `Config` class found 172 distinct
tokens, including its constructor and the `OCIO_DEPRECATED` macro token.
The names without a straightforward `ocio_config_` snake-case spelling were
checked separately: the 2.6 additions are declared in `ocio-sys/src/v2_6.rs`,
several bridge names omit `get`, and the IO-proxy and ICC names split their
acronyms. The deprecated parsing method has an explicit binding. This is a
review aid, not a compiler-level proof of all declarations, overloads, argument
widths, or runtime behavior; the overload and ownership audits remain required.

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
