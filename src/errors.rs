use std::sync::Mutex;

use ::safer_ffi::prelude::*;

static LAST_ERROR: Mutex<Option<anyhow::Error>> = Mutex::new(None);

// Inspired from https://michael-f-bryan.github.io/rust-ffi-guide/errors/return_types.html

/// 傳入 [`anyhow::Error`]，回傳 Option。
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
pub fn update_error_message(error: anyhow::Error) -> Option<()> {
    tracing::error!("Error happened: {error}. Updating error messages…");

    LAST_ERROR.lock().unwrap().replace(error);

    None
}

fn take_error() -> Option<anyhow::Error> {
    LAST_ERROR.lock().ok().and_then(|mut v| v.take())
}

/// 取得 [`update_error_message`] 存入的錯誤訊息。
#[ffi_export]
#[tracing::instrument]
pub fn get_error_message() -> Option<char_p::Box> {
    match take_error() {
        Some(e) => {
            let m = e.to_string();

            match char_p::Box::try_from(m) {
                Ok(v) => Some(v),
                Err(e) => {
                    tracing::error!("Failed to convert error message to char_p::Box: {e}");
                    None
                }
            }
        }
        None => {
            tracing::warn!("No error message found.");
            None
        }
    }
}

/// 釋放 [`get_error_message`] 取出的錯誤訊息。
#[ffi_export]
pub fn free_error_message(v: Option<char_p::Box>) { drop(v) }
