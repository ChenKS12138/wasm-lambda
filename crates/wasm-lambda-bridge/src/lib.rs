#[cfg(feature = "core")]
pub use bridge_codegen as codegen;
#[cfg(feature = "core")]
pub use bridge_core as core;
#[cfg(feature = "core")]
pub use bridge_core::Result;
#[cfg(feature = "core")]
pub use serde_json;

#[cfg(feature = "web")]
pub mod web;
