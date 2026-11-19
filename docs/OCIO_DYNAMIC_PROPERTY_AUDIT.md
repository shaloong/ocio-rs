# Dynamic property signature audit

The selected-class name parity check excludes `DynamicProperty` and its typed
interfaces. The 2.6.0 public header declares a virtual type query, conversion
functions for double, grading primary, RGB curve, hue curve and grading tone,
and typed value getters/setters. `DynamicProperty` in `src/processor.rs`
provides corresponding type and value operations; RGB and hue curves also have
shared native views and editable object paths.

The base class and the double, primary, RGB-curve and tone classes have the
same declarations in 2.4.1, 2.5.2 and 2.6.0 (ignoring whitespace and comments).
The hue interface is absent in 2.4.1 and identical in 2.5.2 and 2.6.0.

| C++ operation | Rust representation |
| --- | --- |
| Base `getType() const noexcept` | `try_property_type` preserves unknown-type errors; the legacy accessor has a documented fallback. |
| `AsDouble`, double get/set | `double_value` and `set_double_value` invoke the native cast and virtual getter/setter; all three scalar property kinds use them. |
| `AsGradingPrimary` and `AsGradingTone`, const value reference and setter | Primary/tone value getters copy fixed-sized fields to Rust, not the property. Setters forward the value to native validation and mutation. |
| `AsGradingRGBCurve` and feature-gated `AsGradingHueCurve` | Shared getters retain the native const pointer. Editable getters copy on request. The shared setters forward the exact native const pointer to `setValue` in one call, without a binding-side editable copy. The stock OCIO implementation still validates and copies the value itself. |
| Protected constructors, deleted copying, virtual destruction | Shared property handles retain and release the native object. Creating new Rust-defined C++ subclasses is not required. |

The fallible value operations perform the equivalent native cast directly;
separately reifying five Rust subtype handles would not add a callable native
operation. Native aliases may share mutable properties; callers using raw
interop must not access them concurrently with mutation.

The 2.4.1, 2.5.0, 2.5.2 and 2.6.0 headers contain the double, primary,
RGB-curve and tone conversions. Hue-curve conversion is absent in 2.4.1 and
present in the other three; Rust exposes it with `v2_5` (whose supported
minimum is 2.5.1). The fallible type query rejects unknown values; internal
typed operations use it rather than the legacy `Exposure` fallback. Boundary
unit tests cover unknown numeric values in baseline and `v2_6` builds; Windows
real-native behavior suites passed on 2.4.1 and 2.6.0. The manual creator
fixture injects a custom property with type value 7 on both versions and
confirms that the fallible type query and double operations reject it.
The manual native creator fixture injects a null RGB curve and verifies count
and resize errors through the bridge on Windows with 2.4.1 and 2.6.0. The
fixture also injects a null hue curve on 2.6.0 and verifies the same operations;
point reads/writes and slope reads/writes/default queries are checked for both
curve types. Object-copy errors were injected for RGB on both versions and hue
on 2.6.0; complete hue replacement with a null source was also checked on 2.6.0.
Shared-value retrieval was likewise checked to return a bridge error for null
RGB values on 2.4.1 and 2.6.0 and null hue values on 2.6.0.
An external RGB curve subclass in the manual fixture verifies that updating a
dynamic property invokes its virtual editable-copy method and preserves the
changed slope on Windows native 2.4.1 and 2.6.0. An external hue subclass on
2.6.0 likewise exercises virtual copying, slope update and draw-only retention.
Exposure properties retained from processor and CPU processor remain usable
after both parents are dropped, with independent writes, in Windows native
2.4.1 and 2.6.0 targeted tests. This does not establish concurrent safety.
Real 2.5.2 and 2.6.0 hue-property tests additionally check the fallible type
query and reject double-value reads and writes. The manual native creator
fixture also exposes a hue property to a baseline-feature Rust build linked to
2.6.0; its fallible type query and double reads/writes all reject it.
The Windows bundled baseline-feature workspace suite also passed with the
runtime explicitly asserted as 2.6.0 (392 library tests, 16 build-configuration
tests, integration suites and 41 documentation tests). This validates the
combination broadly but does not itself create a hue property in that mode.

The Windows behavior suite passed against all three pinned native versions:
6 tests on 2.4.1 and 11 each on 2.5.2 and 2.6.0. Direct shared-setter coverage
includes restoring a retained curve, preserving hue draw-only state, and
rejecting incompatible native property types. The signature mapping is
complete for this header family; it does not establish behavior of arbitrary
third-party subclasses or concurrent mutation.
The primary and tone dynamic-property round trips now exercise all 34 and 31
flattened scalar slots, respectively, with distinct non-default values across
the field groups. They compare the full value after both native property and CPU
processor transfers and check that invalid replacements leave it unchanged.
This is a round-trip check, not an independent numerical test of every field.
