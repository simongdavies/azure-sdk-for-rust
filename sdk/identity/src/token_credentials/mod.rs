//! Access to token credentials through various means
//!
//! Supported means currently include:
//! * The environment
//! * Azure CLI credentials cache
//! * Managed identity
//! * Client secret
mod cli_credentials;
#[cfg(not(feature = "enable_rpaas_wasm_modules"))]
mod client_secret_credentials;
mod default_credentials;
#[cfg(not(feature = "enable_rpaas_wasm_modules"))]
mod environment_credentials;
#[cfg(feature = "enable_rpaas_wasm_modules")]
mod environment_variable_credentials;
#[cfg(not(feature = "enable_rpaas_wasm_modules"))]
mod managed_identity_credentials;

pub use cli_credentials::*;
#[cfg(not(feature = "enable_rpaas_wasm_modules"))]
pub use client_secret_credentials::*;
pub use default_credentials::*;
#[cfg(not(feature = "enable_rpaas_wasm_modules"))]
pub use environment_credentials::*;
#[cfg(feature = "enable_rpaas_wasm_modules")]
pub use environment_variable_credentials::*;
#[cfg(not(feature = "enable_rpaas_wasm_modules"))]
pub use managed_identity_credentials::*;
