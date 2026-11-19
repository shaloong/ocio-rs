# Native custom creator interop check

This unpublished manual fixture verifies importing a C++ `GpuShaderCreator`
subclass that is not a `GpuShaderDesc`, native shared ownership, custom clone,
base settings, dynamic property sharing, and representative GPU shader extractions. It asserts that the
compiled header and linked runtime versions match. It does not execute the
generated shader or prove general subclass correctness.
It also imports a C++ `ConfigIOProxy` subclass and checks virtual dispatch,
binary payloads, exceptions, config loading and ownership retained by
contexts and configs.
It also injects exceptions from C++ primary and tone transform value getters,
checking bridge error propagation, unchanged buffers, subsequent recovery,
and high-level Rust errors through a temporarily installed native config.
The fixture similarly checks all exposure/contrast getters when a native
subclass throws, followed by a successful query on a normal transform.
The same fixture verifies CPU processing with C++-owned image descriptors:
packed RGBA in place, separate packed input and planar output, and an
independent `ImageDesc` subclass with irregular channel offsets.
It also verifies that an external image getter throwing either native OCIO
exception with an empty diagnostic stays an error and leaves pixels untouched.
The fixture injects malformed RGB and, with `v2_5`, hue dynamic properties with null
curves and checks that raw bridge count and mutation calls report errors.
With baseline features linked to OCIO 2.5 or newer, it additionally checks that
a native hue property is rejected by fallible type and double-value operations.

Use a matching static OCIO installation and toolchain, for example:

```powershell
$env:OCIO_INSTALL_DIR = 'C:\path\to\ocio-install'
$env:SYSTEM_DEPS_OPENCOLORIO_LINK = 'static'
cargo run --locked --manifest-path tools/creator-validation/Cargo.toml
```

For OCIO 2.5 or later, add `--features v2_5` to exercise binding configuration.

This is a manual compatibility/release check, not an additional CI job.
