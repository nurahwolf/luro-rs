/// A structure representing an image
#[derive(Clone, Debug, Default, serde::Deserialize, PartialEq, serde::Serialize)]
pub struct CharacterImage {
    /// The URL for the image
    #[serde(default)]
    pub url: String,
    /// If the image is NSFW or not
    #[serde(default)]
    pub nsfw: bool,
    /// If marked as a fav, it can randomly be used as the profile image
    #[serde(default)]
    pub favourite: bool,
    /// The name of the image
    #[serde(default)]
    pub name: String,
    /// The name of the character
    #[serde(default)]
    pub character_name: String,
    /// The ID of the image
    #[serde(default)]
    pub img_id: i64,
    /// The owner of the image
    #[serde(default)]
    pub owner_id: i64,
    /// A URL for the source of an image
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}
