#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// Disable clippy and `u128` not being FFI-safe (see #1)
#![allow(clippy::all)]
#![allow(improper_ctypes)]

include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/generated/bindings.rs"
));

#[cfg(target_os = "windows")]
pub type OnnxEnumInt = i32;
#[cfg(not(target_os = "windows"))]
pub type OnnxEnumInt = u32;

#[cfg(target_os = "windows")]
pub const ORT_API_VERSION_MULTI_PLATFORM: u32 = 7;
#[cfg(not(target_os = "windows"))]
pub const ORT_API_VERSION_MULTI_PLATFORM: u32 = ORT_API_VERSION;
