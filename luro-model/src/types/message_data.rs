use twilight_model::gateway::payload::incoming::MessageUpdate;

#[derive(Clone, Debug, serde::Deserialize, PartialEq, serde::Serialize)]
pub struct MessageData {
    /// Present if the message has been updated with new content, containing said new content
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_content: Option<Vec<MessageUpdate>>,
    /// Has the message been marked as deleted in the database
    #[serde(default)]
    pub deleted: bool,
}
