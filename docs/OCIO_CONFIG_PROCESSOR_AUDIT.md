# Config processor overload audit

Scope: processor construction on `Config` in the OpenColorIO 2.4.1, 2.5.2,
and 2.6.0 public headers. This does not cover the rest of the config API.

| Native family | Rust representation |
| --- | --- |
| Color-space names/roles, optional explicit context | `Config::processor` and its context variants; `ConstConfig::processor` and `processor_with_const_context`. |
| Color-space objects, optional explicit context | Editable objects and typed object references on `Config`; shared objects on `ConstConfig::processor_from_const_color_spaces`. |
| Display and view, optional explicit context | `Config::processor_display` and its context variant; `ConstConfig::processor_display`. |
| Named transform by object or name, optional explicit context | `Config` and `ConstConfig` offer both forms. |
| Generic transform with implicit forward direction, explicit direction, or direction and context | Both config types expose all three choices. The implicit overload is equivalent to passing forward with the config's own context; this does not override the transform object's stored direction. |
| Eight static two-config overloads | `Config` exposes explicit methods. `ConstConfig::processor_to_config` combines a typed color-space or display/view target, optional context pair, and optional interchange pair. Omitted contexts use each config's own context, as in the native convenience overloads. |
| Two static built-in color-space helpers | Both config types support conversion to and from the default built-in config, retaining native interchange selection. |

## Individual instance overloads

The 2.6 header declares thirteen instance `getProcessor` signatures. The
following lists their argument shapes explicitly, using `ConstConfig` as the
read-only path. `None` means call the no-context native overload, not a null
native context. The corresponding editable `Config` methods are listed in
the table above.

| Native arguments (excluding `this`) | `ConstConfig` call |
| --- | --- |
| `(source space object, destination space object)` | `processor_from_const_color_spaces(source, destination, None)` |
| `(context, source space object, destination space object)` | Same method with `Some(context)` |
| `(source name, destination name)` | `processor(source, destination)` |
| `(context, source name, destination name)` | `processor_with_const_context(source, destination, context)` |
| `(source name, display, view, direction)` | `processor_display(..., None)` |
| `(context, source name, display, view, direction)` | `processor_display(..., Some(context))` |
| `(named-transform object, direction)` | `processor_from_const_named_transform(..., None)` |
| `(context, named-transform object, direction)` | `processor_from_const_named_transform(..., Some(context))` |
| `(named-transform name, direction)` | `processor_named_transform(..., None)` |
| `(context, named-transform name, direction)` | `processor_named_transform(..., Some(context))` |
| `(transform)` | `processor_from_const_transform_default_direction(transform)` |
| `(transform, direction)` | `processor_from_const_transform(transform, direction, None)` |
| `(context, transform, direction)` | `processor_from_const_transform(transform, direction, Some(context))` |

For the eight static two-config signatures, `processor_to_config` combines
the independent choices of a color-space versus display/view target, an
optional pair of contexts, and an optional pair of interchange names. Both
context pointers and both interchange names are supplied together, as in the
native signatures; display direction is retained only for a display/view
target. Its bridge dispatches to the matching full native overload, obtaining
each config's current context when the context pair is omitted.

Two read-only configs are passed directly to OCIO. For static pipelines,
the method does not copy configuration data and leaves the component
processor caches to OCIO. For dynamic pipelines, it follows the existing
`ConstConfig` isolation policy: rebuild using fresh editable copies rather
than expose shared dynamic properties across calls. If both arguments refer
to the same config, the rebuild uses one copy. No additional work is added
to CPU pixel processing or shader extraction.

The existing current-config test covers both target types, both display
directions, inferred and explicit interchange, default and explicit contexts,
invalid interchange, dynamic processor independence for identical and
distinct source/destination configs, and both directions of built-in
conversion. The editable-config test suite also
exercises distinct context variables resolving LUTs. No new CI job is needed.
Together these suites passed on Windows: 13 tests with native 2.4.1 and
16 each with native 2.5.2 and 2.6.0.

The raw editable-config transform overloads also preserve OCIO's error for a
null transform or, on the explicitly contextual overload, a null context.
Both editable and const transform-handle bridge paths have native 2.4.1 and
2.6.0 regressions; passing an absent context to an overload that explicitly
uses it as the default-context selector remains supported.
When constructing a processor from a `NamedTransform` object, the bridge
also rejects an object with neither direction set: native OCIO's inverse
fallback dereferences the missing opposite transform in that case. A transform
with either direction remains valid and retains OCIO's inverse fallback.

## Cache controls

`getCacheID()` uses the config's current context. The explicit context
overload can use a retained `ConstContext` or a null context. The latter
hashes the serialized config without consulting external file references;
it is not interchangeable with the no-argument call. Both Rust config
types expose these choices and return owned text.

Native `getProcessorCacheFlags` and `setProcessorCacheFlags` are
const-qualified, but OCIO writes its cache flags independently of the
processor-cache mutex. The shared Rust config exposes a read-only query and
an explicitly unsafe in-place setter for callers that can guarantee native
synchronization. `clearProcessorCache` is non-const and remains on editable
`Config`. Disabling the cache is not equivalent to clearing it.

The safe process-global setter replaces the installed config only if the
flags changed. It installs fresh caches and does not mutate any existing
snapshots or processors. This is a configuration-time operation; callers
must serialize it with other global config writes. Reading the global
flags does not deep-copy the config.

The existing runtime-settings and current-config suites verified the
context/cache-ID distinction, actual global off/on changes, and snapshot
isolation on Windows: 4 tests on native 2.4.1 and 6 each on 2.5.2 and 2.6.0.
