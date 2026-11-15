# OpenColorIO global API signature audit

Scope: the `Global` section of `OpenColorIO.h` in upstream 2.4.1, 2.5.2,
and 2.6.0. The declarations below are present in all three versions with the
same signatures. This table does not cover global stream operators, app-helper
namespaces, other headers, or prove complete behavioral equivalence.

| Native declaration | Rust API | Evidence and limitations |
| --- | --- | --- |
| `void ClearAllCaches()` | `try_clear_all_caches` | `runtime_helpers_behavior_tests`; instance processor caches are separate. |
| `const char * GetVersion()`; `int GetVersionHex()` | `version`, `version_hex` | `runtime_helpers_behavior_tests`; the text getter returns owned Rust text. |
| `LoggingLevel GetLoggingLevel()`; `void SetLoggingLevel(LoggingLevel)` | `try_logging_level`, `try_set_logging_level` | `runtime_helpers_behavior_tests`; global state and callback reentrancy require care. |
| `void SetLoggingFunction(LoggingFunction)`; `void ResetToDefaultLoggingFunction()`; `void LogMessage(LoggingLevel, const char *)` | `set_logging_callback`, `reset_logging_callback`, `try_log_message` | Callback panic and reentrancy tests exist; native versions before 2.5.2 require the explicitly unsafe, externally synchronized callback installation/reset APIs. |
| `void SetComputeHashFunction(ComputeHashFunction)`; `void ResetComputeHashFunction()` | `set_compute_hash_callback`, `reset_compute_hash_callback` | Both are unsafe because native process-global mutation requires external synchronization; file-transform and panic tests exist. |
| `const char * GetEnvVariable(const char *)`; `void SetEnvVariable(const char *, const char *)`; `void UnsetEnvVariable(const char *)`; `bool IsEnvVariablePresent(const char *)` | `get_env_variable`, `set_env_variable`, `unset_env_variable`, `is_env_variable_present` | Unsafe: OCIO and other process environment users must be externally synchronized. The getter copies OCIO's transient buffer. |
| `ConstConfigRcPtr GetCurrentConfig()`; `void SetCurrentConfig(const ConstConfigRcPtr &)` | `try_const_current_config`, `try_set_const_current_config`; editable convenience variants also exist | `const_current_config_behavior_tests`; shared getters retain a snapshot, while OCIO copies the input when installing it. |
| `const char * ResolveConfigPath(const char *) noexcept` | `resolve_config_path` | `runtime_helpers_behavior_tests`; the result is copied into owned Rust text. |
| `void ExtractOCIOZArchive(const char *, const char *)` | `extract_ocioz_archive` | Error-path tests exist; a successful extraction and its filesystem semantics still need explicit cross-version verification. |

The listed behavioral tests include stub branches; real-native verification
must be assessed separately for each version and platform. This inventory is
one subsystem of the [full-coverage plan](OCIO_2_6_PLAN.md), not a claim that
the rest of the public API has been audited.
