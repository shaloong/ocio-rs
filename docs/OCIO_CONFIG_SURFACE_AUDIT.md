# Config callable surface

Scope: the public `Config` declarations in OpenColorIO 2.4.1, 2.5.2,
and 2.6.0. The table groups equivalent overloads and ownership choices.

Both baseline static color-space identification calls also accept borrowed
editable or retained read-only config handles through `ocio-sys` and
`ConfigRef::as_raw_parts`, without binding-side config copies. These native
heuristics temporarily change cache flags, so raw callers must synchronize all
aliases. Safe convenience methods isolate retained read-only inputs.

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

The virtual-display declarations were also checked individually against the
2.6 header. The display-defined writer forwards view, view-transform,
color-space, looks, rule and description in that order; the shared-view writer
uses a single name. Count and indexed enumeration take `ViewType` and a signed
index. The five field queries, remove/clear operations, and monitor-name and
ICC-profile instantiation calls have corresponding bridge paths. The 2.5+
`hasVirtualView`, `isVirtualViewShared`, and `AreVirtualViewsEqual` operations
are feature-gated; equality compares the two supplied configs and a view name.
The older Rust enumeration overloads use a legacy selector conversion; the
`*_by_type` methods expose the native view-origin selector directly. Native
behavior tests cover authored fields, shared versus display-defined views,
enumeration and removal. They do not establish physical monitor or ICC profile
discovery across operating systems.

The short `addDisplayView` overload maps to `Config::add_display`: its
third and fourth arguments are the color-space name and looks expression,
not a view-transform name and viewing rule. The longer native overload maps
to `add_display_view_detailed`, which includes those additional fields.
The legacy `ocio_config_add_display` FFI symbol forwards to the same short
overload. A direct FFI regression verifies both argument positions against
native 2.4.1, 2.5.2 and 2.6.0, and a stub test also requires the symbol to link.
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

The unfiltered color-space and named-transform overloads enumerate active
objects; the filtered forms pass the native reference-space/visibility enums
and signed `int` index in header order. Both named-transform name paths map
the native empty invalid-index sentinel to `None`, as does the separate
view-transform name lookup. Color-space index lookups also map empty or null
missing names to `None`. The role-color-space overloads take either a signed
index or a role name and preserve native empty values, since an authored role
may have an empty value. The two config search-path overloads take no argument
or a signed index and copy the native text; empty path text is not discarded.
The existing collection behavior suite checks active/inactive filters and the
named/view-transform index boundaries on native 2.4.1, 2.5.2 and 2.6.0.

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

The remaining 2.6 declaration groups were checked against the bridge and
the existing paths above. Creation includes empty, raw, environment, file,
stream, built-in and I/O-proxy forms; native const results may be retained or
explicitly copied for editing. Color-space and role enumeration use signed
indices, while filtered enumeration adds reference-space and visibility
selectors. Native color-space, look, view-transform, named-transform and rule
getters return const shared objects; existing read-only handles preserve shared
ownership, and editable copies are separate. Display/view queries distinguish
unfiltered, source-filtered and `ViewType` overloads; active display and view
lists provide string and indexed access, with incremental mutation gated at
2.5. File-path matching has both the name-only and `size_t` rule-index forms;
file and viewing rules are installed as native shared objects. Default luma
uses exactly three doubles. Processor overloads and 2.6-specific alias and ID
operations remain covered by their separately linked audits. These checks map
the declared calls and ownership paths; they do not test every input or every
implementation of an external native subclass.

Windows native config, config core, display management, and runtime-settings
suites passed on 2.4.1 (35 tests), 2.5.2 (45), and 2.6.0 (45). The added
regressions cover both view origins on physical and virtual displays, native
role removal, and rejecting a Unicode separator without changing the config.
