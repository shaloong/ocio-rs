# Application helper signatures

Scope: the seven exported classes in `OpenColorAppHelpers.h`. After removing
comments and whitespace, their declarations are identical between 2.5.2 and
2.6.0. The four baseline classes other than the menu parameters also match
2.4.1. Menu parameters add the category-default control; both merging classes
and that control are available in the supported `v2_5` interface (2.5.1+).
Namespace functions are tracked in the [closeout ledger](OCIO_2_6_CLOSEOUT.md).

| Native class | Rust entry and ownership |
| --- | --- |
| `ColorSpaceMenuParameters` | Factory, cfg-gated category-default setting, config, role, search-reference type, include flags, category/encoding strings and explicit entries are native calls. Construction and replacement accept either editable or retained const configs without copying. `const_config` retains the same object; `config` is an independent editable copy. The `size_t` count and index use `usize`. |
| `ColorSpaceMenuHelper` | Native construction, indexed name/UI name/description/family and hierarchy access, both string translations and both index lookups. Immutable native text is borrowed. Missing names map from `size_t(-1)` to `None`; native empty strings stay empty. |
| `LegacyViewingPipeline` | Display/view plus all four correction stages, looks override and enable flag, both processor overloads with editable or retained configs. The existing getters make native editable copies; copied group containers still share their children. `const_stage` instead retains the native read-only handle, and `set_const_stage` forwards such a handle directly. Native setters make their own copies and accept null to clear. The retained-config processor path reuses native caches for static processors and isolates dynamic properties, like the other `ConstConfig` builders. |
| `MixingColorSpaceManager` | Factory, both mix-space and encoding selections by name/index, counts, names, selected indices, perceptual query, refresh, processor and both slider overloads. Indices use `usize`; processor direction and selected config stay native. Construction and refresh accept a retained `ConstConfig` without copying its contents. |
| `MixingSlider` | Native `float` min/max and bidirectional conversion. A manager-owned reference is an exclusive Rust borrow with no allocation or extra reference count. |
| `ConfigMergingParameters` | Factory and editable copy, filenames/output name/prefixes, boolean controls, override strings, environment variables, replace/append search paths, default and per-section strategies. The six native strategy discriminants match the Rust enum; environment indices and count use `i32`. |
| `ConfigMerger` | Factory, OCIOM loading/serialization, editable copy, search paths/working dir, parameters, merge and version. Native signed counts/indices and unsigned version fields are retained. The final result and indexed result share one Rust entry, with `-1` reserved for the final result and other invalid indices returning `None`. The result can be retained without copying or copied for editing. `params` returns an editable copy; `ocio-sys` exposes the original mutable shared handle for callers that can uphold aliasing requirements. `add_params` retains the native parameter object. |

Native constructors are represented by their factories, destruction by
handle release, and diagnostics by the existing stream-to-text functions.
`ConfigMerger::serialize` emits OCIOM rather than diagnostic text. No
Rust-defined subclasses or duplicate overload spellings are required.

The menu parameter factory and setter accept native shared config pointers;
the helper factory accepts shared read-only parameters and creates its menu
entries at construction. The native helper stores entry names, UI names,
families, descriptions and hierarchy strings in its own immutable entries.
Its name/UI-name translation returns an entry-owned string, or native empty
text, not the caller's input pointer. A regression retains both a menu name
and translated UI name while removing the source color space, then drops the
query input, parameters and config. It passes on native 2.4.1, 2.5.2 and 2.6.0;
these observations do not permit external mutation of native helper storage.

The namespace-level display helper's config and optional context are native
const shared pointers. `ConstConfig::application_display_processor` forwards
those handles and the optional matrix without an editable copy on the first
native call; its identity counterpart follows the same policy. As with other
safe `ConstConfig` processor builders, a dynamic result is rebuilt with an
editable config copy to isolate mutable properties. The `ocio-sys` entry points
also allow direct native cache semantics without this rebuild; callers choosing
that mode must synchronize shared dynamic properties. Native 2.4.1, 2.5.2 and
2.6.0 tests check both context choices, channel-matrix forwarding, pixel
results, identity, and dynamic-property independence. This is not a claim that
the safe dynamic path avoids the native rebuild or config copy.

The two `getSlider` signatures return the same manager-owned native object;
the two-argument form first sets its mixing-space edge values. Rust's
`slider_with_edges` and `slider` borrow that object exclusively, rather than
copying it. The slider getters may report perceptual-space edges, so the
regression verifies mixing-space endpoints and retained state rather than
assuming the getters always equal the supplied values. This regression passed
on Windows with native 2.4.1, 2.5.1, 2.5.2 and 2.6.0.
The current app-helper behavior suite also passed on native 2.4.1 (9 tests),
2.5.2 (14) and 2.6.0 (14).

Windows native behavior suites passed on 2.4.1 (7 tests), 2.5.2 (10), and
2.6.0 (10), including copy/retain semantics, lifetime after dropping parent
handles, mutation through the shared low-level handle, and invalid indices.
The seven per-section merge-strategy setter/getter pairs were also tested
individually on native 2.5.1, 2.5.2 and 2.6.0: updating one leaves the
others unchanged, and an unspecified section inherits the default strategy.
The multi-stage merger test also distinguishes the first result, second result,
and final-result sentinel through editable and retained read-only config paths.
