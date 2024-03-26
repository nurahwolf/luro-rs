pub struct Image {
    pub img_id: i64,
    pub name: String,
    pub nsfw: bool,
    pub owner_id: i64,
    pub source: Option<String>,
    pub url: String,
}
