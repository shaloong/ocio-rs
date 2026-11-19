# Config IO proxy signature audit

Scope: `ConfigIOProxy` in the exact upstream 2.4.1, 2.5.2 and 2.6.0
`OpenColorIO.h` headers. Each version declares a public constructor, virtual
destructor, and the same three pure virtual methods below. This is a
subsystem inventory, not evidence of full-library parity.

| Native declaration | Rust surface | Ownership and behavior |
| --- | --- | --- |
| `std::vector<uint8_t> getLutData(const char *) const` | `ConfigIOProvider::lut_data`; `ConfigIOProxy::read_lut_data_cstr` / `with_lut_data_cstr` | Direct reads do not query the hash first. Empty bytes are ambiguous; native OCIO owns its returned vector and the visitor borrows only for the callback. |
| `std::string getConfigData() const` | `ConfigIOProvider::config_data_bytes`; `ConfigIOProxy::try_config_data_bytes` / `with_config_data` | Bytes and embedded NULs are preserved. The visitor borrows OCIO's result without a second payload copy. Text adapters are conveniences, not a native UTF-8 guarantee. |
| `std::string getFastLutFileHash(const char *) const` | `ConfigIOProvider::fast_lut_file_hash`; `ConfigIOProxy::read_fast_lut_file_hash_cstr` / `with_fast_lut_file_hash_cstr` | Binary bytes are preserved; OCIO interprets an empty hash as file unavailable. |

`ConfigIOProxy::from_provider` provides the native subclass: OCIO can retain
the provider after Rust drops the original handle, and provider access must be
thread-safe. Borrowed provider bytes are copied into the C++ return value
required by OCIO. Rust visitor callbacks borrow that native value without a
second payload copy. Visitor and provider errors or panics are contained at
the bridge; neither may reenter the same OCIO proxy. Native return data and
hash calls are separate and do not form an atomic snapshot.
This panic containment requires unwinding; `panic=abort` cannot be caught.
If dropping a caught panic payload itself panics, the bridge catches that
second panic and intentionally leaks its payload rather than unwinding through
the C ABI. An isolated child-process regression covers provider reads, visitors,
and provider destruction with such a payload.
The bridge rejects a null nonempty payload or a length beyond `ptrdiff_t`
before pointer arithmetic and records failure even if a raw native provider
ignores the write callback's return value.

`unsafe ConfigIOProxy::from_native_shared_ptr` also accepts an existing C++
`ConfigIOProxyRcPtr` compiled against the matching native ABI. It copies the
shared pointer rather than the proxy or data. The caller must ensure the proxy
and any externally owned state remain valid and safe for native concurrent use
until all retaining configs, contexts and other owners have released it. The
manual native interop fixture checks a C++ subclass, null/empty shared pointers,
binary payloads, exception propagation and retained ownership on Windows with
native 2.4.1, 2.5.2 and 2.6.0; this is not a proof for arbitrary subclasses.

The `try_lut_data_cstr` convenience uses the native fast hash as an availability
check for custom providers, whereas the direct `read_*` / `with_*` methods
invoke each virtual operation independently. The built-in in-memory proxy
also exposes registration and presence helpers not present in OCIO's abstract
interface; stored data can exist with an empty hash even though OCIO treats it
as unavailable. Returned proxy handles retain native ownership independently
of the config or context handle that supplied them.

OCIO 2.4--2.6 keeps a process-global file-hash cache keyed by path, not by
proxy. Two proxies assigning different content or availability to the same path
may therefore share a stale cached hash, even across separate contexts. OCIO
also keeps a per-context resolved-path cache. The Rust binding invalidates the
latter when a proxy is attached or detached, without flushing global state.
If a path's meaning changes, applications must coordinate with other OCIO users
and explicitly call `try_clear_all_caches`, then reattach the proxy to each
affected context/config to invalidate its local path cache. This also applies
when a mutable provider changes in place. Prefer stable globally unique paths
for different assets; local invalidation alone cannot isolate proxy namespaces.

Evidence: `tests/config_io_proxy_behavior_tests.rs` covers native processing,
binary and empty hashes, non-UTF-8 paths, direct and borrowed reads, provider and
visitor errors/panics, retained ownership, and concurrent in-memory snapshots.
The visitor lifetime also has a compile-fail documentation test. Real native
tests are necessary: stub coverage does not exercise OCIO virtual dispatch.
The complete suite passed against real 2.4.1, 2.5.2 and 2.6.0 builds on
Windows during this audit; this does not replace cross-platform CI results.
The provider-error regression checks that a failed hash lookup does not poison
a subsequent direct or borrowed LUT read, a failed LUT read does not poison a
subsequent direct hash read, and both failures leave the caller's copy buffer
untouched. These are separate native operations; they are not an atomic snapshot.
