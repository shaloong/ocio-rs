use std::collections::BTreeMap;
use std::ffi::c_void;
use std::ptr::NonNull;

use crate::transform::{transform_from_raw_handle, Transform, TransformHandle};
use crate::{
    cstr_from_mut, cstr_to_opt_string, cstring, Allocation, BitDepth, ColorSpaceDirection,
    OcioError, ReferenceSpaceType, Result,
};
use ocio_sys;

#[cfg(feature = "v2_5")]
pub(crate) fn interchange_attributes_snapshot(
    handle: *mut c_void,
    is_const: bool,
    named_kind: Option<i32>,
) -> Result<BTreeMap<String, String>> {
    unsafe extern "C" fn visit(
        context: *mut c_void,
        name: *const u8,
        name_len: usize,
        value: *const u8,
        value_len: usize,
    ) -> bool {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if context.is_null()
                || name.is_null()
                || value.is_null()
                || name_len > isize::MAX as usize
                || value_len > isize::MAX as usize
            {
                return false;
            }
            let map = unsafe { &mut *context.cast::<BTreeMap<String, String>>() };
            let name = unsafe { std::slice::from_raw_parts(name, name_len) };
            let value = unsafe { std::slice::from_raw_parts(value, value_len) };
            map.insert(
                String::from_utf8_lossy(name).into_owned(),
                String::from_utf8_lossy(value).into_owned(),
            );
            true
        }))
        .unwrap_or(false)
    }
    let mut attributes = BTreeMap::new();
    crate::clear_last_error();
    let ok = unsafe {
        let context = (&mut attributes as *mut BTreeMap<String, String>).cast();
        match named_kind {
            Some(kind) => ocio_sys::ocio_named_item_visit_interchange_attributes(
                handle, kind, is_const, visit, context,
            ),
            None => ocio_sys::ocio_color_space_visit_interchange_attributes(
                handle, is_const, visit, context,
            ),
        }
    };
    crate::ocio_call_status()?;
    if !ok {
        return Err(OcioError::Ocio(
            "interchange attribute visitor failed".into(),
        ));
    }
    Ok(attributes)
}

/// Describes a color space definition stored in a [`Config`](crate::Config).
pub struct ColorSpace {
    pub(crate) handle: NonNull<c_void>,
}

/// An owned, read-only reference to a native OCIO color space.
///
/// The native object is shared rather than copied. Text is borrowed from it;
/// coordinate external native mutations that could invalidate the text.
pub struct ConstColorSpace {
    pub(crate) handle: NonNull<c_void>,
}

/// Borrowed editable or read-only color space for OCIO operations.
#[derive(Clone, Copy)]
pub enum ColorSpaceRef<'a> {
    Editable(&'a ColorSpace),
    ReadOnly(&'a ConstColorSpace),
}
impl<'a> From<&'a ColorSpace> for ColorSpaceRef<'a> {
    fn from(value: &'a ColorSpace) -> Self {
        Self::Editable(value)
    }
}
impl<'a> From<&'a ConstColorSpace> for ColorSpaceRef<'a> {
    fn from(value: &'a ConstColorSpace) -> Self {
        Self::ReadOnly(value)
    }
}
impl ColorSpaceRef<'_> {
    pub(crate) fn raw(self) -> (*mut c_void, bool) {
        match self {
            Self::Editable(value) => (value.handle.as_ptr(), false),
            Self::ReadOnly(value) => (value.handle.as_ptr(), true),
        }
    }
}

