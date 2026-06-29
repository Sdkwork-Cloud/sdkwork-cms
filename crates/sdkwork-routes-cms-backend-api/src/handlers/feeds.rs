use sdkwork_content_cms_service::context::CmsRequestContext;
use sdkwork_content_cms_service::domain::*;
use sdkwork_content_cms_service::service::CmsService;

use crate::dto::request::*;
use crate::dto::response::*;
use crate::mapper::{problem, request as req_mapper, response as res_mapper};

pub async fn list_feeds(
    service: &CmsService,
    ctx: &CmsRequestContext,
    params: ListFeedsQueryParams,
) -> ApiResponse<PaginatedResponse<FeedResponse>> {
    let query = req_mapper::map_list_feeds_params_to_query(params);
    match service.list_feeds(ctx, query).await {
        Ok(page) => {
            let response = res_mapper::map_page_to_paginated_response(page, res_mapper::map_feed_to_response);
            ApiResponse::success(response, Some(ctx.request_id.clone()))
        }
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn create_feed(
    service: &CmsService,
    ctx: &CmsRequestContext,
    req: FeedCreateRequest,
) -> ApiResponse<FeedResponse> {
    let command = req_mapper::map_feed_create_request_to_command(req);
    match service.create_feed(ctx, command).await {
        Ok(feed) => ApiResponse::success(res_mapper::map_feed_to_response(feed), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn retrieve_feed(
    service: &CmsService,
    ctx: &CmsRequestContext,
    feed_id: CmsId,
) -> ApiResponse<FeedResponse> {
    match service.retrieve_feed(ctx, feed_id).await {
        Ok(feed) => ApiResponse::success(res_mapper::map_feed_to_response(feed), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn update_feed(
    service: &CmsService,
    ctx: &CmsRequestContext,
    feed_id: CmsId,
    req: FeedUpdateRequest,
) -> ApiResponse<FeedResponse> {
    let command = req_mapper::map_feed_update_request_to_command(feed_id, req);
    match service.update_feed(ctx, feed_id, command).await {
        Ok(feed) => ApiResponse::success(res_mapper::map_feed_to_response(feed), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn delete_feed(
    service: &CmsService,
    ctx: &CmsRequestContext,
    feed_id: CmsId,
) -> ApiResponse<CommandResponse> {
    match service.delete_feed(ctx, feed_id).await {
        Ok(result) => ApiResponse::success(res_mapper::map_command_result_to_response(result), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn list_feed_rules(
    service: &CmsService,
    ctx: &CmsRequestContext,
    feed_id: CmsId,
    params: ListFeedRulesQueryParams,
) -> ApiResponse<PaginatedResponse<FeedRuleResponse>> {
    let query = req_mapper::map_list_feed_rules_params_to_query(feed_id, params);
    match service.list_feed_rules(ctx, query).await {
        Ok(page) => {
            let response = res_mapper::map_page_to_paginated_response(page, res_mapper::map_feed_rule_to_response);
            ApiResponse::success(response, Some(ctx.request_id.clone()))
        }
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn create_feed_rule(
    service: &CmsService,
    ctx: &CmsRequestContext,
    feed_id: CmsId,
    req: FeedRuleCreateRequest,
) -> ApiResponse<FeedRuleResponse> {
    let command = req_mapper::map_feed_rule_create_request_to_command(feed_id, req);
    match service.create_feed_rule(ctx, command).await {
        Ok(rule) => ApiResponse::success(res_mapper::map_feed_rule_to_response(rule), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn update_feed_rule(
    service: &CmsService,
    ctx: &CmsRequestContext,
    rule_id: CmsId,
    req: FeedRuleUpdateRequest,
) -> ApiResponse<FeedRuleResponse> {
    let command = req_mapper::map_feed_rule_update_request_to_command(req);
    match service.update_feed_rule(ctx, rule_id, command).await {
        Ok(rule) => ApiResponse::success(res_mapper::map_feed_rule_to_response(rule), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn delete_feed_rule(
    service: &CmsService,
    ctx: &CmsRequestContext,
    rule_id: CmsId,
) -> ApiResponse<CommandResponse> {
    match service.delete_feed_rule(ctx, rule_id).await {
        Ok(result) => ApiResponse::success(res_mapper::map_command_result_to_response(result), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn list_feed_items(
    service: &CmsService,
    ctx: &CmsRequestContext,
    feed_id: CmsId,
    params: ListFeedItemsQueryParams,
) -> ApiResponse<PaginatedResponse<FeedItemResponse>> {
    let query = req_mapper::map_list_feed_items_params_to_query(feed_id, params);
    match service.list_feed_items(ctx, query).await {
        Ok(page) => {
            let response = res_mapper::map_page_to_paginated_response(page, res_mapper::map_feed_item_to_response);
            ApiResponse::success(response, Some(ctx.request_id.clone()))
        }
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn upsert_feed_items(
    service: &CmsService,
    ctx: &CmsRequestContext,
    feed_id: CmsId,
    req: FeedItemsUpsertRequest,
) -> ApiResponse<PaginatedResponse<FeedItemResponse>> {
    let command = req_mapper::map_feed_items_upsert_request_to_command(feed_id, req);
    match service.upsert_feed_items(ctx, command).await {
        Ok(page) => {
            let response = res_mapper::map_page_to_paginated_response(page, res_mapper::map_feed_item_to_response);
            ApiResponse::success(response, Some(ctx.request_id.clone()))
        }
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn delete_feed_item(
    service: &CmsService,
    ctx: &CmsRequestContext,
    item_id: CmsId,
) -> ApiResponse<CommandResponse> {
    match service.delete_feed_item(ctx, item_id).await {
        Ok(result) => ApiResponse::success(res_mapper::map_command_result_to_response(result), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn publish_feed(
    service: &CmsService,
    ctx: &CmsRequestContext,
    feed_id: CmsId,
    req: PublishRequest,
) -> ApiResponse<PublishSnapshotResponse> {
    let command = req_mapper::map_publish_request_to_command("feed", feed_id, req);
    match service.publish_feed(ctx, command).await {
        Ok(snapshot) => ApiResponse::success(res_mapper::map_publish_snapshot_to_response(snapshot), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn retrieve_feed_snapshot(
    service: &CmsService,
    ctx: &CmsRequestContext,
    snapshot_id: CmsId,
) -> ApiResponse<FeedSnapshotResponse> {
    match service.retrieve_feed_snapshot(ctx, snapshot_id).await {
        Ok(snapshot) => ApiResponse::success(res_mapper::map_feed_snapshot_to_response(snapshot), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}
