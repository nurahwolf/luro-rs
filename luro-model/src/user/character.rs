#[cfg(feature = "toml-driver")]
use crate::database_driver::drivers::toml::deserialize_fetish;
#[cfg(feature = "toml-driver")]
use crate::database_driver::drivers::toml::deserialize_image;
#[cfg(feature = "toml-driver")]
use crate::database_driver::drivers::toml::serialize_fetish;
#[cfg(feature = "toml-driver")]
use crate::database_driver::drivers::toml::serialize_image;
use core::fmt;

use serde::{Deserialize, Serialize};
use twilight_interactions::command::{CommandOption, CreateOption};
/// The different categories of fetishes a user can have
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, Ord, PartialOrd, Eq, CommandOption, CreateOption)]
pub enum FetishCategory {
    #[option(name = "Favourite - Something this character loves to the end of the world", value = "favourite")]
    Favourite,
    #[option(name = "Love - The character loves this!", value = "love")]
    Love,
    #[option(name = "Like - The character likes this", value = "like")]
    Like,
    #[default]
    #[option(name = "Neutral - The character is neutral on this", value = "neutral")]
    Neutral,
    #[option(name = "Dislike - The character dislikes this", value = "dislike")]
    Dislike,
    #[option(name = "Hate - The character hates this", value = "hate")]
    Hate,
    #[option(name = "Limit - A hard no (limit) that this character refuses to do", value = "limit")]
    Limit,
}

/// A list of assignable fetishes. Used for matching with other users
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, Ord, PartialOrd, Eq)]
pub enum FetishList {
    #[default]
    Custom,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Fetish {
    #[serde(default)]
    pub category: FetishCategory,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub list: FetishList,
}

impl fmt::Display for FetishList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Custom => "Custom",
            }
        )
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CharacterProfile {
    pub prefix: Option<String>,
    pub name: String,
    pub sfw_description: String,
    pub sfw_summary: String,
    pub sfw_icons: Vec<String>,
    pub nsfw_description: Option<String>,
    pub nsfw_summary: Option<String>,
    pub nsfw_icons: Vec<String>,
}

/// A structure representing an image
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CharacterImage {
    /// The URL for the image
    #[serde(default)]
    pub url: String,
    /// If the image is NSFW or not
    #[serde(default)]
    pub nsfw: bool,
    /// If marked as a fav, it can randomly be used as the profile image
    #[serde(default)]
    pub favourite: bool,
    /// The name of the image
    #[serde(default)]
    pub name: String,
    /// The name of the character
    #[serde(default)]
    pub character_name: String,
    /// The ID of the image
    #[serde(default)]
    pub img_id: i64,
    /// The owner of the image
    #[serde(default)]
    pub owner_id: i64,
    /// A URL for the source of an image
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}
