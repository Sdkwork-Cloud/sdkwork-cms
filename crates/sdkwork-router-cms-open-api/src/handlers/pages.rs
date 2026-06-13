use sdkwork_content_cms_service::context::CmsRequestContext;
use sdkwork_content_cms_service::domain::*;
use sdkwork_content_cms_service::service::CmsService;

use super::{ApiResponse, OpenApiPageResponse, OpenApiPageResolveParams};

fn map_page(page: &CmsPageModel) -> OpenApiPageResponse {
    OpenApiPageResponse {
        id: page.id.to_string(),
        site_id: page.site_id.to_string(),
        locale: page.locale.clone(),
        path: page.path.clone(),
        title: page.title.clone(),
    }
}

pub async fn resolve_page(
    service: &CmsService,
    ctx: &CmsRequestContext,
    params: OpenApiPageResolveParams,
) -> ApiResponse<OpenApiPageResponse> {
    let query = DeliveryResolvePageQuery {
        site_code: params.site_code,
        channel_code: params.channel_code,
        locale: params.locale,
        path: params.path,
        preview_token: None,
    };
    match service.delivery_resolve_page(ctx, query).await {
        Ok(page) => ApiResponse::success(map_page(&page)),
        Err(err) => ApiResponse::error_detail(&err.to_string()),
    }
}
