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

The 2.6 declarations were checked individually against the bridge and Rust
call shapes:

| Native declarations | Callable path and ownership |
| --- | --- |
| `Baker::Create()`, `createEditableCopy()` | `Baker::create` and `create_editable_copy`; the native copy retains its config and copies settings and metadata. |
| `getConfig() const`, `setConfig(const ConstConfigRcPtr &)` | `const_config` retains the returned shared config; `config` explicitly makes an editable copy; editable and const setters call native `setConfig`. |
| `getFormat` / `setFormat`, `getInputSpace` / `setInputSpace`, `getShaperSpace` / `setShaperSpace`, `getLooks` / `setLooks`, `getTargetSpace` / `setTargetSpace` | Corresponding text accessors and setters; returned Rust text is owned. |
| `getDisplay()`, `getView()`, `setDisplayView(const char *, const char *)` | Independent display/view getters and a setter preserving the native argument order. |
| `getShaperSize` / `setShaperSize(int)`, `getCubeSize` / `setCubeSize(int)` | Signed `i32` accessors and setters preserve native sizes and the `-1` default. |
| Const and editable `getFormatMetadata()` | The owned metadata handle references native Baker metadata and retains its owner; the editable path also supports reading, so a duplicate const spelling is not required. |
| `bake(std::ostream &) const` | `bake_to_writer` streams through a bounded callback buffer; raw FFI also accepts a native stream under its unsafe ABI contract. |
| `getNumFormats()`, `getFormatNameByIndex(int)`, `getFormatExtensionByIndex(int)` | Static signed-index methods; invalid indices map to `None`. |
| `ProcessorMetadata::Create()`, `getNumFiles()`, `getFile(int)`, `getNumLooks()`, `getLook(int)`, `addFile(const char *)`, `addLook(const char *)` | `ProcessorMetadata` exposes the corresponding editable methods; retained processor metadata also provides const counts and borrowed indexed text. |

The private constructors, deleted copy operations and destructors intended
for native ownership do not require additional Rust callables. This mapping
does not by itself establish every writer failure or third-party stream behavior.
The corresponding Windows native 2.6.0 behavior suites passed 10 Baker and
4 processor-metadata tests, including callback failures, Baker mutation during
output and retained/copy ownership.
