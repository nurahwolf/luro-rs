use core::fmt;
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use twilight_interactions::command::{CommandOption, CreateOption};
/// The different categories of fetishes a user can have
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, Ord, PartialOrd, Eq, CommandOption, CreateOption)]
pub enum FetishCategory {
    #[option(name = "Favourite - Something this character loves to the end of the world", value = "favourite")]
    Favourite,
    #[option(name = "Favourite - The character loves this!", value = "love")]
    Love,
    #[option(name = "Favourite - The character likes this", value = "like")]
    Like,
    #[default]
    #[option(name = "Favourite - The character is neutral on this", value = "neutral")]
    Neutral,
    #[option(name = "Dislike - The character dislikes this", value = "dislike")]
    Dislike,
    #[option(name = "Hate - The character hates this", value = "hate")]
    Hate,
    #[option(name = "Limit - A hard no (limit) that this character refuses to do", value = "limit")]
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
    #[serde(default)]
    pub category: FetishCategory,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
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
    #[serde(default)]
    pub short_description: String,
    /// A detailed description for this character
    #[serde(default)]
    pub description: String,
    /// A detailed description for this character that is only shown in the NSFW profile
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub nsfw_description: Option<String>,
    /// Set to true if there are NSFW details present
    #[serde(default)]
    pub nsfw: bool,
    /// A list of fetishes the character has. NSFW only!
    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    pub fetishes: BTreeMap<usize, Fetish>
}
