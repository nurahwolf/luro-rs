use serde::{Deserialize, Deserializer};
use twilight_model::id::{marker::UserMarker, Id};

pub fn ok_or_default<'de, D>(deserializer: D) -> Result<Option<Id<UserMarker>>, D::Error>
where
    D: Deserializer<'de>
{
    let data: Option<_> = Deserialize::deserialize(deserializer).unwrap_or_default();
    Ok(data)
}
