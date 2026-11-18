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
| Matrix static fit, identity, saturation, scale and view | The existing Rust factories return a native transform populated with the generated coefficients and offsets. Equivalent raw array outputs do not require a second high-level factory family. |
| Exponent four values; with-linear gamma and offset; negative styles | Fixed-size Rust arrays and checked setters preserve native validation. |
| The five classes' type, equality, metadata and lifetime | Base transform operations cover type, direction, validation and editable copies. Typed equality and shared metadata retain the existing native behavior. Native constructors/destructors are represented by factory and handle ownership. |

The Windows regression exercises both legacy pointer and modern integer LUT
dimensions through the raw bridge, plus read/write indices. A rejected value
leaves both LUT contents and caller read buffers unchanged. It also verifies
3D batch ordering against independently addressed grid coordinates.
The five existing suites passed on Windows against each of 2.4.1, 2.5.2,
and 2.6.0 (33 tests per version).
