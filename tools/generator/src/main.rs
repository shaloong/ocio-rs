//! OCIO-rs Bridge Code Generator
//!
//! Reads OCIO C++ public headers and generates:
//!   - ocio-sys/src/bridge.hpp   (C FFI declarations)
//!   - ocio-sys/src/bridge.cpp   (C++ stub/real implementations)
//!   - ocio-sys/src/lib.rs       (Rust FFI declarations)
//!
//! Usage:
//!   cargo run --bin generate_bridge
//!   cargo run --bin generate_bridge -- --check
//!   cargo run --bin generate_bridge -- --ocio-path path/to/ocio/include/OpenColorIO

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

// ═══════════════════════════════════════════════════════════════════════════
// Data structures
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
struct Param {
    cpp_type: String,
    name: String,
}

#[derive(Debug, Clone)]
struct MethodDef {
    name: String,
    return_type: String,
    params: Vec<Param>,
    is_const: bool,
    is_static: bool,
}

#[derive(Debug, Clone)]
struct ClassDef {
    name: String,
    c_prefix: String,
    methods: Vec<MethodDef>,
    is_transform: bool,
}

// ═══════════════════════════════════════════════════════════════════════════
// Utility functions
// ═══════════════════════════════════════════════════════════════════════════

fn camel_to_snake(name: &str) -> String {
    let mut s = String::new();
    let chars: Vec<char> = name.chars().collect();
    for i in 0..chars.len() {
        if i > 0 && chars[i].is_uppercase() && chars[i - 1].is_lowercase() {
            s.push('_');
        }
        if i > 0
            && i + 1 < chars.len()
            && chars[i].is_uppercase()
            && chars[i - 1].is_uppercase()
            && chars[i + 1].is_lowercase()
        {
            s.push('_');
        }
        s.push(chars[i].to_ascii_lowercase());
    }
    s
}

fn method_to_snake(name: &str) -> String {
    match name {
        "getCacheID" => "get_cache_id".into(),
        "getUIName" => "get_ui_name".into(),
        "isNoOp" => "is_no_op".into(),
        _ => camel_to_snake(name),
    }
}

/// Pre-compute unique C names for all methods, handling overloads.
/// Returns Vec<(MethodDef, c_function_name)> where c_function_name is unique.
fn compute_c_names(cls: &ClassDef) -> Vec<(&MethodDef, String)> {
    use std::collections::HashMap;
    let mut name_counts: HashMap<String, usize> = HashMap::new();
    let mut name_ordinals: HashMap<String, usize> = HashMap::new();

    // First pass: count occurrences of each snake_case name
    for m in &cls.methods {
        let sn = method_to_snake(&m.name);
        *name_counts.entry(sn).or_insert(0) += 1;
    }

    // Second pass: assign unique names
    cls.methods.iter().map(|m| {
        let base_name = method_to_snake(&m.name);
        let is_overloaded = *name_counts.get(&base_name).unwrap_or(&0) > 1;

        let c_name = if is_overloaded {
            let ord = name_ordinals.entry(base_name.clone()).or_insert(0);
            *ord += 1;
            format!("ocio_{}_{}_v{}", cls.c_prefix, base_name, *ord)
        } else {
            format!("ocio_{}_{}", cls.c_prefix, base_name)
        };

        (m, c_name)
    }).collect()
}

fn strip_comments(text: &str) -> String {
    let text = strip_block_comments(text);
    let text = regex::Regex::new(r"//[^\n]*")
        .unwrap()
        .replace_all(&text, "");
    text.to_string()
}

