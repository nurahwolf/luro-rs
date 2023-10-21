#[derive(Clone)]
pub struct LuroCharacterImage {
    /// Character name
    pub character_name: String,
    /// The image ID
    pub img_id: i64,
    /// The owner of the image
    pub owner_id: i64,
    /// The URL for the image
    pub url: String,
    /// If the image is NSFW or not
    pub nsfw: bool,
    /// If marked as a fav, it can randomly be used as the profile image
    pub favourite: bool,
    /// The name of the image
    pub name: String,
    /// A URL for the source of an image
    pub source: Option<String>,
}
