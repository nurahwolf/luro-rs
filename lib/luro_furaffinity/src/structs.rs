use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Cookies {
    pub name: String,
    pub value: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBody {
    pub cookies: Vec<Cookies>,
    pub bbcode: bool
}

#[derive(Serialize, Deserialize)]
pub struct Author {
    pub avatar_url: String,
    pub join_date: String,
    pub name: String,
    pub status: String,
    pub title: String
}

#[derive(Serialize, Deserialize)]
pub struct Replies {
    pub id: i64,
    pub author: Author,
    pub date: String,
    pub text: String,
    pub replies: Vec<Replies>
}

#[derive(Serialize, Deserialize)]
pub struct Comments {
    pub author: Author,
    pub date: String,
    pub edited: bool,
    pub hidden: bool,
    pub id: i64,
    pub replies: Vec<Replies>,
    pub reply_to: Option<i64>,
    pub text: String
}

#[derive(Serialize, Deserialize)]
pub struct Stats {
    pub comments: i64,
    pub favorites: i64,
    pub views: i64
}

#[derive(Serialize, Deserialize)]
pub struct UserFolders {
    pub group: String,
    pub name: String,
    pub url: String
}

#[derive(Serialize, Deserialize)]
pub struct FurAffinity {
    pub author: Author,
    pub category: String,
    pub comments: Vec<Comments>,
    pub date: String,
    pub description: String,
    pub favorite: bool,
    pub favorite_toggle_link: String,
    pub file_url: String,
    pub folder: String,
    pub footer: String,
    pub gender: String,
    pub id: i64,
    pub mentions: Vec<String>,
    pub next: Option<i64>,
    pub prev: Option<i64>,
    pub rating: String,
    pub species: String,
    pub stats: Stats,
    pub tags: Vec<String>,
    pub thumbnail_url: String,
    pub title: String,
    pub r#type: String,
    pub user_folders: Vec<UserFolders>
}
