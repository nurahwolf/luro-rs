use serde::{Deserialize, Deserializer};
use twilight_model::{
    id::{marker::UserMarker, Id},
    user::User,
};

pub fn deserialize_user_to_id<'de, D>(deserializer: D) -> Result<Id<UserMarker>, D::Error>
where
    D: Deserializer<'de>,
{
    let data: User = Deserialize::deserialize(deserializer)?;
    Ok(data.id)
}
