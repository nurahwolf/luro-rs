#![feature(async_fn_in_trait)]

use serde::{Deserialize, Serialize};

/// Defaults to the toml driver
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[cfg(feature = "toml-driver")]
pub struct TomlDatabaseDriver {}
#[cfg(feature = "toml-driver")]
pub mod toml_driver;
