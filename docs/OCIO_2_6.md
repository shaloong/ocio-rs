# OpenColorIO compatibility

The unreleased version supports native OpenColorIO `>= 2.4.1, < 2.7`.
The baseline interface works with that range. Enable `v2_5` for APIs requiring
2.5.1 or newer, or `v2_6` for the 2.6 API; `v2_6` includes `v2_5`. Features
select the Rust interface and do not upgrade the installed native library.

The default vendored source remains 2.5.2. To use 2.6, select `system,v2_6`
with an installed 2.6 library, or `bundled,v2_6` with `OCIO_SOURCE_DIR`
pointing to a compatible 2.6 source tree. See the [README](../README.md) for
backend selection and [migration notes](MIGRATION.md) for breaking changes.

The 2.6 interface includes display and view aliases, color interoperability ID
lookup, HDR compatibility, the HMJ fixed function and CLF ID metadata. The
bindings follow the native API; use `ocio-sys` for documented lower-level
interop. Rust hooks for GPU shader creation are not a general implementation
of arbitrary C++ virtual subclasses.

## GPU LUTs with external OCIO

The bundled 2.5.2 source includes a fix for an upstream multi-row GPU LUT
packing error. Installed libraries and `OCIO_SOURCE_DIR` overrides do not
inherit that fix. With an unpatched external build and unknown LUT lengths,
use `GPUProcessor::try_extract_shader_info_checked` or call
`validate_shader_texture_layout` before custom extraction. These conservative
checks generate shader resources twice and may reject non-LUT textures;
resource-generating state must remain unchanged between probe and extraction.
Ordinary extraction retains native behavior. See the Rust API documentation for
the precise contracts and callback ownership requirements.

Native objects and callbacks may have additional aliasing and synchronization
requirements. In particular, global configuration access and processor thread
confinement have changed; consult the [migration notes](MIGRATION.md) before
upgrading.
