use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeliveryListEntriesRequest {
    pub site_code: String,
    pub channel_code: Option<String>,
    pub locale: Option<String>,
    pub content_type_code: Option<String>,
    pub term_code: Option<String>,
    pub cursor: Option<String>,
    pub page_size: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeliveryResolveEntryRequest {
    pub site_code: String,
    pub channel_code: Option<String>,
    pub locale: Option<String>,
    pub slug: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeliveryRetrieveEntryRequest {
    pub entry_id: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeliveryResolvePageRequest {
    pub site_code: String,
    pub channel_code: Option<String>,
    pub locale: Option<String>,
    pub path: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeliveryListFeedItemsRequest {
    pub site_code: String,
    pub feed_code: String,
    pub channel_code: Option<String>,
    pub locale: Option<String>,
    pub cursor: Option<String>,
    pub page_size: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaginationParams {
    pub cursor: Option<String>,
    pub page_size: Option<u32>,
}

impl PaginationParams {
    pub fn effective_limit(&self) -> u32 {
        self.page_size.unwrap_or(20).min(100)
    }
}
