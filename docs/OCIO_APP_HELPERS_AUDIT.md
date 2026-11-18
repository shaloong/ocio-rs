# Application helper signatures

Scope: the seven exported classes in `OpenColorAppHelpers.h`. After removing
comments and whitespace, their declarations are identical between 2.5.2 and
2.6.0. The four baseline classes other than the menu parameters also match
2.4.1. Menu parameters add the category-default control; both merging classes
and that control are available in the supported `v2_5` interface (2.5.1+).
Namespace functions are tracked in the [closeout ledger](OCIO_2_6_CLOSEOUT.md).

| Native class | Rust entry and ownership |
| --- | --- |
| `ColorSpaceMenuParameters` | Factory, cfg-gated category-default setting, config, role, search-reference type, include flags, category/encoding strings and explicit entries are native calls. The native parameters retain the config. `const_config` retains the same object; `config` is an independent editable copy. The `size_t` count and index use `usize`. |
| `ColorSpaceMenuHelper` | Native construction, indexed name/UI name/description/family and hierarchy access, both string translations and both index lookups. Immutable native text is borrowed. Missing names map from `size_t(-1)` to `None`; native empty strings stay empty. |
| `LegacyViewingPipeline` | Display/view plus all four correction stages, looks override and enable flag, both processor overloads. The existing getters make independent editable copies. `const_stage` instead retains the native read-only handle, and `set_const_stage` forwards such a handle directly. Native setters make their own copies and accept null to clear. |
| `MixingColorSpaceManager` | Factory, both mix-space and encoding selections by name/index, counts, names, selected indices, perceptual query, refresh, processor and both slider overloads. Indices use `usize`; processor direction and selected config stay native. |
| `MixingSlider` | Native `float` min/max and bidirectional conversion. A manager-owned reference is an exclusive Rust borrow with no allocation or extra reference count. |
| `ConfigMergingParameters` | Factory and editable copy, filenames/output name/prefixes, boolean controls, override strings, environment variables, replace/append search paths, default and per-section strategies. The six native strategy discriminants match the Rust enum; environment indices and count use `i32`. |
| `ConfigMerger` | Factory, OCIOM loading/serialization, editable copy, search paths/working dir, parameters, merge and version. Native signed counts/indices and unsigned version fields are retained. The final result and indexed result share one Rust entry, with `-1` reserved for the final result and other invalid indices returning `None`. The result can be retained without copying or copied for editing. `params` returns an editable copy; `ocio-sys` exposes the original mutable shared handle for callers that can uphold aliasing requirements. `add_params` retains the native parameter object. |

Native constructors are represented by their factories, destruction by
handle release, and diagnostics by the existing stream-to-text functions.
`ConfigMerger::serialize` emits OCIOM rather than diagnostic text. No
Rust-defined subclasses or duplicate overload spellings are required.

Windows native behavior suites passed on 2.4.1 (7 tests), 2.5.2 (10), and
2.6.0 (10), including copy/retain semantics, lifetime after dropping parent
handles, mutation through the shared low-level handle, and invalid indices.
