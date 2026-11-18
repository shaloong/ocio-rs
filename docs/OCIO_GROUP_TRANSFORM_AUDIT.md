# Group transform ownership audit

Scope: the public GroupTransform and base Transform contracts and native
implementations in OpenColorIO 2.4.1, 2.5.2 and 2.6.0. This covers group
ownership and editing semantics, not every transform subclass.

| Operation | Rust behavior | Ownership and limitation |
| --- | --- | --- |
| Append, prepend, index and replace | `append_transform`, `prepend_transform`, `try_transform`, `set_transform` | OCIO retains shared children. The binding rejects cycles before writes; acyclic DAG sharing is retained. |
| Const child access and count | `ConstTransform::group_child`, `group_child_count` | These call the const group interface without copying the container. A returned child retains native shared ownership, independent of the group handle. |
| Low-level const child inspection | Unsafe `ConstTransform::with_readonly_raw_handle` | A scoped typed FFI handle shares the native object without a transform copy. The caller must invoke only native const operations, prevent concurrent mutation, and keep the handle and derived pointers within the callback. It is not an editable transform. |
| Const transform metadata | Unsafe `ConstTransform::with_readonly_metadata` | Calls the native const overload without copying the transform or metadata. Aliases must not mutate the tree during the callback; borrowed metadata and child handles must not escape it. Types with no metadata return `None`. |
| Native `createEditableCopy` | `create_editable_copy` | Copies the group container and metadata, **not** child transforms. |
| Independent graph copy | `create_deep_copy` | Copies each distinct built-in transform once, preserves sharing within the copied DAG, and rejects cycles/null children. Groups are reconstructed as native OCIO groups; external custom subclasses may have additional state or copy semantics. Explicit opt-in; no pixel-path cost. |
| Remove and clear children | `remove_transform`, `clear_transforms` | OCIO has no public in-place removal; the binding rebuilds this wrapper's group. Existing native references keep the old group. This is an unresolved semantic difference, not in-place mutation. |
| Metadata, direction and write formats | Corresponding group methods | Metadata handles retain their owner; external native mutation requires coordination. |

Supporting in-place removal for every native group would require either an
upstream public mutation API or a carefully validated custom implementation of
the native group interface. A wrapper must preserve shared ownership, metadata,
editable-copy behavior, validation, writing, and 2.4--2.6 ABI compatibility;
replacing the underlying object silently is not equivalent. The existing
behavior is documented and tested until that design is resolved.
