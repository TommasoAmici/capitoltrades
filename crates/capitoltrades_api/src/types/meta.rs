use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Meta {
    #[serde(rename = "paging")]
    paging: Paging,
}

#[derive(Serialize, Deserialize)]
pub struct Paging {
    #[serde(rename = "page")]
    page: i64,

    #[serde(rename = "size")]
    size: i64,

    #[serde(rename = "totalItems")]
    total_items: i64,

    #[serde(rename = "totalPages")]
    total_pages: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    #[serde(rename = "meta")]
    meta: Meta,

    #[serde(rename = "data")]
    pub data: Vec<T>,
}
