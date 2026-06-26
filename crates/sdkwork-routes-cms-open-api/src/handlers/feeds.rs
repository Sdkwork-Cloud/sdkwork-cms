use sdkwork_content_cms_service::context::CmsRequestContext;
use sdkwork_content_cms_service::domain::*;
use sdkwork_content_cms_service::service::CmsService;

use super::{ApiResponse, OpenApiFeedItemResponse, OpenApiFeedItemsParams, PaginatedResponse};

fn map_feed_item(item: &CmsFeedItem) -> OpenApiFeedItemResponse {
    OpenApiFeedItemResponse {
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

pub async fn list_feed_items(
    service: &CmsService,
    ctx: &CmsRequestContext,
    feed_code: String,
    params: OpenApiFeedItemsParams,
) -> ApiResponse<PaginatedResponse<OpenApiFeedItemResponse>> {
    let query = DeliveryFeedItemsQuery {
        site_code: params.site_code,
        feed_code,
        channel_code: params.channel_code,
        locale: params.locale,
        cursor: params.cursor,
        limit: params.limit.unwrap_or(20).min(100),
    };
    match service.delivery_list_feed_items(ctx, query).await {
        Ok(page) => ApiResponse::success(PaginatedResponse {
            items: page.items.iter().map(map_feed_item).collect(),
            next_cursor: page.next_cursor,
        }),
        Err(err) => ApiResponse::error_detail(&err.to_string()),
    }
}
