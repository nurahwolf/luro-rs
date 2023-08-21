#![feature(async_fn_in_trait)]
#![feature(let_chains)]

use std::collections::{BTreeMap, HashMap};

use heck::Hecks;
use message::LuroMessage;
use serde::{de, Deserialize, Deserializer, Serializer};
use story::Story;
use twilight_model::{
    application::interaction::Interaction,
    id::{marker::UserMarker, Id}
};

/// The primary owner user ID. Used for some defaults, as well as to say who owns the bot. This MUST  be set, even if a group of people own Luro, as its used as a fallback for when data is not tied to a specific user. For example, see [Story].
pub const PRIMARY_BOT_OWNER: Id<UserMarker> = Id::new(373524896187416576);
// Luro's primary owner(s)
pub const BOT_OWNERS: [Id<UserMarker>; 2] = [Id::new(373524896187416576), Id::new(138791390279630849)];
/// Luro's main accent colour
pub const ACCENT_COLOUR: u32 = 0xDABEEF;
/// Luro's DANGER colour
pub const COLOUR_DANGER: u32 = 0xD35F5F;
/// Transparent embed color (dark theme)
pub const COLOUR_TRANSPARENT: u32 = 0x2F3136;
/// Luro's SUCCESS colour
pub const COLOUR_SUCCESS: u32 = 0xA0D995;

pub mod database;
pub mod guild;
pub mod heck;
pub mod legacy;
pub mod message;
pub mod role;
pub mod story;
pub mod user;

/// A simple wrapper around quotes. Primary key is the ID of the story.
pub type Quotes = BTreeMap<usize, LuroMessage>;

pub type Stories = BTreeMap<usize, Story>;

/// A [HashMap] containing an [Interaction], keyed by a [String]. Generally the message ID, but can be other markers too. This is primarily used for recalling interactions in the future
pub type CommandManager = HashMap<String, Interaction>;

pub fn serialize_heck_id<S>(input: &[usize], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    s.collect_seq(input.iter().map(|vec| vec.to_string()))
}

pub fn deserialize_heck_id<'de, D>(deserializer: D) -> Result<Vec<usize>, D::Error>
where
    D: Deserializer<'de>
{
    let input = Vec::<String>::deserialize(deserializer)?;

    let data = input.into_iter().map(|vec| vec.parse().unwrap_or(0)).collect::<Vec<usize>>();

    Ok(data)
}

pub fn serialize_heck<S>(input: &Hecks, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    let data = input
        .clone()
        .into_iter()
        .map(|(str_key, value)| (str_key.to_string(), value))
        .collect::<BTreeMap<String, _>>();

    s.collect_map(data)
}

pub fn deserialize_heck<'de, D>(deserializer: D) -> Result<Hecks, D::Error>
where
    D: Deserializer<'de>
{
    let str_map = BTreeMap::<String, _>::deserialize(deserializer)?;
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

pub fn serialize_story<S>(input: &Stories, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    let data = input
        .clone()
        .into_iter()
        .map(|(str_key, value)| (str_key.to_string(), value))
        .collect::<BTreeMap<String, _>>();

    s.collect_map(data)
}

pub fn deserialize_story<'de, D>(deserializer: D) -> Result<Stories, D::Error>
where
    D: Deserializer<'de>
{
    let str_map = BTreeMap::<String, _>::deserialize(deserializer)?;
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

pub fn serialize_toml<S, T>(input: &BTreeMap<usize, T>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: serde::Serialize
{
    let data = input
        .iter()
        .map(|(key, value)| (key.to_string(), value))
        .collect::<BTreeMap<String, _>>();

    s.collect_map(data)
}

pub fn deserialize_toml<'de, D, T>(deserializer: D) -> Result<BTreeMap<usize, T>, D::Error>
where
    D: Deserializer<'de>,
    T: serde::Deserialize<'de>
{
    let str_map = BTreeMap::<String, T>::deserialize(deserializer)?;
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
            .collect::<Result<BTreeMap<usize, T>, _>>()?
    };
    // multiple strings could parse to the same int, e.g "0" and "00"
    if data.len() < original_len {
        return Err(de::Error::custom("detected duplicate integer key"));
    }
    Ok(data)
}
