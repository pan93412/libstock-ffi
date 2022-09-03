use std::io::Cursor;

use safer_ffi::prelude::*;
use wmjtyd_libstock::data::serializer::{StructDeserializer, StructSerializer};

/// 序列化指定結構體，並將序列化內容存入 buf
pub fn serialize<S, E>(
    src: S,
    buf: c_slice::Mut<u8>,
    written_size: &mut usize,
) -> anyhow::Result<()>
where
    S: StructSerializer<Err = E>,
    E: std::error::Error + Send + Sync + 'static,
{
    // 記錄寫入的長度。
    let mut cursor = Cursor::new(buf.as_slice());

    src.serialize(&mut cursor)?;

    *written_size = cursor.position() as usize;

    Ok(())
}

/// 序列化指定結構體的 C FFI 版本
///
/// 詳見 [`serialize`]。
pub fn serialize_ffi<'a, FFI, S, E>(
    src: &'a FFI,
    buf: c_slice::Mut<u8>,
    written_size: &mut usize,
) -> anyhow::Result<()>
where
    FFI: ReprC + 'a,
    S: TryFrom<&'a FFI, Error = anyhow::Error> + StructSerializer<Err = E>,
    E: std::error::Error + Send + Sync + 'static,
{
    let src = S::try_from(src)?;

    serialize(src, buf, written_size)
}

/// 從 data 讀取資料，並將資料反序列化為指定結構體
pub fn deserialize<S, E>(data: &c_slice::Ref<'_, u8>) -> Result<S, anyhow::Error>
where
    S: StructDeserializer<Err = E>,
    E: std::error::Error + Send + Sync + 'static,
{
    Ok(S::deserialize(&mut data.as_slice())?)
}

/// 從 data 讀取資料，並將資料反序列化為指定的 FFI 結構體
pub fn deserialize_ffi<FFI, S, E>(data: &c_slice::Ref<'_, u8>) -> Result<repr_c::Box<FFI>, anyhow::Error>
where
    FFI: TryFrom<S, Error = anyhow::Error> + ReprC,
    S: StructDeserializer<Err = E>,
    E: std::error::Error + Send + Sync + 'static,
{
    Ok(repr_c::Box::<FFI>::new(FFI::try_from(deserialize::<S, _>(data)?)?))
}

macro_rules! construct_serializer {
    ($name:ident, $ffi_struct:ident, $rust_struct:ident) => {
        #[::safer_ffi::ffi_export]
        #[doc = concat!("將傳入的 ", stringify!($ffi_struct), " 序列化，並將內容放入 buf 中")]
        #[doc = ""]
        #[doc = "written_size 是實際寫入的大小。"]
        pub fn $name(
            src: &$ffi_struct,
            buf: ::safer_ffi::prelude::c_slice::Mut<'_, u8>,
            written_size: &mut usize,
        ) -> bool {
            let result = $crate::serializer::serialize_ffi::<$ffi_struct, $rust_struct, _>(
                src,
                buf,
                written_size,
            );

            result.map_or_else(
                |e| {
                    $crate::errors::update_error_message(e);
                    false
                },
                |_| true,
            )
        }
    };
}

macro_rules! construct_deserializer {
    ($name:ident, $ffi_struct:ident, $rust_struct:ident) => {
        #[::safer_ffi::ffi_export]
        #[doc = concat!("將傳入的 data 反序列化回 ", stringify!($ffi_struct), " 結構體")]
        #[doc = ""]
        #[doc = "請稍後使用本結構體對應的 free 函數釋放記憶體，否則可能導致 Memory Leak。"]
        pub fn $name(data: &::safer_ffi::prelude::c_slice::Ref<'_, u8>) -> Option<::safer_ffi::prelude::repr_c::Box<$ffi_struct>> {
            let result = $crate::serializer::deserialize_ffi::<$ffi_struct, $rust_struct, _>(
                data
            );

            result.map_or_else(
                |e| {
                    $crate::errors::update_error_message(e);
                    None
                },
                Some,
            )
        }
    }
}

macro_rules! construct_free_function {
    ($name:ident, $ffi_struct:ident) => {
        #[::safer_ffi::ffi_export]
        #[doc = concat!("釋放 ", stringify!($ffi_struct), " 佔用的記憶體")]
        pub fn $name(v: Option<::safer_ffi::prelude::repr_c::Box<$ffi_struct>>) {
            drop(v)
        }
    }
}

pub(crate) use {construct_serializer, construct_deserializer, construct_free_function};
