use std::ffi::c_void;
use std::ptr::NonNull;

use crate::{cstr_from_mut, cstring, OcioError, Result};
use ocio_sys;

/// Holds filename-based routing rules that map media to color spaces and views.
pub struct FileRules {
    pub(crate) handle: NonNull<c_void>,
}

impl FileRules {
    /// Create a new, empty set of file rules.
    pub fn create() -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_file_rules_create() };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Create an editable deep copy of these file rules.
    pub fn create_editable_copy(&self) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_file_rules_create_editable_copy(self.handle.as_ptr() as *mut c_void)
        };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Get the total number of rule entries.
    pub fn num_entries(&self) -> u64 {
        unsafe { ocio_sys::ocio_file_rules_get_num_entries(self.handle.as_ptr()) as u64 }
    }

    /// Look up the index for a rule name.
    ///
    /// This compatibility method returns `u64::MAX` when OCIO rejects the name.
    /// Prefer [`Self::try_index_for_rule`] to preserve the OCIO error, or
    /// [`Self::rule_index`] for an optional lookup.
    pub fn index_for_rule(&self, rule_name: impl AsRef<str>) -> u64 {
        self.try_index_for_rule(rule_name).unwrap_or(u64::MAX)
    }

    /// Look up a rule index while preserving OCIO missing-rule errors.
    pub fn try_index_for_rule(&self, rule_name: impl AsRef<str>) -> Result<u64> {
        let rule_name = cstring(rule_name)?;
        crate::clear_last_error();
        let index = unsafe {
            ocio_sys::ocio_file_rules_get_index_for_rule(
                self.handle.as_ptr(),
                rule_name.as_ptr().cast(),
            ) as u64
        };
        crate::ocio_call_status()?;
        Ok(index)
    }

    /// Look up the index for a rule name.
    ///
    /// Returns `None` when the rule is absent or the rule name is not a valid
    /// C string for the OCIO ABI.
    pub fn rule_index(&self, rule_name: impl AsRef<str>) -> Option<u64> {
        let rule_name = rule_name.as_ref();
        let index = self.try_index_for_rule(rule_name).ok()?;
        match self.name(index) {
            Some(found_name) if found_name == rule_name => Some(index),
            _ => None,
        }
    }

    /// Get the rule name at the given index.
    pub fn name(&self, rule_index: u64) -> Option<String> {
        self.try_name(rule_index).ok().flatten()
    }

    /// Get a rule name while preserving invalid-index and bridge failures.
    pub fn try_name(&self, rule_index: u64) -> Result<Option<String>> {
        crate::clear_last_error();
        let name = unsafe {
            cstr_from_mut(ocio_sys::ocio_file_rules_get_name(
                self.handle.as_ptr(),
                rule_index as usize,
            ))
        };
        crate::ocio_call_status()?;
        Ok(name)
    }

    /// Get the filename pattern for the rule at the given index.
    pub fn pattern(&self, rule_index: u64) -> Option<String> {
        self.try_pattern(rule_index).ok().flatten()
    }

    /// Get a filename pattern while preserving invalid-index and bridge failures.
    pub fn try_pattern(&self, rule_index: u64) -> Result<Option<String>> {
        crate::clear_last_error();
        let pattern = unsafe {
            cstr_from_mut(ocio_sys::ocio_file_rules_get_pattern(
                self.handle.as_ptr(),
                rule_index as usize,
            ))
        };
        crate::ocio_call_status()?;
        Ok(pattern)
    }

    /// Set the filename pattern for the rule at the given index.
    pub fn set_pattern(&self, rule_index: u64, pattern: impl AsRef<str>) -> Result<()> {
        let pattern = cstring(pattern)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_file_rules_set_pattern(
                self.handle.as_ptr(),
                rule_index as usize,
                pattern.as_ptr().cast(),
            )
        };
        crate::ocio_call_status()
    }

    /// Get the file extension for the rule at the given index.
    pub fn extension(&self, rule_index: u64) -> Option<String> {
        self.try_extension(rule_index).ok().flatten()
    }

    /// Get a file extension while preserving invalid-index and bridge failures.
    pub fn try_extension(&self, rule_index: u64) -> Result<Option<String>> {
        crate::clear_last_error();
        let extension = unsafe {
            cstr_from_mut(ocio_sys::ocio_file_rules_get_extension(
                self.handle.as_ptr(),
                rule_index as usize,
            ))
        };
        crate::ocio_call_status()?;
        Ok(extension)
    }

    /// Set the file extension for the rule at the given index.
    pub fn set_extension(&self, rule_index: u64, extension: impl AsRef<str>) -> Result<()> {
        let extension = cstring(extension)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_file_rules_set_extension(
                self.handle.as_ptr(),
                rule_index as usize,
                extension.as_ptr().cast(),
            )
        };
        crate::ocio_call_status()
    }

    /// Get the regex pattern for the rule at the given index.
    pub fn regex(&self, rule_index: u64) -> Option<String> {
        self.try_regex(rule_index).ok().flatten()
    }

    /// Get a regex pattern while preserving invalid-index and bridge failures.
    pub fn try_regex(&self, rule_index: u64) -> Result<Option<String>> {
        crate::clear_last_error();
        let regex = unsafe {
            cstr_from_mut(ocio_sys::ocio_file_rules_get_regex(
                self.handle.as_ptr(),
                rule_index as usize,
            ))
        };
        crate::ocio_call_status()?;
        Ok(regex)
    }

    /// Set the regex pattern for the rule at the given index.
    pub fn set_regex(&self, rule_index: u64, regex: impl AsRef<str>) -> Result<()> {
        let regex = cstring(regex)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_file_rules_set_regex(
                self.handle.as_ptr(),
                rule_index as usize,
                regex.as_ptr().cast(),
            )
        };
        crate::ocio_call_status()
    }

    /// Get the color space name for the rule at the given index.
    pub fn color_space(&self, rule_index: u64) -> Option<String> {
        self.try_color_space(rule_index).ok().flatten()
    }

    /// Get a rule color-space name while preserving invalid-index and bridge failures.
    pub fn try_color_space(&self, rule_index: u64) -> Result<Option<String>> {
        crate::clear_last_error();
        let color_space = unsafe {
            cstr_from_mut(ocio_sys::ocio_file_rules_get_color_space(
                self.handle.as_ptr(),
                rule_index as usize,
            ))
        };
        crate::ocio_call_status()?;
        Ok(color_space)
    }

    /// Set the color space name for the rule at the given index.
    pub fn set_color_space(&self, rule_index: u64, color_space: impl AsRef<str>) -> Result<()> {
        let color_space = cstring(color_space)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_file_rules_set_color_space(
                self.handle.as_ptr(),
                rule_index as usize,
                color_space.as_ptr().cast(),
            )
        };
        crate::ocio_call_status()
    }

    /// Get the number of custom keys on the rule at the given index.
    pub fn num_custom_keys(&self, rule_index: u64) -> u64 {
        unsafe {
            ocio_sys::ocio_file_rules_get_num_custom_keys(self.handle.as_ptr(), rule_index as usize)
                as u64
        }
    }

    /// Get the name of the custom key at the given key index.
    pub fn custom_key_name(&self, rule_index: u64, key: u64) -> Option<String> {
        self.try_custom_key_name(rule_index, key).ok().flatten()
    }

    /// Get a custom-key name while preserving invalid-index and bridge failures.
    pub fn try_custom_key_name(&self, rule_index: u64, key: u64) -> Result<Option<String>> {
        crate::clear_last_error();
        let name = unsafe {
            cstr_from_mut(ocio_sys::ocio_file_rules_get_custom_key_name(
                self.handle.as_ptr(),
                rule_index as usize,
                key as usize,
            ))
        };
        crate::ocio_call_status()?;
        Ok(name)
    }

    /// Get the value of the custom key at the given key index.
    pub fn custom_key_value(&self, rule_index: u64, key: u64) -> Option<String> {
        self.try_custom_key_value(rule_index, key).ok().flatten()
    }

    /// Get a custom-key value while preserving invalid-index and bridge failures.
    pub fn try_custom_key_value(&self, rule_index: u64, key: u64) -> Result<Option<String>> {
        crate::clear_last_error();
        let value = unsafe {
            cstr_from_mut(ocio_sys::ocio_file_rules_get_custom_key_value(
                self.handle.as_ptr(),
                rule_index as usize,
                key as usize,
            ))
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Set a custom key-value pair on the rule at the given index.
    pub fn set_custom_key(
        &self,
        rule_index: u64,
        key: impl AsRef<str>,
        value: impl AsRef<str>,
    ) -> Result<()> {
        let key = cstring(key)?;
        let value = cstring(value)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_file_rules_set_custom_key(
                self.handle.as_ptr(),
                rule_index as usize,
                key.as_ptr().cast(),
                value.as_ptr().cast(),
            )
        };
        crate::ocio_call_status()
    }

    /// Insert a new rule with a filename pattern and extension at the given index.
    pub fn insert_rule(
        &self,
        rule_index: u64,
        name: impl AsRef<str>,
        color_space: impl AsRef<str>,
        pattern: impl AsRef<str>,
        extension: impl AsRef<str>,
    ) -> Result<()> {
        let name = cstring(name)?;
        let color_space = cstring(color_space)?;
        let pattern = cstring(pattern)?;
        let extension = cstring(extension)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_file_rules_insert_rule(
                self.handle.as_ptr(),
                rule_index as usize,
                name.as_ptr().cast(),
                color_space.as_ptr().cast(),
                pattern.as_ptr().cast(),
                extension.as_ptr().cast(),
            );
        }
        crate::ocio_call_status()
    }

    /// Insert a new rule with a regex pattern at the given index.
    pub fn insert_rule_regex(
        &self,
        rule_index: u64,
        name: impl AsRef<str>,
        color_space: impl AsRef<str>,
        regex: impl AsRef<str>,
    ) -> Result<()> {
        let name = cstring(name)?;
        let color_space = cstring(color_space)?;
        let regex = cstring(regex)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_file_rules_insert_rule_v1(
                self.handle.as_ptr(),
                rule_index as usize,
                name.as_ptr().cast(),
                color_space.as_ptr().cast(),
                regex.as_ptr().cast(),
            );
        }
        crate::ocio_call_status()
    }

    #[deprecated(since = "0.2.0", note = "compat alias; prefer insert_rule_regex()")]
    pub fn insert_rule_v1(
        &self,
        rule_index: u64,
        name: impl AsRef<str>,
        color_space: impl AsRef<str>,
        regex: impl AsRef<str>,
    ) -> Result<()> {
        self.insert_rule_regex(rule_index, name, color_space, regex)
    }

    #[deprecated(
        since = "0.2.0",
        note = "panics on OCIO errors; prefer try_insert_path_search_rule()"
    )]
    pub fn insert_path_search_rule(&self, rule_index: u64) {
        self.try_insert_path_search_rule(rule_index)
            .expect("FileRules::insert_path_search_rule failed");
    }

    /// Insert OCIO's built-in path-search rule at `rule_index`.
    pub fn try_insert_path_search_rule(&self, rule_index: u64) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_file_rules_insert_path_search_rule(
                self.handle.as_ptr(),
                rule_index as usize,
            )
        };
        crate::ocio_call_status()
    }

    /// Set the default color space used when no rule matches.
    pub fn set_default_rule_color_space(&self, color_space: impl AsRef<str>) -> Result<()> {
        let color_space = cstring(color_space)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_file_rules_set_default_rule_color_space(
                self.handle.as_ptr(),
                color_space.as_ptr().cast(),
            )
        };
        crate::ocio_call_status()
    }

    #[deprecated(
        since = "0.2.0",
        note = "panics on OCIO errors; prefer try_remove_rule()"
    )]
    pub fn remove_rule(&self, rule_index: u64) {
        self.try_remove_rule(rule_index)
            .expect("FileRules::remove_rule failed");
    }

    /// Remove the rule at `rule_index`.
    pub fn try_remove_rule(&self, rule_index: u64) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_file_rules_remove_rule(self.handle.as_ptr(), rule_index as usize) };
        crate::ocio_call_status()
    }

    #[deprecated(
        since = "0.2.0",
        note = "panics on OCIO errors; prefer try_increase_rule_priority()"
    )]
    pub fn increase_rule_priority(&self, rule_index: u64) {
        self.try_increase_rule_priority(rule_index)
            .expect("FileRules::increase_rule_priority failed");
    }

    /// Move the rule at `rule_index` toward the front of the rule list.
    pub fn try_increase_rule_priority(&self, rule_index: u64) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_file_rules_increase_rule_priority(
                self.handle.as_ptr(),
                rule_index as usize,
            )
        };
        crate::ocio_call_status()
    }

    #[deprecated(
        since = "0.2.0",
        note = "panics on OCIO errors; prefer try_decrease_rule_priority()"
    )]
    pub fn decrease_rule_priority(&self, rule_index: u64) {
        self.try_decrease_rule_priority(rule_index)
            .expect("FileRules::decrease_rule_priority failed");
    }

    /// Move the rule at `rule_index` toward the end of the rule list.
    pub fn try_decrease_rule_priority(&self, rule_index: u64) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_file_rules_decrease_rule_priority(
                self.handle.as_ptr(),
                rule_index as usize,
            )
        };
        crate::ocio_call_status()
    }

    /// Check whether the file rules are in their default (empty) state.
    pub fn is_default(&self) -> bool {
        unsafe { ocio_sys::ocio_file_rules_is_default(self.handle.as_ptr() as *mut c_void) }
    }
}

