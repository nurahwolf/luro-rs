use std::collections::BTreeMap;

// Serialise a BTreeMap, changing the key from usize to String
pub fn toml_deserializer<T>(input: BTreeMap<usize, T>) -> BTreeMap<String, T> {
    input
        .into_iter()
        .map(|(str_key, value)| (str_key.to_string(), value))
        .collect::<BTreeMap<String, T>>()
}