impl ConstColorSpace {
    /// Share the native transform without an editable copy.
    pub fn try_const_transform(
        &self,
        direction: ColorSpaceDirection,
    ) -> Result<Option<crate::transform::ConstTransform>> {
        crate::transform::ConstTransform::from_parent(self.handle.as_ptr(), 1, direction as i32)
    }
    /// Copy the native transform in `direction` into an independent editable transform.
    pub fn copy_transform(&self, direction: ColorSpaceDirection) -> Result<Option<Transform>> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_const_color_space_copy_transform(self.handle.as_ptr(), direction as i32)
        };
        crate::ocio_call_status()?;
        Ok(transform_from_raw_handle(handle))
    }
    fn text(&self, field: i32, index: usize) -> Result<Option<&std::ffi::CStr>> {
        crate::clear_last_error();
        let ptr =
            unsafe { ocio_sys::ocio_const_color_space_text(self.handle.as_ptr(), field, index) };
        crate::ocio_call_status()?;
        Ok(if ptr.is_null() {
            None
        } else {
            Some(unsafe { std::ffi::CStr::from_ptr(ptr) })
        })
    }
    fn number(&self, field: i32) -> Result<i64> {
        crate::clear_last_error();
        let value = unsafe { ocio_sys::ocio_const_color_space_number(self.handle.as_ptr(), field) };
        crate::ocio_call_status()?;
        Ok(value)
    }
    fn contains(&self, field: i32, value: impl AsRef<str>) -> Result<bool> {
        let value = cstring(value)?;
        self.contains_cstr(field, &value)
    }
    fn contains_cstr(&self, field: i32, value: &std::ffi::CStr) -> Result<bool> {
        crate::clear_last_error();
        let present = unsafe {
            ocio_sys::ocio_const_color_space_contains(self.handle.as_ptr(), field, value.as_ptr())
        };
        crate::ocio_call_status()?;
        Ok(present)
    }
    /// Borrow the family without copying.
    pub fn family(&self) -> Result<Option<&std::ffi::CStr>> {
        self.text(0, 0)
    }
    /// Borrow the equality group without copying.
    pub fn equality_group(&self) -> Result<Option<&std::ffi::CStr>> {
        self.text(1, 0)
    }
    /// Borrow the encoding without copying.
    pub fn encoding(&self) -> Result<Option<&std::ffi::CStr>> {
        self.text(2, 0)
    }
    /// Borrow a named alias; out-of-range indices return `None`.
    pub fn alias(&self, index: usize) -> Result<Option<&std::ffi::CStr>> {
        self.text(3, index)
    }
    /// Borrow a category; out-of-range indices return `None`.
    pub fn category(&self, index: usize) -> Result<Option<&std::ffi::CStr>> {
        self.text(4, index)
    }
    /// Borrow the interchange identifier without copying.
    #[cfg(feature = "v2_5")]
    pub fn interop_id(&self) -> Result<Option<&std::ffi::CStr>> {
        self.text(5, 0)
    }
    /// Borrow a native interchange attribute. Unknown attribute names report an error.
    #[cfg(feature = "v2_5")]
    pub fn interchange_attribute(&self, name: &std::ffi::CStr) -> Result<Option<&std::ffi::CStr>> {
        crate::clear_last_error();
        let ptr = unsafe {
            ocio_sys::ocio_const_color_space_interchange_attribute(
                self.handle.as_ptr(),
                name.as_ptr(),
            )
        };
        crate::ocio_call_status()?;
        Ok(if ptr.is_null() {
            None
        } else {
            Some(unsafe { std::ffi::CStr::from_ptr(ptr) })
        })
    }
    /// Return an owned snapshot of all native interchange attributes.
    #[cfg(feature = "v2_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_5")))]
    pub fn try_interchange_attributes(&self) -> Result<BTreeMap<String, String>> {
        interchange_attributes_snapshot(self.handle.as_ptr(), true, None)
    }
    /// Number of aliases.
    pub fn num_aliases(&self) -> Result<usize> {
        crate::clear_last_error();
        let count = unsafe { ocio_sys::ocio_const_color_space_alias_count(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(count)
    }
    /// Number of categories.
    pub fn num_categories(&self) -> Result<usize> {
        usize::try_from(self.number(1)?)
            .map_err(|_| OcioError::Ocio("invalid category count".into()))
    }
    /// Whether OCIO recognizes an alias.
    pub fn has_alias(&self, value: impl AsRef<str>) -> Result<bool> {
        self.contains(0, value)
    }
    /// Check an alias without constructing a C string.
    pub fn has_alias_cstr(&self, value: &std::ffi::CStr) -> Result<bool> {
        self.contains_cstr(0, value)
    }
    /// Whether OCIO recognizes a category.
    pub fn has_category(&self, value: impl AsRef<str>) -> Result<bool> {
        self.contains(1, value)
    }
    /// Check a category without constructing a C string.
    pub fn has_category_cstr(&self, value: &std::ffi::CStr) -> Result<bool> {
        self.contains_cstr(1, value)
    }
    /// Native bit depth.
    pub fn bit_depth(&self) -> Result<BitDepth> {
        Ok(match self.number(2)? {
            1 => BitDepth::Uint8,
            2 => BitDepth::Uint10,
            3 => BitDepth::Uint12,
            4 => BitDepth::Uint14,
            5 => BitDepth::Uint16,
            6 => BitDepth::Uint32,
            7 => BitDepth::F16,
            8 => BitDepth::F32,
            _ => BitDepth::Unknown,
        })
    }
    /// Reference space used by this color space.
    pub fn reference_space_type(&self) -> Result<ReferenceSpaceType> {
        match self.number(3)? {
            0 => Ok(ReferenceSpaceType::Scene),
            1 => Ok(ReferenceSpaceType::Display),
            _ => Err(OcioError::Ocio("unknown reference space type".into())),
        }
    }
    /// Whether this color space represents non-color data.
    pub fn is_data(&self) -> Result<bool> {
        Ok(self.number(4)? != 0)
    }
    /// Native allocation mode.
    pub fn allocation(&self) -> Result<Allocation> {
        Ok(match self.number(5)? {
            1 => Allocation::Uniform,
            2 => Allocation::Lg2,
            _ => Allocation::Unknown,
        })
    }
    /// Copy allocation parameters into a caller-provided buffer without allocation.
    pub fn allocation_vars_into(&self, output: &mut [f32]) -> Result<()> {
        let count = usize::try_from(self.number(6)?)
            .map_err(|_| OcioError::Ocio("invalid allocation parameter count".into()))?;
        if output.len() != count {
            return Err(OcioError::InvalidInput(
                "allocation parameter buffer length mismatch".into(),
            ));
        }
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_const_color_space_allocation_vars(
                self.handle.as_ptr(),
                output.as_mut_ptr(),
                output.len(),
            )
        };
        crate::ocio_call_status()
    }
    /// Number of allocation parameters.
    pub fn num_allocation_vars(&self) -> Result<usize> {
        usize::try_from(self.number(6)?)
            .map_err(|_| OcioError::Ocio("invalid allocation parameter count".into()))
    }
    /// Borrow the native name without allocating.
    pub fn name(&self) -> Result<&std::ffi::CStr> {
        crate::clear_last_error();
        let ptr = unsafe { ocio_sys::ocio_const_color_space_get_name(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        if ptr.is_null() {
            return Err(OcioError::Ocio("null color-space name".into()));
        }
        Ok(unsafe { std::ffi::CStr::from_ptr(ptr) })
    }
    /// Borrow the native description without allocating.
    pub fn description(&self) -> Result<&std::ffi::CStr> {
        crate::clear_last_error();
        let ptr = unsafe { ocio_sys::ocio_const_color_space_get_description(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        if ptr.is_null() {
            return Err(OcioError::Ocio("null color-space description".into()));
        }
        Ok(unsafe { std::ffi::CStr::from_ptr(ptr) })
    }
    /// Create an independently editable color space.
    pub fn create_editable_copy(&self) -> Result<ColorSpace> {
        crate::clear_last_error();
        let handle =
            unsafe { ocio_sys::ocio_const_color_space_create_editable_copy(self.handle.as_ptr()) };
        crate::handle_result(handle).map(|handle| ColorSpace { handle })
    }
}
impl Drop for ConstColorSpace {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_const_color_space_destroy(self.handle.as_ptr()) }
    }
}

impl ColorSpace {
    /// Share the native transform without an editable copy.
    pub fn try_const_transform(
        &self,
        direction: ColorSpaceDirection,
    ) -> Result<Option<crate::transform::ConstTransform>> {
        crate::transform::ConstTransform::from_parent(self.handle.as_ptr(), 0, direction as i32)
    }
    /// Create a new default color space.
    pub fn create() -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe { ocio_sys::ocio_color_space_create() };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Create a scene- or display-referred color space.
    pub fn create_with_reference_space(reference_space: ReferenceSpaceType) -> Result<Self> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_color_space_create_with_reference_space(reference_space as i32)
        };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Create an editable copy of this color space.
    pub fn create_editable_copy(&self) -> Result<Self> {
        crate::clear_last_error();
        let handle =
            unsafe { ocio_sys::ocio_color_space_create_editable_copy(self.handle.as_ptr()) };
        crate::handle_result(handle).map(|handle| Self { handle })
    }

    /// Get the name of this color space.
    pub fn name(&self) -> Option<String> {
        unsafe { cstr_from_mut(ocio_sys::ocio_color_space_get_name(self.handle.as_ptr())) }
    }

    /// Set the name of this color space.
    pub fn set_name(&self, name: impl AsRef<str>) -> Result<()> {
        let n = cstring(name)?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_color_space_set_name(self.handle.as_ptr(), n.as_ptr().cast()) };
        crate::ocio_call_status()
    }

    /// Get the family of this color space.
    pub fn family(&self) -> Option<String> {
        unsafe { cstr_from_mut(ocio_sys::ocio_color_space_get_family(self.handle.as_ptr())) }
    }

    /// Set the family of this color space.
    pub fn set_family(&self, family: impl AsRef<str>) -> Result<()> {
        let f = cstring(family)?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_color_space_set_family(self.handle.as_ptr(), f.as_ptr().cast()) };
        crate::ocio_call_status()
    }

    /// Get the equality group name for this color space.
    pub fn equality_group(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_color_space_get_equality_group(
                self.handle.as_ptr(),
            ))
        }
    }

    /// Set the equality group name for this color space.
    pub fn set_equality_group(&self, group: impl AsRef<str>) -> Result<()> {
        let g = cstring(group)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_color_space_set_equality_group(self.handle.as_ptr(), g.as_ptr().cast())
        };
        crate::ocio_call_status()
    }

    /// Get the description of this color space.
    pub fn description(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_color_space_get_description(
                self.handle.as_ptr(),
            ))
        }
    }

    /// Set the description of this color space.
    pub fn set_description(&self, description: impl AsRef<str>) -> Result<()> {
        let d = cstring(description)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_color_space_set_description(self.handle.as_ptr(), d.as_ptr().cast())
        };
        crate::ocio_call_status()
    }

    /// Get the bit depth of this color space.
    pub fn bit_depth(&self) -> BitDepth {
        let b = unsafe { ocio_sys::ocio_color_space_get_bit_depth(self.handle.as_ptr()) };
        match b {
            1 => BitDepth::Uint8,
            2 => BitDepth::Uint10,
            3 => BitDepth::Uint12,
            4 => BitDepth::Uint14,
            5 => BitDepth::Uint16,
            6 => BitDepth::Uint32,
            7 => BitDepth::F16,
            8 => BitDepth::F32,
            _ => BitDepth::Unknown,
        }
    }

    /// Set the bit depth of this color space, panicking on error.
    pub fn set_bit_depth(&self, bit_depth: BitDepth) {
        self.try_set_bit_depth(bit_depth)
            .expect("failed to set color space bit depth");
    }

    /// Set the color space bit depth and surface any OCIO validation error.
    pub fn try_set_bit_depth(&self, bit_depth: BitDepth) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_color_space_set_bit_depth(self.handle.as_ptr(), bit_depth as i32) };
        crate::ocio_call_status()
    }

    /// Get the reference space type of this color space.
    pub fn reference_space_type(&self) -> ReferenceSpaceType {
        let r =
            unsafe { ocio_sys::ocio_color_space_get_reference_space_type(self.handle.as_ptr()) };
        match r {
            1 => ReferenceSpaceType::Display,
            _ => ReferenceSpaceType::Scene,
        }
    }

    /// Returns whether this color space is a data-only transform.
    pub fn is_data(&self) -> bool {
        unsafe { ocio_sys::ocio_color_space_is_data(self.handle.as_ptr()) }
    }

    /// Set the data flag, panicking on error.
    pub fn set_is_data(&self, is_data: bool) {
        self.try_set_is_data(is_data)
            .expect("failed to set color space data flag");
    }

    /// Set the color space data flag and surface any OCIO validation error.
    pub fn try_set_is_data(&self, is_data: bool) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_color_space_set_is_data(self.handle.as_ptr(), is_data) };
        crate::ocio_call_status()
    }

    /// Get the category of this color space.
    pub fn category(&self) -> Option<String> {
        self.category_by_index(0)
    }

    /// Get a category by index.
    ///
    /// Returns `None` when `index` does not refer to an authored category.
    pub fn category_by_index(&self, index: i32) -> Option<String> {
        self.try_category_by_index(index).ok().flatten()
    }

    /// Get a category while preserving bridge errors.
    pub fn try_category_by_index(&self, index: i32) -> Result<Option<String>> {
        crate::clear_last_error();
        let value = unsafe {
            cstr_from_mut(ocio_sys::ocio_color_space_get_category(
                self.handle.as_ptr(),
                index,
            ))
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Set the category of this color space.
    pub fn set_category(&self, category: impl AsRef<str>) -> Result<()> {
        let c = cstring(category)?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_color_space_add_category(self.handle.as_ptr(), c.as_ptr().cast()) };
        crate::ocio_call_status()
    }

    /// Get the allocation mode of this color space.
    pub fn try_allocation(&self) -> Result<Allocation> {
        crate::clear_last_error();
        let a = unsafe { ocio_sys::ocio_color_space_get_allocation(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(match a {
            1 => Allocation::Uniform,
            2 => Allocation::Lg2,
            _ => Allocation::Unknown,
        })
    }

    /// Get the allocation mode of this color space.
    pub fn allocation(&self) -> Allocation {
        self.try_allocation().unwrap_or(Allocation::Unknown)
    }

    /// Set the allocation mode, panicking on error.
    pub fn set_allocation(&self, allocation: Allocation) {
        self.try_set_allocation(allocation)
            .expect("failed to set color space allocation");
    }

    /// Set the allocation mode and surface any OCIO validation error.
    pub fn try_set_allocation(&self, allocation: Allocation) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_color_space_set_allocation(self.handle.as_ptr(), allocation as i32)
        };
        crate::ocio_call_status()
    }

    /// Get the number of allocation variables.
    pub fn try_allocation_num_vars(&self) -> Result<i32> {
        crate::clear_last_error();
        let v = unsafe { ocio_sys::ocio_color_space_get_allocation_num_vars(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(v)
    }

    /// Get the number of allocation variables.
    pub fn allocation_num_vars(&self) -> i32 {
        self.try_allocation_num_vars().unwrap_or(0)
    }

    /// Get the allocation variables as a slice of floats.
    pub fn try_allocation_vars(&self) -> Result<Vec<f32>> {
        let n = crate::transform::checked_native_count(
            self.try_allocation_num_vars()?,
            "ColorSpace::try_allocation_vars",
        )?;
        if n == 0 {
            return Ok(Vec::new());
        }
        let mut vars =
            crate::transform::reserve_native_items(n, "ColorSpace::try_allocation_vars")?;
        vars.resize(n, 0.0f32);
        self.read_allocation_vars(&mut vars)?;
        Ok(vars)
    }

    /// Copy allocation variables into an exactly sized caller-provided buffer.
    /// The buffer is left untouched if its length does not match the native count.
    pub fn try_allocation_vars_into(&self, output: &mut [f32]) -> Result<()> {
        let count = crate::transform::checked_native_count(
            self.try_allocation_num_vars()?,
            "ColorSpace::try_allocation_vars_into",
        )?;
        if output.len() != count {
            return Err(OcioError::InvalidInput(
                "allocation parameter buffer length mismatch".into(),
            ));
        }
        if count == 0 {
            return Ok(());
        }
        self.read_allocation_vars(output)
    }

    fn read_allocation_vars(&self, output: &mut [f32]) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_color_space_get_allocation_vars(
                self.handle.as_ptr(),
                output.as_mut_ptr() as *mut c_void,
            );
        }
        crate::ocio_call_status()
    }

    /// Get the allocation variables as a slice of floats.
    pub fn allocation_vars(&self) -> Vec<f32> {
        self.try_allocation_vars().unwrap_or_default()
    }

    /// Set allocation-domain parameters for this color space.
    pub fn set_allocation_vars(&self, vars: &[f32]) -> Result<()> {
        let num_vars = i32::try_from(vars.len()).map_err(|_| {
            OcioError::InvalidInput(
                "ColorSpace::set_allocation_vars: too many allocation values".to_owned(),
            )
        })?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_color_space_set_allocation_vars(
                self.handle.as_ptr(),
                num_vars,
                vars.as_ptr() as *mut c_void,
            );
        }
        crate::ocio_call_status()
    }

    /// Get the encoding of this color space.
    pub fn encoding(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_color_space_get_encoding(
                self.handle.as_ptr(),
            ))
        }
    }

    /// Set the encoding of this color space.
    pub fn set_encoding(&self, encoding: impl AsRef<str>) -> Result<()> {
        let e = cstring(encoding)?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_color_space_set_encoding(self.handle.as_ptr(), e.as_ptr().cast()) };
        crate::ocio_call_status()
    }

    /// Get an independent editable copy of the transform for the given direction.
    pub fn transform(&self, direction: ColorSpaceDirection) -> Option<Transform> {
        self.try_transform(direction).ok().flatten()
    }

    /// Copy the transform for the given direction, preserving OCIO query failures.
    pub fn try_transform(&self, direction: ColorSpaceDirection) -> Result<Option<Transform>> {
        crate::clear_last_error();
        let handle = unsafe {
            ocio_sys::ocio_color_space_get_transform(self.handle.as_ptr(), direction as i32)
        };
        crate::ocio_call_status()?;
        Ok(transform_from_raw_handle(handle))
    }

    /// Set the transform for the given direction (panics on error).
    pub fn set_transform(&self, transform: &impl TransformHandle, direction: ColorSpaceDirection) {
        self.try_set_transform(transform, direction)
            .expect("failed to set color space transform");
    }

    /// Try to attach a transform for the given direction.
    ///
    /// Prefer this method when OCIO validation errors must be handled by the
    /// caller.
    pub fn try_set_transform(
        &self,
        transform: &impl TransformHandle,
        direction: ColorSpaceDirection,
    ) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_color_space_set_transform(
                self.handle.as_ptr(),
                transform.as_ptr() as *mut c_void,
                direction as i32,
            );
        }
        crate::ocio_call_status()
    }

    /// Set a shared read-only transform in `direction`.
    /// OCIO stores an independent copy of the supplied transform.
    pub fn try_set_const_transform(
        &self,
        transform: &crate::transform::ConstTransform,
        direction: ColorSpaceDirection,
    ) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_color_space_set_const_transform(
                self.handle.as_ptr(),
                transform.handle.as_ptr(),
                direction as i32,
            )
        };
        crate::ocio_call_status()
    }

    /// Remove the transform in `direction`, leaving the other direction unchanged.
    pub fn try_clear_transform(&self, direction: ColorSpaceDirection) -> Result<()> {
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_color_space_set_const_transform(
                self.handle.as_ptr(),
                std::ptr::null_mut(),
                direction as i32,
            )
        };
        crate::ocio_call_status()
    }

    /// Get the number of aliases, or zero on error or overflow.
    pub fn num_aliases(&self) -> i32 {
        self.try_num_aliases()
            .ok()
            .and_then(|n| i32::try_from(n).ok())
            .unwrap_or(0)
    }

    /// Get the full native alias count, preserving bridge failures.
    pub fn try_num_aliases(&self) -> Result<usize> {
        crate::clear_last_error();
        let count = unsafe { ocio_sys::ocio_color_space_get_num_aliases(self.handle.as_ptr()) };
        crate::ocio_call_status()?;
        Ok(count)
    }

    /// Get an alias by native index, returning `None` when out of range.
    pub fn try_alias_at(&self, index: usize) -> Result<Option<String>> {
        if index >= self.try_num_aliases()? {
            return Ok(None);
        }
        crate::clear_last_error();
        let value = unsafe {
            cstr_from_mut(ocio_sys::ocio_color_space_get_alias(
                self.handle.as_ptr(),
                index,
            ))
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Get the alias at the given index.
    pub fn alias(&self, index: i32) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_color_space_get_alias(
                self.handle.as_ptr(),
                index as usize,
            ))
        }
    }

    /// Add an alias for this color space.
    pub fn add_alias(&self, alias: impl AsRef<str>) -> Result<()> {
        let a = cstring(alias)?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_color_space_add_alias(self.handle.as_ptr(), a.as_ptr().cast()) };
        crate::ocio_call_status()
    }

    /// Remove an alias from this color space.
    pub fn remove_alias(&self, alias: impl AsRef<str>) -> Result<()> {
        let a = cstring(alias)?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_color_space_remove_alias(self.handle.as_ptr(), a.as_ptr().cast()) };
        crate::ocio_call_status()
    }

    /// Clear all aliases, panicking on error.
    pub fn clear_aliases(&self) {
        self.try_clear_aliases()
            .expect("failed to clear color space aliases");
    }

    /// Clear all aliases and surface any OCIO validation error.
    pub fn try_clear_aliases(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_color_space_clear_aliases(self.handle.as_ptr()) };
        crate::ocio_call_status()
    }

    /// Returns whether a transform is defined for the given direction.
    pub fn is_transform_defined(&self, direction: ColorSpaceDirection) -> bool {
        unsafe {
            ocio_sys::ocio_color_space_is_transform_defined(self.handle.as_ptr(), direction as i32)
        }
    }

    // ── v2.5.1 new methods ──

    /// Add a category to this color space.
    pub fn add_category(&self, category: impl AsRef<str>) -> Result<()> {
        let c = cstring(category)?;
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_color_space_add_category(self.handle.as_ptr(), c.as_ptr().cast()) };
        crate::ocio_call_status()
    }

    /// Remove a category from this color space.
    pub fn remove_category(&self, category: impl AsRef<str>) -> Result<()> {
        let c = cstring(category)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_color_space_remove_category(self.handle.as_ptr(), c.as_ptr().cast())
        };
        crate::ocio_call_status()
    }

    /// Clear all categories, panicking on error.
    pub fn clear_categories(&self) {
        self.try_clear_categories()
            .expect("failed to clear color space categories");
    }

    /// Clear all categories and surface any OCIO validation error.
    pub fn try_clear_categories(&self) -> Result<()> {
        crate::clear_last_error();
        unsafe { ocio_sys::ocio_color_space_clear_categories(self.handle.as_ptr() as *mut c_void) };
        crate::ocio_call_status()
    }

    /// Returns whether this color space has the given category.
    pub fn has_category(&self, category: impl AsRef<str>) -> bool {
        self.try_has_category(category).unwrap_or(false)
    }

    /// Check a category while preserving invalid input and bridge errors.
    pub fn try_has_category(&self, category: impl AsRef<str>) -> Result<bool> {
        self.try_has_category_cstr(&cstring(category)?)
    }

    /// Check a category without allocating a C string.
    pub fn try_has_category_cstr(&self, category: &std::ffi::CStr) -> Result<bool> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_color_space_has_category(self.handle.as_ptr(), category.as_ptr().cast())
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Get the number of categories for this color space.
    pub fn num_categories(&self) -> i32 {
        self.try_num_categories().unwrap_or(0)
    }

    /// Get the native category count, preserving bridge errors.
    pub fn try_num_categories(&self) -> Result<i32> {
        crate::clear_last_error();
        let count = unsafe {
            ocio_sys::ocio_color_space_get_num_categories(self.handle.as_ptr() as *mut c_void)
        };
        crate::ocio_call_status()?;
        if count < 0 {
            return Err(OcioError::ValidationFailed(
                "negative native category count".into(),
            ));
        }
        Ok(count)
    }

    /// Returns whether this color space has the given alias.
    pub fn has_alias(&self, alias: impl AsRef<str>) -> bool {
        self.try_has_alias(alias).unwrap_or(false)
    }

    /// Check an alias while preserving invalid input and bridge errors.
    pub fn try_has_alias(&self, alias: impl AsRef<str>) -> Result<bool> {
        self.try_has_alias_cstr(&cstring(alias)?)
    }

    /// Check an alias without allocating a C string.
    pub fn try_has_alias_cstr(&self, alias: &std::ffi::CStr) -> Result<bool> {
        crate::clear_last_error();
        let value = unsafe {
            ocio_sys::ocio_color_space_has_alias(self.handle.as_ptr(), alias.as_ptr().cast())
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Get the interop identifier for this color space.
    #[cfg(feature = "v2_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_5")))]
    pub fn interop_id(&self) -> Option<String> {
        unsafe {
            cstr_from_mut(ocio_sys::ocio_color_space_get_interop_id(
                self.handle.as_ptr() as *mut c_void,
            ))
        }
    }

    /// Set the interop identifier for this color space.
    #[cfg(feature = "v2_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_5")))]
    pub fn set_interop_id(&self, interop_id: impl AsRef<str>) -> Result<()> {
        let interop_id = cstring(interop_id)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_color_space_set_interop_id(
                self.handle.as_ptr(),
                interop_id.as_ptr().cast(),
            )
        };
        crate::ocio_call_status()
    }

    /// Set an interchange attribute by name and value.
    #[cfg(feature = "v2_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_5")))]
    pub fn set_interchange_attribute(
        &self,
        name: impl AsRef<str>,
        value: impl AsRef<str>,
    ) -> Result<()> {
        let name = cstring(name)?;
        let value = cstring(value)?;
        crate::clear_last_error();
        unsafe {
            ocio_sys::ocio_color_space_set_interchange_attribute(
                self.handle.as_ptr(),
                name.as_ptr().cast(),
                value.as_ptr().cast(),
            )
        };
        crate::ocio_call_status()
    }

    /// Get an interchange attribute value by name.
    #[cfg(feature = "v2_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_5")))]
    pub fn interchange_attribute(&self, name: impl AsRef<str>) -> Option<String> {
        self.try_interchange_attribute(name).ok().flatten()
    }

    /// Get an interchange attribute value while preserving invalid-name errors.
    #[cfg(feature = "v2_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_5")))]
    pub fn try_interchange_attribute(&self, name: impl AsRef<str>) -> Result<Option<String>> {
        let name = cstring(name)?;
        crate::clear_last_error();
        let value = unsafe {
            cstr_to_opt_string(ocio_sys::ocio_color_space_get_interchange_attribute(
                self.handle.as_ptr(),
                name.as_ptr(),
            ))
        };
        crate::ocio_call_status()?;
        Ok(value)
    }

    /// Get all interchange attributes as a map.
    #[cfg(feature = "v2_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_5")))]
    pub fn interchange_attributes(&self) -> BTreeMap<String, String> {
        self.try_interchange_attributes().unwrap_or_default()
    }

    /// Return an owned snapshot of all native interchange attributes, preserving errors.
    #[cfg(feature = "v2_5")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_5")))]
    pub fn try_interchange_attributes(&self) -> Result<BTreeMap<String, String>> {
        interchange_attributes_snapshot(self.handle.as_ptr(), false, None)
    }
}

