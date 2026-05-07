/// Safe wrapper migrator for OCIO v2.5.1 upgrade.
/// Phase 1: Applies renames to src/*.rs
/// Phase 2: Generates Rust stub functions for removed FFI functions
///          in ocio-sys/src/lib.rs, keeping safe wrappers unchanged.
///
/// Usage:
///   rustc tools/migrate_src.rs -o migrate.exe && ./migrate.exe

use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    // Paths
    let old_lib_path = "old_lib_for_migration.txt"; // git show HEAD~1:ocio-sys/src/lib.rs
    let new_hpp_path = "ocio-sys/src/bridge.hpp";
    let lib_rs_path = "ocio-sys/src/lib.rs";
    let src_dir = "src";

    // Check if old lib exists
    if !fs::exists(old_lib_path).unwrap_or(false) {
        eprintln!("ERROR: {} not found. Run first:", old_lib_path);
        eprintln!("  git show HEAD~1:ocio-sys/src/lib.rs > {}", old_lib_path);
        return;
    }

    // 1. Build mapping
    let (renames, removed) = build_mappings(old_lib_path, new_hpp_path);
    eprintln!("Renames: {}, Removed: {}", renames.len(), removed.len());

    // 2. Apply renames to safe wrappers
    apply_renames_to_src(src_dir, &renames);

    // 3. Generate stubs for removed functions
    let stubs = generate_stubs(old_lib_path, &removed);
    append_stubs_to_lib_rs(lib_rs_path, &stubs);
    eprintln!("Generated {} stubs in {}", stubs.len(), lib_rs_path);
    eprintln!("Done! Now run: cargo build");
}

fn build_mappings(old_lib: &str, new_hpp: &str) -> (HashMap<String, String>, HashSet<String>) {
    let mut renames = HashMap::new();
    let mut removed = HashSet::new();

    // Known renames (v2.4.x → v2.5.1)
    let known: Vec<(&str, &str)> = vec![
        ("ocio_builtin_config_registry_get_config_by_index", "ocio_builtin_config_registry_get_builtin_config"),
        ("ocio_builtin_config_registry_get_config_by_name", "ocio_builtin_config_registry_get_builtin_config_by_name"),
        ("ocio_builtin_config_registry_get_config_name", "ocio_builtin_config_registry_get_builtin_config_name"),
        ("ocio_builtin_config_registry_get_config_ui_name", "ocio_builtin_config_registry_get_builtin_config_ui_name"),
        ("ocio_builtin_config_registry_is_config_recommended", "ocio_builtin_config_registry_is_builtin_config_recommended"),
        ("ocio_config_set_default_display", "ocio_config_set_active_displays"),
        ("ocio_config_set_default_view", "ocio_config_set_active_views"),
        ("ocio_config_get_looks", "ocio_config_get_look"),
        ("ocio_set_logging_level_to_override", "ocio_set_logging_level"),
        ("ocio_color_space_set_category", "ocio_color_space_add_category"),
        // Signature changes → redirect to _v1 (where old API is preserved)
        ("ocio_config_get_num_color_spaces", "ocio_config_get_num_color_spaces_v1"),
        ("ocio_config_get_color_space_name_by_index", "ocio_config_get_color_space_name_by_index_v1"),
        ("ocio_config_get_num_named_transforms", "ocio_config_get_num_named_transforms_v1"),
        ("ocio_config_get_named_transform_name_by_index", "ocio_config_get_named_transform_name_by_index_v1"),
        ("ocio_config_get_processor_from_configs", "ocio_config_get_processor_from_configs_v1"),
    ];
    for (old, new) in known {
        renames.insert(old.to_string(), new.to_string());
    }

    // Read files
    let new_content = fs::read_to_string(new_hpp).unwrap_or_default();

    // Extract old function names from old lib.rs
    let old_content = fs::read_to_string(old_lib).unwrap_or_default();
    let old_names = extract_function_names(&old_content);

    for name in &old_names {
        if renames.contains_key(name) {
            continue; // Already handled
        }
        // Check if this function exists in the new bridge
        let search = format!("{}(", name);
        if !new_content.contains(&search) {
            eprintln!("  REMOVED: {}", name);
            removed.insert(name.clone());
        }
    }

    (renames, removed)
}

