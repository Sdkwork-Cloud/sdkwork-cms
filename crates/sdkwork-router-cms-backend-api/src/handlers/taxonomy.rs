use sdkwork_content_cms_service::context::CmsRequestContext;
use sdkwork_content_cms_service::domain::*;
use sdkwork_content_cms_service::service::CmsService;

use crate::dto::request::*;
use crate::dto::response::*;
use crate::mapper::{problem, request as req_mapper, response as res_mapper};

pub async fn list_taxonomies(
    service: &CmsService,
    ctx: &CmsRequestContext,
    site_id: CmsId,
    params: ListBySiteQueryParams,
) -> ApiResponse<PaginatedResponse<TaxonomyResponse>> {
    let query = req_mapper::map_list_by_site_params_to_query(site_id, params);
    match service.list_taxonomies(ctx, query).await {
        Ok(page) => {
            let response = res_mapper::map_page_to_paginated_response(page, res_mapper::map_taxonomy_to_response);
            ApiResponse::success(response, Some(ctx.request_id.clone()))
        }
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, Some(ctx.request_id.clone())), Some(ctx.request_id.clone())),
    }
}

pub async fn create_taxonomy(
    service: &CmsService,
    ctx: &CmsRequestContext,
    site_id: CmsId,
    req: TaxonomyCreateRequest,
) -> ApiResponse<TaxonomyResponse> {
    let command = req_mapper::map_taxonomy_create_request_to_command(site_id, req);
    match service.create_taxonomy(ctx, command).await {
        Ok(taxonomy) => ApiResponse::success(res_mapper::map_taxonomy_to_response(taxonomy), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, Some(ctx.request_id.clone())), Some(ctx.request_id.clone())),
    }
}

pub async fn update_taxonomy(
    service: &CmsService,
    ctx: &CmsRequestContext,
    taxonomy_id: CmsId,
    req: TaxonomyUpdateRequest,
) -> ApiResponse<TaxonomyResponse> {
    let command = req_mapper::map_taxonomy_update_request_to_command(req);
    match service.update_taxonomy(ctx, taxonomy_id, command).await {
        Ok(taxonomy) => ApiResponse::success(res_mapper::map_taxonomy_to_response(taxonomy), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, Some(ctx.request_id.clone())), Some(ctx.request_id.clone())),
    }
}

pub async fn delete_taxonomy(
    service: &CmsService,
    ctx: &CmsRequestContext,
    taxonomy_id: CmsId,
) -> ApiResponse<CommandResponse> {
    match service.delete_taxonomy(ctx, taxonomy_id).await {
        Ok(result) => ApiResponse::success(res_mapper::map_command_result_to_response(result), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, Some(ctx.request_id.clone())), Some(ctx.request_id.clone())),
    }
}

pub async fn list_taxonomy_terms(
    service: &CmsService,
    ctx: &CmsRequestContext,
    taxonomy_id: CmsId,
    params: ListBySiteQueryParams,
) -> ApiResponse<PaginatedResponse<TaxonomyTermResponse>> {
    let query = ListTaxonomyTermsQuery {
        taxonomy_id,
        cursor: params.cursor,
        limit: params.limit.unwrap_or(20).min(100),
    };
    match service.list_taxonomy_terms(ctx, query).await {
        Ok(page) => {
            let response = res_mapper::map_page_to_paginated_response(page, res_mapper::map_taxonomy_term_to_response);
            ApiResponse::success(response, Some(ctx.request_id.clone()))
        }
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, Some(ctx.request_id.clone())), Some(ctx.request_id.clone())),
    }
}

pub async fn create_taxonomy_term(
    service: &CmsService,
    ctx: &CmsRequestContext,
    taxonomy_id: CmsId,
    req: TaxonomyTermCreateRequest,
) -> ApiResponse<TaxonomyTermResponse> {
    let command = req_mapper::map_taxonomy_term_create_request_to_command(taxonomy_id, req);
    match service.create_taxonomy_term(ctx, command).await {
        Ok(term) => ApiResponse::success(res_mapper::map_taxonomy_term_to_response(term), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, Some(ctx.request_id.clone())), Some(ctx.request_id.clone())),
    }
}

pub async fn update_taxonomy_term(
    service: &CmsService,
    ctx: &CmsRequestContext,
    term_id: CmsId,
    req: TaxonomyTermUpdateRequest,
) -> ApiResponse<TaxonomyTermResponse> {
    let command = req_mapper::map_taxonomy_term_update_request_to_command(req);
    match service.update_taxonomy_term(ctx, term_id, command).await {
        Ok(term) => ApiResponse::success(res_mapper::map_taxonomy_term_to_response(term), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, Some(ctx.request_id.clone())), Some(ctx.request_id.clone())),
    }
}

pub async fn delete_taxonomy_term(
    service: &CmsService,
    ctx: &CmsRequestContext,
    term_id: CmsId,
) -> ApiResponse<CommandResponse> {
    match service.delete_taxonomy_term(ctx, term_id).await {
        Ok(result) => ApiResponse::success(res_mapper::map_command_result_to_response(result), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, Some(ctx.request_id.clone())), Some(ctx.request_id.clone())),
    }
}