impl Drop for ColorSpace {
    fn drop(&mut self) {
        unsafe { ocio_sys::ocio_color_space_destroy(self.handle.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_color_space() {
        let cs = ColorSpace::create();
        assert!(cs.is_ok());
    }

    #[test]
    fn create_display_reference_color_space() {
        let cs = ColorSpace::create_with_reference_space(ReferenceSpaceType::Display).unwrap();
        assert_eq!(cs.reference_space_type(), ReferenceSpaceType::Display);
        assert_eq!(
            cs.create_editable_copy().unwrap().reference_space_type(),
            ReferenceSpaceType::Display
        );
        assert_eq!(
            ColorSpace::create().unwrap().reference_space_type(),
            ReferenceSpaceType::Scene
        );
    }

    #[test]
    fn color_space_methods_no_crash() {
        let cs = ColorSpace::create().unwrap();
        let _ = cs.name();
        let _ = cs.family();
        let _ = cs.description();
        let _ = cs.bit_depth();
        let _ = cs.reference_space_type();
        let _ = cs.is_data();
        let _ = cs.allocation();
        let _ = cs.encoding();
    }

    #[test]
    fn set_name() {
        let cs = ColorSpace::create().unwrap();
        assert!(cs.set_name("MyColorSpace").is_ok());
    }

    #[test]
    fn set_is_data() {
        let cs = ColorSpace::create().unwrap();
        cs.set_is_data(true);
        let _ = cs.is_data();
    }

    #[test]
    fn transform_no_crash() {
        let cs = ColorSpace::create().unwrap();
        // In stub mode, returns None since bridge has no transform
        let _ = cs.transform(ColorSpaceDirection::ToReference);
    }

    #[test]
    fn set_transform_no_crash() {
        let cs = ColorSpace::create().unwrap();
        let ft = crate::transform::FileTransform::create().unwrap();
        cs.set_transform(&ft, ColorSpaceDirection::ToReference);
    }

    #[test]
    fn aliases_no_crash() {
        let cs = ColorSpace::create().unwrap();
        let _ = cs.num_aliases();
        let _ = cs.alias(0);
        assert!(cs.add_alias("test_alias").is_ok());
        assert!(cs.remove_alias("test_alias").is_ok());
        cs.try_clear_aliases().unwrap();
    }

    #[test]
    fn create_editable_copy_round_trip() {
        let cs = ColorSpace::create().unwrap();
        assert!(cs.set_name("EditableColorSpace").is_ok());
        let copy = cs.create_editable_copy().unwrap();
        if !crate::is_stub_build() {
            assert_eq!(copy.name().as_deref(), Some("EditableColorSpace"));
        }
    }

    #[cfg(feature = "v2_5")]
    #[test]
    fn interchange_attribute_no_crash() {
        let cs = ColorSpace::create().unwrap();
        assert!(cs
            .set_interchange_attribute("amf_transform_ids", "urn:ampas:aces:transformId:v1.5:CSC")
            .is_ok());
        let _ = cs.interchange_attribute("amf_transform_ids");
        let _ = cs.interchange_attributes();
    }

    #[cfg(feature = "v2_5")]
    #[test]
    #[cfg_attr(ocio_stub, ignore = "requires a real OpenColorIO build")]
    fn interchange_attribute_real_round_trip() {
        if crate::is_stub_build() {
            return;
        }

        let cs = ColorSpace::create().unwrap();
        cs.set_interchange_attribute("amf_transform_ids", "urn:test:colorspace")
            .unwrap();
        assert_eq!(
            cs.interchange_attribute("amf_transform_ids").as_deref(),
            Some("urn:test:colorspace")
        );
        assert_eq!(
            cs.interchange_attributes()
                .get("amf_transform_ids")
                .map(String::as_str),
            Some("urn:test:colorspace")
        );
    }

    #[test]
    fn is_transform_defined_no_crash() {
        let cs = ColorSpace::create().unwrap();
        let _ = cs.is_transform_defined(ColorSpaceDirection::ToReference);
        let _ = cs.is_transform_defined(ColorSpaceDirection::FromReference);
    }
}
