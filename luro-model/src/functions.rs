use dashmap::DashMap;
use serde::{de, Deserialize, Deserializer, Serializer};

use crate::{heck::Heck, story::Story};

pub fn serialize_heck<S>(input: &DashMap<usize, Heck>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    let data = input
        .clone()
        .into_iter()
        .map(|(str_key, value)| (str_key.to_string(), value))
        .collect::<DashMap<String, _>>();

    s.collect_map(data)
}

pub fn deserialize_heck<'de, D>(deserializer: D) -> Result<DashMap<usize, Heck>, D::Error>
where
    D: Deserializer<'de>
{
    let str_map = DashMap::<String, _>::deserialize(deserializer)?;
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
            .collect::<Result<DashMap<_, _>, _>>()?
    };
    // multiple strings could parse to the same int, e.g "0" and "00"
    if data.len() < original_len {
        return Err(de::Error::custom("detected duplicate integer key"));
    }
    Ok(data)
}

pub fn serialize_story<S>(input: &DashMap<usize, Story>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    let data = input
        .clone()
        .into_iter()
        .map(|(str_key, value)| (str_key.to_string(), value))
        .collect::<DashMap<String, _>>();

    s.collect_map(data)
}

pub fn deserialize_story<'de, D>(deserializer: D) -> Result<DashMap<usize, Story>, D::Error>
where
    D: Deserializer<'de>
{
    let str_map = DashMap::<String, _>::deserialize(deserializer)?;
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
            .collect::<Result<DashMap<_, _>, _>>()?
    };
    // multiple strings could parse to the same int, e.g "0" and "00"
    if data.len() < original_len {
        return Err(de::Error::custom("detected duplicate integer key"));
    }
    Ok(data)
}
