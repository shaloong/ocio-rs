# Core transform signatures

The declarations of the eleven classes below are unchanged across the
2.4.1, 2.5.2 and 2.6.0 headers after removing comments and whitespace. This
maps callable operations, not a requirement to mirror C++ overload spelling.
Factories and handle destruction cover the native constructors/destructors;
the common transform handle covers direction, validation, type and copying.

| Native class | Callable mapping |
| --- | --- |
| AllocationTransform | Allocation enum, signed `int` count, float variables and checked output slice; native validation and editable copy. |
| BuiltinTransform | Style, description, factory and native registry enumeration; style updates delegate to OCIO. |
| CDLTransform | Default/file/group factories; style, ID, description, saturation and luma; fixed-size slope/offset/power (3) and SOP (9) arrays; equality and metadata. |
| ColorSpaceTransform | Source/destination and data bypass; native validation through the base transform. |
| DisplayViewTransform | Source, display, view, looks and data bypass flags; native validation through the base transform. |
| ExposureContrastTransform | Style, exposure/contrast/gamma values and independent dynamic toggles; pivot, log step and mid-gray; equality and metadata. |
| FileTransform | Source, CCC ID, CDL style, interpolation and native supported-format enumeration; direction and validation. |
| FixedFunctionTransform | Style-only and parameterized factories, native enum translation, `size_t` parameter count, checked caller buffers, parameter updates, equality and metadata. |
| GradingPrimaryTransform and GradingToneTransform | Native style/value, dynamic state/toggles, metadata and equality; values use the corresponding Rust value structures and native validation. |
| LookTransform | Source/destination, look expression, bypass flag and static result-color-space resolution using the native context and look parsing. |

Existing typed and read-only handles cover immutable access where native
objects are const. Raw calls retain their documented pointer/length contracts;
safe methods use fixed arrays or validate output capacity before crossing the
bridge. A diagnostic stream insertion operator or a second wrapper for an
equivalent native overload is not required. The 2.6 fixed-function enum
insertion is separately covered by the [enum audit](OCIO_ENUM_ABI_AUDIT.md).
