use serde::{de, Deserialize, Deserializer, Serializer};
use std::collections::BTreeMap;

use crate::user::character::Fetish;

pub mod deserialize_heck;
pub mod deserialize_role_positions;
pub mod deserialize_user_to_id;
pub mod ok_or_default;
pub mod serialize_heck;
pub mod serialize_role_positions;

// Serialise a BTreeMap, changing the key from usize to String
pub fn serialize_fetish<S,>(input: &BTreeMap<usize, Fetish,>, serializer: S,) -> Result<S::Ok, S::Error,>
where
    S: Serializer,
{
    let data = input
        .iter()
        .map(|(str_key, value,)| (str_key.to_string(), value.clone(),),)
        .collect::<BTreeMap<String, Fetish,>>();

    serializer.collect_map(data,)
}

// Deserialise a BTreeMap, changing the key from [String] to [usize]
pub fn deserialize_fetish<'de, D,>(deserializer: D,) -> Result<BTreeMap<usize, Fetish,>, D::Error,>
where
    D: Deserializer<'de,>,
{
    let input = BTreeMap::<String, Fetish,>::deserialize(deserializer,)?;
    let original_len = input.len();
    let data = input
        .into_iter()
        .map(|(key, value,)| {
            (
                match key.parse() {
                    Ok(usize_key,) => usize_key,
                    Err(_,) => todo!(),
                },
                value,
            )
        },)
        .collect::<BTreeMap<usize, Fetish,>>();

    // multiple strings could parse to the same int, e.g "0" and "00"
    if data.len() < original_len {
        return Err(de::Error::custom("detected duplicate integer key",),);
    }

    Ok(data,)
}

pub fn serialize_wordsize<S,>(input: &BTreeMap<usize, usize,>, serializer: S,) -> Result<S::Ok, S::Error,>
where
    S: Serializer,
{
    let data = input
        .iter()
        .map(|(str_key, value,)| (str_key.to_string(), *value,),)
        .collect::<BTreeMap<String, usize,>>();

    serializer.collect_map(data,)
}

pub fn deserialize_wordsize<'de, D,>(deserializer: D,) -> Result<BTreeMap<usize, usize,>, D::Error,>
where
    D: Deserializer<'de,>,
{
    let str_map = BTreeMap::<String, usize,>::deserialize(deserializer,)?;
    let original_len = str_map.len();
    let data = {
        str_map
            .into_iter()
            .map(|(str_key, value,)| match str_key.parse() {
                Ok(int_key,) => Ok((int_key, value,),),
                Err(_,) => Err(de::Error::invalid_value(
                    de::Unexpected::Str(&str_key,),
                    &"a non-negative integer",
                ),),
            },)
            .collect::<Result<BTreeMap<_, _,>, _,>>()?
    };
    // multiple strings could parse to the same int, e.g "0" and "00"
    if data.len() < original_len {
        return Err(de::Error::custom("detected duplicate integer key",),);
    }
    Ok(data,)
}
