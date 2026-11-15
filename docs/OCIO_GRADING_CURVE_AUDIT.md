# Grading curve signature-family audit

Scope: public `GradingBSplineCurve`, `GradingRGBCurve`, and
`GradingHueCurve` declarations in OpenColorIO 2.4.2 and 2.6.0. This is a
partial inventory; it does not establish full-library parity. See
`tests/grading_bspline_behavior_tests.rs` and the grading transform behavior
tests for representative validation.

| Native operation | Rust surface | Remaining concern |
| --- | --- | --- |
| Spline `Create(size)` and point-list constructor | `GradingBSplineCurve::create`, validated and `_unvalidated` point-list constructors | Deferred validation matches native construction; verify remaining boundary cases. |
| Spline type / hue-type constructor overloads | `create_with_spline_type`, `create_with_hue_type` and point-list variants (`v2_5`) | Exact 2.5.1 floor belongs in release validation. |
| Editable copy, point count/coordinates, slopes, validation | `GradingBSplineCurve` methods | Getters return values rather than native references; no borrowed mutable point view. |
| Spline type get/set | `spline_type`, `set_spline_type` (`v2_5`) | Native 2.4 does not provide this API. |
| RGB/Hue curve set constructors from style, copy, or constituent splines | `create`, `create_editable_copy`, `create_from_curves`, `create_from_shared_curves` | Explicitly test more constructor failures and native versions. |
| Curve-set `getCurve` mutable overload | `curve(&mut self)` returns an independently owned native shared handle | Repeated handles alias the same spline, as upstream; no cross-thread sharing. |
| Curve-set `getCurve` const overload | `curve_ref(&self)` borrows without copying; `curve_shared(&self)` retains native shared ownership beyond the parent | Other previously acquired mutable aliases remain live; concurrent native access requires external synchronization. |
| Curve-set identity, validation, equality and hue draw-only flag | Corresponding Rust methods | Equality and native version differences need continued behavioral verification. |
| Native diagnostic stream operators | `serialize` on splines, borrowed spline views and curve sets | Native diagnostic output is not a stable interchange format. |

Transform and dynamic-property `getValue` return native const curve sets.
`SharedGradingRGBCurve` and `SharedGradingHueCurve` retain those values without
deep copying, and can retain child splines or make explicit editable copies.
Replacing the source value does not retarget a retained value. The typed
editable `from_transform` and legacy raw-value paths create independent copies;
`set_transform` delegates native validation and copying. Native callbacks,
shared mutable aliases, and arbitrary external FFI users remain subject to the
documented thread confinement and ownership requirements.
