use std::collections::BTreeMap;

use luro_model::Stories;
use serde::Serializer;

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