fn extract_function_names(content: &str) -> Vec<String> {
    let mut names = Vec::new();
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("pub fn ocio_") {
            if let Some(paren) = line.find('(') {
                let name = line[7..paren].to_string(); // Skip "pub fn "
                names.push(name);
            }
        }
    }
    names
}

fn apply_renames_to_src(dir: &str, renames: &HashMap<String, String>) {
    let dir_path = std::path::Path::new(dir);
    process_dir(dir_path, renames);
}

fn process_dir(dir: &std::path::Path, renames: &HashMap<String, String>) {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name().unwrap().to_str().unwrap();
            if name != "bin" {
                process_dir(&path, renames);
            }
        } else if path.extension().map_or(false, |e| e == "rs") {
            process_file(&path, renames);
        }
    }
}

fn process_file(path: &std::path::Path, renames: &HashMap<String, String>) {
    let content = fs::read_to_string(path).unwrap();
    let mut modified = content.clone();
    let mut changed = false;

    for (old, new) in renames {
        if modified.contains(old.as_str()) {
            modified = modified.replace(old.as_str(), new.as_str());
            changed = true;
        }
    }

    if changed {
        fs::write(path, &modified).unwrap();
        eprintln!("  Updated: {}", path.display());
    }
}

fn generate_stubs(old_lib: &str, removed: &HashSet<String>) -> Vec<String> {
    let mut stubs = Vec::new();
    let old_content = fs::read_to_string(old_lib).unwrap_or_default();

    stubs.push(String::new());
    stubs.push("// ════════════════════════════════════════════════════════".into());
    stubs.push("// v2.5.1 compatibility stubs for removed FFI functions".into());
    stubs.push("// TODO: migrate safe wrappers, then remove these stubs".into());
    stubs.push("// ════════════════════════════════════════════════════════".into());

    for name in removed.iter() {
        // Find declaration in old lib
        let decl = old_content
            .lines()
            .find(|l| l.trim().contains(&format!("pub fn {}(", name)))
            .unwrap_or("");

        if decl.is_empty() {
            eprintln!("  WARNING: No declaration found for {}", name);
            continue;
        }

        // Parse params and return type
        let Some(params_start_idx) = decl.find('(') else { continue; };
        let Some(params_end_idx) = decl.find(')') else { continue; };
        let params = &decl[params_start_idx + 1..params_end_idx];

        // Determine return type
        let ret_type: String;
        if let Some(arrow_pos) = decl.find("->") {
            let ret_part = &decl[arrow_pos + 2..];
            let ret_end = ret_part.find(';').unwrap_or(ret_part.len());
            ret_type = ret_part[..ret_end].trim().to_string();
        } else {
            ret_type = "()".to_string();
        }

        // Generate stub body
        let body = match ret_type.as_str() {
            "()" => String::new(),
            "*mut c_void" => "    std::ptr::null_mut()".into(),
            "*const i8" => "    std::ptr::null()".into(),
            "i32" | "u32" | "i64" | "u64" | "usize" => "    0".into(),
            "bool" => "    false".into(),
            "f32" | "f64" => "    0.0".into(),
            "i8" => "    0i8".into(),
            _ => "    Default::default()".into(),
        };

        let stub = format!("#[allow(dead_code)] pub unsafe fn {}({}) -> {} {{\n{}\n}}",
            name, params, ret_type, body);
        stubs.push(stub);
    }
    stubs
}

fn append_stubs_to_lib_rs(path: &str, stubs: &[String]) {
    let content = fs::read_to_string(path).unwrap();
    let mut new_content = content;
    for stub in stubs {
        new_content.push('\n');
        new_content.push_str(stub);
    }
    fs::write(path, &new_content).unwrap();
}
