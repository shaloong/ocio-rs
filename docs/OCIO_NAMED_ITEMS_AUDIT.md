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

## Individual Look and ViewTransform declarations

The following maps the declarations themselves, rather than assuming that a
matching method name implies matching arguments or ownership. `NamedTransform`
is enumerated separately in the [signature audit](OCIO_NAMED_TRANSFORM_AUDIT.md).

| Native signature | Rust path and disposition |
| --- | --- |
| `Look::Create()`, `createEditableCopy() const` | `Look::create`, `create_editable_copy`, `ConstLook::create_editable_copy`; native shared handles, independent editable copies. |
| `Look` name, process-space and description `get*() const` / `set*(const char*)` | Editable getters/setters in `Look`; borrowed getters in `ConstLook`. The Rust string setters reject embedded NULs. |
| `Look::getTransform()` / `getInverseTransform()` and their `set*(const ConstTransformRcPtr&)` | `try_const_transform` / `try_const_inverse_transform` share native results; the fallible editable and const setters forward the corresponding direction. Native setters copy and cannot safely accept null. |
| `ViewTransform::Create(ReferenceSpaceType)`, `createEditableCopy() const` | `ViewTransform::create`, editable and const `create_editable_copy`; the reference-space argument is preserved. |
| `ViewTransform` name, family and description `get*() const` / `set*(const char*)` | Editable getters/setters in `ViewTransform`; borrowed getters in `ConstViewTransform`. |
| `hasCategory(const char*)`, `addCategory(const char*)`, `removeCategory(const char*)`, `getNumCategories()`, `getCategory(int)`, `clearCategories()` | Fallible editable category methods and const queries; the editable indexed getter takes `i32`, while the const `usize` index is checked before conversion to native `int`. |
| `getReferenceSpaceType() const` | Fallible editable and const queries validate the native enum value. |
| `getTransform(ViewTransformDirection) const`, `setTransform(const ConstTransformRcPtr&, ViewTransformDirection)` | Shared const getter and fallible editable/const setters retain the direction; `None` clears the native transform. |
| Both classes: `getInterchangeAttribute(const char*)`, `setInterchangeAttribute(const char*, const char*)`, `getInterchangeAttributes()` | `v2_5`-gated fallible lookup/set and snapshot paths; the snapshot crosses the bridge in one visit and owns its strings. The feature requires native 2.5.1 or newer. These declarations are absent from the 2.4.1 header. |
| Deleted copy operations, non-public constructors and native destructors | No extra Rust constructors or direct destructor calls: native handles own their lifetime. Stream operators are diagnostic output, covered above. |

The table covers these two classes' public declarations in the inspected 2.6.0
header and their 2.4.1 version boundary. It does not establish arbitrary native
subclass behavior or a complete cross-platform runtime matrix.

`const_config_item_behavior_tests` verifies copied/shared ownership, retained
transforms, direction handling and diagnostic output. The bridge rejects null
Look transform pointers before native OCIO can dereference them. In contrast,
the raw ColorSpace and NamedTransform setters accept null to clear either native
direction, matching OCIO; both paths have native 2.4.1 and 2.6.0 regressions.
The suite is part of the existing 2.6 PR job. Individual method coverage does not prove every combination or
substitute for a full native signature and behavior audit of `Config`.
