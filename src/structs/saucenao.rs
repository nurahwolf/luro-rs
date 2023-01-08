use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Header {
    pub user_id: String,
    pub account_type: String,
    pub short_limit: String,
    pub long_limit: String,
    pub long_remaining: u64,
    pub short_remaining: u64,
    pub status: u64,
    pub results_requested: String,
    pub index: HashMap<String, Index>,
    pub search_depth: String,
    pub minimum_similarity: f64,
    pub query_image_display: String,
    pub query_image: String,
    pub results_returned: u64
}

#[derive(Serialize, Deserialize)]
pub struct Index {
    pub status: u64,
    pub parent_id: u64,
    pub id: u64,
    pub results: Option<u64>
}

#[derive(Serialize, Deserialize)]
pub struct HeaderResult {
    pub similarity: String,
    pub thumbnail: String,
    pub index_id: u64,
    pub index_name: String,
    pub dupes: u64,
    pub hidden: u64
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub ext_urls: Vec<String>,
    pub title: Option<String>,
    pub pixiv_id: Option<u64>,
    pub member_name: Option<String>,
    pub member_id: Option<u64>
}

#[derive(Serialize, Deserialize)]
pub struct Results {
    pub header: HeaderResult,
    pub data: Data
}

#[derive(Serialize, Deserialize)]
pub struct SauceNAO {
    pub header: Header,
    pub results: Vec<Results>
}