/// Strip /* ... */ and /** ... */ block comments including multi-line ones.
/// Uses a manual state machine to handle large blocks reliably.
fn strip_block_comments(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut result = String::new();
    let mut i = 0;
    while i < chars.len() {
        if i + 1 < chars.len() && chars[i] == '/' && chars[i + 1] == '*' {
            // Enter block comment
            i += 2;
            let mut depth = 1usize;
            while i + 1 < chars.len() && depth > 0 {
                if chars[i] == '/' && chars[i + 1] == '*' {
                    depth += 1;
                    i += 2;
                } else if chars[i] == '*' && chars[i + 1] == '/' {
                    depth -= 1;
                    i += 2;
                } else {
                    i += 1;
                }
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}

fn find_matching_brace(text: &str, start: usize) -> usize {
    let mut depth = 1usize;
    let mut i = start;
    let bytes = text.as_bytes();
    while i < bytes.len() && depth > 0 {
        match bytes[i] {
            b'{' => depth += 1,
            b'}' => depth -= 1,
            _ => {}
        }
        i += 1;
    }
    i - 1
}

fn parse_params(params_str: &str) -> Vec<Param> {
    let trimmed = params_str.trim();
    if trimmed.is_empty() || trimmed == "void" {
        return vec![];
    }

    let mut params = Vec::new();
    let mut depth = 0i32;
    let mut current = String::new();

    for ch in trimmed.chars() {
        match ch {
            '(' | '<' => {
                depth += 1;
                current.push(ch);
            }
            ')' | '>' => {
                depth -= 1;
                current.push(ch);
            }
            ',' if depth == 0 => {
                params.push(current.trim().to_string());
                current = String::new();
            }
            _ => current.push(ch),
        }
    }
    if !current.trim().is_empty() {
        params.push(current.trim().to_string());
    }

    params
        .into_iter()
        .filter_map(|p| {
            let p = p.trim().to_string();
            if p.is_empty() {
                return None;
            }
            // Split into type and name
            let parts: Vec<&str> = p.split_whitespace().collect();
            if parts.len() >= 2 {
                let name = parts.last().unwrap().trim_matches('*').trim();
                let name = name.split('=').next().unwrap_or("").trim();
                let name = name.split('[').next().unwrap_or("").trim();
                let cpp_type = parts[..parts.len() - 1].join(" ");
                Some(Param {
                    cpp_type: cpp_type.trim().to_string(),
                    name: name.to_string(),
                })
            } else if parts.len() == 1 {
                Some(Param {
                    cpp_type: parts[0].to_string(),
                    name: String::new(),
                })
            } else {
                None
            }
        })
        .collect()
}

// ═══════════════════════════════════════════════════════════════════════════
// Type mapping
// ═══════════════════════════════════════════════════════════════════════════

/// Returns (c_type, stub_return, wraps_handle)
fn map_return_type(cpp_type: &str) -> (&str, &str, bool) {
    let cpp_type = cpp_type.trim();

    // RcPtr types → void* handle
    if RCPTR_SET.contains(&cpp_type) || cpp_type.ends_with("RcPtr") {
        return ("void*", "nullptr", true);
    }

    match cpp_type {
        "const char*" => ("const char*", "nullptr", false),
        "void" => ("void", "", false),
        "int" => ("int", "0", false),
        "unsigned int" => ("unsigned int", "0", false),
        "size_t" => ("size_t", "0", false),
        "bool" => ("bool", "false", false),
        "float" => ("float", "0.0f", false),
        "double" => ("double", "0.0", false),
        "char" => ("char", "'\\0'", false),
        _ => {
            // Check enum types
            if ENUM_TYPES.contains(&cpp_type) {
                ("int", "0", false)
            } else if cpp_type.contains("RcPtr") && !cpp_type.starts_with("const char") {
                ("void*", "nullptr", true)
            } else {
                // Fallback: unknown C++ types map to void*
                // but don't wrap them as handles (they're raw pointers/refs)
                ("void*", "nullptr", false)
            }
        }
    }
}

/// Escape Rust reserved keywords used as parameter names
fn escape_rust_ident(name: &str) -> String {
    match name {
        "type" | "ref" | "move" | "mod" | "pub" | "use" | "impl" | "fn"
        | "struct" | "enum" | "trait" | "static" | "dyn" | "where" | "async"
        | "await" | "loop" | "match" | "if" | "else" | "return" | "let" | "mut"
        | "self" | "crate" | "super" | "extern" | "unsafe" | "const" | "box"
        | "in" | "become" | "final" | "macro" | "override" | "priv" | "typeof"
        | "unsized" | "virtual" | "yield" | "do" | "abstract" | "catch" | "try"
        => format!("{}_param", name),
        _ => name.to_string(),
    }
}

fn map_c_to_rust(c_type: &str) -> &str {
    match c_type {
        "void" => "()",
        "void*" => "*mut c_void",
        "const char*" => "*const i8",
        "char" => "i8",
        "int" => "i32",
        "unsigned int" => "u32",
        "size_t" => "usize",
        "bool" => "bool",
        "float" => "f32",
        "double" => "f64",
        "long" => "i64",
        "unsigned long" => "u64",
        _ => {
            if c_type.ends_with('*') {
                "*mut c_void"
            } else {
                "i32"
            }
        }
    }
}

fn real_member_for_class(class_name: &str, is_transform: bool) -> &str {
    if is_transform {
        return "transform";
    }
    match class_name {
        "Config" => "config",
        "Processor" => "processor",
        "CPUProcessor" => "cpu",
        "GPUProcessor" => "gpu",
        "Baker" => "baker",
        "Context" => "context",
        "ColorSpace" => "colorSpace",
        "Look" => "look",
        "NamedTransform" => "transform",
        "ViewTransform" => "transform",
        "ColorSpaceSet" => "set",
        "FileRules" => "rules",
        "DynamicProperty" => "prop",
        "FormatMetadata" => "metadata",
        "GpuShaderDesc" => "gpuShaderDesc",
        "BuiltinConfigRegistry" => "registry",
        _ => "inner",
    }
}

fn rcptr_for_class(class_name: &str, is_transform: bool) -> String {
    if is_transform {
        format!("{}RcPtr", class_name)
    } else {
        match class_name {
            "Config" => "ConstConfigRcPtr".into(),
            "Processor" => "ConstProcessorRcPtr".into(),
            "CPUProcessor" => "ConstCPUProcessorRcPtr".into(),
            "GPUProcessor" => "ConstGPUProcessorRcPtr".into(),
            "Baker" => "BakerRcPtr".into(),
            "Context" => "ContextRcPtr".into(),
            "ColorSpace" => "ColorSpaceRcPtr".into(),
            "Look" => "LookRcPtr".into(),
            "NamedTransform" => "NamedTransformRcPtr".into(),
            "ViewTransform" => "ViewTransformRcPtr".into(),
            "ColorSpaceSet" => "ColorSpaceSetRcPtr".into(),
            "FileRules" => "FileRulesRcPtr".into(),
            "FormatMetadata" => "FormatMetadata".into(),
            "DynamicProperty" => "DynamicPropertyRcPtr".into(),
            "BuiltinConfigRegistry" => "BuiltinConfigRegistry".into(),
            "GpuShaderDesc" => "GpuShaderDescRcPtr".into(),
            _ => format!("{}RcPtr", class_name),
        }
    }
}

lazy_static::lazy_static! {
    static ref RCPTR_SET: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for t in &[
            "ConstConfigRcPtr", "ConfigRcPtr",
            "ConstProcessorRcPtr", "ProcessorRcPtr",
            "ConstCPUProcessorRcPtr", "CPUProcessorRcPtr",
            "ConstGPUProcessorRcPtr", "GPUProcessorRcPtr",
            "ConstColorSpaceRcPtr", "ColorSpaceRcPtr",
            "ConstLookRcPtr", "LookRcPtr",
            "ConstNamedTransformRcPtr", "NamedTransformRcPtr",
            "ConstViewTransformRcPtr", "ViewTransformRcPtr",
            "ConstBakerRcPtr", "BakerRcPtr",
            "ConstContextRcPtr", "ContextRcPtr",
            "ConstColorSpaceSetRcPtr", "ColorSpaceSetRcPtr",
            "ConstFileRulesRcPtr", "FileRulesRcPtr",
            "ConstFormatMetadataRcPtr", "FormatMetadataRcPtr",
            "ConstTransformRcPtr", "TransformRcPtr",
            "ConstAllocationTransformRcPtr", "AllocationTransformRcPtr",
            "ConstBuiltinTransformRcPtr", "BuiltinTransformRcPtr",
            "ConstCDLTransformRcPtr", "CDLTransformRcPtr",
            "ConstColorSpaceTransformRcPtr", "ColorSpaceTransformRcPtr",
            "ConstDisplayViewTransformRcPtr", "DisplayViewTransformRcPtr",
            "ConstExponentTransformRcPtr", "ExponentTransformRcPtr",
            "ConstExponentWithLinearTransformRcPtr", "ExponentWithLinearTransformRcPtr",
            "ConstExposureContrastTransformRcPtr", "ExposureContrastTransformRcPtr",
            "ConstFileTransformRcPtr", "FileTransformRcPtr",
            "ConstFixedFunctionTransformRcPtr", "FixedFunctionTransformRcPtr",
            "ConstGradingPrimaryTransformRcPtr", "GradingPrimaryTransformRcPtr",
            "ConstGradingRGBCurveTransformRcPtr", "GradingRGBCurveTransformRcPtr",
            "ConstGradingToneTransformRcPtr", "GradingToneTransformRcPtr",
            "ConstGroupTransformRcPtr", "GroupTransformRcPtr",
            "ConstLogAffineTransformRcPtr", "LogAffineTransformRcPtr",
            "ConstLogCameraTransformRcPtr", "LogCameraTransformRcPtr",
            "ConstLogTransformRcPtr", "LogTransformRcPtr",
            "ConstLookTransformRcPtr", "LookTransformRcPtr",
            "ConstLut1DTransformRcPtr", "Lut1DTransformRcPtr",
            "ConstLut3DTransformRcPtr", "Lut3DTransformRcPtr",
            "ConstMatrixTransformRcPtr", "MatrixTransformRcPtr",
            "ConstRangeTransformRcPtr", "RangeTransformRcPtr",
            "ConstDynamicPropertyRcPtr", "DynamicPropertyRcPtr",
            "ConstGpuShaderDescRcPtr", "GpuShaderDescRcPtr",
            "ConstGradingHueCurveTransformRcPtr", "GradingHueCurveTransformRcPtr",
            "ConstGradingHueCurveRcPtr", "GradingHueCurveRcPtr",
        ] {
            s.insert(*t);
        }
        s
    };

    static ref ENUM_TYPES: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for t in &[
            "BitDepth", "TransformDirection", "Interpolation",
            "OptimizationFlags", "GpuLanguage", "LoggingLevel",
            "ReferenceSpaceType", "ColorSpaceDirection",
            "Allocation", "TransformType", "FixedFunctionStyle",
            "ExposureContrastStyle", "CDLStyle",
            "GradingStyle", "GradingBSplineCurveStyle",
            "Lut1DHueAdjust", "NegativeStyle",
        ] {
            s.insert(*t);
        }
        s
    };
}

// ═══════════════════════════════════════════════════════════════════════════
// Parser
// ═══════════════════════════════════════════════════════════════════════════

const SKIP_CLASSES: &[&str] = &[
    "Exception", "ExceptionMissingFile", "ImageDesc",
    "PackedImageDesc", "PlanarImageDesc", "GpuShaderCreator",
    "ProcessorMetadata", "ViewingRules", "Transform",
    "GradingBSplineCurve", "DynamicProperty", "DynamicPropertyDouble",
    "DynamicPropertyGradingPrimary", "DynamicPropertyGradingRGBCurve",
    "DynamicPropertyGradingHueCurve", "DynamicPropertyGradingTone",
    "SystemMonitors", "BuiltinTransformRegistry", "ConfigIOProxy",
    "GradingRGBCurve", "GradingHueCurve",
    // v2.6+ classes that aren't bridged yet
    "ConfigMerger", "ConfigMergingParameters",
    "MixingColorSpaceManager", "LegacyViewingPipeline",
];

// Classes that are transforms (use TransformHandleBase)
const TRANSFORM_CLASSES: &[&str] = &[
    "AllocationTransform", "BuiltinTransform", "CDLTransform",
    "ColorSpaceTransform", "DisplayViewTransform",
    "ExponentTransform", "ExponentWithLinearTransform",
    "ExposureContrastTransform", "FileTransform",
    "FixedFunctionTransform",
    "GradingPrimaryTransform", "GradingRGBCurveTransform",
    "GradingHueCurveTransform", "GradingToneTransform",
    "GroupTransform", "LogAffineTransform", "LogCameraTransform",
    "LogTransform", "LookTransform", "Lut1DTransform",
    "Lut3DTransform", "MatrixTransform", "RangeTransform",
];

fn is_transform(class_name: &str) -> bool {
    TRANSFORM_CLASSES.contains(&class_name)
}

fn parse_ocio_headers(include_dir: &Path) -> HashMap<String, ClassDef> {
    let mut classes = HashMap::new();

    let mut entries: Vec<PathBuf> = std::fs::read_dir(include_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map_or(false, |ext| ext == "h"))
        .collect();
    entries.sort();

    for header_path in &entries {
        let fname = header_path.file_name().unwrap().to_str().unwrap();
        if fname == "OpenColorABI.h" {
            continue;
        }

        let content = match std::fs::read_to_string(header_path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let text = strip_comments(&content);

        // Find "class OCIOEXPORT ClassName [bases] {"
        let class_re = regex::Regex::new(
            r"class\s+OCIOEXPORT\s+(\w+)\s*(?::\s*[^{]+)?\s*\{"
        ).unwrap();

        for cap in class_re.captures_iter(&text) {
            let class_name = cap[1].to_string();

            if SKIP_CLASSES.contains(&class_name.as_str()) {
                continue;
            }

            let m = cap.get(0).unwrap();
            let body_start = m.end();
            let body_end = find_matching_brace(&text, body_start);
            let class_body = &text[body_start..body_end];

            // Extract public sections
            let sections: Vec<&str> = regex::Regex::new(r"\b(private|protected)\s*:")
                .unwrap()
                .split(class_body)
                .collect();
            let public_text = sections.first().unwrap_or(&"");

            // Parse methods
            let method_re = regex::Regex::new(
                r"(?:(static|virtual)\s+)?((?:const\s+)?\w+(?:\s*\*)?(?:\s*&)?)\s+(\w+)\s*\(([^)]*)\)\s*(?:const)?\s*(?:=\s*0\s*)?;"
            ).unwrap();

            let mut methods = Vec::new();
            let xt = class_name == "Config";

            for mm in method_re.captures_iter(public_text) {
                let is_static = mm.get(1).map_or(false, |m| m.as_str() == "static");
                let ret_type = mm[2].trim().to_string();
                let method_name = mm[3].to_string();

                // Skip constructors, destructors, operators
                if method_name == class_name || method_name.starts_with('~') {
                    continue;
                }
                if method_name.starts_with("operator") {
                    continue;
                }
                // Skip method names that parse as numbers (regex artifacts)
                if method_name.chars().all(|c| c.is_numeric()) {
                    continue;
                }
                // Skip static factory methods that are handled manually
                if class_name == "Config" && method_name.starts_with("Create") {
                    continue;
                }
                // Skip createEditableCopy — handled specially
                if method_name == "createEditableCopy" {
                    continue;
                }
                let params_str = mm[4].to_string();
                // Safety: skip methods with garbled types (comment artifacts)
                if ret_type.contains("*/") || ret_type.contains("/*")
                    || ret_type.len() > 200 || method_name.len() > 100 {
                    continue;
                }
                let all_params = parse_params(&params_str);
                if all_params.iter().any(|p|
                    p.cpp_type.contains("*/") || p.cpp_type.contains("/*")
                    || p.cpp_type.len() > 200 || p.name.len() > 100
                ) {
                    continue;
                }

                let params = all_params;

                if is_static && !xt {
                    continue;
                }

                methods.push(MethodDef {
                    name: method_name,
                    return_type: ret_type,
                    params,
                    is_const: mm.get(0).unwrap().as_str().contains("const"),
                    is_static,
                });
            }

            if !methods.is_empty() {
                let c_prefix = camel_to_snake(&class_name);
                let xt = is_transform(&class_name);
                classes.insert(
                    class_name.clone(),
                    ClassDef {
                        name: class_name,
                        c_prefix,
                        methods,
                        is_transform: xt,
                    },
                );
            }
        }
    }

    classes
}

// ═══════════════════════════════════════════════════════════════════════════
// Code Generators
// ═══════════════════════════════════════════════════════════════════════════

fn generate_bridge_hpp(classes: &HashMap<String, ClassDef>) -> String {
    let mut l = String::new();

    l.push_str("#pragma once\n\n");
    l.push_str("#include <stddef.h>\n#include <stdbool.h>\n\n");
    l.push_str("#ifndef BEGIN_TRY\n#define BEGIN_TRY try {\n#endif\n\n");
    l.push_str("#ifndef END_TRY\n#define END_TRY(return_value) \\\n");
    l.push_str("  } catch (...) {             \\\n");
    l.push_str("    return (return_value);    \\\n  }\n#endif\n\n");
    l.push_str("#ifndef END_TRY_VOID\n#define END_TRY_VOID \\\n  } catch (...) {}\n#endif\n\n");
    l.push_str("extern \"C\" {\n\n");

    // Runtime
    l.push_str("// --- Runtime ---\nbool ocio_runtime_is_stub(void);\n\n");

    // Global utility
    l.push_str("// --- Global utility functions ---\n");
    l.push_str("const char* ocio_get_version(void);\n");
    l.push_str("int ocio_get_version_hex(void);\n");
    l.push_str("int ocio_get_logging_level(void);\n");
    l.push_str("void ocio_set_logging_level(int level);\n\n");

    // Global config
    l.push_str("// --- Global config ---\n");
    l.push_str("void* ocio_get_current_config(void);\n");
    l.push_str("void ocio_set_current_config(void* config);\n");
    l.push_str("void ocio_clear_all_caches(void);\n\n");

    // BuiltinConfigRegistry
    if let Some(cls) = classes.get("BuiltinConfigRegistry") {
        l.push_str("// --- BuiltinConfigRegistry ---\n");
        l.push_str("void* ocio_builtin_config_registry_get(void);\n");
        gen_hpp_class(&mut l, cls);
        l.push('\n');
    }

    // Config
    if let Some(cls) = classes.get("Config") {
        l.push_str("// --- Config ---\n");
        l.push_str("void* ocio_config_create_raw(void);\n");
        l.push_str("void* ocio_config_create_from_file(const char* path);\n");
        l.push_str("void ocio_config_destroy(void* handle);\n\n");
        gen_hpp_class(&mut l, cls);
        l.push('\n');
    }

    // Simple classes
    for name in &[
        "FileRules", "ColorSpace", "ColorSpaceSet", "Look", "NamedTransform",
        "ViewTransform", "Processor", "CPUProcessor", "GPUProcessor",
        "GpuShaderDesc", "Baker", "Context", "FormatMetadata",
    ] {
        if let Some(cls) = classes.get(*name) {
            l.push_str(&format!("// --- {} ---\n", name));
            l.push_str(&format!("void* ocio_{}_create(void);\n", cls.c_prefix));
            l.push_str(&format!("void ocio_{}_destroy(void* handle);\n\n", cls.c_prefix));
            gen_hpp_class(&mut l, cls);
            l.push('\n');
        }
    }

    // Transform classes
    for name in TRANSFORM_CLASSES {
        if let Some(cls) = classes.get(*name) {
            l.push_str(&format!("// --- {} ---\n", name));
            l.push_str(&format!("void* ocio_{}_create(void);\n", cls.c_prefix));
            l.push_str(&format!("void ocio_{}_destroy(void* handle);\n\n", cls.c_prefix));
            gen_hpp_class(&mut l, cls);
            l.push('\n');
        }
    }

    // DynamicProperty
    if let Some(cls) = classes.get("DynamicProperty") {
        l.push_str("// --- DynamicProperty ---\n");
        l.push_str("void ocio_dynamic_property_destroy(void* handle);\n");
        gen_hpp_class(&mut l, cls);
        l.push('\n');
    }

    l.push_str("}  // extern \"C\"\n");
    l
}

fn gen_hpp_class(l: &mut String, cls: &ClassDef) {
    for (m, c_name) in compute_c_names(cls) {
        let (ret_c, _, _) = map_return_type(&m.return_type);
        let params = build_c_params(&m.params, &c_name);
        l.push_str(&format!("{} {}({}{}{});\n",
            ret_c, c_name,
            "void* handle",
            if params.is_empty() { "" } else { ", " },
            params));
    }
}

fn build_c_params(params: &[Param], _func_name: &str) -> String {
    use std::collections::HashMap;
    let mut name_counts: HashMap<String, usize> = HashMap::new();

    params
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let c_type = map_param_type(&p.cpp_type);
            let base_name = if p.name.is_empty() {
                format!("arg{}", i)
            } else {
                p.name.clone()
            };
            // Ensure unique parameter names
            let count = name_counts.entry(base_name.clone()).or_insert(0);
            *count += 1;
            let name = if *count > 1 {
                format!("{}_{}", base_name, count)
            } else {
                base_name
            };
            format!("{} {}", c_type, name)
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn map_param_type(cpp_type: &str) -> &str {
    match cpp_type.trim() {
        "const char*" => "const char*",
        t if t.ends_with("RcPtr") || t.ends_with("RcPtr&") => "void*",
        t if t.contains("RcPtr") => "void*",
        "int" => "int",
        "unsigned int" => "unsigned int",
        "size_t" => "size_t",
        "bool" => "bool",
        "float" => "float",
        "double" => "double",
        "char" => "char",
        t if ENUM_TYPES.contains(t) => "int",
        _ => "void*",
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// bridge.cpp generation
// ═══════════════════════════════════════════════════════════════════════════

fn generate_bridge_cpp(classes: &HashMap<String, ClassDef>) -> String {
    let mut l = String::new();

    // Preamble
    l.push_str("#include \"bridge.hpp\"\n\n");
    l.push_str("#include <cstring>\n#include <memory>\n#include <stdexcept>\n");
    l.push_str("#include <string>\n#include <fstream>\n\n");
    l.push_str("#ifndef OCIO_RS_STUB\n");
    l.push_str("#include <OpenColorIO/OpenColorIO.h>\n");
    l.push_str("#include <OpenColorIO/OpenColorTransforms.h>\n");
    l.push_str("namespace ocio = OCIO_NAMESPACE;\n");
    l.push_str("#endif\n\n");
    l.push_str("namespace ocio_rs_bridge {\n\n");

    // Handle structs
    gen_handle_structs(&mut l, classes);

    // Stub section
    l.push_str("#ifdef OCIO_RS_STUB\n\n");
    gen_stub_structs(&mut l, classes);
    gen_stub_make_functions(&mut l, classes);
    l.push_str("\n#else // real OCIO\n\n");

    // Real section
    gen_real_structs(&mut l, classes);
    gen_transform_get_ocio(&mut l, classes);
    gen_real_accessors(&mut l, classes);
    gen_real_make_functions(&mut l, classes);
    l.push_str("\n#endif // OCIO_RS_STUB\n\n");
    l.push_str("}  // namespace ocio_rs_bridge\n\n");

    // extern "C" implementations
    l.push_str("// =====================================================================\n");
    l.push_str("// extern \"C\" implementations\n");
    l.push_str("// =====================================================================\n\n");
    l.push_str("extern \"C\" {\n\n");

    // Runtime
    l.push_str("// --- Runtime ---\n");
    l.push_str("bool ocio_runtime_is_stub(void) {\n");
    l.push_str("#ifdef OCIO_RS_STUB\n  return true;\n#else\n  return false;\n#endif\n}\n\n");

    // Global utility
    l.push_str("// --- Global utility functions ---\n");
    l.push_str("const char* ocio_get_version(void) {\n");
    l.push_str("#ifdef OCIO_RS_STUB\n  return \"stub\";\n#else\n");
    l.push_str("  try { return ocio::GetVersion(); } catch (...) { return nullptr; }\n#endif\n}\n\n");
    l.push_str("int ocio_get_version_hex(void) {\n");
    l.push_str("#ifdef OCIO_RS_STUB\n  return 0;\n#else\n");
    l.push_str("  try { return ocio::GetVersionHex(); } catch (...) { return 0; }\n#endif\n}\n\n");
    l.push_str("int ocio_get_logging_level(void) {\n");
    l.push_str("#ifdef OCIO_RS_STUB\n  return 0;\n#else\n");
    l.push_str("  try { return static_cast<int>(ocio::GetLoggingLevel()); } catch (...) { return 0; }\n#endif\n}\n\n");
    l.push_str("void ocio_set_logging_level(int level) {\n");
    l.push_str("#ifdef OCIO_RS_STUB\n  (void)level;\n#else\n");
    l.push_str("  try { ocio::SetLoggingLevel(static_cast<ocio::LoggingLevel>(level)); } catch (...) {}\n#endif\n}\n\n");

    // Global config
    l.push_str("// --- Global config ---\n");
    l.push_str("void* ocio_get_current_config(void) {\n");
    l.push_str("#ifdef OCIO_RS_STUB\n  return nullptr;\n#else\n  try {\n");
    l.push_str("    auto cfg = ocio::GetCurrentConfig();\n    if (!cfg) return nullptr;\n");
    l.push_str("    auto handle = std::make_unique<ocio_rs_bridge::ConfigHandle>();\n");
    l.push_str("    handle->inner = std::make_shared<ocio_rs_bridge::RealConfig>(ocio_rs_bridge::RealConfig{cfg});\n");
    l.push_str("    return handle.release();\n  } catch (...) { return nullptr; }\n#endif\n}\n\n");
    l.push_str("void ocio_set_current_config(void* config) {\n");
    l.push_str("#ifdef OCIO_RS_STUB\n  (void)config;\n#else\n  try {\n");
    l.push_str("    ocio::SetCurrentConfig(ocio_rs_bridge::get_real_config(config));\n");
    l.push_str("  } catch (...) {}\n#endif\n}\n\n");
    l.push_str("void ocio_clear_all_caches(void) {\n");
    l.push_str("#ifdef OCIO_RS_STUB\n#else\n  try { ocio::ClearAllCaches(); } catch (...) {}\n#endif\n}\n\n");

    // BuiltinConfigRegistry
    if let Some(cls) = classes.get("BuiltinConfigRegistry") {
        l.push_str("// --- BuiltinConfigRegistry ---\n");
        l.push_str("void* ocio_builtin_config_registry_get(void) {\n");
        l.push_str("#ifdef OCIO_RS_STUB\n  return nullptr;\n#else\n  try {\n");
        l.push_str("    auto& registry = ocio::BuiltinConfigRegistry::Get();\n");
        l.push_str("    auto handle = std::make_unique<ocio_rs_bridge::BuiltinConfigRegistryHandle>();\n");
        l.push_str("    handle->inner = std::make_shared<ocio_rs_bridge::RealBuiltinConfigRegistry>(\n");
        l.push_str("      ocio_rs_bridge::RealBuiltinConfigRegistry{&registry});\n");
        l.push_str("    return handle.release();\n  } catch (...) { return nullptr; }\n#endif\n}\n\n");
        gen_cpp_methods(&mut l, cls);
    }

    // Config
    l.push_str("\n// --- Config ---\n");
    l.push_str("void* ocio_config_create_raw(void) {\n");
    l.push_str("#ifdef OCIO_RS_STUB\n  return ocio_rs_bridge::make_stub_config().release();\n#else\n");
    l.push_str("  auto handle = ocio_rs_bridge::make_real_config_raw();\n");
    l.push_str("  if (!handle) return nullptr;\n  return handle.release();\n#endif\n}\n\n");
    l.push_str("void* ocio_config_create_from_file(const char* path) {\n");
    l.push_str("#ifdef OCIO_RS_STUB\n  (void)path; return ocio_rs_bridge::make_stub_config().release();\n#else\n");
    l.push_str("  auto handle = ocio_rs_bridge::make_real_config_from_file(path);\n");
    l.push_str("  if (!handle) return nullptr;\n  return handle.release();\n#endif\n}\n\n");
    l.push_str("void ocio_config_destroy(void* handle) {\n");
    l.push_str("  delete static_cast<ocio_rs_bridge::ConfigHandle*>(handle);\n}\n\n");
    if let Some(cls) = classes.get("Config") {
        gen_cpp_methods(&mut l, cls);
    }
    l.push('\n');

    // Simple classes
    for name in &[
        "FileRules", "ColorSpace", "ColorSpaceSet", "Look", "NamedTransform",
        "ViewTransform", "Processor", "CPUProcessor", "GPUProcessor",
        "GpuShaderDesc", "Baker", "Context", "FormatMetadata",
    ] {
        if let Some(cls) = classes.get(*name) {
            gen_simple_class_cpp(&mut l, cls);
        }
    }

    // Transform classes
    for name in TRANSFORM_CLASSES {
        if let Some(cls) = classes.get(*name) {
            gen_simple_class_cpp(&mut l, cls);
        }
    }

    // DynamicProperty
    if let Some(cls) = classes.get("DynamicProperty") {
        gen_simple_class_cpp(&mut l, cls);
    }

    l.push_str("}  // extern \"C\"\n");
    l
}

fn gen_handle_structs(l: &mut String, _classes: &HashMap<String, ClassDef>) {
    l.push_str("// --- Handle types ---\n\n");
    l.push_str("struct ConfigHandle { std::shared_ptr<void> inner; };\n");
    l.push_str("struct ProcessorHandle { std::shared_ptr<void> inner; };\n");
    l.push_str("struct CPUProcessorHandle { std::shared_ptr<void> inner; };\n");
    l.push_str("struct GPUProcessorHandle { std::shared_ptr<void> inner; };\n");
    l.push_str("struct GpuShaderDescHandle { std::shared_ptr<void> inner; };\n\n");

    // TransformHandleBase
    l.push_str("struct TransformHandleBase {\n");
    l.push_str("  virtual ~TransformHandleBase() = default;\n");
    l.push_str("  virtual int get_transform_type_tag() const = 0;\n");
    l.push_str("#ifndef OCIO_RS_STUB\n");
    l.push_str("  virtual OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() = 0;\n");
    l.push_str("#endif\n};\n\n");

    // Transform tag mapping
    let tag_map: HashMap<&str, &str> = [
        ("AllocationTransform", "0"), ("BuiltinTransform", "1"),
        ("CDLTransform", "2"), ("ColorSpaceTransform", "3"),
        ("DisplayViewTransform", "4"), ("ExponentTransform", "5"),
        ("ExponentWithLinearTransform", "6"), ("ExposureContrastTransform", "7"),
        ("FileTransform", "8"), ("FixedFunctionTransform", "9"),
        ("GradingPrimaryTransform", "10"), ("GradingRGBCurveTransform", "11"),
        ("GradingHueCurveTransform", "12"), ("GradingToneTransform", "13"),
        ("GroupTransform", "14"), ("LogAffineTransform", "15"),
        ("LogCameraTransform", "16"), ("LogTransform", "17"),
        ("LookTransform", "18"), ("Lut1DTransform", "19"),
        ("Lut3DTransform", "20"), ("MatrixTransform", "21"),
        ("RangeTransform", "22"),
    ].iter().cloned().collect();

    for name in TRANSFORM_CLASSES {
        if let Some(tag) = tag_map.get(name) {
            l.push_str(&format!("struct {}Handle : TransformHandleBase {{ std::shared_ptr<void> inner;\n", name));
            l.push_str(&format!("  int get_transform_type_tag() const override {{ return {}; }}\n", tag));
            l.push_str("#ifndef OCIO_RS_STUB\n");
            l.push_str(&format!("  OCIO_NAMESPACE::TransformRcPtr get_ocio_transform() override;\n"));
            l.push_str("#endif\n};\n");
        }
    }
    l.push('\n');

    // Non-transform handles
    l.push_str("struct BakerHandle { std::shared_ptr<void> inner; };\n");
    l.push_str("struct ContextHandle { std::shared_ptr<void> inner; };\n");
    l.push_str("struct ColorSpaceHandle { std::shared_ptr<void> inner; };\n");
    l.push_str("struct LookHandle { std::shared_ptr<void> inner; };\n");
    l.push_str("struct ViewTransformHandle { std::shared_ptr<void> inner; };\n");
    l.push_str("struct NamedTransformHandle { std::shared_ptr<void> inner; };\n");
    l.push_str("struct DynamicPropertyHandle { std::shared_ptr<void> inner; };\n");
    l.push_str("struct BuiltinConfigRegistryHandle { std::shared_ptr<void> inner; };\n");
    l.push_str("struct FileRulesHandle { std::shared_ptr<void> inner; };\n");
    l.push_str("struct ColorSpaceSetHandle { std::shared_ptr<void> inner; };\n");
    l.push_str("struct FormatMetadataHandle { std::shared_ptr<void> inner; };\n");
}

fn gen_stub_structs(l: &mut String, classes: &HashMap<String, ClassDef>) {
    l.push_str("// --- Stub wrapper structs ---\n");
    for cls in classes.values() {
        if cls.name == "BuiltinConfigRegistry" { continue; }
        l.push_str(&format!("struct Stub{} {{}};\n", cls.name));
    }
    l.push_str("struct StubBuiltinConfigRegistry {\n");
    l.push_str("  int getNumBuiltinConfigs() { return 0; }\n");
    l.push_str("  const char* getBuiltinConfigName(int) { return nullptr; }\n");
    l.push_str("};\n");
}

fn gen_stub_make_functions(l: &mut String, classes: &HashMap<String, ClassDef>) {
    l.push_str("\n// --- Stub make functions ---\n");
    for cls in classes.values() {
        let prefix = &cls.c_prefix;
        let handle = format!("{}Handle", cls.name);
        let name = &cls.name;
        l.push_str(&format!("static std::unique_ptr<{}> make_stub_{}() {{\n", handle, prefix));
        l.push_str(&format!("  auto handle = std::make_unique<{}>();\n", handle));
        l.push_str(&format!("  handle->inner = std::make_shared<Stub{}>();\n", name));
        l.push_str("  return handle;\n}\n\n");
    }
    // Config special — renamed to avoid collision with for-loop-generated version
    l.push_str("static std::unique_ptr<ConfigHandle> make_stub_config_raw() {\n");
    l.push_str("  auto handle = std::make_unique<ConfigHandle>();\n");
    l.push_str("  handle->inner = std::make_shared<StubConfig>();\n");
    l.push_str("  return handle;\n}\n");
}

fn gen_real_structs(l: &mut String, classes: &HashMap<String, ClassDef>) {
    l.push_str("// --- Real OCIO wrapper types ---\n");
    for cls in classes.values() {
        if cls.name == "BuiltinConfigRegistry" {
            l.push_str("struct RealBuiltinConfigRegistry {\n");
            l.push_str("  ocio::BuiltinConfigRegistry* registry;\n};\n");
            continue;
        }
        let member = real_member_for_class(&cls.name, cls.is_transform);
        let rcptr = rcptr_for_class(&cls.name, cls.is_transform);
        if cls.name == "FormatMetadata" {
            l.push_str("struct RealFormatMetadata {\n  ocio::FormatMetadata* metadata;\n  bool owned;\n};\n");
        } else {
            l.push_str(&format!("struct Real{} {{\n  ocio::{} {};\n}};\n", cls.name, rcptr, member));
        }
    }
}

fn gen_transform_get_ocio(l: &mut String, classes: &HashMap<String, ClassDef>) {
    l.push_str("\n// --- TransformHandleBase out-of-line ---\n");
    for name in TRANSFORM_CLASSES {
        if classes.contains_key(*name) {
            l.push_str(&format!("ocio::TransformRcPtr {}Handle::get_ocio_transform() {{\n", name));
            l.push_str(&format!("  return std::static_pointer_cast<Real{}>(inner)->transform;\n}}\n", name));
        }
    }
}

fn gen_real_accessors(l: &mut String, classes: &HashMap<String, ClassDef>) {
    l.push_str("\n// --- Real accessor functions ---\n");

    let accessors: Vec<(&str, &str, &str)> = vec![
        ("config", "ConfigHandle", "RealConfig"),
        ("file_rules", "FileRulesHandle", "RealFileRules"),
        ("color_space", "ColorSpaceHandle", "RealColorSpace"),
        ("color_space_set", "ColorSpaceSetHandle", "RealColorSpaceSet"),
        ("look", "LookHandle", "RealLook"),
        ("named_transform", "NamedTransformHandle", "RealNamedTransform"),
        ("view_transform", "ViewTransformHandle", "RealViewTransform"),
        ("processor", "ProcessorHandle", "RealProcessor"),
        ("cpu_processor", "CPUProcessorHandle", "RealCPUProcessor"),
        ("gpu_processor", "GPUProcessorHandle", "RealGPUProcessor"),
        ("gpu_shader_desc", "GpuShaderDescHandle", "RealGpuShaderDesc"),
        ("baker", "BakerHandle", "RealBaker"),
        ("context", "ContextHandle", "RealContext"),
        ("format_metadata", "FormatMetadataHandle", "RealFormatMetadata"),
        ("dynamic_property", "DynamicPropertyHandle", "RealDynamicProperty"),
        ("builtin_config_registry", "BuiltinConfigRegistryHandle", "RealBuiltinConfigRegistry"),
    ];

    for (prefix, handle, real) in &accessors {
        let class_name = real.strip_prefix("Real").unwrap_or(real);
        if !classes.contains_key(class_name) && *prefix != "builtin_config_registry" {
            continue;
        }
        let member = real_member_for_class(class_name, is_transform(prefix));
        let ret_type = if *prefix == "builtin_config_registry" {
            "ocio::BuiltinConfigRegistry*".to_string()
        } else {
            format!("ocio::{}", rcptr_for_class(class_name, is_transform(prefix)))
        };
        l.push_str(&format!("static {} get_real_{}(void* handle) {{\n", ret_type, prefix));
        l.push_str(&format!("  auto* h = static_cast<ocio_rs_bridge::{}*>(handle);\n", handle));
        l.push_str(&format!("  return std::static_pointer_cast<ocio_rs_bridge::{}>(h->inner)->{};\n", real, member));
        l.push_str("}\n\n");
    }

    // Transform accessors
    for name in TRANSFORM_CLASSES {
        if !classes.contains_key(*name) { continue; }
        let prefix = camel_to_snake(name);
        let handle = format!("{}Handle", name);
        let real = format!("Real{}", name);
        l.push_str(&format!("static ocio::{}RcPtr get_real_{}(void* handle) {{\n", name, prefix));
        l.push_str(&format!("  auto* h = static_cast<ocio_rs_bridge::{}*>(handle);\n", handle));
        l.push_str(&format!("  return std::static_pointer_cast<ocio_rs_bridge::{}>(h->inner)->transform;\n", real));
        l.push_str("}\n\n");
    }
}

fn gen_real_make_functions(l: &mut String, classes: &HashMap<String, ClassDef>) {
    l.push_str("\n// --- Real make functions ---\n");

    for cls in classes.values() {
        let prefix = &cls.c_prefix;
        let handle = format!("{}Handle", cls.name);
        let real = format!("Real{}", cls.name);
        let member = real_member_for_class(&cls.name, cls.is_transform);

        if cls.name == "BuiltinConfigRegistry" { continue; }
        if cls.name == "Config" || cls.name == "Processor" || cls.name == "CPUProcessor"
            || cls.name == "GPUProcessor" || cls.name == "FormatMetadata"
            || cls.name == "GpuShaderDesc" || cls.name == "DynamicProperty"
        {
            continue;
        }

        let create_call = if cls.is_transform {
            format!("ocio::{}::Create()", cls.name)
        } else {
            match cls.name.as_str() {
                "Baker" => "ocio::Baker::Create()".into(),
                "Context" => "ocio::Context::Create()".into(),
                "FileRules" => "ocio::FileRules::Create()".into(),
                "ColorSpace" => "ocio::ColorSpace::Create()".into(),
                "ColorSpaceSet" => "ocio::ColorSpaceSet::Create()".into(),
                _ => continue,
            }
        };

        l.push_str(&format!("static std::unique_ptr<{}> make_real_{}() {{\n", handle, prefix));
        l.push_str("  try {\n");
        l.push_str(&format!("    auto handle = std::make_unique<{}>();\n", handle));
        l.push_str(&format!("    auto obj = std::make_shared<{}>();\n", real));
        l.push_str(&format!("    obj->{} = {};\n", member, create_call));
        l.push_str("    handle->inner = obj;\n    return handle;\n");
        l.push_str("  } catch (...) { return nullptr; }\n}\n\n");
    }

    // Config special
    l.push_str("static std::unique_ptr<ConfigHandle> make_real_config_raw() {\n");
    l.push_str("  try {\n    auto handle = std::make_unique<ConfigHandle>();\n");
    l.push_str("    auto config = std::make_shared<RealConfig>();\n");
    l.push_str("    config->config = ocio::Config::CreateRaw();\n");
    l.push_str("    handle->inner = config;\n    return handle;\n");
    l.push_str("  } catch (...) { return nullptr; }\n}\n\n");
    l.push_str("static std::unique_ptr<ConfigHandle> make_real_config_from_file(const char* path) {\n");
    l.push_str("  try {\n    auto handle = std::make_unique<ConfigHandle>();\n");
    l.push_str("    auto config = std::make_shared<RealConfig>();\n");
    l.push_str("    config->config = ocio::Config::CreateFromFile(path);\n");
    l.push_str("    if (!config->config) return nullptr;\n");
    l.push_str("    handle->inner = config;\n    return handle;\n");
    l.push_str("  } catch (...) { return nullptr; }\n}\n");
}

fn gen_simple_class_cpp(l: &mut String, cls: &ClassDef) {
    let prefix = &cls.c_prefix;
    let handle = format!("{}Handle", cls.name);

    l.push_str(&format!("// --- {} ---\n\n", cls.name));

    // Create
    l.push_str(&format!("void* ocio_{}_create(void) {{\n", prefix));
    l.push_str("#ifdef OCIO_RS_STUB\n");
    l.push_str(&format!("  return ocio_rs_bridge::make_stub_{}().release();\n", prefix));
    l.push_str("#else\n");
    l.push_str(&format!("  auto handle = ocio_rs_bridge::make_real_{}();\n", prefix));
    l.push_str("  if (!handle) return nullptr;\n  return handle.release();\n#endif\n}\n\n");

    // Destroy
    l.push_str(&format!("void ocio_{}_destroy(void* handle) {{\n", prefix));
    l.push_str(&format!("  delete static_cast<ocio_rs_bridge::{}*>(handle);\n", handle));
    l.push_str("}\n\n");

    // Methods
    gen_cpp_methods(l, cls);
    l.push('\n');
}

/// Extract clean class name from RcPtr type
fn extract_class_from_rcptr(t: &str) -> String {
    let t = t.trim();
    // Remove trailing &
    let t = t.trim_end_matches('&');
    // Remove leading const/&
    let t = t.trim_start_matches("const ");
    let t = t.trim();
    // Remove leading Const prefix
    let t = if t.starts_with("Const") && t.len() > 5 && t[5..].chars().next().map_or(false, |c| c.is_uppercase()) {
        &t[5..]
    } else {
        t
    };
    // Remove trailing RcPtr
    let t = t.trim_end_matches("RcPtr");
    t.to_string()
}

fn gen_cpp_methods(l: &mut String, cls: &ClassDef) {
    let prefix = &cls.c_prefix;

    for (m, c_name) in compute_c_names(cls) {
        let (ret_c, stub_ret, wraps) = map_return_type(&m.return_type);

        // Build parameter strings
        let mut func_params = Vec::new();
        let mut call_args = Vec::new();
        let mut unwrap_stmts = Vec::new();

        // Use the same unique naming as build_c_params
        let mut cpp_name_counts: HashMap<String, usize> = HashMap::new();
        for p in &m.params {
            let c_ptype = map_param_type(&p.cpp_type);
            let base_name = if p.name.is_empty() { "arg".to_string() } else { p.name.clone() };
            let count = cpp_name_counts.entry(base_name.clone()).or_insert(0);
            *count += 1;
            let pname = if *count > 1 {
                format!("{}_{}", base_name, *count)
            } else {
                base_name
            };
            func_params.push(format!("{} {}", c_ptype, pname));

            // Check if this param is an RcPtr handle that needs unwrapping
            if p.cpp_type.contains("RcPtr") && !p.cpp_type.starts_with("const char") {
                let class_hint = extract_class_from_rcptr(&p.cpp_type);
                if class_hint == "const" || class_hint.is_empty() { call_args.push(pname.to_string()); continue; }
                let handle_name = format!("{}Handle", class_hint);
                let real_name = format!("Real{}", class_hint);
                let member = if is_transform_rcptr(&p.cpp_type) { "transform" }
                    else { real_member_for_class(&class_hint, false) };
                // Cast to Handle type first, then unwrap inner to Real type
                unwrap_stmts.push(format!(
                    "auto* _{p}_h = static_cast<ocio_rs_bridge::{handle}*>({p});",
                    p = pname, handle = handle_name
                ));
                unwrap_stmts.push(format!(
                    "auto {p}_ptr = std::static_pointer_cast<ocio_rs_bridge::{real}>(_{p}_h->inner)->{mem};",
                    p = pname, real = real_name, mem = member
                ));
                call_args.push(format!("{}_ptr", pname));
            } else {
                call_args.push(pname.to_string());
            }
        }

        let params_str = if func_params.is_empty() {
            String::new()
        } else {
            format!(", {}", func_params.join(", "))
        };

        let accessor = format!("ocio_rs_bridge::get_real_{}", prefix);
        let call = if call_args.is_empty() {
            format!("{}(handle)->{}()", accessor, m.name)
        } else {
            format!("{}(handle)->{}({})", accessor, m.name, call_args.join(", "))
        };

        // Function header
        l.push_str(&format!("{} {}(void* handle{}) {{\n", ret_c, c_name, params_str));

        // Stub
        l.push_str("#ifdef OCIO_RS_STUB\n");
        let void_suppress = func_params.iter()
            .map(|p| {
                let parts: Vec<&str> = p.split_whitespace().collect();
                if parts.len() >= 2 {
                    format!("(void){};", parts.last().unwrap())
                } else { String::new() }
            })
            .collect::<Vec<_>>()
            .join(" ");
        l.push_str(&format!("  (void)handle; {}\n", void_suppress));
        if ret_c == "void" {
            l.push_str(&format!("  return;\n"));
        } else {
            l.push_str(&format!("  return {};\n", stub_ret));
        }

        // Real
        l.push_str("#else\n  try {\n");
        for stmt in &unwrap_stmts {
            l.push_str(&format!("    {}\n", stmt));
        }

        if wraps {
            l.push_str(&format!("    auto result = {};\n", call));
            l.push_str("    if (!result) return nullptr;\n");
            let class_hint = extract_class_from_rcptr(&m.return_type);
            if class_hint == "FormatMetadata" {
                // FormatMetadata is special - not RcPtr based
                l.push_str("    auto out_handle = std::make_unique<ocio_rs_bridge::FormatMetadataHandle>();\n");
                l.push_str("    out_handle->inner = std::make_shared<ocio_rs_bridge::RealFormatMetadata>(ocio_rs_bridge::RealFormatMetadata{result, true});\n");
            } else if !class_hint.is_empty() && class_hint != "const" && class_hint != "BuiltinConfigRegistry" {
                let wrap_handle = format!("{}Handle", class_hint);
                let wrap_real = format!("Real{}", class_hint);
                l.push_str(&format!("    auto out_handle = std::make_unique<ocio_rs_bridge::{}>();\n", wrap_handle));
                l.push_str(&format!("    out_handle->inner = std::make_shared<ocio_rs_bridge::{}>(ocio_rs_bridge::{}{{result}});\n", wrap_real, wrap_real));
            } else {
                l.push_str("    auto out_handle = std::make_unique<ocio_rs_bridge::ConfigHandle>();\n");
                l.push_str("    out_handle->inner = std::make_shared<ocio_rs_bridge::RealConfig>(ocio_rs_bridge::RealConfig{result});\n");
            }
            l.push_str("    return out_handle.release();\n");
        } else if ret_c == "void" {
            l.push_str(&format!("    {};\n", call));
        } else {
            l.push_str(&format!("    return {};\n", call));
        }

        l.push_str(&format!("  }} catch (...) {{ return {}; }}\n#endif\n", if ret_c == "void" { "" } else { stub_ret }));
        l.push_str("}\n\n");
    }
}

fn is_transform_rcptr(cpp_type: &str) -> bool {
    let clean = cpp_type.replace("Const", "").replace("RcPtr", "").replace("&", "");
    TRANSFORM_CLASSES.contains(&clean.as_str())
}

// ═══════════════════════════════════════════════════════════════════════════
// lib.rs generation
// ═══════════════════════════════════════════════════════════════════════════

fn generate_lib_rs(classes: &HashMap<String, ClassDef>) -> String {
    let mut l = String::new();
    l.push_str("use std::ffi::c_void;\n\n");
    l.push_str("unsafe extern \"C\" {\n");

    // Runtime
    l.push_str("    // --- Runtime ---\n");
    l.push_str("    pub fn ocio_runtime_is_stub() -> bool;\n\n");

    // Global
    l.push_str("    // --- Global utility functions ---\n");
    l.push_str("    pub fn ocio_get_version() -> *const i8;\n");
    l.push_str("    pub fn ocio_get_version_hex() -> i32;\n");
    l.push_str("    pub fn ocio_get_logging_level() -> i32;\n");
    l.push_str("    pub fn ocio_set_logging_level(level: i32);\n\n");

    l.push_str("    // --- Global config ---\n");
    l.push_str("    pub fn ocio_get_current_config() -> *mut c_void;\n");
    l.push_str("    pub fn ocio_set_current_config(config: *mut c_void);\n");
    l.push_str("    pub fn ocio_clear_all_caches();\n\n");

    // BuiltinConfigRegistry
    if let Some(cls) = classes.get("BuiltinConfigRegistry") {
        l.push_str("    // --- BuiltinConfigRegistry ---\n");
        l.push_str("    pub fn ocio_builtin_config_registry_get() -> *mut c_void;\n");
        gen_rs_class(&mut l, cls);
    }

    // Config
    l.push_str("\n    // --- Config ---\n");
    l.push_str("    pub fn ocio_config_create_raw() -> *mut c_void;\n");
    l.push_str("    pub fn ocio_config_create_from_file(path: *const i8) -> *mut c_void;\n");
    l.push_str("    pub fn ocio_config_destroy(handle: *mut c_void);\n");
    if let Some(cls) = classes.get("Config") {
        gen_rs_class(&mut l, cls);
    }

    // Simple classes
    for name in &[
        "FileRules", "ColorSpace", "ColorSpaceSet", "Look", "NamedTransform",
        "ViewTransform", "Processor", "CPUProcessor", "GPUProcessor",
        "GpuShaderDesc", "Baker", "Context", "FormatMetadata",
    ] {
        if let Some(cls) = classes.get(*name) {
            l.push_str(&format!("\n    // --- {} ---\n", name));
            l.push_str(&format!("    pub fn ocio_{}_create() -> *mut c_void;\n", cls.c_prefix));
            l.push_str(&format!("    pub fn ocio_{}_destroy(handle: *mut c_void);\n", cls.c_prefix));
            gen_rs_class(&mut l, cls);
        }
    }

    // Transforms
    for name in TRANSFORM_CLASSES {
        if let Some(cls) = classes.get(*name) {
            l.push_str(&format!("\n    // --- {} ---\n", name));
            l.push_str(&format!("    pub fn ocio_{}_create() -> *mut c_void;\n", cls.c_prefix));
            l.push_str(&format!("    pub fn ocio_{}_destroy(handle: *mut c_void);\n", cls.c_prefix));
            gen_rs_class(&mut l, cls);
        }
    }

    // DynamicProperty
    if let Some(cls) = classes.get("DynamicProperty") {
        l.push_str("\n    // --- DynamicProperty ---\n");
        l.push_str("    pub fn ocio_dynamic_property_destroy(handle: *mut c_void);\n");
        gen_rs_class(&mut l, cls);
    }

    l.push_str("}\n");
    l
}

fn gen_rs_class(l: &mut String, cls: &ClassDef) {
    for (m, c_name) in compute_c_names(cls) {
        let (ret_c, _, _) = map_return_type(&m.return_type);
        let rust_ret = map_c_to_rust(ret_c);

        let mut params = vec!["handle: *mut c_void".to_string()];
        let mut used_names: HashMap<String, usize> = HashMap::new();
        for (i, p) in m.params.iter().enumerate() {
            let c_ptype = map_param_type(&p.cpp_type);
            let rust_ptype = map_c_to_rust(c_ptype);
            let base_name = if p.name.is_empty() {
                format!("arg{}", i)
            } else {
                escape_rust_ident(&p.name)
            };
            let count = used_names.entry(base_name.clone()).or_insert(0);
            *count += 1;
            let pname = if *count > 1 {
                format!("{}_{}", base_name, *count)
            } else {
                base_name
            };
            params.push(format!("{}: {}", pname, rust_ptype));
        }
        l.push_str(&format!("    pub fn {}({}) -> {};\n",
            c_name, params.join(", "), rust_ret));
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Main
// ═══════════════════════════════════════════════════════════════════════════

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let check_only = args.iter().any(|a| a == "--check");
    let ocio_path = args.iter()
        .position(|a| a == "--ocio-path")
        .and_then(|i| args.get(i + 1).cloned());

    // Determine paths
    // The generator is at tools/generator/; the ocio-rs project root is two levels up
    let gen_dir = Path::new(&std::env::current_exe().unwrap()).parent().unwrap().to_path_buf();
    let root = if gen_dir.ends_with("tools/generator") {
        gen_dir.parent().unwrap().parent().unwrap().to_path_buf()
    } else {
        // Fallback: assume we're running from the project root
        PathBuf::from(".")
    };
    let include_dir = if let Some(ref p) = ocio_path {
        PathBuf::from(p)
    } else {
        root.join("third_party/OpenColorIO/include/OpenColorIO")
    };

    let bridge_hpp = root.join("ocio-sys/src/bridge.hpp");
    let bridge_cpp = root.join("ocio-sys/src/bridge.cpp");
    let lib_rs = root.join("ocio-sys/src/lib.rs");

    if !include_dir.exists() {
        eprintln!("ERROR: OCIO headers not found at {}", include_dir.display());
        eprintln!("  Specify path with --ocio-path or place OCIO in third_party/OpenColorIO/");
        std::process::exit(1);
    }

    eprintln!("Parsing OCIO headers from: {}", include_dir.display());
    let classes = parse_ocio_headers(&include_dir);
    eprintln!("Found {} classes", classes.len());

    let total_methods: usize = classes.values().map(|c| c.methods.len()).sum();
    eprintln!("Total instance methods: {}", total_methods);

    for (name, cls) in &classes {
        eprintln!("  {}: {} methods", name, cls.methods.len());
    }

    if check_only {
        eprintln!("\n--check mode: parsed successfully, no files written.");
        return;
    }

    eprintln!("\nGenerating {}...", bridge_hpp.display());
    let hpp = generate_bridge_hpp(&classes);
    fs::write(&bridge_hpp, &hpp).unwrap();
    eprintln!("  Wrote {} lines", hpp.lines().count());

    eprintln!("Generating {}...", bridge_cpp.display());
    let cpp = generate_bridge_cpp(&classes);
    fs::write(&bridge_cpp, &cpp).unwrap();
    eprintln!("  Wrote {} lines", cpp.lines().count());

    eprintln!("Generating {}...", lib_rs.display());
    let lib = generate_lib_rs(&classes);
    fs::write(&lib_rs, &lib).unwrap();
    eprintln!("  Wrote {} lines", lib.lines().count());

    eprintln!("\nDone. Review the generated files before committing.");
}
