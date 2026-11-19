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
| `addColorSpace`, `addColorSpaces` | Editable set accepts editable or const color-space and set handles; OCIO stores copies | Bulk addition preserves OCIO's non-transactional error behavior. |
| `removeColorSpace`, `removeColorSpaces`, `clearColorSpaces` | Editable set exposes fallible variants and accepts either set type for bulk removal | Self-removal is handled as clearing the set to avoid native iterator invalidation. |
| Union, intersection, difference | Both set types accept editable or const operands and return native const sets | No extra editable copy is made by the Rust wrapper. |

`tests/color_space_set_behavior_tests.rs` covers lookup, copy independence,
aliases, missing entries, native set operations and retained handle lifetime.
These tests do not by themselves prove every native alias-collision, error,
or concurrency behavior.

Lookup and membership resolve aliases case-insensitively, whereas native
`removeColorSpace` matches canonical names only. The binding preserves this
distinction. A retained read-only color-space handle keeps the prior native
object alive if the set replaces or removes its entry.

The individual 2.5.2 declarations were also checked against the bridge and
Rust call shapes, rather than only by method name:

| Native signature | Callable Rust path |
| --- | --- |
| `Create() -> ColorSpaceSetRcPtr`; `createEditableCopy() const -> ColorSpaceSetRcPtr` | `create` and `create_editable_copy`; the latter asks OCIO to copy the entries. |
| `operator==(const ColorSpaceSet &) const`, `operator!=` | `try_equals` dispatches the native equality operator; negating its successful result implements inequality. OCIO compares membership by names rather than comparing entire color-space objects. |
| `getNumColorSpaces() const -> int` | `num_color_spaces` on editable sets and `try_num_color_spaces` on retained const sets, with the signed native width. |
| `getColorSpaceNameByIndex(int) const -> const char *` | `try_color_space_name_by_index` copies the returned name and represents a null invalid-index result as `None`. |
| `getColorSpaceByIndex(int) const -> ConstColorSpaceRcPtr`; `getColorSpace(const char *) const -> ConstColorSpaceRcPtr` | The retained `try_const_color_space_by_index` and `try_const_color_space` paths preserve native shared ownership and avoid an editable copy. |
| `getColorSpaceIndex(const char *) const -> int`; `hasColorSpace(const char *) const -> bool` | `try_color_space_index` and `try_has_color_space`; missing indices retain native `-1`. |
| `addColorSpace(const ConstColorSpaceRcPtr &)`; `addColorSpaces(const ConstColorSpaceSetRcPtr &)` | `try_add_color_space_ref` and `try_add_color_spaces_ref` forward editable or const shared handles; OCIO copies the inserted entries. |
| `removeColorSpace(const char *)`; `removeColorSpaces(const ConstColorSpaceSetRcPtr &)`; `clearColorSpaces()` | `remove_color_space`, `try_remove_color_spaces_ref` and `try_clear_color_spaces`; self-removal clears directly to avoid native iterator invalidation. |
| Free `operator\|\|`, `operator&&`, `operator-` on two `ConstColorSpaceSetRcPtr` values | `union`, `intersection` and `difference` forward both operands and retain the native const result. |

The private constructors and the destructor documented for native binding
infrastructure do not require independent Rust callables. This declaration
check does not establish behavior for every possible alias collision or
external concurrent access.
