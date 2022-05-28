#[cfg(feature = "hostcall")]
pub mod hostcall;

#[cfg(feature = "value")]
pub mod value;

pub type Result<T> = anyhow::Result<T>;
