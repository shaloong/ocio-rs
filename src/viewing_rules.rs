use std::ffi::c_void;
use std::ptr::NonNull;

use crate::{cstr_from_mut, cstring, OcioError, Result};
use ocio_sys;

/// Holds OCIO viewing rules used to select active views for color spaces.
///
/// Viewing rules are attached to a [`crate::Config`] and are typically used to
/// control which display views are considered active for particular color
/// spaces, encodings, and custom key/value metadata.
pub struct ViewingRules {
    pub(crate) handle: NonNull<c_void>,
}

impl ViewingRules {
    /// Create an empty set of viewing rules.
    pub fn create() -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_viewing_rules_create() };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Create an editable clone of the viewing rules.
    pub fn create_editable_copy(&self) -> Result<Self> {
        crate::clear_last_error();
        let handle =
            unsafe { ocio_sys::ocio_viewing_rules_create_editable_copy(self.handle.as_ptr()) };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Return the number of authored viewing rules.
    pub fn num_entries(&self) -> u64 {
        unsafe { ocio_sys::ocio_viewing_rules_get_num_entries(self.handle.as_ptr()) as u64 }
    }

    /// Look up the index for a rule name.
    ///
    /// This returns the raw OCIO result and may use implementation-defined
    /// fallback values when the rule is absent. Prefer [`Self::rule_index`] for a
    /// Rust-level presence check.
    pub fn index_for_rule(&self, rule_name: impl AsRef<str>) -> u64 {
        let rule_name = match cstring(rule_name) {
            Ok(value) => value,
            Err(_) => return u64::MAX,
        };
        unsafe {
            ocio_sys::ocio_viewing_rules_get_index_for_rule(
                self.handle.as_ptr(),
                rule_name.as_ptr().cast(),
            ) as u64
        }
    }

    /// Look up the index for a rule name.
    ///
    /// Returns `None` when the rule is absent or the rule name is not a valid
    /// C string for the OCIO ABI.
    pub fn rule_index(&self, rule_name: impl AsRef<str>) -> Option<u64> {
        let rule_name = rule_name.as_ref();
        let index = self.index_for_rule(rule_name);
        match self.name(index) {
            Some(found_name) if found_name == rule_name => Some(index),
            _ => None,
        }
    }

    /// Return the authored name for a rule index.
    pub fn name(&self, rule_index: u64) -> Option<String> {
        self.try_name(rule_index).ok().flatten()
    }

    /// Return the authored name for a rule index while preserving bridge failures.
    pub fn try_name(&self, rule_index: u64) -> Result<Option<String>> {
        crate::clear_last_error();
        let name = unsafe {
            cstr_from_mut(ocio_sys::ocio_viewing_rules_get_name(
                self.handle.as_ptr(),
                rule_index as usize,
            ))
        };
        crate::ocio_call_status()?;
        Ok(name)
    }

    /// Return the number of color spaces attached to a rule.
    pub fn num_color_spaces(&self, rule_index: u64) -> u64 {
        self.try_num_color_spaces(rule_index).unwrap_or(0)
    }

    /// Return the number of color spaces attached to a rule while preserving bridge failures.
    pub fn try_num_color_spaces(&self, rule_index: u64) -> Result<u64> {
        crate::clear_last_error();
        let count = unsafe {
            ocio_sys::ocio_viewing_rules_get_num_color_spaces(
                self.handle.as_ptr(),
                rule_index as usize,
            ) as u64
        };
        crate::ocio_call_status()?;
        Ok(count)
    }

    /// Return one color-space name attached to a rule.
    pub fn color_space(&self, rule_index: u64, color_space_index: u64) -> Option<String> {
        self.try_color_space(rule_index, color_space_index)
            .ok()
            .flatten()
    }

    /// Return a color-space name while preserving invalid-index and bridge failures.
    pub fn try_color_space(
        &self,
        rule_index: u64,
        color_space_index: u64,
    ) -> Result<Option<String>> {
        crate::clear_last_error();
        let color_space = unsafe {
            cstr_from_mut(ocio_sys::ocio_viewing_rules_get_color_space(
                self.handle.as_ptr(),
                rule_index as usize,
                color_space_index as usize,
            ))
        };
        crate::ocio_call_status()?;
        Ok(color_space)
    }

    /// Append a color-space selector to a rule.
    pub fn add_color_space(&self, rule_index: u64, color_space: impl AsRef<str>) -> Result<()> {
        let color_space = cstring(color_space)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_viewing_rules_add_color_space(
                self.handle.as_ptr(),
                rule_index as usize,
                color_space.as_ptr().cast(),
            )
        };
        crate::ocio_call_status()
    }

    /// Remove one color-space selector from a rule.
    ///
    /// This compatibility method panics when OCIO rejects the indices. Prefer
    /// [`Self::try_remove_color_space`] to handle errors explicitly.
    #[deprecated(
        since = "0.2.0",
        note = "panic-on-error compatibility method; prefer try_remove_color_space()"
    )]
    pub fn remove_color_space(&self, rule_index: u64, color_space_index: u64) {
        self.try_remove_color_space(rule_index, color_space_index)
            .expect("ViewingRules::remove_color_space failed");
    }

    /// Remove one color-space selector from a rule and surface any OCIO validation error.
    pub fn try_remove_color_space(&self, rule_index: u64, color_space_index: u64) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_viewing_rules_remove_color_space(
                self.handle.as_ptr(),
                rule_index as usize,
                color_space_index as usize,
            )
        };
        crate::ocio_call_status()
    }

    /// Return the number of encodings attached to a rule.
    pub fn num_encodings(&self, rule_index: u64) -> u64 {
        self.try_num_encodings(rule_index).unwrap_or(0)
    }

    /// Return the number of encodings attached to a rule while preserving bridge failures.
    pub fn try_num_encodings(&self, rule_index: u64) -> Result<u64> {
        crate::clear_last_error();
        let count = unsafe {
            ocio_sys::ocio_viewing_rules_get_num_encodings(
                self.handle.as_ptr(),
                rule_index as usize,
            ) as u64
        };
        crate::ocio_call_status()?;
        Ok(count)
    }

    /// Return one encoding attached to a rule.
    pub fn encoding(&self, rule_index: u64, encoding_index: u64) -> Option<String> {
        self.try_encoding(rule_index, encoding_index).ok().flatten()
    }

    /// Return an encoding while preserving invalid-index and bridge failures.
    pub fn try_encoding(&self, rule_index: u64, encoding_index: u64) -> Result<Option<String>> {
        crate::clear_last_error();
        let encoding = unsafe {
            cstr_from_mut(ocio_sys::ocio_viewing_rules_get_encoding(
                self.handle.as_ptr(),
                rule_index as usize,
                encoding_index as usize,
            ))
        };
        crate::ocio_call_status()?;
        Ok(encoding)
    }

    /// Append an encoding selector to a rule.
    pub fn add_encoding(&self, rule_index: u64, encoding: impl AsRef<str>) -> Result<()> {
        let encoding = cstring(encoding)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_viewing_rules_add_encoding(
                self.handle.as_ptr(),
                rule_index as usize,
                encoding.as_ptr().cast(),
            )
        };
        crate::ocio_call_status()
    }

    /// Remove one encoding selector from a rule.
    ///
    /// This compatibility method panics when OCIO rejects the indices. Prefer
    /// [`Self::try_remove_encoding`] to handle errors explicitly.
    #[deprecated(
        since = "0.2.0",
        note = "panic-on-error compatibility method; prefer try_remove_encoding()"
    )]
    pub fn remove_encoding(&self, rule_index: u64, encoding_index: u64) {
        self.try_remove_encoding(rule_index, encoding_index)
            .expect("ViewingRules::remove_encoding failed");
    }

    /// Remove one encoding selector from a rule and surface any OCIO validation error.
    pub fn try_remove_encoding(&self, rule_index: u64, encoding_index: u64) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_viewing_rules_remove_encoding(
                self.handle.as_ptr(),
                rule_index as usize,
                encoding_index as usize,
            )
        };
        crate::ocio_call_status()
    }

    /// Return the number of custom keys attached to a rule.
    pub fn num_custom_keys(&self, rule_index: u64) -> u64 {
        self.try_num_custom_keys(rule_index).unwrap_or(0)
    }

    /// Return the number of custom keys attached to a rule while preserving bridge failures.
    pub fn try_num_custom_keys(&self, rule_index: u64) -> Result<u64> {
        crate::clear_last_error();
        let count = unsafe {
            ocio_sys::ocio_viewing_rules_get_num_custom_keys(
                self.handle.as_ptr(),
                rule_index as usize,
            ) as u64
        };
        crate::ocio_call_status()?;
        Ok(count)
    }

    /// Return the name of one custom key attached to a rule.
    pub fn custom_key_name(&self, rule_index: u64, key_index: u64) -> Option<String> {
        self.try_custom_key_name(rule_index, key_index)
            .ok()
            .flatten()
    }

    /// Return a custom-key name while preserving invalid-index and bridge failures.
    pub fn try_custom_key_name(&self, rule_index: u64, key_index: u64) -> Result<Option<String>> {
        crate::clear_last_error();
        let name = unsafe {
            cstr_from_mut(ocio_sys::ocio_viewing_rules_get_custom_key_name(
                self.handle.as_ptr(),
                rule_index as usize,
                key_index as usize,
            ))
        };
        crate::ocio_call_status()?;
        Ok(name)
    }

    /// Return the value of one custom key attached to a rule.
    pub fn custom_key_value(&self, rule_index: u64, key_index: u64) -> Option<String> {
        self.try_custom_key_value(rule_index, key_index)
            .ok()
            .flatten()
    }

    /// Return a custom-key value while preserving invalid-index and bridge failures.
    pub fn try_custom_key_value(&self, rule_index: u64, key_index: u64) -> Result<Option<String>> {
        crate::clear_last_error();
        let value = unsafe {
            cstr_from_mut(ocio_sys::ocio_viewing_rules_get_custom_key_value(
                self.handle.as_ptr(),
                rule_index as usize,
                key_index as usize,
            ))
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Set or replace one custom key/value pair on a rule.
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
            ocio_sys::ocio_viewing_rules_set_custom_key(
                self.handle.as_ptr(),
                rule_index as usize,
                key.as_ptr().cast(),
                value.as_ptr().cast(),
            )
        };
        crate::ocio_call_status()
    }

    /// Insert a new viewing rule at the requested index.
    pub fn insert_rule(&self, rule_index: u64, rule_name: impl AsRef<str>) -> Result<()> {
        let rule_name = cstring(rule_name)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_viewing_rules_insert_rule(
                self.handle.as_ptr(),
                rule_index as usize,
                rule_name.as_ptr().cast(),
            )
        };
        crate::ocio_call_status()
    }

    /// Remove a viewing rule by index.
    ///
    /// This compatibility method panics when OCIO rejects the index. Prefer
    /// [`Self::try_remove_rule`] to handle errors explicitly.
    #[deprecated(
        since = "0.2.0",
        note = "panic-on-error compatibility method; prefer try_remove_rule()"
    )]
    pub fn remove_rule(&self, rule_index: u64) {
        self.try_remove_rule(rule_index)
            .expect("ViewingRules::remove_rule failed");
    }

    /// Remove a viewing rule by index and surface any OCIO validation error.
    pub fn try_remove_rule(&self, rule_index: u64) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_viewing_rules_remove_rule(self.handle.as_ptr(), rule_index as usize)
        };
        crate::ocio_call_status()
    }
}

