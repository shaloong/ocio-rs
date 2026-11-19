# Grading curve signature-family audit

Scope: public `GradingBSplineCurve`, `GradingRGBCurve`, `GradingHueCurve`,
`GradingRGBCurveTransform`, and `GradingHueCurveTransform` declarations in
OpenColorIO 2.4.1, 2.5.2, and 2.6.0. Hue curves are absent in 2.4.1;
their API and the spline-type additions are gated by `v2_5`. This audit maps
these five classes; it does not establish full-library parity. See
`tests/grading_bspline_behavior_tests.rs` and the grading transform behavior
tests for representative validation.

| Native operation | Rust surface | Semantics and limits |
| --- | --- | --- |
| Spline `Create(size)` and point-list constructor | `GradingBSplineCurve::create`, validated and `_unvalidated` point-list constructors | The bridge populates the native points in one call; the unvalidated form preserves native deferred validation. |
| Spline type / hue-type constructor overloads | `create_with_spline_type`, `create_with_hue_type` and point-list variants (`v2_5`) | Exact 2.5.1 floor belongs in release validation. |
| Editable copy, point count/coordinates, slopes, validation | `GradingBSplineCurve` methods; `copy_points_into` on editable and borrowed splines | Getters return values rather than native references; no borrowed mutable point view. The bulk reader uses caller-owned storage. |
| Spline type get/set | `spline_type`, `set_spline_type` (`v2_5`) | Native 2.4 does not provide this API. |
| RGB/Hue curve set constructors from style, copy, or constituent splines | `create`, `create_editable_copy`, `create_from_curves`, `create_from_shared_curves` | OCIO copies constituent splines itself; safe Rust cannot pass a null constituent. |
| Curve-set `getCurve` mutable overload | `curve(&mut self)` returns an independently owned native shared handle | Repeated handles alias the same spline, as upstream; no cross-thread sharing. |
| Curve-set `getCurve` const overload | `curve_ref(&self)` borrows without copying; `curve_shared(&self)` retains native shared ownership beyond the parent | Other previously acquired mutable aliases remain live; concurrent native access requires external synchronization. |
| Curve-set identity, validation, equality, hue draw-only flag, and `GetBSplineTypeForHueCurveType` | Corresponding editable and shared queries, editable draw-only setter, and `b_spline_type_for_hue_curve_type` | Equality and validation dispatch to OCIO rather than comparing a Rust snapshot. |
| Both transforms: factory, metadata, equality, style, dynamic toggles | Typed factories and accessors; base transform covers direction, type, validation and editable copy | RGB includes linear-to-log bypass; Hue includes RGB-to-HSY selection. |
| Both transforms: `getSlope`, `setSlope`, `slopesAreDefault` | Checked typed methods and `ocio-sys` transform entry points | Legacy typed slope indices are `i32`; the direct FFI preserves the complete `size_t` domain (`usize`). |
| Both transforms: `getValue`, `setValue` | `SharedGrading*Curve::from_transform` and `set_transform`; editable copies are opt-in | The shared setter passes the native const handle without a bridge-side copy. |
| Native diagnostic stream operators | `serialize` on splines, borrowed spline views, editable and shared curve sets | Native diagnostic output is not a stable interchange format. |

The 2.6 transform declarations were checked individually: each factory takes
one `GradingStyle`; each style setter resets values when the style changes.
Each value getter returns a const shared curve set, and each setter takes a
const shared curve set and may reject an invalid value. Slope access takes the
corresponding RGB or Hue curve enum, a `size_t` index and a `float` value;
`slopesAreDefault` takes only the curve enum. Both transforms have the three
dynamic controls, equality and const/editable metadata. RGB additionally has
the boolean lin-to-log bypass, whereas Hue has the `HSYTransformStyle` getter
and setter. Hue is gated at `v2_5`; RGB is available in all pinned versions.
The native shared-value path avoids forcing the legacy Rust value snapshot;
using that snapshot remains an explicit copying convenience.

The 2.6 curve-value declarations were additionally checked by signature:
the spline has six factory shapes (count or points, each with no type,
spline type or hue type), editable copy, `size_t` point count and index,
mutable/const point access, `float` slopes, validation and spline type.
The RGB and hue sets each have style, existing-set and constituent-spline
factories, editable copy, validation, identity, and mutable/const child
access. Hue additionally exposes the type mapping and draw-only get/set.
For OCIO's stock curve implementations the existing-set factory yields the
same copied curve values as an editable copy; arbitrary external subclasses
may implement their virtual copy differently. Points are read or updated as
values rather than exposing C++ member references.
The four RGB and eight hue constituent arguments preserve header order.
Shared or borrowed child handles retain the same native ownership constraints
described below; these mappings do not prove every possible curve behavior.

Transform and dynamic-property `getValue` return native const curve sets.
`SharedGradingRGBCurve` and `SharedGradingHueCurve` retain those values without
deep copying, and can retain child splines or make explicit editable copies.
Their `curve_ref` methods additionally borrow a child without allocating a new
ownership handle; the borrow cannot outlive its shared parent. Their `slope`
methods read a child slope in one bridge call without creating a child handle.
Borrowed spline views also compare directly using the native equality operator,
without copying either curve. The semantics are native-version-specific: OCIO
2.4.1 compares control points but not slopes, whereas 2.6.0 also compares slope
and spline type. The borrowed comparison regression passed on Windows against
native 2.4.1, 2.5.2 and 2.6.0, including equality with a retained alias and
comparison with a distinct modified curve.
Replacing the source value does not retarget a retained value. The typed
editable `from_transform` and legacy raw-value paths create independent copies;
editable and shared `set_transform` delegate native validation and copying
without an extra bridge-side curve copy. Native callbacks,
shared mutable aliases, and arbitrary external FFI users remain subject to the
documented thread confinement and ownership requirements.

The point-list bridge uses the size-based native factory and assigns each
control point, matching the native initializer-list implementation; the
unvalidated entry preserves its deferred validation behavior. RGB and Hue
constituent factories were checked in native header order (four and eight
curves respectively). Regressions use a different slope for every slot on
both editable and retained-const inputs; the RGB test also checks that later
source mutation does not change either constructed result. The suite passed
on native 2.4.1 (RGB only), 2.5.2, and 2.6.0. These tests do not imply all
spline shapes or external mutable aliases are covered.
