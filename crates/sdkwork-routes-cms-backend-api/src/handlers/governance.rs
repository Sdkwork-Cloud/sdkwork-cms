use sdkwork_content_cms_service::context::CmsRequestContext;
use sdkwork_content_cms_service::domain::*;
use sdkwork_content_cms_service::service::CmsService;

use crate::dto::request::*;
use crate::dto::response::*;
use crate::mapper::{problem, request as req_mapper, response as res_mapper};

pub async fn list_audit_logs(
    service: &CmsService,
    ctx: &CmsRequestContext,
    params: ListAuditLogsQueryParams,
) -> ApiResponse<PaginatedResponse<AuditLogResponse>> {
    let query = req_mapper::map_list_audit_logs_params_to_query(params);
    match service.list_audit_logs(ctx, query).await {
        Ok(page) => {
            let response = res_mapper::map_page_to_paginated_response(page, res_mapper::map_audit_log_to_response);
            ApiResponse::success(response, Some(ctx.request_id.clone()))
        }
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, Some(ctx.request_id.clone())), Some(ctx.request_id.clone())),
    }
}

pub async fn list_outbox_events(
    service: &CmsService,
    ctx: &CmsRequestContext,
    params: ListOutboxEventsQueryParams,
) -> ApiResponse<PaginatedResponse<OutboxEventResponse>> {
    let query = req_mapper::map_list_outbox_events_params_to_query(params);
    match service.list_outbox_events(ctx, query).await {
        Ok(page) => {
            let response = res_mapper::map_page_to_paginated_response(page, res_mapper::map_outbox_event_to_response);
            ApiResponse::success(response, Some(ctx.request_id.clone()))
        }
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, Some(ctx.request_id.clone())), Some(ctx.request_id.clone())),
    }
}

pub async fn retry_outbox_event(
    service: &CmsService,
    ctx: &CmsRequestContext,
    event_id: CmsId,
    _req: RetryOutboxEventRequest,
) -> ApiResponse<CommandResponse> {
    let command = RetryOutboxEventCommand {
        event_id,
        reason: None,
    };
    match service.retry_outbox_event(ctx, command).await {
        Ok(result) => ApiResponse::success(res_mapper::map_command_result_to_response(result), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, Some(ctx.request_id.clone())), Some(ctx.request_id.clone())),
    }
}
