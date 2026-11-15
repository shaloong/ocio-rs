use std::ffi::c_void;
use std::io::Write;

use crate::{OcioError, Result};

type WriteCallback = unsafe extern "C" fn(*mut c_void, *const u8, usize) -> bool;

/// # Safety
/// `invoke` must call the callback synchronously with the supplied context and
/// valid input bytes, and must not retain the context or callback afterward.
pub(crate) unsafe fn write_to<W: Write + ?Sized>(
    writer: &mut W,
    label: &'static str,
    invoke: impl FnOnce(*mut c_void, WriteCallback) -> bool,
) -> Result<()> {
    struct State<'a, W: ?Sized> {
        writer: &'a mut W,
        error: Option<OcioError>,
        label: &'static str,
    }
    unsafe extern "C" fn write<W: Write + ?Sized>(
        context: *mut c_void,
        data: *const u8,
        len: usize,
    ) -> bool {
        let state = unsafe { &mut *context.cast::<State<'_, W>>() };
        if state.error.is_some() {
            return false;
        }
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let bytes = if len == 0 {
                &[]
            } else {
                unsafe { std::slice::from_raw_parts(data, len) }
            };
            state.writer.write_all(bytes)
        }));
        match result {
            Ok(Ok(())) => true,
            Ok(Err(error)) => {
                state.error = Some(OcioError::Ocio(error.to_string()));
                false
            }
            Err(_) => {
                state.error = Some(OcioError::Ocio(format!(
                    "{} output writer panicked",
                    state.label
                )));
                false
            }
        }
    }
    let mut state = State {
        writer,
        error: None,
        label,
    };
    crate::clear_last_error();
    let ok = invoke((&mut state as *mut State<'_, W>).cast(), write::<W>);
    if let Some(error) = state.error {
        return Err(error);
    }
    crate::ocio_call_status()?;
    if !ok {
        return Err(OcioError::Ocio(format!("{label} output writer failed")));
    }
    Ok(())
}
