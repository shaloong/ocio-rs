# ColorSpace signature audit

Scope: public `ColorSpace` declarations in OCIO 2.4.1, 2.5.2 and 2.6.0.
This inventory covers signature families, not every behavior or the rest of
the library. The interop ID and interchange attributes require the relevant
2.5.1-or-newer interface feature.

| Native signature family | Rust surface | Remaining work |
| --- | --- | --- |
| `Create` overloads, `createEditableCopy` | `create`, `create_with_reference_space`, editable-copy methods | Actual reference space and copy independence are covered by behavior tests. |
| Name, aliases, family, equality group, description | Editable accessors and mutation; borrowed const metadata | Several legacy editable getters are infallible convenience methods. |
| Interop ID and interchange attributes | Editable accessors, const ID and individual const attribute; owned map snapshots on both handles | Snapshots preserve native errors and visit one native map rather than repeatedly rebuilding it. |
| Bit depth, reference space, categories, encoding, data | Editable accessors and mutation; const readers | Legacy editable getters may collapse native errors. |
| Allocation and variables | Editable vector access and setter; caller-buffer reads on editable and const handles | `try_allocation_vars_into` and `ConstColorSpace::allocation_vars_into` require an exactly sized buffer; the native variable count is a signed `int`. |
| `getTransform`, `setTransform` | Shared const and editable-copy getters; editable and const setters; directional clear | OCIO copies transforms when setting; retained const getters outlive their source. |

`tests/color_space_behavior_tests.rs` checks representative metadata, native
configuration/processor behavior and transform copy/clear ownership. It does
not establish every error or lifetime combination.

For the index and buffer signatures, OCIO uses `size_t` for alias count and
index, but `int` for category count/index and allocation-variable count. The
fallible alias and category queries preserve those respective widths. The
allocation setter checks that the Rust slice length fits `int` before calling
`setAllocationVars(count, values)`; both caller-buffer getters require the
native count, rather than allowing OCIO to write beyond the provided slice.

The 2.6.0 public declarations were additionally checked individually against
the bridge and Rust entry points:

| Native declarations | Callable path and argument/ownership distinction |
| --- | --- |
| `Create()`, `Create(ReferenceSpaceType)`, `createEditableCopy()` | `create`, `create_with_reference_space`, and editable-copy methods on both handle types. |
| `getName` / `setName`, `getFamily` / `setFamily`, `getEqualityGroup` / `setEqualityGroup`, `getDescription` / `setDescription` | Editable getters return owned strings and setters forward strings; the retained const handle borrows the corresponding native text. |
| `getNumAliases`, `getAlias(size_t)`, `hasAlias`, `addAlias`, `removeAlias`, `clearAliases` | `try_num_aliases`, `try_alias_at`, `try_has_alias`, and the three editable mutations. Const reads preserve `size_t` and may borrow alias text. |
| `getInteropID` / `setInteropID`, `getInterchangeAttribute`, `setInterchangeAttribute`, `getInterchangeAttributes` | `v2_5`-gated ID and attribute methods; const single-value reads borrow native text, while the native map return is an owned snapshot. |
| `getBitDepth` / `setBitDepth`, `getReferenceSpaceType`, `getEncoding` / `setEncoding`, `isData` / `setIsData` | Editable scalar and text accessors; const reads are available separately. The reference-space type is selected by the factory, not a setter. |
| `hasCategory`, `addCategory`, `removeCategory`, `getNumCategories`, `getCategory(int)`, `clearCategories` | Fallible editable operations and const queries; the const unsigned index is checked before conversion to native `int`. |
| `getAllocation` / `setAllocation`, `getAllocationNumVars`, `getAllocationVars(float *)`, `setAllocationVars(int, const float *)` | Editable scalar methods and checked slice reads/writes; const allocation and caller-buffer reads retain the same native values. |
| `getTransform(ColorSpaceDirection)`, `setTransform(const ConstTransformRcPtr &, ColorSpaceDirection)` | Shared const getter and editable/const setters preserve the direction; `try_clear_transform` passes a null shared pointer to clear the selected native slot. |

The native copy constructor and assignment are deleted, the constructors are
private, and native destruction is represented by handle release. The stream
operator is diagnostic, not a separate color-management operation. This check
does not establish behavior under arbitrary concurrent native aliases.
