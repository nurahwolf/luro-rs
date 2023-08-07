use std::collections::{BTreeMap, HashMap};

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use twilight_model::{
    id::{
        marker::{MessageMarker, UserMarker},
        Id
    },
    user::{PremiumType, UserFlags},
    util::ImageHash
};

use crate::{luro_message::LuroMessage, user_actions::UserActions, user_marriages::UserMarriages};

/// Some nice functionality primarily around [User] and [Member], with some added goodness
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LuroUser {
    /// Accent color of the user's banner.
    ///
    /// This is an integer representation of a hexadecimal color code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<ImageHash>,
    /// Hash of the user's banner image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<ImageHash>,
    #[serde(default)]
    pub bot: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discriminator: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<UserFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<UserMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_enabled: Option<bool>,
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_type: Option<PremiumType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_flags: Option<UserFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    /// How many words they have said
    #[serde(default)]
    pub wordcount: usize,
    /// The sum of the length of all the words they have said. This is then divided by their wordcount to work out an average.
    #[serde(default)]
    pub averagesize: usize,
    #[serde(deserialize_with = "deserialize_wordsize", serialize_with = "serialize_wordsize")]
    /// A hashmap containing the word length, and how many times it has appeared
    /// TODO: The key for this is stored as a string on the disk which is only needed for toml
    #[serde(default)]
    pub wordsize: BTreeMap<usize, usize>,
    /// A hashmap containing a count on how often a particular word appears
    #[serde(default)]
    pub words: BTreeMap<String, usize>,
    /// An tuple of warnings wrapped in a vec. The first value is the warning, and the second is whoever warned the person
    #[serde(default)]
    pub warnings: Vec<(String, Id<UserMarker>)>,
    #[serde(default)]
    pub messages: BTreeMap<Id<MessageMarker>, LuroMessage>,
    #[serde(default)]
    pub moderation_actions: Vec<UserActions>,
    #[serde(default)]
    pub moderation_actions_performed: usize,
    /// A simple tally of how many times a user has fucked up and needed to edit their message.
    #[serde(default)]
    pub message_edits: usize,
    /// The user's marriages
    #[serde(default)]
    pub marriages: HashMap<Id<UserMarker>, UserMarriages>
}

pub fn serialize_wordsize<S>(input: &BTreeMap<usize, usize>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    let data = input
        .iter()
        .map(|(str_key, value)| (str_key.to_string(), *value))
        .collect::<BTreeMap<String, usize>>();

    s.collect_map(data)
}

pub fn deserialize_wordsize<'de, D>(deserializer: D) -> Result<BTreeMap<usize, usize>, D::Error>
where
    D: Deserializer<'de>
{
    let str_map = BTreeMap::<String, usize>::deserialize(deserializer)?;
    let original_len = str_map.len();
    let data = {
        str_map
            .into_iter()
            .map(|(str_key, value)| match str_key.parse() {
                Ok(int_key) => Ok((int_key, value)),
                Err(_) => Err(de::Error::invalid_value(
                    de::Unexpected::Str(&str_key),
                    &"a non-negative integer"
                ))
            })
            .collect::<Result<BTreeMap<_, _>, _>>()?
    };
    // multiple strings could parse to the same int, e.g "0" and "00"
    if data.len() < original_len {
        return Err(de::Error::custom("detected duplicate integer key"));
    }
    Ok(data)
}
