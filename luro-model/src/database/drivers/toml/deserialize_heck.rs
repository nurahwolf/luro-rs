use std::collections::BTreeMap;

use serde::{de, Deserialize, Deserializer};

use crate::heck::Hecks;

pub fn deserialize_heck<'de, D,>(deserializer: D,) -> Result<Hecks, D::Error,>
where
    D: Deserializer<'de,>,
{
    let str_map = BTreeMap::<String, _,>::deserialize(deserializer,)?;
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