impl Drop for ViewingRules {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_viewing_rules_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_viewing_rules() {
        assert!(ViewingRules::create().is_ok());
    }

    #[test]
    fn viewing_rules_methods_no_crash() {
        let rules = ViewingRules::create().unwrap();
        let _ = rules.num_entries();
        let _ = rules.index_for_rule("RuleA");
        let _ = rules.name(0);
        let _ = rules.num_color_spaces(0);
        let _ = rules.color_space(0, 0);
        let _ = rules.num_encodings(0);
        let _ = rules.encoding(0, 0);
        let _ = rules.num_custom_keys(0);
        let _ = rules.custom_key_name(0, 0);
        let _ = rules.custom_key_value(0, 0);
    }

    #[test]
    fn viewing_rules_mutation_no_crash() {
        let rules = ViewingRules::create().unwrap();
        assert!(rules.insert_rule(0, "RuleA").is_ok());
        assert!(rules.add_color_space(0, "raw").is_ok());
        rules.try_remove_color_space(0, 0).unwrap();
        assert!(rules.add_encoding(0, "scene-linear").is_ok());
        rules.try_remove_encoding(0, 0).unwrap();
        assert!(rules.set_custom_key(0, "camera", "A001").is_ok());
        let _ = rules.create_editable_copy();
        rules.try_remove_rule(0).unwrap();
    }
}
