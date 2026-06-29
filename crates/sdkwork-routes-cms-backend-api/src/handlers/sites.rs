use sdkwork_content_cms_service::context::CmsRequestContext;
use sdkwork_content_cms_service::domain::*;
use sdkwork_content_cms_service::service::CmsService;

use crate::dto::request::*;
use crate::dto::response::*;
use crate::mapper::{problem, request as req_mapper, response as res_mapper};

pub async fn list_sites(
    service: &CmsService,
    ctx: &CmsRequestContext,
    params: ListSitesQueryParams,
) -> ApiResponse<PaginatedResponse<SiteResponse>> {
    let query = req_mapper::map_list_sites_params_to_query(params);
    match service.list_sites(ctx, query).await {
        Ok(page) => {
            let response = res_mapper::map_page_to_paginated_response(page, res_mapper::map_site_to_response);
            ApiResponse::success(response, Some(ctx.request_id.clone()))
        }
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn create_site(
    service: &CmsService,
    ctx: &CmsRequestContext,
    req: SiteCreateRequest,
) -> ApiResponse<SiteResponse> {
    let command = req_mapper::map_site_create_request_to_command(req);
    match service.create_site(ctx, command).await {
        Ok(site) => ApiResponse::success(res_mapper::map_site_to_response(site), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn retrieve_site(
    service: &CmsService,
    ctx: &CmsRequestContext,
    site_id: CmsId,
) -> ApiResponse<SiteResponse> {
    match service.retrieve_site(ctx, site_id).await {
        Ok(site) => ApiResponse::success(res_mapper::map_site_to_response(site), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn update_site(
    service: &CmsService,
    ctx: &CmsRequestContext,
    site_id: CmsId,
    req: SiteUpdateRequest,
) -> ApiResponse<SiteResponse> {
    let command = req_mapper::map_site_update_request_to_command(req);
    match service.update_site(ctx, site_id, command).await {
        Ok(site) => ApiResponse::success(res_mapper::map_site_to_response(site), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn delete_site(
    service: &CmsService,
    ctx: &CmsRequestContext,
    site_id: CmsId,
) -> ApiResponse<CommandResponse> {
    match service.delete_site(ctx, site_id).await {
        Ok(result) => ApiResponse::success(res_mapper::map_command_result_to_response(result), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn list_channels(
    service: &CmsService,
    ctx: &CmsRequestContext,
    site_id: CmsId,
    params: ListBySiteQueryParams,
) -> ApiResponse<PaginatedResponse<ChannelResponse>> {
    let query = req_mapper::map_list_by_site_params_to_query(site_id, params);
    match service.list_channels(ctx, query).await {
        Ok(page) => {
            let response = res_mapper::map_page_to_paginated_response(page, res_mapper::map_channel_to_response);
            ApiResponse::success(response, Some(ctx.request_id.clone()))
        }
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn create_channel(
    service: &CmsService,
    ctx: &CmsRequestContext,
    site_id: CmsId,
    req: ChannelCreateRequest,
) -> ApiResponse<ChannelResponse> {
    let command = req_mapper::map_channel_create_request_to_command(site_id, req);
    match service.create_channel(ctx, command).await {
        Ok(channel) => ApiResponse::success(res_mapper::map_channel_to_response(channel), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn update_channel(
    service: &CmsService,
    ctx: &CmsRequestContext,
    channel_id: CmsId,
    req: ChannelUpdateRequest,
) -> ApiResponse<ChannelResponse> {
    let command = req_mapper::map_channel_update_request_to_command(req);
    match service.update_channel(ctx, channel_id, command).await {
        Ok(channel) => ApiResponse::success(res_mapper::map_channel_to_response(channel), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn delete_channel(
    service: &CmsService,
    ctx: &CmsRequestContext,
    channel_id: CmsId,
) -> ApiResponse<CommandResponse> {
    match service.delete_channel(ctx, channel_id).await {
        Ok(result) => ApiResponse::success(res_mapper::map_command_result_to_response(result), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}
