use core::fmt;
use std::{fmt::Display, str::FromStr};

use anyhow::bail;

use super::CustomId;

impl CustomId {
    /// Create a new custom id.
    pub fn new(name: impl Into<String>, id: String) -> Self {
        Self {
            name: name.into(),
            id: Some(id)
        }
    }

    /// Create a new custom id with only a name.
    pub fn name(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            id: None
        }
    }
}

impl FromStr for CustomId {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() {
            bail!("expected non-empty custom id");
        }

        match value.split_once(':') {
            Some((name, id)) => Ok(CustomId {
                name: name.to_owned(),
                id: Some(id.to_owned())
            }),
            None => Ok(CustomId {
                name: value.to_owned(),
                id: None
            })
        }
    }
}

impl Display for CustomId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(id) = &self.id {
            f.write_str(&self.name)?;
            f.write_str(":")?;
            f.write_str(id)
        } else {
            f.write_str(&self.name)
        }
    }
}
