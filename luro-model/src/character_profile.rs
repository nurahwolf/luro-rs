use core::fmt;
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
/// The different categories of fetishes a user can have
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, Ord, PartialOrd, Eq)]
pub enum FetishCategory {
    Favourite,
    Love,
    Like,
    #[default]
    Neutral,
    Dislike,
    Hate,
    Limit
}

/// A list of assignable fetishes. Used for matching with other users
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, Ord, PartialOrd, Eq)]
pub enum FetishList {
    #[default]
    Custom
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Fetish {
    pub category: FetishCategory,
    pub description: String,
    pub list: FetishList
}

impl fmt::Display for FetishList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Custom => "Custom"
            }
        )
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CharacterProfile {
    /// A short description for this character
    pub short_description: String,
    /// A detailed description for this character
    pub description: String,
    /// A detailed description for this character that is only shown in the NSFW profile
    pub nsfw_description: Option<String>,
    /// Set to true if there are NSFW details present
    pub nsfw: bool,
    /// A list of fetishes the character has. NSFW only!
    pub fetishes: BTreeMap<usize, Fetish>
}
