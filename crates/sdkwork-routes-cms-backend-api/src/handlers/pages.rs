use sdkwork_content_cms_service::context::CmsRequestContext;
use sdkwork_content_cms_service::domain::*;
use sdkwork_content_cms_service::service::CmsService;

use crate::dto::request::*;
use crate::dto::response::*;
use crate::mapper::{problem, request as req_mapper, response as res_mapper};

pub async fn list_pages(
    service: &CmsService,
    ctx: &CmsRequestContext,
    params: ListPagesQueryParams,
) -> ApiResponse<PaginatedResponse<PageResponse>> {
    let query = req_mapper::map_list_pages_params_to_query(params);
    match service.list_pages(ctx, query).await {
        Ok(page) => {
            let response = res_mapper::map_page_to_paginated_response(page, res_mapper::map_page_to_response);
            ApiResponse::success(response, Some(ctx.request_id.clone()))
        }
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn create_page(
    service: &CmsService,
    ctx: &CmsRequestContext,
    req: PageCreateRequest,
) -> ApiResponse<PageResponse> {
    let command = req_mapper::map_page_create_request_to_command(req);
    match service.create_page(ctx, command).await {
        Ok(page) => ApiResponse::success(res_mapper::map_page_to_response(page), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn retrieve_page(
    service: &CmsService,
    ctx: &CmsRequestContext,
    page_id: CmsId,
) -> ApiResponse<PageResponse> {
    match service.retrieve_page(ctx, page_id).await {
        Ok(page) => ApiResponse::success(res_mapper::map_page_to_response(page), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn update_page(
    service: &CmsService,
    ctx: &CmsRequestContext,
    page_id: CmsId,
    req: PageUpdateRequest,
) -> ApiResponse<PageResponse> {
    let command = req_mapper::map_page_update_request_to_command(page_id, req);
    match service.update_page(ctx, page_id, command).await {
        Ok(page) => ApiResponse::success(res_mapper::map_page_to_response(page), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn delete_page(
    service: &CmsService,
    ctx: &CmsRequestContext,
    page_id: CmsId,
) -> ApiResponse<CommandResponse> {
    match service.delete_page(ctx, page_id).await {
        Ok(result) => ApiResponse::success(res_mapper::map_command_result_to_response(result), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn replace_page_blocks(
    service: &CmsService,
    ctx: &CmsRequestContext,
    page_id: CmsId,
    req: PageBlocksRequest,
) -> ApiResponse<PageResponse> {
    let command = req_mapper::map_page_blocks_request_to_command(page_id, req);
    match service.replace_page_blocks(ctx, command).await {
        Ok(page) => ApiResponse::success(res_mapper::map_page_to_response(page), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn publish_page(
    service: &CmsService,
    ctx: &CmsRequestContext,
    page_id: CmsId,
    req: PublishRequest,
) -> ApiResponse<PublishSnapshotResponse> {
    let command = req_mapper::map_publish_request_to_command("page", page_id, req);
    match service.publish_page(ctx, command).await {
        Ok(snapshot) => ApiResponse::success(res_mapper::map_publish_snapshot_to_response(snapshot), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}
