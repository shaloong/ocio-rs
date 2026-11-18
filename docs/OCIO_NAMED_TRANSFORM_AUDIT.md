# NamedTransform signature audit

Scope: `NamedTransform` declarations in the local OpenColorIO 2.4.1, 2.5.2,
and 2.6.0 public headers. The declarations in this class match across these
versions. This inventory is not a claim of full-library or full behavioral parity.

| Native API | Rust surface | Semantics and limitations |
| --- | --- | --- |
| `Create`, `createEditableCopy` | `NamedTransform::create`, `create_editable_copy`, `ConstNamedTransform::create_editable_copy` | Native shared ownership; editable copies are independent. |
| `getName/setName`, family, description, encoding | Corresponding `NamedTransform` getters/setters and `ConstNamedTransform` getters | Editable convenience getters allocate owned strings; const views can borrow native strings. Editable getters currently do not expose every bridge failure. |
| `getNumAliases`, `getAlias(size_t)`, `hasAlias`, add/remove/clear | `try_num_aliases`, `try_alias_at`, `try_has_alias[_cstr]`, mutation methods; corresponding const queries | Full-width fallible queries avoid narrowing. New indexed query maps out-of-range to `None`, while OCIO returns an empty string. Legacy methods retain their existing behavior. |
| Category count, indexed read, membership, add/remove/clear | `try_num_categories`, `try_category`, `try_has_category[_cstr]`, mutation methods; corresponding const queries | Native category index/count use `int`. Fallible editable queries preserve bridge errors; convenience methods retain fallback behavior. |
| `getTransform`, `setTransform` | `try_const_transform`, `try_transform`, `try_set_transform`, `try_set_const_transform` | Shared native read or independent editable copy; OCIO copies supplied transforms. `None` clears one direction. |
| Static `GetTransform` | `resolve_const_transform` on editable and const wrappers | The bridge checks for missing transforms before invoking native resolution: upstream 2.6 dereferences the opposite transform when the requested one is missing. An entirely empty transform returns an error instead of dereferencing null. |
| Stream insertion | `serialize` | Diagnostic output, not a stable interchange format. |
| Virtual destructor, protected constructor, deleted copy operations | Native handle `Drop`, editable-copy API | Arbitrary Rust implementations of this abstract native class are not exposed; native virtual subclassing remains an open parity gap. |

Representative real-native coverage is in `named_transform_behavior_tests` and
`const_config_item_behavior_tests`; the Windows static 2.4.1, 2.5.2 and 2.6.0
workspace suites passed before this documentation-only inventory. They do not
test arbitrary third-party subclasses, all error paths, or other platforms.
After the fallible category queries were added, the named-transform and
color-space behavior suites passed on Windows static native 2.4.1 and 2.6.0,
including negative and maximum category indices.
