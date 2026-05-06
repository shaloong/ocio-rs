//! OCIO-rs API Parity Checker
//!
//! Three-level comparison:
//!   L1: bridge.hpp <> lib.rs FFI declarations
//!   L2: bridge.hpp <> Rust safe wrapper methods
//!   L3: OCIO C++ headers <> bridge.hpp (if OCIO source available)
//!
//! Usage: cargo run --bin check_parity [-- --check-l3] [--verbose] [--json report.json]

use std::collections::{HashMap, HashSet, BTreeMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

const ROOT: &str = env!("CARGO_MANIFEST_DIR");
const BRIDGE_HPP: &str = "ocio-sys/src/bridge.hpp";
const LIB_RS: &str = "ocio-sys/src/lib.rs";
const SRC_DIR: &str = "src";
const OCIO_INCLUDE: &str = "third_party/OpenColorIO/include/OpenColorIO";

#[derive(Debug, Clone)]
struct CFunc {
    name: String,
    return_type: String,
    params: String,
    class_prefix: String,
    method_name: String,
}

#[derive(Debug)]
struct L1Result {
    total_hpp: usize,
    total_lib: usize,
    missing_in_lib: Vec<CFunc>,
    extra_in_lib: Vec<String>,
}

#[derive(Debug)]
struct L2Result {
    total: usize,
    matched: usize,
    missing: Vec<L2Missing>,
}

#[derive(Debug)]
struct L2Missing {
    rust_class: String,
    expected_method: String,
    bridge_func: String,
}

#[derive(Debug)]
struct L3Result {
    total_cpp_methods: usize,
    matched: usize,
    missing: BTreeMap<String, Vec<L3Missing>>,
}

#[derive(Debug)]
struct L3Missing {
    cpp_name: String,
    expected_snake: String,
}

// ─── C prefix → Rust struct name ───

fn c_prefix_to_rust(prefix: &str) -> Option<&'static str> {
    match prefix {
        "config" => Some("Config"),
        "file_rules" => Some("FileRules"),
        "color_space" => Some("ColorSpace"),
        "color_space_set" => Some("ColorSpaceSet"),
        "look" => Some("Look"),
        "named_transform" => Some("NamedTransform"),
        "view_transform" => Some("ViewTransform"),
        "processor" => Some("Processor"),
        "cpu_processor" => Some("CPUProcessor"),
        "gpu_processor" => Some("GPUProcessor"),
        "gpu_shader_desc" => Some("GpuShaderDesc"),
        "baker" => Some("Baker"),
        "context" => Some("Context"),
        "file_transform" => Some("FileTransform"),
        "cdl_transform" => Some("CDLTransform"),
        "exponent_transform" => Some("ExponentTransform"),
        "exponent_with_linear_transform" => Some("ExponentWithLinearTransform"),
        "matrix_transform" => Some("MatrixTransform"),
        "log_transform" => Some("LogTransform"),
        "range_transform" => Some("RangeTransform"),
        "group_transform" => Some("GroupTransform"),
        "builtin_transform" => Some("BuiltinTransform"),
        "fixed_function_transform" => Some("FixedFunctionTransform"),
        "lut1d_transform" => Some("Lut1DTransform"),
        "lut3d_transform" => Some("Lut3DTransform"),
        "exposure_contrast_transform" => Some("ExposureContrastTransform"),
        "color_space_transform" => Some("ColorSpaceTransform"),
        "look_transform" => Some("LookTransform"),
        "display_view_transform" => Some("DisplayViewTransform"),
        "grading_primary_transform" => Some("GradingPrimaryTransform"),
        "grading_tone_transform" => Some("GradingToneTransform"),
        "grading_rgb_curve_transform" => Some("GradingRGBCurveTransform"),
        "grading_hue_curve_transform" => Some("GradingHueCurveTransform"),
        "allocation_transform" => Some("AllocationTransform"),
        "log_affine_transform" => Some("LogAffineTransform"),
        "log_camera_transform" => Some("LogCameraTransform"),
        "dynamic_property" => Some("DynamicProperty"),
        "format_metadata" => Some("FormatMetadata"),
        "builtin_config_registry" => Some("BuiltinConfigRegistry"),
        _ => None,
    }
}

