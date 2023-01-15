use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Header {
    pub user_id: Option<String>,
    pub account_type: Option<String>,
    pub short_limit: Option<String>,
    pub long_limit: Option<String>,
    pub long_remaining: Option<u64>,
    pub short_remaining: Option<u64>,
    pub status: i64,
    pub results_requested: Option<String>,
    pub index: Option<HashMap<String, Index>>,
    pub search_depth: Option<String>,
    pub minimum_similarity: Option<f64>,
    pub query_image_display: Option<String>,
    pub query_image: Option<String>,
    pub results_returned: Option<u64>,
    pub message: Option<String>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Index {
    pub status: u64,
    pub parent_id: u64,
    pub id: u64,
    pub results: Option<u64>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HeaderResult {
    pub similarity: String,
    pub thumbnail: String,
    pub index_id: u64,
    pub index_name: String,
    pub dupes: u64,
    pub hidden: u64
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Data {
    pub ext_urls: Vec<String>,
    pub title: Option<String>,
    pub pixiv_id: Option<u64>,
    pub member_name: Option<String>,
    pub member_id: Option<u64>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Results {
    pub header: HeaderResult,
    pub data: Data
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SauceNAO {
    pub header: Header,
    pub results: Option<Vec<Results>>
}
