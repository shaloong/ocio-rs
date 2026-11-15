# ColorSpaceSet signature audit

Compared the public `ColorSpaceSet` declarations in OCIO 2.4.1, 2.5.2,
and 2.6.0. This is a focused class audit, not a full-library coverage claim.
All operations below exist in each of those native versions.

| Native signature family | Rust surface and ownership | Remaining work |
| --- | --- | --- |
| `Create`, `createEditableCopy` | `ColorSpaceSet::create`, `create_editable_copy`; const results can explicitly create an editable copy | Native allocation and exceptions are surfaced as `Result`. |
| `operator==`, `operator!=` | `try_equals` compares native names; inequality can be obtained by negation | No dedicated fallible inequality method. |
| Count, name by index | Both set types expose count and copied names | Mutable count currently has only an infallible convenience method. |
| `getColorSpace` / `getColorSpaceByIndex` | Both set types provide retained `ConstColorSpace` handles, or explicit editable copies | The retained handle keeps the native object alive independently of its set. |
| `getColorSpaceIndex`, `hasColorSpace` | Fallible name and alias lookups on both set types | Existing mutable convenience methods discard input and bridge errors. |
| `addColorSpace`, `addColorSpaces` | Editable set accepts editable color-space and set handles; OCIO stores copies | Direct arguments from const color-space and const set handles are not yet exposed. |
| `removeColorSpace`, `removeColorSpaces`, `clearColorSpaces` | Editable set exposes fallible variants | Bulk removal cannot yet take a const set directly. |
| Union, intersection, difference | Both set types accept editable or const operands and return native const sets | No extra editable copy is made by the Rust wrapper. |

`tests/color_space_set_behavior_tests.rs` covers lookup, copy independence,
aliases, missing entries, native set operations and retained handle lifetime.
These tests do not by themselves prove every native alias-collision, error,
or concurrency behavior.
