use sdkwork_content_cms_service::context::CmsRequestContext;
use sdkwork_content_cms_service::domain::*;
use sdkwork_content_cms_service::service::CmsService;

use crate::dto::request::*;
use crate::dto::response::*;
use crate::mapper::{problem, request as req_mapper, response as res_mapper};

pub async fn list_content_types(
    service: &CmsService,
    ctx: &CmsRequestContext,
    site_id: CmsId,
    params: ListBySiteQueryParams,
) -> ApiResponse<PaginatedResponse<ContentTypeResponse>> {
    let query = req_mapper::map_list_by_site_params_to_query(site_id, params);
    match service.list_content_types(ctx, query).await {
        Ok(page) => {
            let response = res_mapper::map_page_to_paginated_response(page, res_mapper::map_content_type_to_response);
            ApiResponse::success(response, Some(ctx.request_id.clone()))
        }
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, Some(ctx.request_id.clone())), Some(ctx.request_id.clone())),
    }
}

pub async fn create_content_type(
    service: &CmsService,
    ctx: &CmsRequestContext,
    site_id: CmsId,
    req: ContentTypeCreateRequest,
) -> ApiResponse<ContentTypeResponse> {
    let command = req_mapper::map_content_type_create_request_to_command(site_id, req);
    match service.create_content_type(ctx, command).await {
        Ok(ct) => ApiResponse::success(res_mapper::map_content_type_to_response(ct), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, Some(ctx.request_id.clone())), Some(ctx.request_id.clone())),
    }
}

pub async fn retrieve_content_type(
    service: &CmsService,
    ctx: &CmsRequestContext,
    content_type_id: CmsId,
) -> ApiResponse<ContentTypeResponse> {
    match service.retrieve_content_type(ctx, content_type_id).await {
        Ok(ct) => ApiResponse::success(res_mapper::map_content_type_to_response(ct), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, Some(ctx.request_id.clone())), Some(ctx.request_id.clone())),
    }
}

pub async fn update_content_type(
    service: &CmsService,
    ctx: &CmsRequestContext,
    content_type_id: CmsId,
    req: ContentTypeUpdateRequest,
) -> ApiResponse<ContentTypeResponse> {
    let command = req_mapper::map_content_type_update_request_to_command(req);
    match service.update_content_type(ctx, content_type_id, command).await {
        Ok(ct) => ApiResponse::success(res_mapper::map_content_type_to_response(ct), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, Some(ctx.request_id.clone())), Some(ctx.request_id.clone())),
    }
}

pub async fn delete_content_type(
    service: &CmsService,
    ctx: &CmsRequestContext,
    content_type_id: CmsId,
) -> ApiResponse<CommandResponse> {
    match service.delete_content_type(ctx, content_type_id).await {
        Ok(result) => ApiResponse::success(res_mapper::map_command_result_to_response(result), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, Some(ctx.request_id.clone())), Some(ctx.request_id.clone())),
    }
}

pub async fn list_content_fields(
    service: &CmsService,
    ctx: &CmsRequestContext,
    content_type_id: CmsId,
    params: ListBySiteQueryParams,
) -> ApiResponse<PaginatedResponse<ContentFieldResponse>> {
    let query = ListContentFieldsQuery {
        content_type_id,
        cursor: params.cursor,
        limit: params.limit.unwrap_or(20).min(100),
    };
    match service.list_content_fields(ctx, query).await {
        Ok(page) => {
            let response = res_mapper::map_page_to_paginated_response(page, res_mapper::map_content_field_to_response);
            ApiResponse::success(response, Some(ctx.request_id.clone()))
        }
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, Some(ctx.request_id.clone())), Some(ctx.request_id.clone())),
    }
}

pub async fn create_content_field(
    service: &CmsService,
    ctx: &CmsRequestContext,
    content_type_id: CmsId,
    req: ContentFieldCreateRequest,
) -> ApiResponse<ContentFieldResponse> {
    let command = req_mapper::map_content_field_create_request_to_command(content_type_id, req);
    match service.create_content_field(ctx, command).await {
        Ok(field) => ApiResponse::success(res_mapper::map_content_field_to_response(field), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, Some(ctx.request_id.clone())), Some(ctx.request_id.clone())),
    }
}

pub async fn update_content_field(
    service: &CmsService,
    ctx: &CmsRequestContext,
    field_id: CmsId,
    req: ContentFieldUpdateRequest,
) -> ApiResponse<ContentFieldResponse> {
    let command = req_mapper::map_content_field_update_request_to_command(req);
    match service.update_content_field(ctx, field_id, command).await {
        Ok(field) => ApiResponse::success(res_mapper::map_content_field_to_response(field), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, Some(ctx.request_id.clone())), Some(ctx.request_id.clone())),
    }
}

pub async fn delete_content_field(
    service: &CmsService,
    ctx: &CmsRequestContext,
    field_id: CmsId,
) -> ApiResponse<CommandResponse> {
    match service.delete_content_field(ctx, field_id).await {
        Ok(result) => ApiResponse::success(res_mapper::map_command_result_to_response(result), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, Some(ctx.request_id.clone())), Some(ctx.request_id.clone())),
    }
}
