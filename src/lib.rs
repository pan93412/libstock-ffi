//! The C FFI of libstock.

pub mod structure;
pub mod fields;
pub mod errors;

#[cfg(feature = "headers")]
pub mod header;

pub(crate) mod serializer;

pub use errors::*;

/// 啟用 Logging，輸出所有日誌。
#[safer_ffi::ffi_export]
pub fn enable_logging() {
    tracing_subscriber::fmt::init();
}