fn known_c_prefixes() -> Vec<&'static str> {
    let mut v: Vec<&'static str> = vec![
        "log_camera_transform", "log_affine_transform",
        "exposure_contrast_transform", "exponent_with_linear_transform",
        "grading_rgb_curve_transform", "grading_hue_curve_transform",
        "grading_primary_transform", "grading_tone_transform",
        "display_view_transform", "fixed_function_transform",
        "color_space_transform", "builtin_config_registry",
        "color_space_set", "allocation_transform", "look_transform",
        "lut1d_transform", "lut3d_transform", "builtin_transform",
        "matrix_transform", "exponent_transform", "group_transform",
        "cdl_transform", "file_transform", "log_transform", "range_transform",
        "view_transform", "named_transform", "dynamic_property",
        "format_metadata", "gpu_shader_desc", "gpu_processor",
        "cpu_processor", "color_space", "file_rules",
        "config", "processor", "baker", "context", "look",
    ];
    v.sort_by_key(|a| std::cmp::Reverse(a.len()));
    v
}

// ─── Helpers ───

fn strip_comments(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '/' {
            match chars.peek() {
                Some(&'/') => {
                    chars.next();
                    while let Some(&c) = chars.peek() {
                        if c == '\n' { break; }
                        chars.next();
                    }
                }
                Some(&'*') => {
                    chars.next();
                    let mut depth = 1;
                    while let Some(c) = chars.next() {
                        if c == '/' && chars.peek() == Some(&'*') { depth += 1; }
                        if c == '*' && chars.peek() == Some(&'/') { chars.next(); depth -= 1; if depth == 0 { break; } }
                    }
                }
                _ => out.push(ch),
            }
        } else {
            out.push(ch);
        }
    }
    out
}

fn camel_to_snake(name: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = name.chars().collect();
    for i in 0..chars.len() {
        if i > 0 && chars[i].is_uppercase() && chars[i-1].is_lowercase() {
            out.push('_');
        }
        if i > 0 && i + 1 < chars.len()
            && chars[i].is_uppercase() && chars[i-1].is_uppercase() && chars[i+1].is_lowercase()
        {
            out.push('_');
        }
        out.push(chars[i].to_ascii_lowercase());
    }
    out
}

fn classify_ocio_func(name: &str) -> (String, String) {
    let rest = name.strip_prefix("ocio_").unwrap_or(name);
    for prefix in known_c_prefixes() {
        if let Some(suffix) = rest.strip_prefix(&format!("{}_", prefix)) {
            return (prefix.to_string(), suffix.to_string());
        }
    }
    ("global".to_string(), rest.to_string())
}

// ─── Parsers ───

fn parse_bridge_hpp(path: &Path) -> Vec<CFunc> {
    let text = strip_comments(&fs::read_to_string(path).unwrap_or_default());
    let mut funcs = Vec::new();

    let mut buffer = String::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        buffer.push_str(trimmed);
        buffer.push(' ');
        if trimmed.ends_with(';') {
            let decl = buffer.trim();
            if decl.contains("ocio_") {
                if let Some(f) = parse_single_decl(decl) {
                    funcs.push(f);
                }
            }
            buffer.clear();
        }
    }
    funcs
}

fn parse_single_decl(decl: &str) -> Option<CFunc> {
    let decl = decl.trim_end_matches(';').trim();
    let paren_pos = decl.find('(')?;
    let before_paren = &decl[..paren_pos].trim();
    let params = decl[paren_pos + 1..].trim_end_matches(')').trim().to_string();

    let words: Vec<&str> = before_paren.split_whitespace().collect();
    if words.len() < 2 {
        return None;
    }
    let name = words[words.len() - 1].trim_end_matches(')');
    let return_type = words[..words.len() - 1].join(" ");

    if !name.starts_with("ocio_") {
        return None;
    }

    let (class_prefix, method_name) = classify_ocio_func(name);

    Some(CFunc {
        name: name.to_string(),
        return_type,
        params,
        class_prefix,
        method_name,
    })
}

