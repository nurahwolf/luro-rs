use std::collections::BTreeMap;

use serde::Serializer;

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
