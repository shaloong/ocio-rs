# ColorSpace signature audit

Scope: public `ColorSpace` declarations in OCIO 2.4.1, 2.5.2 and 2.6.0.
This inventory covers signature families, not every behavior or the rest of
the library. The interop ID and interchange attributes require the relevant
2.5.1-or-newer interface feature.

| Native signature family | Rust surface | Remaining work |
| --- | --- | --- |
| `Create` overloads, `createEditableCopy` | `create`, `create_with_reference_space`, editable-copy methods | Actual reference space and copy independence are covered by behavior tests. |
| Name, aliases, family, equality group, description | Editable accessors and mutation; borrowed const metadata | Several legacy editable getters are infallible convenience methods. |
| Interop ID and interchange attributes | Editable accessors, const ID and individual const attribute | Full interchange-attribute map is not yet exposed on const handles. |
| Bit depth, reference space, categories, encoding, data | Editable accessors and mutation; const readers | Legacy editable getters may collapse native errors. |
| Allocation and variables | Editable vector access and setter; const buffer-based reads | Full caller-buffer read is currently available on const handles. |
| `getTransform`, `setTransform` | Shared const and editable-copy getters; editable and const setters; directional clear | OCIO copies transforms when setting; retained const getters outlive their source. |

`tests/color_space_behavior_tests.rs` checks representative metadata, native
configuration/processor behavior and transform copy/clear ownership. It does
not establish every error or lifetime combination.
