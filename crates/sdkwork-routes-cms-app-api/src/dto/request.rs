use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeliveryBootstrapRequest {
    pub site_code: String,
    pub channel_code: Option<String>,
    pub locale: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeliveryResolveEntryRequest {
    pub site_code: String,
    pub channel_code: Option<String>,
    pub locale: Option<String>,
    pub slug: String,
    pub preview_token: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeliveryRetrieveEntryRequest {
    pub entry_id: i64,
    pub preview_token: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeliveryResolvePageRequest {
    pub site_code: String,
    pub channel_code: Option<String>,
    pub locale: Option<String>,
    pub path: String,
    pub preview_token: Option<String>,
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
