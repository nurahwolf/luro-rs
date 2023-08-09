#![feature(async_fn_in_trait)]

use serde::{Deserialize, Serialize};

#[cfg(feature = "toml-driver")]
pub mod toml_driver;

/// Defaults to the toml driver
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[cfg(feature = "toml-driver")]
pub struct TomlDatabaseDriver {}
