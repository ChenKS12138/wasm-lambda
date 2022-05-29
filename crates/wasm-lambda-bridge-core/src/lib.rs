pub type Result<T> = anyhow::Result<T>;

#[cfg(feature = "hostcall")]
pub mod hostcall;

#[cfg(feature = "value")]
pub mod value;

#[cfg(feature = "web")]
pub mod web;
