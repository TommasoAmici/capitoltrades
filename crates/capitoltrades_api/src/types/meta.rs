use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Meta {
    paging: Paging,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paging {
    page: i64,
    size: i64,
    total_items: i64,
    total_pages: i64,
}

#[derive(Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    meta: Meta,
    pub data: Vec<T>,
}

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    pub data: T,
}
