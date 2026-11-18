# Native exception translation

OpenColorIO 2.4.1, 2.5.2, and 2.6.0 contain identical public declarations
for `Exception` and `ExceptionMissingFile`. The Rust API represents these
as error values rather than constructing or unwinding C++ exception objects.

| Native behavior | Rust translation on checked bridge paths |
| --- | --- |
| `Exception(const char*)` and inherited `what()` | `OcioError::Ocio` with the native diagnostic text. |
| Derived `ExceptionMissingFile` | `OcioError::MissingFile`, classified by dynamic C++ type rather than the diagnostic text. Direct `CDLTransform::CreateFromFile` with an out-of-range index can produce this exception even for an existing file. |
| Empty native `what()` | Preserved as an error with an empty message. The bridge tracks presence separately from the contents of the message. |
| Bridge `std::exception` or unknown C++ exception | Generic `OcioError::Ocio`; an unknown exception uses a fallback diagnostic. |

Native behavior may change without a signature change. A nonempty config
path that fails to open produces the base exception on 2.4.1 and 2.5.2,
and the missing-file exception on 2.6.0. The binding does not guess the
category from message text on older versions. When native `FileTransform`
wraps a format-specific lookup failure in the base exception, the binding
preserves that public entry point's base category.

The last-error slot is thread-local. The raw ABI reports kind 1 for
missing-file, and 0 for both generic and absent errors. A null message
pointer distinguishes the absent case; a present empty string is non-null.
Rust consumes checked errors so they are not visible to the next operation.
Validation wrappers using `validation_status` deliberately classify errors
by operation as `ValidationFailed`. Legacy convenience wrappers and bridge
entry points that intentionally return fallback values are not proven
fallible by this audit.

The existing missing-file suite covers config paths, LUT lookup, unavailable
CDL corrections, successful recovery, and thread-local consumption. The
manual C++ creator fixture throws both native types with empty messages from
an external image getter and checks propagation through metadata, diagnostics,
and CPU application. Neither check requires a new CI job.
