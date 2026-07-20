use sdkwork_content_cms_service::context::CmsRequestContext;
use sdkwork_content_cms_service::domain::*;
use sdkwork_content_cms_service::service::CmsService;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeliveryBootstrapParams {
    pub channel_code: Option<String>,
    pub locale: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeliveryEntryResolveParams {
    pub channel_code: Option<String>,
    pub locale: Option<String>,
    pub slug: String,
    pub preview_token: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeliveryEntryRetrieveParams {
    pub preview_token: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeliveryPageResolveParams {
    pub channel_code: Option<String>,
    pub locale: Option<String>,
    pub path: String,
    pub preview_token: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeliveryFeedItemsParams {
    pub channel_code: Option<String>,
    pub locale: Option<String>,
    pub cursor: Option<String>,
    pub page_size: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiResponse<T: Serialize> {
    pub ok: bool,
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            ok: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error_detail(msg: &str) -> Self {
        Self {
            ok: false,
            data: None,
            error: Some(serde_json::json!({
                "detail": msg,
            })),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeliveryBootstrapResponse {
    pub site: DeliverySiteResponse,
    pub channels: Vec<DeliveryChannelResponse>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeliverySiteResponse {
    pub id: String,
    pub code: String,
    pub name: String,
    pub default_locale: String,
    pub settings_json: String,
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
    pub locale: String,
    pub title: String,
    pub slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    pub published_at: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeliveryPageResponse {
    pub id: String,
    pub site_id: String,
    pub locale: String,
    pub path: String,
    pub title: String,
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
pub struct PaginatedResponse<T: Serialize> {
    pub items: Vec<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

fn map_delivery_site(site: &CmsSite) -> DeliverySiteResponse {
    DeliverySiteResponse {
        id: site.id.to_string(),
        code: site.code.clone(),
        name: site.name.clone(),
        default_locale: site.default_locale.clone(),
        settings_json: site.settings_json.clone(),
    }
}

fn map_delivery_channel(channel: &CmsChannel) -> DeliveryChannelResponse {
    DeliveryChannelResponse {
        id: channel.id.to_string(),
        code: channel.code.clone(),
        name: channel.name.clone(),
        channel_kind: channel.channel_kind.clone(),
    }
}

fn map_delivery_entry(entry: &CmsEntry) -> DeliveryEntryResponse {
    DeliveryEntryResponse {
        id: entry.id.to_string(),
        uuid: entry.uuid.clone(),
        site_id: entry.site_id.to_string(),
        content_type_id: entry.content_type_id.to_string(),
        locale: entry.locale.clone(),
        title: entry.title.clone(),
        slug: entry.slug.clone(),
        summary: entry.summary.clone(),
        published_at: entry.published_at.clone(),
    }
}

fn map_delivery_page(page: &CmsPageModel) -> DeliveryPageResponse {
    DeliveryPageResponse {
        id: page.id.to_string(),
        site_id: page.site_id.to_string(),
        locale: page.locale.clone(),
        path: page.path.clone(),
        title: page.title.clone(),
    }
}

fn map_delivery_feed_item(item: &CmsFeedItem) -> DeliveryFeedItemResponse {
    DeliveryFeedItemResponse {
        id: item.id.to_string(),
        feed_id: item.feed_id.to_string(),
        entry_id: item.entry_id.map(|id| id.to_string()),
        page_id: item.page_id.map(|id| id.to_string()),
        external_url: item.external_url.clone(),
        item_kind: item.item_kind.clone(),
        pinned: item.pinned,
        sort_order: item.sort_order,
    }
}

pub async fn retrieve_bootstrap(
    service: &CmsService,
    ctx: &CmsRequestContext,
    site_code: String,
    params: DeliveryBootstrapParams,
) -> ApiResponse<DeliveryBootstrapResponse> {
    let query = DeliveryBootstrapQuery {
        site_code,
        channel_code: params.channel_code,
        locale: params.locale,
    };
    match service.delivery_bootstrap(ctx, query).await {
        Ok(bootstrap) => ApiResponse::success(DeliveryBootstrapResponse {
            site: map_delivery_site(&bootstrap.site),
            channels: bootstrap
                .channels
                .iter()
                .map(map_delivery_channel)
                .collect(),
        }),
        Err(err) => ApiResponse::error_detail(&err.to_string()),
    }
}

pub async fn resolve_entry(
    service: &CmsService,
    ctx: &CmsRequestContext,
    site_code: String,
    params: DeliveryEntryResolveParams,
) -> ApiResponse<DeliveryEntryResponse> {
    let query = DeliveryResolveEntryQuery {
        site_code,
        channel_code: params.channel_code,
        locale: params.locale,
        slug: params.slug,
        preview_token: params.preview_token,
    };
    match service.delivery_resolve_entry(ctx, query).await {
        Ok(entry) => ApiResponse::success(map_delivery_entry(&entry)),
        Err(err) => ApiResponse::error_detail(&err.to_string()),
    }
}

pub async fn retrieve_entry(
    service: &CmsService,
    ctx: &CmsRequestContext,
    entry_id: CmsId,
    params: DeliveryEntryRetrieveParams,
) -> ApiResponse<DeliveryEntryResponse> {
    let query = DeliveryRetrieveEntryQuery {
        entry_id,
        preview_token: params.preview_token,
    };
    match service.delivery_retrieve_entry(ctx, query).await {
        Ok(entry) => ApiResponse::success(map_delivery_entry(&entry)),
        Err(err) => ApiResponse::error_detail(&err.to_string()),
    }
}

pub async fn resolve_page(
    service: &CmsService,
    ctx: &CmsRequestContext,
    site_code: String,
    params: DeliveryPageResolveParams,
) -> ApiResponse<DeliveryPageResponse> {
    let query = DeliveryResolvePageQuery {
        site_code,
        channel_code: params.channel_code,
        locale: params.locale,
        path: params.path,
        preview_token: params.preview_token,
    };
    match service.delivery_resolve_page(ctx, query).await {
        Ok(page) => ApiResponse::success(map_delivery_page(&page)),
        Err(err) => ApiResponse::error_detail(&err.to_string()),
    }
}

pub async fn list_feed_items(
    service: &CmsService,
    ctx: &CmsRequestContext,
    site_code: String,
    feed_code: String,
    params: DeliveryFeedItemsParams,
) -> ApiResponse<PaginatedResponse<DeliveryFeedItemResponse>> {
    let query = DeliveryFeedItemsQuery {
        site_code,
        feed_code,
        channel_code: params.channel_code,
        locale: params.locale,
        cursor: params.cursor,
        limit: params.page_size.unwrap_or(20).min(100),
    };
    match service.delivery_list_feed_items(ctx, query).await {
        Ok(page) => ApiResponse::success(PaginatedResponse {
            items: page.items.iter().map(map_delivery_feed_item).collect(),
            next_cursor: page.next_cursor,
        }),
        Err(err) => ApiResponse::error_detail(&err.to_string()),
    }
}
