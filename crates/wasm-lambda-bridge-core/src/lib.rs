pub type Result<T> = anyhow::Result<T>;

#[cfg(feature = "hostcall")]
pub mod hostcall;

#[cfg(feature = "value")]
pub mod value;

pub static WASM_MODULE_NAME: &str = "wasm-lambda-bridge";
