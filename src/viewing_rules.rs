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
        let handle = unsafe { ocio_sys::ocio_viewing_rules_create() };
        NonNull::new(handle)
            .map(|h| Self { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    /// Create an editable clone of the viewing rules.
    pub fn create_editable_copy(&self) -> Result<Self> {
        let handle =
            unsafe { ocio_sys::ocio_viewing_rules_create_editable_copy(self.handle.as_ptr()) };
        NonNull::new(handle)
            .map(|h| Self { handle: h })
            .ok_or(OcioError::AllocationFailed)
    }

    /// Return the number of authored viewing rules.
    pub fn num_entries(&self) -> u64 {
        unsafe { ocio_sys::ocio_viewing_rules_get_num_entries(self.handle.as_ptr()) as u64 }
    }

    /// Look up the index for a rule name.
    ///
    /// This returns the raw OCIO result and may use implementation-defined
    /// fallback values when the rule is absent. Prefer [`rule_index`] for a
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
        unsafe {
            cstr_from_mut(ocio_sys::ocio_viewing_rules_get_name(
                self.handle.as_ptr(),
                rule_index as usize,
            ))
        }
    }

    /// Return the number of color spaces attached to a rule.
    pub fn num_color_spaces(&self, rule_index: u64) -> u64 {
        unsafe {
            ocio_sys::ocio_viewing_rules_get_num_color_spaces(
                self.handle.as_ptr(),
                rule_index as usize,
            ) as u64
        }
    }

    /// Return one color-space name attached to a rule.
    pub fn color_space(&self, rule_index: u64, color_space_index: u64) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_viewing_rules_get_color_space(
                self.handle.as_ptr(),
                rule_index as usize,
                color_space_index as usize,
            ))
        }
    }

    /// Append a color-space selector to a rule.
    pub fn add_color_space(&self, rule_index: u64, color_space: impl AsRef<str>) -> Result<()> {
        let color_space = cstring(color_space)?;
        unsafe {
            ocio_sys::ocio_viewing_rules_add_color_space(
                self.handle.as_ptr(),
                rule_index as usize,
                color_space.as_ptr().cast(),
            )
        };
        Ok(())
    }

    /// Remove one color-space selector from a rule.
    pub fn remove_color_space(&self, rule_index: u64, color_space_index: u64) {
        unsafe {
            ocio_sys::ocio_viewing_rules_remove_color_space(
                self.handle.as_ptr(),
                rule_index as usize,
                color_space_index as usize,
            )
        };
    }

    /// Return the number of encodings attached to a rule.
    pub fn num_encodings(&self, rule_index: u64) -> u64 {
        unsafe {
            ocio_sys::ocio_viewing_rules_get_num_encodings(
                self.handle.as_ptr(),
                rule_index as usize,
            ) as u64
        }
    }

    /// Return one encoding attached to a rule.
    pub fn encoding(&self, rule_index: u64, encoding_index: u64) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_viewing_rules_get_encoding(
                self.handle.as_ptr(),
                rule_index as usize,
                encoding_index as usize,
            ))
        }
    }

    /// Append an encoding selector to a rule.
    pub fn add_encoding(&self, rule_index: u64, encoding: impl AsRef<str>) -> Result<()> {
        let encoding = cstring(encoding)?;
        unsafe {
            ocio_sys::ocio_viewing_rules_add_encoding(
                self.handle.as_ptr(),
                rule_index as usize,
                encoding.as_ptr().cast(),
            )
        };
        Ok(())
    }

    /// Remove one encoding selector from a rule.
    pub fn remove_encoding(&self, rule_index: u64, encoding_index: u64) {
        unsafe {
            ocio_sys::ocio_viewing_rules_remove_encoding(
                self.handle.as_ptr(),
                rule_index as usize,
                encoding_index as usize,
            )
        };
    }

    /// Return the number of custom keys attached to a rule.
    pub fn num_custom_keys(&self, rule_index: u64) -> u64 {
        unsafe {
            ocio_sys::ocio_viewing_rules_get_num_custom_keys(
                self.handle.as_ptr(),
                rule_index as usize,
            ) as u64
        }
    }

    /// Return the name of one custom key attached to a rule.
    pub fn custom_key_name(&self, rule_index: u64, key_index: u64) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_viewing_rules_get_custom_key_name(
                self.handle.as_ptr(),
                rule_index as usize,
                key_index as usize,
            ))
        }
    }

    /// Return the value of one custom key attached to a rule.
    pub fn custom_key_value(&self, rule_index: u64, key_index: u64) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_viewing_rules_get_custom_key_value(
                self.handle.as_ptr(),
                rule_index as usize,
                key_index as usize,
            ))
        }
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
        unsafe {
            ocio_sys::ocio_viewing_rules_set_custom_key(
                self.handle.as_ptr(),
                rule_index as usize,
                key.as_ptr().cast(),
                value.as_ptr().cast(),
            )
        };
        Ok(())
    }

    /// Insert a new viewing rule at the requested index.
    pub fn insert_rule(&self, rule_index: u64, rule_name: impl AsRef<str>) -> Result<()> {
        let rule_name = cstring(rule_name)?;
        unsafe {
            ocio_sys::ocio_viewing_rules_insert_rule(
                self.handle.as_ptr(),
                rule_index as usize,
                rule_name.as_ptr().cast(),
            )
        };
        Ok(())
    }

    /// Remove a viewing rule by index.
    pub fn remove_rule(&self, rule_index: u64) {
        unsafe {
            ocio_sys::ocio_viewing_rules_remove_rule(self.handle.as_ptr(), rule_index as usize)
        };
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
        rules.remove_color_space(0, 0);
        assert!(rules.add_encoding(0, "scene-linear").is_ok());
        rules.remove_encoding(0, 0);
        assert!(rules.set_custom_key(0, "camera", "A001").is_ok());
        let _ = rules.create_editable_copy();
        rules.remove_rule(0);
    }
}
