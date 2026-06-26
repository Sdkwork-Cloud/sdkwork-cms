use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeliveryBootstrapResponse {
    pub site: DeliverySiteResponse,
    pub channels: Vec<DeliveryChannelResponse>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeliverySiteResponse {
    pub id: String,
    pub uuid: String,
    pub code: String,
    pub name: String,
    pub default_locale: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeliveryChannelResponse {
    pub id: String,
    pub code: String,
    pub name: String,
    pub channel_kind: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeliveryEntryResponse {
    pub id: String,
    pub uuid: String,
    pub site_id: String,
    pub content_type_id: String,
    pub channel_id: Option<String>,
    pub locale: String,
    pub title: String,
    pub slug: String,
    pub summary: Option<String>,
    pub published_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeliveryPageResponse {
    pub id: String,
    pub site_id: String,
    pub channel_id: Option<String>,
    pub locale: String,
    pub path: String,
    pub title: String,
    pub publication_status: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeliveryFeedItemResponse {
    pub id: String,
    pub feed_id: String,
    pub entry_id: Option<String>,
    pub page_id: Option<String>,
    pub external_url: Option<String>,
    pub item_kind: String,
    pub pinned: bool,
    pub sort_order: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeliveryListResponse<T> {
    pub items: Vec<T>,
    pub next_cursor: Option<String>,
    pub total: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProblemDetailResponse {
    pub r#type: String,
    pub title: String,
    pub status: u16,
    pub detail: String,
    pub instance: Option<String>,
    pub request_id: Option<String>,
}
