# Config IO proxy signature audit

Scope: `ConfigIOProxy` in the exact upstream 2.4.1, 2.5.2 and 2.6.0
`OpenColorIO.h` headers. Each version declares a public constructor, virtual
destructor, and the same three pure virtual methods below. This is a
subsystem inventory, not evidence of full-library parity.

| Native declaration | Rust surface | Ownership and behavior |
| --- | --- | --- |
| `std::vector<uint8_t> getLutData(const char *) const` | `ConfigIOProvider::lut_data`; `ConfigIOProxy::read_lut_data_cstr` / `with_lut_data_cstr` | Direct reads do not query the hash first. Empty bytes are ambiguous; native OCIO owns its returned vector and the visitor borrows only for the callback. |
| `std::string getConfigData() const` | `ConfigIOProvider::config_data_bytes`; `ConfigIOProxy::try_config_data_bytes` | Bytes and embedded NULs are preserved. Text adapters are conveniences, not a native UTF-8 guarantee. |
| `std::string getFastLutFileHash(const char *) const` | `ConfigIOProvider::fast_lut_file_hash`; `ConfigIOProxy::read_fast_lut_file_hash_cstr` / `with_fast_lut_file_hash_cstr` | Binary bytes are preserved; OCIO interprets an empty hash as file unavailable. |

`ConfigIOProxy::from_provider` provides the native subclass: OCIO can retain
the provider after Rust drops the original handle, and provider access must be
thread-safe. Borrowed provider bytes are copied into the C++ return value
required by OCIO. Rust visitor callbacks borrow that native value without a
second payload copy. Visitor and provider errors or panics are contained at
the bridge; neither may reenter the same OCIO proxy. Native return data and
hash calls are separate and do not form an atomic snapshot.

The `try_lut_data_cstr` convenience uses the native fast hash as an availability
check for custom providers, whereas the direct `read_*` / `with_*` methods
invoke each virtual operation independently. The built-in in-memory proxy
also exposes registration and presence helpers not present in OCIO's abstract
interface; stored data can exist with an empty hash even though OCIO treats it
as unavailable. Returned proxy handles retain native ownership independently
of the config or context handle that supplied them.

Evidence: `tests/config_io_proxy_behavior_tests.rs` covers native processing,
binary and empty hashes, non-UTF-8 paths, direct and borrowed reads, provider and
visitor errors/panics, retained ownership, and concurrent in-memory snapshots.
The visitor lifetime also has a compile-fail documentation test. Real native
tests are necessary: stub coverage does not exercise OCIO virtual dispatch.
The complete suite passed against real 2.4.1, 2.5.2 and 2.6.0 builds on
Windows during this audit; this does not replace cross-platform CI results.
