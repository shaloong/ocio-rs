# FormatMetadata signature and ownership audit

Scope: the `FormatMetadata` declarations in the installed OpenColorIO 2.4.1
and 2.6.0 `OpenColorTransforms.h` files. Their public signatures match for
this class. This is a class-level inventory, not proof of full-library parity.

| Native operation | Rust surface | Evidence and limitations |
| --- | --- | --- |
| Element name/value getters and setters | `FormatMetadata::{try_element_name,try_element_value,set_element_name,set_element_value}`; `ConstFormatMetadata` borrowed getters | Native owns strings; editable getters copy and readonly getters borrow. Processor metadata retains its owner; transform metadata is available within an unsafe read-only scope. Native root naming/value restrictions are preserved. |
| Attribute count, name, value by index/name, add | Fallible `FormatMetadata` methods and readonly `ConstFormatMetadata` counterparts | OCIO uses an empty string for an absent attribute, not a distinct absent value. The compatibility getters intentionally collapse errors. |
| Child count and const/editable `getChildElement` | `try_num_children`, `try_child_element`, `ConstFormatMetadata::{num_children,child}` | Editable paths survive sibling vector reallocation; clearing or assigning an ancestor invalidates old descendants instead of silently retargeting. Processor-owned readonly children retain their processor. Transform-owned children must not escape the read-only scope. |
| `addChildElement`, `clear` | `add_child_element`, `try_clear` | Child additions and ancestor invalidation are covered by behavior tests. |
| Assignment operator | `assign`, `assign_const` | Overlapping editable trees snapshot the source first; descendants are invalidated before mutation, including on partial failure. Self-assignment is a no-op. Processor-owned sources do not need an intermediate editable copy. |
| `getName`/`setName`, `getID`/`setID` | `try_name`, `set_name`, `try_id`, `set_id`; readonly borrowed getters | Names and IDs are native metadata attributes, not separate Rust state. |
| Stream insertion operator | `to_native_string` on both types | Delegates to OCIO's native diagnostic stream operator and copies the result. It is not a safe XML serializer; non-UTF-8 output is converted lossily. |
| Protected constructor, deleted copy constructor and virtual destructor | Native objects are owned by transforms, bakers or processors; wrapper handles retain owners and release on drop | No standalone public C++ constructor is exposed. `ConstFormatMetadata::create_editable_copy` uses an owned native group metadata container. |

The editable and readonly behavior suites passed on Windows with native static
2.4.1, 2.5.2 and 2.6.0, including ownership after parent drop, sibling growth,
invalidation, aliasing assignment, and stream output. Self-assignment is also
checked through separately acquired handles to the same native element; live
descendants remain valid. Allocation failures,
32-bit builds, external native mutation and other operating systems have not
been exhaustively validated. Both suites already belong to existing CI jobs;
no separate matrix or job is required.
