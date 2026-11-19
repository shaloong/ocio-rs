# LUT, matrix, and exponent transform signatures

The declarations of `Lut1DTransform`, `Lut3DTransform`, `MatrixTransform`,
`ExponentTransform`, and `ExponentWithLinearTransform` match in the pinned
OpenColorIO 2.4.1, 2.5.2, and 2.6.0 headers (ignoring comments and whitespace).

| Native signature family | Rust path |
| --- | --- |
| LUT default and sized factories, including 1D half domain | `create`, `create_with_length`, `create_with_grid_size`. Native code owns the LUT and validates its dimensions. |
| LUT resize/size and per-entry float get/set | Existing 64-bit Rust methods cover the native `unsigned long` domain. The bridge rejects values outside that domain, including legacy pointer-encoded indices on LLP64, before touching the LUT. Native errors remain distinguishable from a checked Rust out-of-range lookup returning `None`. |
| LUT file depth, interpolation, 1D hue adjustment and half/raw flags | Existing getters and checked setters call native operations. The half-domain length invariant is checked by native validation. |
| 3D bulk data (binding convenience) | The existing flat RGB triplets use red-coordinate-fastest order. No duplicate native bulk method exists in these headers. |
| Matrix and offset get/set; normalized file depths | The fixed-size Rust arrays match the 16/4-element native inputs and outputs, with checked methods. |
| Matrix static fit, identity, saturation, scale and view | Existing factories return a native transform. `fit_into`, `sat_into`, `scale_into`, and `view_into` call the corresponding native static function directly into caller-owned arrays; raw FFI also exposes the native identity array operation. |
| Exponent four values; with-linear gamma and offset; negative styles | Fixed-size Rust arrays and checked setters preserve native validation. |
| The five classes' type, equality, metadata and lifetime | Base transform operations cover type, direction, validation and editable copies. Typed equality and shared metadata retain the existing native behavior. Native constructors/destructors are represented by factory and handle ownership. |

The Windows regression exercises both legacy pointer and modern integer LUT
dimensions through the raw bridge, plus read/write indices. A rejected value
leaves both LUT contents and caller read buffers unchanged. It also verifies
3D batch ordering against independently addressed grid coordinates.
The five existing suites passed on Windows against each of 2.4.1, 2.5.2,
and 2.6.0 (33 tests per version).

The individual LUT factory and value signatures have also been checked against
the 2.6 header: the 1D factory accepts `unsigned long` length plus half-domain,
while the 3D factory accepts one `unsigned long` grid size. `try_value` and
`set_value` use one 1D coordinate or three 3D coordinates and preserve the
three native `float` channels. An out-of-range safe read is `None`; callers
requiring OCIO's error for the same read can use the raw entry point. The 3D
bulk convenience copies to or from `f64` triplets in red-fastest order; native
OCIO internally stores its array blue-fastest. It is not a borrowed native
array, and the single-entry paths do not allocate that bulk snapshot.

The individual matrix and exponent signatures were checked against the 2.6
header and bridge. Matrix get/set calls use 16 `double` values for the matrix
and four for the offset; file bit-depths and the two metadata overloads remain
separate. `Fit` takes four RGBA input arrays, `Sat` a scalar and three luma
coefficients, `Scale` four channel factors, and `View` a mutable four-element
integer channel mask plus three luma coefficients. The safe direct helpers
write into caller arrays; the raw FFI supplies `Identity` without constructing
a transform. OCIO may modify `Fit` outputs before reporting an error.

The basic exponent uses a four-double value array, while exponent-with-linear
uses separate four-double gamma and offset arrays. Both forward negative style,
native equality, and editable/const metadata operations. The bridge copies each
four-element buffer to a C++ array reference, preserving the native signature;
it does not allocate. Existing behavior suites exercise native value round
trips, invalid negative styles, copying, and processors. The strengthened matrix
fit test now requires successful construction and checks nonzero, distinct input
bounds against independently calculated output. Matrix and both exponent suites
passed on Windows native 2.4.1, 2.5.2 and 2.6.0.
