# Public enum version deltas

Compared named enums in the OpenColorIO public headers at 2.4.1, 2.5.0,
2.5.2, and 2.6.0, ignoring comments, whitespace, and trailing commas.
The headers contain 30, 34, 34, and 35 enum declarations respectively.
Twenty-six declarations are unchanged across those versions. The differences
below cover the remaining nine declarations.

| Native enum | Version change and bridge behavior |
| --- | --- |
| `TransformType` | 2.5.0 inserts hue curve at 10. The const type query normalizes the 2.4 tail to stable tags; editable dispatch identifies native types by their named enumerators. |
| `GpuLanguage` | 2.5.0 inserts Vulkan at 4. Explicit conversion in both directions preserves the existing language slots for Rust. Raw slot 4 on 2.4.1 is now rejected rather than accidentally becoming HLSL. Unknown values are not coerced into a native enum. |
| `FixedFunctionStyle` | 2.5.0 appends the three HSY styles. 2.6.0 inserts HMJ before them. The bridge retains HSY at 20–22 and assigns HMJ stable slot 23. Unsupported slots are rejected at the native version boundary. |
| `DynamicPropertyType` | 2.5.0 appends hue curve without renumbering other properties. Rust uses the feature/version policy recorded in the [dynamic property audit](OCIO_DYNAMIC_PROPERTY_AUDIT.md). |
| `MergeStrategies`, `HueCurveType`, `HSYTransformStyle`, `BSplineType` | Introduced by 2.5.0; the declarations are otherwise unchanged through 2.6.0. The additive `v2_5` Rust interface has a 2.5.1 native floor. |
| `ConfigCompatibility` | 2.6.0 adds one HDR display requirement. The Rust HDR helper calls that requirement directly; no separate enum is necessary. |

This comparison detects changes in native declarations; it does not certify
that every call accepting a stable enum validates arbitrary raw integer input.
Native-only sentinels and aliases need not have separate Rust variants when
the binding preserves the corresponding operation. `OptimizationFlags`
and `ProcessorCacheFlags` are unchanged; their native storage widths are
addressed separately in the [closeout ledger](OCIO_2_6_CLOSEOUT.md).

The enum-text and fixed-function suites passed on Windows: 6 tests each
on 2.4.1 and 2.5.0, and 7 each on 2.5.2 and 2.6.0. The raw bridge
regression checks both unavailable and available slots independent of the
Rust version feature, and reports errors for negative and outsize inputs.
