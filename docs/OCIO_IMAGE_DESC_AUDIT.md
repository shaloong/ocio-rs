# Image descriptor API audit

Scope: the C++ `ImageDesc`, `PackedImageDesc` and `PlanarImageDesc`
declarations in native OpenColorIO 2.4.1, 2.5.2 and 2.6.0. The three header
declaration blocks match exactly; this does not imply identical CPU processing
behavior across versions.

| C++ API family | Rust surface and ownership |
| --- | --- |
| Abstract base constructor, virtual destruction | `NativeImageDesc` owns the native implementation. Fixed-layout constructors borrow the pixels; `from_provider` owns a Rust provider that may own or borrow the pixels. Raw interop never destroys C++-owned descriptors. |
| Four packed constructors (3/4 channels or ordering, default or explicit strides and bit depth) | `NativeImageDesc::packed` uses explicit geometry and `ChannelOrdering`; RGB/RGBA encode the numeric channel counts. Rust callers provide concrete strides rather than forwarding the native `AutoStride` sentinel. Checked storage is required. |
| Two planar constructors (default or explicit strides and bit depth) | `NativeImageDesc::planar` uses explicit shared signed strides, three borrowed channel slices, and optional alpha. It does not silently convert bit depth. |
| `getWidth`, `getHeight`, `getBitDepth`, x/y stride, `isRGBAPacked`, `isFloat` | `NativeImageDescInfo` reports native results without returning pixel addresses. The packed predicate is not a class test. |
| `getRData`, `getGData`, `getBData`, `getAData`, packed `getData` | `NativeImageDataPointers` returns the original first-pixel pointers through one FFI call. Packed storage order need not match RGB order. No allocation, pixel copy, or Rust reference is introduced. Null alpha is preserved; packed data is null for a non-packed descriptor. For externally owned descriptors the caller must uphold the native lifetime and synchronization contracts. |
| Packed channel ordering, channel count, channel stride | `NativeImageLayout::Packed` retains these separately from per-pixel and row strides. Native enum discriminants are reported without transmuting them. |
| `operator<<(ostream&, const ImageDesc&)` | Diagnostic text uses the upstream operator and owns its result on the Rust side. It can contain buffer addresses and is not a serialization format. |

The safe owned descriptor methods require enough space for native pointer
stepping. The existing slice-based CPU methods handle supported unpadded
layouts separately; weakening descriptor validation is not a substitute.
The low-level RGB and RGBA batch bridges check byte and row strides before
narrowing them to `ptrdiff_t`. Windows i686 and x86-64 stub regressions cover
the preflight; they do not establish 32-bit native OCIO runtime behavior.
The shared check uses a conservative fast bound to avoid division for ordinary
strides and runs once per batch, not per pixel.
The safe fixed-layout custom descriptor covers irregular interleaved layouts
over one checked slice. For other layouts, `ImageDescProvider` implements
every base getter in Rust. It is an unsafe trait because OCIO trusts the
composite layout, pointer provenance, storage lifetime, and fast-path
predicates. The provider must describe one coherent image throughout each
operation. OCIO queries it during layout setup, not inside pixel loops.
Callback failure from a panic becomes a native error; native destruction
releases its provider exactly once. Existing fixed-layout descriptors do not
allocate Rust callback state or dispatch through Rust. Externally implemented
C++ subclasses remain usable through raw interop.

The native interface exposes one x stride and one y stride shared by all
channels. Independent per-channel strides require a different processing
abstraction, not a different implementation of these getters.

The CPU behavior suite passed on Windows against all three versions (42 tests
each). It covers packed BGR storage order, negative planar stride, irregular
custom offsets, nullable pointers, diagnostics, and pixel processing. The
Rust-provider regression exercises native in-place processing and conversion
between two providers, recovery after a getter panics, and exactly-once
destruction. Its stub case verifies cleanup when native construction fails. The
manual C++ fixture additionally verified non-null alpha addresses, packed
storage base and native diagnostics on externally owned 2.4.1 descriptors.
Neither requires a new CI job.
