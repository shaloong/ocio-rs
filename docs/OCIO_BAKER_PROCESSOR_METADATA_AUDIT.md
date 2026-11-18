# Baker and processor metadata signatures

The public declarations of `Baker` and `ProcessorMetadata` match across
OpenColorIO 2.4.1, 2.5.2, and 2.6.0 (ignoring comments and whitespace).

| Native operation | Rust path |
| --- | --- |
| Baker factory, editable copy and destruction | Native factories and managed handles. |
| Baker get/set config | `const_config` retains the native value, `config` creates an editable copy, and both setter paths delegate to native `setConfig`, which copies the input config. |
| Baker metadata, format, source/shaper/target, looks, display/view and LUT sizes | Existing native-backed accessors and setters; source and target validation stays in OCIO. |
| Baker static format count, name and extension | Existing `i32` queries match native `int`; the native empty string is the invalid-index sentinel. |
| Baker `bake(std::ostream&)` | Raw native stream entry, or `bake_to_writer` with bounded buffering. Callback baking makes a native settings/metadata snapshot once and retains its config pointer. This keeps pointers valid if a writer mutates the original Baker during output, without buffering the whole LUT. |
| Processor metadata create, file/look count, indexed get and append | Existing `ProcessorMetadata` methods match the native integer domain, file deduplication and look ordering. Invalid indices return empty native strings, not exceptions. |
| Processor-owned metadata | Shared const access borrows native text; editable access reconstructs a separate native metadata value. Dropping the processor does not invalidate the shared handle. |

The writer adapter shared by Baker and OCIOZ archive output also contains
panics during Rust I/O error formatting and panic-payload destruction.
Previously written bytes cannot be rolled back. The raw Baker stream
ABI now records native failures in the existing error slot.
