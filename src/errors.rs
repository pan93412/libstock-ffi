use std::error::Error;
use std::ffi::OsString;
use std::os::raw::c_char;
use std::sync::Mutex;

use ::safer_ffi::prelude::*;

static LAST_ERROR: Mutex<Option<anyhow::Error>> = Mutex::new(None);

// Inspired from https://michael-f-bryan.github.io/rust-ffi-guide/errors/return_types.html

/// 傳入一個錯誤物件，回傳 Option。
///
/// 這個 [`Option`] 可以用來做 Early Return。
/// `safer-ffi` 會將 `None` 轉換為 null pointer。
///
/// # Example
///
/// ```
/// use libstock_ffi::errors::update_error_message;
///
/// fn error_func() -> Option<()> {
///     update_error_message("Error message.")?;
///     Some(())
/// }
///
/// assert_eq(error_func(), None);
/// ```
pub fn update_error_message<E: Error + 'static + Send + Sync>(error: E) -> Option<()> {
    log::error!("Error happened: {error}. Updating error messages…");

    LAST_ERROR.lock().unwrap().replace(error.into());

    None
}

pub fn take_error() -> Option<anyhow::Error> {
    LAST_ERROR.lock().ok().and_then(|mut v| v.take())
}

/// Calculate the number of bytes in the last error's error message **not**
/// including any trailing `null` characters.
///
/// The return value is based on your system's representation.
#[ffi_export]
pub fn last_error_length() -> isize {
    let error = LAST_ERROR.lock();
    let error = match error {
        Ok(error) => error,
        Err(e) => {
            log::error!("Failed to get LAST_ERROR: {e}");
            return -1;
        }
    };

    error
        .as_ref()
        .map(|v| {
            let os_str = OsString::from(v.to_string());
            os_str.len() + 1
        })
        .unwrap_or(0) as isize
}

#[ffi_export]
pub fn get_error_message(mut buffer: c_slice::Mut<'_, c_char>) -> isize {
    let error = match take_error() {
        Some(e) => e,
        None => {
            log::warn!("No error message found.");
            return -1;
        }
    };

    let errmsg = error.to_string();
    let errlen = errmsg.len();

    if errlen >= buffer.len() {
        log::warn!("Buffer provided for writing the last error message is too small.");
        log::warn!(
            "Expected at least {} bytes but got {}",
            errlen + 1,
            buffer.len()
        );
        return -1;
    }

    // Copy the error message to the buffer.
    unsafe {
        // SAFETY: We have ensured that the buffer is always
        // greater than the length of the error message.
        std::ptr::copy_nonoverlapping(errmsg.as_ptr(), buffer.as_mut_ptr() as *mut u8, errlen);
    }

    // Add a trailing null so people using the string as a `char *` don't
    // accidentally read into garbage.
    buffer[errlen] = 0;

    errlen as isize
}
