use std::collections::BTreeMap;

use serde::Serializer;

use crate::heck::Hecks;

pub fn serialize_heck<S,>(input: &Hecks, s: S,) -> Result<S::Ok, S::Error,>
where
    S: Serializer,
{
    let data = input
        .clone()
        .into_iter()
        .map(|(str_key, value,)| (str_key.to_string(), value,),)
        .collect::<BTreeMap<String, _,>>();

    s.collect_map(data,)
}
