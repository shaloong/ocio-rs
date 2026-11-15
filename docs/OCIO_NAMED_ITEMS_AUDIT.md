# Look and named/view transform audit

Scope: `Look`, `NamedTransform`, and `ViewTransform` public declaration families
in the 2.4.1, 2.5.2, and 2.6.0 headers, including native stream operators.
This is not a full config or transform behavior matrix.

| Family | Rust surface | Important behavior |
| --- | --- | --- |
| Create, editable copy, name and description | Editable types and `ConstLook`, `ConstNamedTransform`, `ConstViewTransform` | Config lookups offer retained immutable handles or editable copies. |
| Look process space and forward/inverse transforms | `Look` and `ConstLook` accessors | Setters copy native transforms. OCIO 2.4--2.6 dereferences the input, so a null transform is not a safe clear operation. |
| Named transform aliases, categories, family, encoding and directions | `NamedTransform`, `ConstNamedTransform` | The native static transform resolver may synthesize the inverse when only the other direction exists. |
| View transform categories, reference space and directional transforms | `ViewTransform`, `ConstViewTransform` | The directional setter supports clearing a transform. |
| Look/view interchange attributes | Editable and const lookup/snapshot methods (`v2_5`) | The feature targets the minimum supported 2.5.1 ABI; unknown attribute names are errors. |
| Stream insertion operators | `serialize` on editable and const handles | Owned diagnostic text, not a stable interchange format; it may include sensitive paths. |

`const_config_item_behavior_tests` verifies copied/shared ownership, retained
transforms, direction handling and diagnostic output. The bridge rejects null
Look transform pointers before native OCIO can dereference them. It is part of the existing
2.6 PR job. Individual method coverage does not prove every combination or
substitute for a full native signature and behavior audit of `Config`.
