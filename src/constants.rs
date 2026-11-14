//! Standard OCIO names and identifiers, as defined by OpenColorIO 2.4–2.6.
//!
//! These are compile-time strings and work without a native library, including
//! in stub builds. They do not query or modify the process environment.

/// Environment variable specifying the active OCIO configuration.
pub const OCIO_CONFIG_ENVVAR: &str = "OCIO";
/// Environment variable overriding active displays.
pub const OCIO_ACTIVE_DISPLAYS_ENVVAR: &str = "OCIO_ACTIVE_DISPLAYS";
/// Environment variable overriding active views.
pub const OCIO_ACTIVE_VIEWS_ENVVAR: &str = "OCIO_ACTIVE_VIEWS";
/// Environment variable listing inactive color spaces.
pub const OCIO_INACTIVE_COLORSPACES_ENVVAR: &str = "OCIO_INACTIVE_COLORSPACES";
/// Environment variable controlling processor optimizations.
pub const OCIO_OPTIMIZATION_FLAGS_ENVVAR: &str = "OCIO_OPTIMIZATION_FLAGS";
/// Environment variable listing user categories.
pub const OCIO_USER_CATEGORIES_ENVVAR: &str = "OCIO_USER_CATEGORIES";

/// Default color-space role.
pub const ROLE_DEFAULT: &str = "default";
/// Reference color-space role.
pub const ROLE_REFERENCE: &str = "reference";
/// Data color-space role.
pub const ROLE_DATA: &str = "data";
/// Color-picking role.
pub const ROLE_COLOR_PICKING: &str = "color_picking";
/// Scene-linear color-space role.
pub const ROLE_SCENE_LINEAR: &str = "scene_linear";
/// Compositing-log color-space role.
pub const ROLE_COMPOSITING_LOG: &str = "compositing_log";
/// Color-timing role.
pub const ROLE_COLOR_TIMING: &str = "color_timing";
/// Texture-painting role.
pub const ROLE_TEXTURE_PAINT: &str = "texture_paint";
/// Matte-painting role.
pub const ROLE_MATTE_PAINT: &str = "matte_paint";
/// Rendering role.
pub const ROLE_RENDERING: &str = "rendering";
/// Scene-referred interchange role.
pub const ROLE_INTERCHANGE_SCENE: &str = "aces_interchange";
/// Display-referred interchange role.
pub const ROLE_INTERCHANGE_DISPLAY: &str = "cie_xyz_d65_interchange";

/// Special shared-view color-space name that resolves to the display name.
pub const OCIO_VIEW_USE_DISPLAY_NAME: &str = "<USE_DISPLAY_NAME>";

/// CLF description element name.
pub const METADATA_DESCRIPTION: &str = "Description";
/// CLF info element name.
pub const METADATA_INFO: &str = "Info";
/// CLF input descriptor element name.
pub const METADATA_INPUT_DESCRIPTOR: &str = "InputDescriptor";
/// CLF output descriptor element name.
pub const METADATA_OUTPUT_DESCRIPTOR: &str = "OutputDescriptor";
/// CLF name attribute.
pub const METADATA_NAME: &str = "name";
/// CLF ID attribute.
pub const METADATA_ID: &str = "id";
/// ST 2136-1 ID element name, introduced in OCIO 2.6.
#[cfg(feature = "v2_6")]
pub const METADATA_ID_ELEMENT: &str = "Id";

/// Environment variable disabling all OCIO caches.
pub const OCIO_DISABLE_ALL_CACHES: &str = "OCIO_DISABLE_ALL_CACHES";
/// Environment variable disabling processor caches.
pub const OCIO_DISABLE_PROCESSOR_CACHES: &str = "OCIO_DISABLE_PROCESSOR_CACHES";
/// Environment variable disabling cache fallback.
pub const OCIO_DISABLE_CACHE_FALLBACK: &str = "OCIO_DISABLE_CACHE_FALLBACK";
/// Default config file name without its extension.
pub const OCIO_CONFIG_DEFAULT_NAME: &str = "config";
/// Config file extension, including the leading dot.
pub const OCIO_CONFIG_DEFAULT_FILE_EXT: &str = ".ocio";
/// OCIO archive file extension, including the leading dot.
pub const OCIO_CONFIG_ARCHIVE_FILE_EXT: &str = ".ocioz";
/// Prefix of a built-in config URI.
pub const OCIO_BUILTIN_URI_PREFIX: &str = "ocio://";