impl Drop for FileRules {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_file_rules_destroy(self.handle.as_ptr() as *mut c_void) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_file_rules() {
        let rules = FileRules::create();
        assert!(rules.is_ok());
    }

    #[test]
    fn file_rules_methods_no_crash() {
        let rules = FileRules::create().unwrap();
        let _ = rules.num_entries();
        let _ = rules.name(0);
        let _ = rules.pattern(0);
        let _ = rules.extension(0);
        let _ = rules.regex(0);
        let _ = rules.color_space(0);
        let _ = rules.num_custom_keys(0);
        let _ = rules.custom_key_name(0, 0);
        let _ = rules.custom_key_value(0, 0);
        let _ = rules.is_default();
        let _ = rules.index_for_rule("Default");
    }

    #[test]
    fn set_pattern_extension_regex_no_crash() {
        let rules = FileRules::create().unwrap();
        rules
            .insert_rule(0, "TestRule", "ACEScg", "*.dpx", "dpx")
            .unwrap();
        assert!(rules.set_pattern(0, "*.exr").is_ok());
        assert!(rules.set_extension(0, "exr").is_ok());
        assert!(rules.set_regex(0, ".*").is_ok());
        assert!(rules.set_color_space(0, "ACEScg").is_ok());
    }

    #[test]
    fn insert_remove_rule_no_crash() {
        let rules = FileRules::create().unwrap();
        assert!(rules
            .insert_rule(0, "TestRule", "ACEScg", "*.exr", "exr")
            .is_ok());
        assert!(rules
            .insert_rule_regex(1, "RegexRule", "ACEScg", ".*\\.exr")
            .is_ok());
        rules.try_insert_path_search_rule(2).unwrap();
        assert!(rules.set_default_rule_color_space("ACEScg").is_ok());
        rules.try_remove_rule(0).unwrap();
    }

    #[test]
    fn priority_no_crash() {
        let rules = FileRules::create().unwrap();
        rules
            .insert_rule(0, "TestRule", "ACEScg", "*.exr", "exr")
            .unwrap();
        rules
            .insert_rule(1, "SecondRule", "ACEScg", "*.dpx", "dpx")
            .unwrap();
        rules.try_increase_rule_priority(1).unwrap();
        rules.try_decrease_rule_priority(0).unwrap();
    }

    #[test]
    fn custom_keys_no_crash() {
        let rules = FileRules::create().unwrap();
        assert!(rules.set_custom_key(0, "key1", "value1").is_ok());
    }

    #[test]
    fn create_editable_copy_no_crash() {
        let rules = FileRules::create().unwrap();
        let _ = rules.create_editable_copy();
    }
}
