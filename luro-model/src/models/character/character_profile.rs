use twilight_model::id::{marker::UserMarker, Id};

#[derive(Clone, Debug, serde::Deserialize, PartialEq, serde::Serialize)]
pub struct CharacterProfile {
    pub user_id: Id<UserMarker>,
    pub prefix: Option<String>,
    pub name: String,
    pub colour: Option<u32>,
    pub nickname: Option<String>,
    pub sfw_description: String,
    pub sfw_summary: String,
    pub sfw_icon: String,
    pub nsfw_description: Option<String>,
    pub nsfw_summary: Option<String>,
    pub nsfw_icon: Option<String>,
}
