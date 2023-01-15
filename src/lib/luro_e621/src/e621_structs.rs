use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub width: i64,
    pub height: i64,
    pub ext: String,
    pub size: i64,
    pub md5: String,
    pub url: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sample {
    pub has: bool,
    pub height: i32,
    pub width: i32,
    pub url: Option<String>,
    pub alternates: Alternates
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Alternates {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Preview {
    pub width: i32,
    pub height: i32,
    pub url: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Score {
    pub up: i32,
    pub down: i32,
    pub total: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tags {
    pub general: Vec<String>,
    pub species: Vec<String>,
    pub character: Vec<String>,
    pub artist: Vec<String>,
    pub invalid: Vec<String>,
    pub lore: Vec<String>,
    pub meta: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Flags {
    pub pending: bool,
    pub flagged: bool,
    pub note_locked: bool,
    pub status_locked: bool,
    pub rating_locked: bool,
    pub deleted: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Relationships {
    pub parent_id: Option<i64>,
    pub has_children: bool,
    pub has_active_children: bool,
    pub children: Vec<i32>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct E621Posts {
    pub posts: Vec<E621Post>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct E621Post {
    pub id: i32,
    pub created_at: String,
    pub updated_at: String,
    pub file: File,
    pub preview: Preview,
    pub sample: Sample,
    pub score: Score,
    pub tags: Tags,
    pub locked_tags: Vec<String>,
    pub change_seq: i32,
    pub flags: Flags,
    pub rating: String,
    pub fav_count: i32,
    pub sources: Vec<String>,
    pub pools: Vec<i32>,
    pub relationships: Relationships,
    pub approver_id: Option<i32>,
    pub uploader_id: i32,
    pub description: String,
    pub comment_count: i32,
    pub is_favorited: bool,
    pub has_notes: bool,
    pub duration: Option<f64>
}
