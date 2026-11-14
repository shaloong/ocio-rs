use crate::{Config, FileRules, Result, ViewingRules};
use std::ffi::{c_void, CStr};
use std::ptr::NonNull;

struct Rules {
    handle: NonNull<c_void>,
    kind: i32,
}
impl Rules {
    fn number(&self, field: i32, rule: usize, name: Option<&CStr>) -> Result<usize> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_const_rules_number(
                self.handle.as_ptr(),
                self.kind,
                field,
                rule,
                name.map_or(std::ptr::null(), CStr::as_ptr),
            )
        };
        crate::ocio_call_status()?;
        Ok(value)
    }
    fn text(&self, field: i32, rule: usize, index: usize) -> Result<Option<&CStr>> {
        crate::clear_last_error();
        let ptr = unsafe {
            ocio_sys::ocio_const_rules_text(self.handle.as_ptr(), self.kind, field, rule, index)
        };
        crate::ocio_call_status()?;
        Ok(if ptr.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(ptr) })
        })
    }
    fn copy(&self) -> Result<NonNull<c_void>> {
        crate::clear_last_error();
        let ptr =
            unsafe { ocio_sys::ocio_const_rules_editable_copy(self.handle.as_ptr(), self.kind) };
        crate::handle_result(ptr)
    }
}
impl Drop for Rules {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_const_rules_destroy(self.handle.as_ptr(), self.kind) }
    }
}
/// Shared read-only file rules. Text is borrowed from the native rules object.
pub struct ConstFileRules(Rules);
/// Shared read-only viewing rules. Text is borrowed from the native rules object.
pub struct ConstViewingRules(Rules);

macro_rules! common {
    ($ty:ident, $editable:ident) => {
        impl $ty {
            /// Number of rules.
            pub fn num_entries(&self) -> Result<usize> {
                self.0.number(0, 0, None)
            }
            /// Index of a named rule; OCIO reports missing names as errors.
            pub fn index_for_rule(&self, name: &CStr) -> Result<usize> {
                self.0.number(1, 0, Some(name))
            }
            /// Borrow a rule name.
            pub fn name(&self, rule: usize) -> Result<Option<&CStr>> {
                self.0.text(0, rule, 0)
            }
            /// Number of custom key/value entries on a rule.
            pub fn num_custom_keys(&self, rule: usize) -> Result<usize> {
                self.0.number(2, rule, None)
            }
            /// Borrow a custom key name.
            pub fn custom_key_name(&self, rule: usize, index: usize) -> Result<Option<&CStr>> {
                self.0.text(5, rule, index)
            }
            /// Borrow a custom key value.
            pub fn custom_key_value(&self, rule: usize, index: usize) -> Result<Option<&CStr>> {
                self.0.text(6, rule, index)
            }
            /// Create an independently editable rules object.
            pub fn create_editable_copy(&self) -> Result<$editable> {
                Ok($editable {
                    handle: self.0.copy()?,
                })
            }
        }
    };
}
common!(ConstFileRules, FileRules);
common!(ConstViewingRules, ViewingRules);
impl ConstFileRules {
    /// Whether this is the native default rule set.
    pub fn is_default(&self) -> Result<bool> {
        Ok(self.0.number(3, 0, None)? != 0)
    }
    /// Borrow a rule's filename pattern.
    pub fn pattern(&self, rule: usize) -> Result<Option<&CStr>> {
        self.0.text(1, rule, 0)
    }
    /// Borrow a rule's filename extension.
    pub fn extension(&self, rule: usize) -> Result<Option<&CStr>> {
        self.0.text(2, rule, 0)
    }
    /// Borrow a rule's regular expression.
    pub fn regex(&self, rule: usize) -> Result<Option<&CStr>> {
        self.0.text(3, rule, 0)
    }
    /// Borrow a rule's color space.
    pub fn color_space(&self, rule: usize) -> Result<Option<&CStr>> {
        self.0.text(4, rule, 0)
    }
}
impl ConstViewingRules {
    /// Number of color spaces on a rule.
    pub fn num_color_spaces(&self, rule: usize) -> Result<usize> {
        self.0.number(4, rule, None)
    }
    /// Number of encodings on a rule.
    pub fn num_encodings(&self, rule: usize) -> Result<usize> {
        self.0.number(5, rule, None)
    }
    /// Borrow a color-space name.
    pub fn color_space(&self, rule: usize, index: usize) -> Result<Option<&CStr>> {
        self.0.text(7, rule, index)
    }
    /// Borrow an encoding name.
    pub fn encoding(&self, rule: usize, index: usize) -> Result<Option<&CStr>> {
        self.0.text(8, rule, index)
    }
}
fn lookup(config: *mut c_void, kind: i32) -> Result<Option<Rules>> {
    crate::clear_last_error();
    let ptr = unsafe { ocio_sys::ocio_config_get_const_rules(config, kind) };
    crate::ocio_call_status()?;
    Ok(NonNull::new(ptr).map(|handle| Rules { handle, kind }))
}
impl Config {
    /// Share the config's read-only native file rules without copying.
    pub fn try_const_file_rules(&self) -> Result<Option<ConstFileRules>> {
        Ok(lookup(self.handle.as_ptr(), 0)?.map(ConstFileRules))
    }
    /// Share the config's read-only native viewing rules without copying.
    pub fn try_const_viewing_rules(&self) -> Result<Option<ConstViewingRules>> {
        Ok(lookup(self.handle.as_ptr(), 1)?.map(ConstViewingRules))
    }
}
