use std::collections::BTreeMap;

use dashmap::DashMap;
use serde::{de, Deserialize, Deserializer, Serializer};

use crate::{heck::Heck, story::Story};

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
