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

For primary and tone transform value reads, `try_value` returns native bridge
errors rather than silently substituting defaults. The existing `value`
compatibility method retains its style-default fallback on failure. Both use a
fixed-size stack buffer; the native getter may still perform its own work.
The manual C++ subclass fixture makes each native getter throw and checks the
raw bridge error message, unchanged output buffer, and subsequent successful
read on Windows with native 2.4.1, 2.5.2 and 2.6.0. This exercises the bridge
boundary. The same standalone fixture also installs the custom transforms in
a temporary native config, obtains editable Rust handles and verifies that
both high-level `try_value` calls return the native error. It restores the
process-global config after the check and does not test arbitrary subclasses.
The same fixture constructs primary and tone values in native C++ with
individually selected fields, installs them in a temporary config, and compares
all 34 and 31 Rust fields against independent expected values. It passed on
Windows with native 2.4.1, 2.5.2 and 2.6.0. This checks native-to-Rust field
mapping independently of a binding-side write/read round trip.

Exposure/contrast getters have matching `try_*` queries for style, all six
scalar values and the three dynamic flags. They preserve native exceptions;
legacy infallible getters retain their fallback behavior. A C++ subclass that
throws from each getter was exercised through the high-level Rust methods on
Windows with native 2.4.1, 2.5.2 and 2.6.0, and the normal round-trip suite
passed on the same versions. Stub fallible queries report that native OCIO is
unavailable. This does not establish behavior for every external subclass.

CDL style, ID and first SOP description also have fallible queries, in
addition to the existing fallible numeric queries. The original infallible
methods retain their fallback values. A native subclass throwing from those
three getters was tested through the Rust methods on Windows with OCIO 2.4.1,
2.5.2 and 2.6.0; the normal CDL behavior suite passed against 2.6.0.

### Individual factory and buffer checks

* `AllocationTransform` has one public factory, `Create()`. Its remaining
  class-specific signatures are `getAllocation()`, `setAllocation(Allocation)`,
  `getNumVars() -> int`, `getVars(float *)`, and
  `setVars(int, const float *)`. These map respectively to the typed allocation
  accessors, signed count, caller-buffer read, and slice setter. The setter
  checks conversion of the slice length to `int`; the safe output method
  verifies the exact count before the native write. Direction, type,
  validation, and editable copying are inherited or overridden base-transform
  operations. Copy construction and assignment are not public native paths.
  The native behavior test checks copy independence, an unchanged undersized
  output buffer, and clearing the variable list. The class does not declare
  format-metadata access; the compatibility helper may therefore return `None`.
* `BuiltinTransform` adds `Create()`, `getStyle()`, `setStyle(const char *)`,
  and `getDescription()` to the common transform operations. The Rust methods
  call these native signatures; setting a style preserves native errors, and
  the copied style and description strings do not borrow mutable native
  storage. Enumeration comes from the separate native registry, not from
  additional `BuiltinTransform` factory overloads. It has no class-specific
  format-metadata method or public copy constructor.
* The 2.6 `ColorSpaceTransform` instance fields are source, destination and
  data-bypass; `DisplayViewTransform` adds display, view and looks-bypass.
  Their Rust setters forward the native fields, and the common transform path
  supplies type, direction, validation and copying. `FileTransform` forwards
  source, CCC ID, CDL style and interpolation, and exposes the four native
  reader-format queries. `ExposureContrastTransform` forwards style,
  exposure/contrast/gamma, their three independent dynamic toggles, pivot,
  log exposure step and log mid-gray, plus equality and metadata. Legacy
  infallible getters retain their existing fallback behavior; this audit does
  not claim that those getters surface every native error. The four real-native
  2.6 behavior suites passed on Windows (13 tests total), covering representative
  processor, field, copy, dynamic and validation paths.
  The four non-grading transform declarations were also checked individually:
  `ColorSpaceTransform` declares source/destination and data-bypass pairs;
  `DisplayViewTransform` declares source, display, view, looks-bypass and
  data-bypass pairs; `FileTransform` declares source, CCC ID, CDL style and
  interpolation pairs plus count, indexed name/extension and extension-support
  static queries; `LookTransform` declares source/destination, looks and
  skip-conversion pairs plus `GetLooksResultColorSpace(config, context, looks)`.
  Each has a corresponding typed or raw call, in addition to the common
  factory, copy, direction, type and validation paths. Their four behavior
  suites passed on Windows against native 2.4.1 and 2.6.0. This checks declared
  signatures and representative behavior, not every possible input or file.
* `CDLTransform::Create`, `CreateFromFile(src, cccId)`, and
  `CreateGroupFromFile(src)` map to the three corresponding Rust factories.
  File selection and group loading have native behavior tests. Slope, offset,
  power and luma use three doubles; SOP uses nine. Style, saturation, ID,
  first SOP description, equality, and both metadata access modes are covered
  by the typed or existing const/raw handle paths. The shared transform path
  supplies direction, validation, type and editable copying.
* Both `FixedFunctionTransform::Create` shapes are callable through `create`
  and `create_with_params`. The former uses the bridge's parameterized native
  overload with an empty list. In the checked 2.4.1, 2.5.2 and 2.6.0 implementations,
  both no-parameter construction paths initialize the same empty parameter
  vector and validate it. OCIO 2.4.1 performs pointer arithmetic even for an
  empty range in both the factory and `setParams`, so the bridge substitutes
  a valid address when the count is zero and rejects a null pointer for a
  nonempty range before calling OCIO.
  The native `size_t` count is retained by `try_num_params_usize`, input
  parameters use a slice, and `try_params_into` verifies the caller's buffer
  length before the native write. Style, equality, metadata and common
  transform operations are also exposed. Existing native tests exercise
  parameter round trips, error handling and processor execution; they do not
  exhaust every style or every possible parameter value.

Existing typed and read-only handles cover immutable access where native
objects are const. Raw calls retain their documented pointer/length contracts;
safe methods use fixed arrays or validate output capacity before crossing the
bridge. A diagnostic stream insertion operator or a second wrapper for an
equivalent native overload is not required. The 2.6 fixed-function enum
insertion is separately covered by the [enum audit](OCIO_ENUM_ABI_AUDIT.md).
