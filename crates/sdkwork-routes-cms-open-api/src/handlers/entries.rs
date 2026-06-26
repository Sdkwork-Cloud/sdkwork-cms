use sdkwork_content_cms_service::context::CmsRequestContext;
use sdkwork_content_cms_service::domain::*;
use sdkwork_content_cms_service::service::CmsService;

use super::{ApiResponse, OpenApiEntryResponse, OpenApiListEntriesParams, PaginatedResponse, map_entry};

pub async fn list_entries(
    service: &CmsService,
    ctx: &CmsRequestContext,
    params: OpenApiListEntriesParams,
) -> ApiResponse<PaginatedResponse<OpenApiEntryResponse>> {
    let query = DeliveryListEntriesQuery {
        site_code: params.site_code,
        channel_code: params.channel_code,
        locale: params.locale,
        content_type_code: params.content_type_code,
        term_code: params.term_code,
        cursor: params.cursor,
        limit: params.limit.unwrap_or(20).min(100),
    };
    match service.delivery_list_entries(ctx, query).await {
        Ok(page) => ApiResponse::success(PaginatedResponse {
            items: page.items.iter().map(map_entry).collect(),
            next_cursor: page.next_cursor,
        }),
        Err(err) => ApiResponse::error_detail(&err.to_string()),
    }
}

pub async fn resolve_entry(
    service: &CmsService,
    ctx: &CmsRequestContext,
    params: super::OpenApiEntryResolveParams,
) -> ApiResponse<OpenApiEntryResponse> {
    let query = DeliveryResolveEntryQuery {
        site_code: params.site_code,
        channel_code: params.channel_code,
        locale: params.locale,
        slug: params.slug,
        preview_token: None,
    };
    match service.delivery_resolve_entry(ctx, query).await {
        Ok(entry) => ApiResponse::success(map_entry(&entry)),
        Err(err) => ApiResponse::error_detail(&err.to_string()),
    }
}

pub async fn retrieve_entry(
    service: &CmsService,
    ctx: &CmsRequestContext,
    entry_id: CmsId,
) -> ApiResponse<OpenApiEntryResponse> {
    let query = DeliveryRetrieveEntryQuery {
        entry_id,
        preview_token: None,
    };
    match service.delivery_retrieve_entry(ctx, query).await {
        Ok(entry) => ApiResponse::success(map_entry(&entry)),
        Err(err) => ApiResponse::error_detail(&err.to_string()),
    }
}