fn parse_lib_rs(path: &Path) -> HashSet<String> {
    let text = strip_comments(&fs::read_to_string(path).unwrap_or_default());
    let mut funcs = HashSet::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("pub fn ocio_") {
            if let Some(name) = trimmed
                .strip_prefix("pub fn ")
                .and_then(|rest| rest.split('(').next())
            {
                funcs.insert(name.trim().to_string());
            }
        }
    }
    funcs
}

fn parse_rust_wrappers(src_dir: &Path) -> HashMap<String, HashSet<String>> {
    let mut methods: HashMap<String, HashSet<String>> = HashMap::new();

    fn visit_dir(dir: &Path, methods: &mut HashMap<String, HashSet<String>>) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_dir() {
                    visit_dir(&path, methods);
                } else if path.extension().map_or(false, |e| e == "rs") {
                    parse_rust_file(&path, methods);
                }
            }
        }
    }

    visit_dir(src_dir, &mut methods);
    methods
}

fn parse_rust_file(path: &Path, methods: &mut HashMap<String, HashSet<String>>) {
    let text = strip_comments(&fs::read_to_string(path).unwrap_or_default());

    let mut pos = 0;
    while let Some(impl_start) = text[pos..].find("impl ") {
        let start = pos + impl_start + 5;
        let end_of_line = text[start..].find('\n').unwrap_or(text.len() - start);
        let header = &text[start..start + end_of_line];

        let is_trait_impl = header.contains(" for ");

        // Extract struct name (first identifier after "impl " that isn't "for")
        let struct_name = header
            .split(|c: char| c == '{' || c == '<' || c.is_whitespace())
            .find(|s| !s.is_empty() && *s != "for")
            .unwrap_or("")
            .trim()
            .to_string();

        let brace_pos = text[start..].find('{');
        if brace_pos.is_none() {
            pos = start + end_of_line;
            continue;
        }
        let body_start = start + brace_pos.unwrap() + 1;
        let mut depth = 1;
        let mut body_end = body_start;
        for (i, ch) in text[body_start..].char_indices() {
            match ch {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        body_end = body_start + i;
                        break;
                    }
                }
                _ => {}
            }
        }
        let body = &text[body_start..body_end];

        if !is_trait_impl && !struct_name.is_empty() {
            let entry = methods.entry(struct_name.clone()).or_default();
            for fn_line in body.lines() {
                let trimmed = fn_line.trim();
                if trimmed.starts_with("pub fn ") {
                    if let Some(name) = trimmed
                        .strip_prefix("pub fn ")
                        .and_then(|rest| {
                            // Handle generic params: method<A, B>(...) -> method
                            let no_generics = if let Some(angle) = rest.find('<') {
                                let before_angle = &rest[..angle];
                                // Make sure the angle bracket is for generics (before '(')
                                if let Some(paren) = rest.find('(') {
                                    if angle < paren {
                                        before_angle.trim()
                                    } else {
                                        rest.split('(').next().unwrap_or("")
                                    }
                                } else {
                                    before_angle.trim()
                                }
                            } else {
                                rest.split('(').next().unwrap_or("")
                            };
                            Some(no_generics.to_string())
                        })
                    {
                        entry.insert(name);
                    }
                }
            }
        }

        pos = body_end + 1;
    }
}

