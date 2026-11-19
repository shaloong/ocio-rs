# Base, log and range transform signatures

The `Transform`, `LogTransform`, `LogAffineTransform`,
`LogCameraTransform`, and `RangeTransform` declarations match across
the pinned 2.4.1, 2.5.2, and 2.6.0 public headers.

| Native signature family | Rust path |
| --- | --- |
| Base editable copy, direction read/write, type query, validation | `TransformHandle` covers editable values; `ConstTransform` provides native read/validate and creates an editable copy for mutation. Type tags are normalized across native minor versions. |
| Base diagnostic stream and native ownership | Native diagnostic text is available through the existing generic transform handles. Each declared subclass formatter need not have a parallel Rust method. Shared handles and `Drop` manage destruction; deleted/protected native construction is not a callable factory. |
| Log, log-affine and range default factories; log-camera required break argument | Existing constructors. The camera constructor takes the same three-element input as native; no implicit break is required in Rust. |
| Log base and affine/camera RGB slope/offset values | Existing scalar and fixed-array getters/setters. The native state remains authoritative; validation is a separate operation. |
| Camera break and optional linear slope | Existing required break get/set and optional slope get/set/unset. A native `false` now leaves caller-owned output unchanged; the Rust getter returns `None`. No copied uninitialized data on the ordinary absent path. |
| Range endpoints, presence and unset; style and file bit depths | Existing accessors preserve the native endpoint/presence distinction. Use `has_*` for presence rather than interpreting a numeric getter as an optional value. |
| Metadata and exact equality | Existing typed methods and retained metadata handles. |

The log-camera regression verifies the raw buffer before setting the slope,
when set, and after clearing it. The check runs in the existing transform
suite; no new CI job or public API is needed.
The five existing transform suites passed on Windows with each pinned
version (16 tests per version).

The 2.6 signatures for `LogAffineTransform` and `LogCameraTransform` use
three-element `double` array references for log-side slope and offset and
linear-side slope and offset. The camera also requires a three-element break
in its factory, provides get/set for that break, and has a distinct optional
three-element linear slope with get/set/unset. The bridge converts the C ABI
buffers into fixed C++ arrays; the Rust methods preserve the RGB length and
copy no heap storage. Both classes expose the common base operations, equality,
and const/editable metadata. The strengthened native behavior tests use distinct
values for every field and channel, including the optional slope, and passed
with 2.4.1, 2.5.2, and 2.6.0. This verifies those specific mappings, not all
possible processor or numerical behavior.
