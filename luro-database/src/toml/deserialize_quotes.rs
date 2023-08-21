use crate::toml::anyhow;
use std::collections::BTreeMap;

// Deserialise a BTreeMap, changing the key from [String] to [usize]
pub fn deserialize_quotes<T>(input: BTreeMap<String, T>) -> anyhow::Result<BTreeMap<usize, T>> {
    let original_len = input.len();
    let data = input
        .into_iter()
        .map(|(key, value)| {
            (
                match key.parse() {
                    Ok(usize_key) => usize_key,
                    Err(_) => todo!()
                },
                value
            )
        })
        .collect::<BTreeMap<usize, T>>();

    // multiple strings could parse to the same int, e.g "0" and "00"
    if data.len() < original_len {
        return Err(anyhow!("detected duplicate integer key"));
    }

    Ok(data)
}