fn parse_ocio_cpp_headers(include_dir: &Path) -> HashMap<String, Vec<(String, String)>> {
    let mut classes: HashMap<String, Vec<(String, String)>> = HashMap::new();
    if !include_dir.exists() {
        return classes;
    }

    let cpp_classes = [
        "Config", "FileRules", "ColorSpace", "ColorSpaceSet", "Look", "NamedTransform",
        "ViewTransform", "Processor", "CPUProcessor", "GPUProcessor", "GpuShaderDesc",
        "Baker", "Context", "FileTransform", "CDLTransform", "ExponentTransform",
        "ExponentWithLinearTransform", "MatrixTransform", "LogTransform", "RangeTransform",
        "GroupTransform", "BuiltinTransform", "FixedFunctionTransform", "Lut1DTransform",
        "Lut3DTransform", "ExposureContrastTransform", "ColorSpaceTransform",
        "LookTransform", "DisplayViewTransform", "GradingPrimaryTransform",
        "GradingToneTransform", "GradingRGBCurveTransform", "GradingHueCurveTransform",
        "AllocationTransform", "LogAffineTransform", "LogCameraTransform",
        "FormatMetadata", "BuiltinConfigRegistry",
    ];

    let entries: Vec<_> = fs::read_dir(include_dir)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "h") && e.path().file_name()
            .map_or(false, |n| n != "OpenColorABI.h" && n != "OpenColorAppHelpers.h"))
        .collect();

    for entry in entries {
        let text = strip_comments(&fs::read_to_string(entry.path()).unwrap_or_default());

        for &cpp_class in &cpp_classes {
            let class_pat = format!("class OCIOEXPORT {}", cpp_class);
            if let Some(class_start) = text.find(&class_pat) {
                let after_keyword = class_start + class_pat.len();
                if let Some(brace_pos) = text[after_keyword..].find('{') {
                    let body_start = after_keyword + brace_pos + 1;
                    let mut depth = 1;
                    let mut body_end = body_start;
                    for (i, ch) in text[body_start..].char_indices() {
                        match ch {
                            '{' => depth += 1,
                            '}' => {
                                depth -= 1;
                                if depth == 0 {
                                    body_end = body_start + i;
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                    let class_body = &text[body_start..body_end];

                    let public_section = class_body
                        .split("private:")
                        .next()
                        .unwrap_or(class_body)
                        .split("protected:")
                        .next()
                        .unwrap_or(class_body);

                    let methods: Vec<(String, String)> = public_section
                        .lines()
                        .filter(|line| {
                            let t = line.trim();
                            t.ends_with(';') && !t.starts_with("//") && !t.starts_with("typedef")
                                && !t.starts_with("using ") && !t.starts_with("friend ")
                                && t.contains('(') && t.contains(')')
                        })
                        .filter_map(|line| {
                            let t = line.trim().trim_end_matches(';').trim();
                            let paren_pos = t.find('(')?;
                            let before = &t[..paren_pos];
                            let last_word = before.split_whitespace().last()?;
                            if last_word.starts_with('~') || last_word == "operator" {
                                return None;
                            }
                            if cpp_class == last_word {
                                return None; // Constructor
                            }
                            let method_name = last_word.to_string();
                            Some((method_name, t.to_string()))
                        })
                        .collect();

                    if !methods.is_empty() {
                        classes.entry(cpp_class.to_string())
                            .or_default()
                            .extend(methods);
                    }
                }
            }
        }
    }
    classes
}

// ─── Comparisons ───

fn run_l1(bridge_funcs: &[CFunc], lib_funcs: &HashSet<String>) -> L1Result {
    let mut missing = Vec::new();
    for f in bridge_funcs {
        if !lib_funcs.contains(&f.name) {
            missing.push(f.clone());
        }
    }
    let bridge_names: HashSet<_> = bridge_funcs.iter().map(|f| &f.name).collect();
    let extra: Vec<_> = lib_funcs.iter()
        .filter(|n| !bridge_names.contains(*n))
        .cloned()
        .collect();

    L1Result {
        total_hpp: bridge_funcs.len(),
        total_lib: lib_funcs.len(),
        missing_in_lib: missing,
        extra_in_lib: extra,
    }
}

// L2: Only the truly irregular mappings that can't be derived by get_/set_ stripping.
fn l2_overrides() -> HashMap<&'static str, &'static str> {
    [
        // Constructor/static factory variants
        ("create_raw", "raw"),
        ("create_from_file", "from_file"),
        // Processor naming differences
        ("get_cache_id_n", "cache_id_with_context"),
        ("get_processor_transform", "processor_from_transform"),
        ("get_default_scene_to_display_view_transform", "get_default_scene_to_display_view_transform"),
        // BuiltinTransform: completely different static method names
        ("get_num_styles", "num_builtin_styles"),
        ("get_style_by_index", "builtin_style"),
        ("is_valid_style", "is_valid_builtin_style"),
        // CDLTransform: power -> power_ (trailing underscore for keyword collision)
        ("get_power", "power_"),
        // DynamicProperty: different method names
        ("get_type", "property_type"),
        ("double_get_value", "double_value"),
        ("double_set_value", "set_double_value"),
        // DynamicProperty typed getters
        ("grading_primary_get_value", "grading_primary_value"),
        ("grading_primary_set_value", "set_grading_primary_value"),
        ("grading_tone_get_value", "grading_tone_value"),
        ("grading_tone_set_value", "set_grading_tone_value"),
        ("grading_rgb_curve_get_num_control_points", "grading_rgb_curve_num_control_points"),
        ("grading_rgb_curve_get_control_point", "grading_rgb_curve_control_point"),
        ("grading_rgb_curve_get_slope", "grading_rgb_curve_slope"),
        ("grading_hue_curve_get_num_control_points", "grading_hue_curve_num_control_points"),
        ("grading_hue_curve_get_control_point", "grading_hue_curve_control_point"),
        ("grading_hue_curve_get_slope", "grading_hue_curve_slope"),
        // ColorSpaceSet: get_color_space_by_name -> color_space (by name variant)
        ("get_color_space_by_name", "get_color_space"),
        // DVT/LookTransform
        ("get_looks_bypass", "looks_bypass"),
        ("get_data_bypass", "data_bypass"),
        ("get_skip_color_space_conversion", "skip_color_space_conversion"),
        // GpuShaderDesc: get_texture -> texture_info (returns struct, not raw fields)
        ("get_texture", "texture_info"),
        // FormatMetadata: get_num_children_elements -> num_children
        ("get_num_children_elements", "num_children"),
        // MatrixTransform: static factories differ
        ("create_fit", "fit"),
        ("create_identity", "identity"),
        ("create_sat", "sat"),
        ("create_scale", "scale"),
        ("create_view", "view"),
        // Config: role color space by name
        ("get_role_color_space_by_name", "role_color_space"),
        // Config: get_display_view_transform_name (strip get_ gives display_view_transform_name ✓)
        // Config: get_color_space_set (strip get_ gives color_space_set ✓, with generics fix)
    ].iter().cloned().collect()
}

fn run_l2(bridge_funcs: &[CFunc], rust_methods: &HashMap<String, HashSet<String>>) -> L2Result {
    let mut missing = Vec::new();
    let mut total = 0;
    let mut matched = 0;
    let overrides = l2_overrides();

    for f in bridge_funcs {
        if let Some(rust_class) = c_prefix_to_rust(&f.class_prefix) {
            total += 1;

            // Skip destroy methods (Drop trait handles them)
            if f.method_name == "destroy" {
                matched += 1;
                continue;
            }

            let rust_method = if let Some(methods) = rust_methods.get(rust_class) {
                find_rust_method(&f.method_name, methods, &overrides)
            } else {
                None
            };

            if rust_method.is_some() {
                matched += 1;
            } else {
                missing.push(L2Missing {
                    rust_class: rust_class.to_string(),
                    expected_method: f.method_name.clone(),
                    bridge_func: f.name.clone(),
                });
            }
        }
    }

    L2Result { total, matched, missing }
}

fn find_rust_method(
    bridge_method: &str,
    rust_methods: &HashSet<String>,
    overrides: &HashMap<&'static str, &'static str>,
) -> Option<String> {
    // 1. Check override map first
    if let Some(ovr) = overrides.get(bridge_method) {
        let target = ovr.to_string();
        if rust_methods.contains(&target) {
            return Some(target);
        }
    }

    // 2. Try exact match (handles create_from_file, get_config_by_name, etc.)
    if rust_methods.contains(bridge_method) {
        return Some(bridge_method.to_string());
    }

    // 3. Strip get_ prefix (handles get_name -> name, get_allocation -> allocation, etc.)
    if let Some(stripped) = bridge_method.strip_prefix("get_") {
        if rust_methods.contains(stripped) {
            return Some(stripped.to_string());
        }
    }

    // 4. Strip set_ prefix
    if let Some(stripped) = bridge_method.strip_prefix("set_") {
        if rust_methods.contains(stripped) {
            return Some(stripped.to_string());
        }
    }

    None
}

fn run_l3(bridge_funcs: &[CFunc], cpp_classes: &HashMap<String, Vec<(String, String)>>) -> L3Result {
    let bridge_methods: HashMap<String, HashSet<String>> = bridge_funcs.iter()
        .fold(HashMap::new(), |mut acc, f| {
            if let Some(rust_class) = c_prefix_to_rust(&f.class_prefix) {
                acc.entry(rust_class.to_string()).or_default().insert(f.method_name.clone());
            }
            acc
        });

    let cpp_to_rust: HashMap<&str, &str> = [
        ("Config", "Config"), ("FileRules", "FileRules"), ("ColorSpace", "ColorSpace"),
        ("ColorSpaceSet", "ColorSpaceSet"), ("Look", "Look"), ("NamedTransform", "NamedTransform"),
        ("ViewTransform", "ViewTransform"), ("Processor", "Processor"),
        ("CPUProcessor", "CPUProcessor"), ("GPUProcessor", "GPUProcessor"),
        ("GpuShaderDesc", "GpuShaderDesc"), ("Baker", "Baker"), ("Context", "Context"),
        ("FileTransform", "FileTransform"), ("CDLTransform", "CDLTransform"),
        ("ExponentTransform", "ExponentTransform"),
        ("ExponentWithLinearTransform", "ExponentWithLinearTransform"),
        ("MatrixTransform", "MatrixTransform"), ("LogTransform", "LogTransform"),
        ("RangeTransform", "RangeTransform"), ("GroupTransform", "GroupTransform"),
        ("BuiltinTransform", "BuiltinTransform"), ("FixedFunctionTransform", "FixedFunctionTransform"),
        ("Lut1DTransform", "Lut1DTransform"), ("Lut3DTransform", "Lut3DTransform"),
        ("ExposureContrastTransform", "ExposureContrastTransform"),
        ("ColorSpaceTransform", "ColorSpaceTransform"), ("LookTransform", "LookTransform"),
        ("DisplayViewTransform", "DisplayViewTransform"),
        ("GradingPrimaryTransform", "GradingPrimaryTransform"),
        ("GradingToneTransform", "GradingToneTransform"),
        ("GradingRGBCurveTransform", "GradingRGBCurveTransform"),
        ("GradingHueCurveTransform", "GradingHueCurveTransform"),
        ("AllocationTransform", "AllocationTransform"),
        ("LogAffineTransform", "LogAffineTransform"), ("LogCameraTransform", "LogCameraTransform"),
        ("FormatMetadata", "FormatMetadata"), ("BuiltinConfigRegistry", "BuiltinConfigRegistry"),
    ].iter().cloned().collect();

    let method_name_overrides: HashMap<&str, &str> = [
        ("CreateRaw", "raw"), ("CreateFromFile", "from_file"),
        ("CreateEditableCopy", "create_editable_copy"),
        ("getProcessor", "processor"),
        ("getDefaultCPUProcessor", "default_cpu_processor"),
        ("getOptimizedCPUProcessor", "optimized_cpu_processor"),
        ("getDefaultGPUProcessor", "default_gpu_processor"),
        ("getOptimizedGPUProcessor", "optimized_gpu_processor"),
        ("getDynamicProperty", "dynamic_property"),
        ("isNoOp", "is_no_op"), ("hasChannelCrosstalk", "has_channel_crosstalk"),
    ].iter().cloned().collect();

    let mut total_cpp = 0;
    let mut matched = 0;
    let mut missing = BTreeMap::new();

    for (cpp_class, methods) in cpp_classes {
        let rust_class = cpp_to_rust.get(cpp_class.as_str());
        if rust_class.is_none() { continue; }
        let rust_class = rust_class.unwrap();
        let bridged = bridge_methods.get(*rust_class);

        total_cpp += methods.len();
        for (method_name, _signature) in methods {
            if method_name == "Create" { continue; }

            let expected = method_name_overrides.get(method_name.as_str())
                .map(|s| s.to_string())
                .unwrap_or_else(|| {
                    let snake = camel_to_snake(method_name);
                    if snake.starts_with("get_") {
                        snake[4..].to_string()
                    } else if snake.starts_with("set_") {
                        snake[4..].to_string()
                    } else {
                        snake
                    }
                });

            let found = if let Some(bridged_set) = bridged {
                bridged_set.contains(&expected)
                    || bridged_set.contains(&format!("get_{}", expected))
                    || bridged_set.contains(&format!("set_{}", expected))
            } else {
                false
            };

            if found {
                matched += 1;
            } else {
                missing.entry(cpp_class.clone()).or_insert_with(Vec::new)
                    .push(L3Missing { cpp_name: method_name.clone(), expected_snake: expected });
            }
        }
    }

    L3Result { total_cpp_methods: total_cpp, matched, missing }
}

// ─── Report ───

fn print_report(l1: &L1Result, l2: &L2Result, l3: Option<&L3Result>) -> bool {
    let mut has_gaps = false;

    println!("{}", "=".repeat(64));
    println!("  OCIO-rs API Parity Report");
    println!("{}", "=".repeat(64));
    println!();

    // L1
    println!("[L1] bridge.hpp <-> lib.rs (FFI declarations)");
    println!("  bridge.hpp functions: {}", l1.total_hpp);
    println!("  lib.rs declarations:  {}", l1.total_lib);
    if !l1.missing_in_lib.is_empty() {
        has_gaps = true;
        println!("  MISSING in lib.rs: {}", l1.missing_in_lib.len());
        for f in &l1.missing_in_lib {
            println!("    - {} ({})", f.name, f.return_type);
        }
    } else {
        println!("  Status: OK");
    }
    if !l1.extra_in_lib.is_empty() {
        has_gaps = true;
        println!("  EXTRA in lib.rs (not in bridge.hpp): {}", l1.extra_in_lib.len());
        for e in &l1.extra_in_lib {
            println!("    - {}", e);
        }
    }
    println!();

    // L2
    println!("[L2] bridge.hpp <-> Rust safe wrappers");
    println!("  Total: {} wrapped functions, {} matched", l2.total, l2.matched);
    if !l2.missing.is_empty() {
        has_gaps = true;
        println!("  MISSING Rust wrapper methods: {}", l2.missing.len());
        let mut by_class: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
        for m in &l2.missing {
            by_class.entry(&m.rust_class).or_default().push(&m.expected_method);
        }
        for (cls, meths) in &by_class {
            println!("    {}: {}", cls, meths.join(", "));
        }
    } else {
        println!("  Status: OK");
    }
    println!();

    // L3
    if let Some(l3) = l3 {
        println!("[L3] OCIO C++ headers <-> bridge.hpp");
        println!("  C++ methods total: {}", l3.total_cpp_methods);
        println!("  Bridged: {}/{}", l3.matched, l3.total_cpp_methods);
        if !l3.missing.is_empty() {
            has_gaps = true;
            let total_missing: usize = l3.missing.values().map(|v| v.len()).sum();
            println!("  NOT YET BRIDGED: {} methods", total_missing);
            for (cls, meths) in &l3.missing {
                println!("    {}: {} methods", cls, meths.len());
                for m in meths.iter().take(5) {
                    println!("      - {}() -> {}", m.cpp_name, m.expected_snake);
                }
                if meths.len() > 5 {
                    println!("      ... and {} more", meths.len() - 5);
                }
            }
        } else {
            println!("  Status: OK");
        }
        println!();
    }

    println!("{}", "=".repeat(64));
    if has_gaps {
        println!("  RESULT: GAPS FOUND");
    } else {
        println!("  RESULT: ALL CLEAN");
    }
    println!("{}", "=".repeat(64));

    has_gaps
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let check_l3 = args.iter().any(|a| a == "--check-l3");
    let verbose = args.iter().any(|a| a == "--verbose" || a == "-v");
    let json_out = args.iter().position(|a| a == "--json")
        .and_then(|i| args.get(i + 1).cloned());

    let root = PathBuf::from(ROOT);

    // Parse
    let bridge_funcs = parse_bridge_hpp(&root.join(BRIDGE_HPP));
    let lib_funcs = parse_lib_rs(&root.join(LIB_RS));
    let rust_methods = parse_rust_wrappers(&root.join(SRC_DIR));

    // Compare
    let l1 = run_l1(&bridge_funcs, &lib_funcs);
    let l2 = run_l2(&bridge_funcs, &rust_methods);

    let l3 = if check_l3 {
        let cpp_classes = parse_ocio_cpp_headers(&root.join(OCIO_INCLUDE));
        Some(run_l3(&bridge_funcs, &cpp_classes))
    } else {
        None
    };

    let has_gaps = print_report(&l1, &l2, l3.as_ref());

    if verbose {
        println!();
        println!("{}", "-".repeat(64));
        println!("  Per-Class Function Counts (bridge.hpp)");
        println!("{}", "-".repeat(64));
        let mut class_counts: BTreeMap<&str, usize> = BTreeMap::new();
        for f in &bridge_funcs {
            *class_counts.entry(&f.class_prefix).or_default() += 1;
        }
        for (cls, count) in &class_counts {
            if let Some(rust_name) = c_prefix_to_rust(cls) {
                println!("  {:>5}  {:35} -> {}", count, cls, rust_name);
            } else {
                println!("  {:>5}  {:35} -> [global]", count, cls);
            }
        }
        println!("  {:>5}  total", bridge_funcs.len());
    }

    if let Some(path) = &json_out {
        let missing_l1: Vec<_> = l1.missing_in_lib.iter().map(|f| f.name.clone()).collect();
        let missing_l2: Vec<_> = l2.missing.iter().map(|m| format!("{}::{}", m.rust_class, m.expected_method)).collect();
        let json = format!(
            r#"{{
  "L1": {{
    "total_hpp": {},
    "total_lib": {},
    "missing_in_lib": {:?},
    "extra_in_lib": {:?}
  }},
  "L2": {{
    "total": {},
    "matched": {},
    "missing": {:?}
  }},
  "L3": {}
}}"#,
            l1.total_hpp, l1.total_lib, missing_l1, l1.extra_in_lib,
            l2.total, l2.matched, missing_l2,
            if let Some(ref l3) = l3 {
                let missing_count: usize = l3.missing.values().map(|v| v.len()).sum();
                format!(r#"{{ "total_cpp_methods": {}, "matched": {}, "missing_count": {} }}"#,
                    l3.total_cpp_methods, l3.matched, missing_count)
            } else { r#""skipped (use --check-l3)""#.to_string() }
        );
        fs::write(path, json).unwrap_or_else(|e| eprintln!("Failed to write JSON: {}", e));
        println!("  JSON report saved to {}", path);
    }

    if has_gaps {
        process::exit(1);
    }
}
