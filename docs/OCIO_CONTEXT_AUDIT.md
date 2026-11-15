# Context interface audit

Scope: `Context` in the OpenColorIO 2.4.1, 2.5.2 and 2.6.0 public headers,
including the overloads used by editable and read-only Rust handles. This is
a signature-family audit and selected behavior evidence, not a claim of full
library parity or exhaustive filesystem and environment coverage.

| Native family | Rust surface | Ownership and behavior |
| --- | --- | --- |
| `Create`, `createEditableCopy` | `Context::create`, `create_editable_copy`; `ConstContext::create_editable_copy` | Editable copy is independent; a const config context remains shared and read-only. |
| Search paths and working directory | Editable setters, clearing, indexed/count and combined getters; const getters | Text is returned owned; native indexed out-of-range results are preserved. |
| String variables, count/index, merge and clear | `set_string_var`, `remove_string_var`, `try_clear_string_vars`, `try_add_string_vars`; editable/const getters | Removal differs from an empty value. A const context can be a merge source. |
| Environment mode and loading | `set_environment_mode`, `environment_mode`, `load_environment` | Loading reads process environment according to native mode. |
| String and file resolution, with or without used variables | `try_resolve_*` and `*_with_used_context`; const counterparts | Returned strings are owned; output context is mutable; missing file errors are preserved. |
| Cache ID, ConfigIOProxy, stream output | `try_cache_id`, proxy accessors, `serialize`; const counterparts | Proxies retain shared native ownership; stream output is diagnostic and may contain sensitive data. |

OCIO 2.4.1 through 2.6.0 clears the context variable map without invalidating
its cached resolutions or cache ID. The bridge follows native clearing with
removal of an absent nonempty variable, triggering OCIO's existing cache
invalidation without adding a variable. The regression covers cached string
and file resolution, used-variable output, and cache identity. This workaround
is on the mutation path only; resolution itself remains a direct native call.
