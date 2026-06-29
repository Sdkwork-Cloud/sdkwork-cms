use sdkwork_content_cms_service::context::CmsRequestContext;
use sdkwork_content_cms_service::domain::*;
use sdkwork_content_cms_service::service::CmsService;

use crate::dto::request::*;
use crate::dto::response::*;
use crate::mapper::{problem, request as req_mapper, response as res_mapper};

pub async fn list_entries(
    service: &CmsService,
    ctx: &CmsRequestContext,
    params: ListEntriesQueryParams,
) -> ApiResponse<PaginatedResponse<EntryResponse>> {
    let query = req_mapper::map_list_entries_params_to_query(params);
    match service.list_entries(ctx, query).await {
        Ok(page) => {
            let response = res_mapper::map_page_to_paginated_response(page, res_mapper::map_entry_to_response);
            ApiResponse::success(response, Some(ctx.request_id.clone()))
        }
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn create_entry(
    service: &CmsService,
    ctx: &CmsRequestContext,
    req: EntryCreateRequest,
) -> ApiResponse<EntryResponse> {
    let command = req_mapper::map_entry_create_request_to_command(req);
    match service.create_entry(ctx, command).await {
        Ok(entry) => ApiResponse::success(res_mapper::map_entry_to_response(entry), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn retrieve_entry(
    service: &CmsService,
    ctx: &CmsRequestContext,
    entry_id: CmsId,
) -> ApiResponse<EntryResponse> {
    match service.retrieve_entry(ctx, entry_id).await {
        Ok(entry) => ApiResponse::success(res_mapper::map_entry_to_response(entry), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn update_entry(
    service: &CmsService,
    ctx: &CmsRequestContext,
    entry_id: CmsId,
    req: EntryUpdateRequest,
) -> ApiResponse<EntryResponse> {
    let command = req_mapper::map_entry_update_request_to_command(entry_id, req);
    match service.update_entry(ctx, entry_id, command).await {
        Ok(entry) => ApiResponse::success(res_mapper::map_entry_to_response(entry), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn delete_entry(
    service: &CmsService,
    ctx: &CmsRequestContext,
    entry_id: CmsId,
) -> ApiResponse<CommandResponse> {
    match service.delete_entry(ctx, entry_id).await {
        Ok(result) => ApiResponse::success(res_mapper::map_command_result_to_response(result), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn replace_entry_body(
    service: &CmsService,
    ctx: &CmsRequestContext,
    entry_id: CmsId,
    req: EntryBodyRequest,
) -> ApiResponse<EntryResponse> {
    let command = req_mapper::map_entry_body_request_to_command(entry_id, req);
    match service.replace_entry_body(ctx, command).await {
        Ok(entry) => ApiResponse::success(res_mapper::map_entry_to_response(entry), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn replace_entry_fields(
    service: &CmsService,
    ctx: &CmsRequestContext,
    entry_id: CmsId,
    req: EntryFieldsRequest,
) -> ApiResponse<EntryResponse> {
    let command = req_mapper::map_entry_fields_request_to_command(entry_id, req);
    match service.replace_entry_fields(ctx, command).await {
        Ok(entry) => ApiResponse::success(res_mapper::map_entry_to_response(entry), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn list_entry_media(
    service: &CmsService,
    ctx: &CmsRequestContext,
    entry_id: CmsId,
    params: ListBySiteQueryParams,
) -> ApiResponse<PaginatedResponse<MediaRefResponse>> {
    let query = ListEntryMediaQuery {
        entry_id,
        cursor: params.cursor,
        limit: params.limit.unwrap_or(20).min(100),
    };
    match service.list_entry_media(ctx, query).await {
        Ok(page) => {
            let response = res_mapper::map_page_to_paginated_response(page, res_mapper::map_media_ref_to_response);
            ApiResponse::success(response, Some(ctx.request_id.clone()))
        }
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn attach_entry_media(
    service: &CmsService,
    ctx: &CmsRequestContext,
    entry_id: CmsId,
    req: EntryMediaAttachRequest,
) -> ApiResponse<MediaRefResponse> {
    let command = req_mapper::map_entry_media_attach_request_to_command(entry_id, req);
    match service.attach_entry_media(ctx, command).await {
        Ok(media) => ApiResponse::success(res_mapper::map_media_ref_to_response(media), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn delete_entry_media(
    service: &CmsService,
    ctx: &CmsRequestContext,
    media_id: CmsId,
) -> ApiResponse<CommandResponse> {
    match service.delete_entry_media(ctx, media_id).await {
        Ok(result) => ApiResponse::success(res_mapper::map_command_result_to_response(result), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn replace_entry_terms(
    service: &CmsService,
    ctx: &CmsRequestContext,
    entry_id: CmsId,
    req: EntryTermsRequest,
) -> ApiResponse<EntryResponse> {
    let command = req_mapper::map_entry_terms_request_to_command(entry_id, req);
    match service.replace_entry_terms(ctx, command).await {
        Ok(entry) => ApiResponse::success(res_mapper::map_entry_to_response(entry), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn list_entry_versions(
    service: &CmsService,
    ctx: &CmsRequestContext,
    entry_id: CmsId,
    params: ListBySiteQueryParams,
) -> ApiResponse<PaginatedResponse<EntryVersionResponse>> {
    let query = ListEntryVersionsQuery {
        entry_id,
        cursor: params.cursor,
        limit: params.limit.unwrap_or(20).min(100),
    };
    match service.list_entry_versions(ctx, query).await {
        Ok(page) => {
            let response = res_mapper::map_page_to_paginated_response(page, res_mapper::map_entry_version_to_response);
            ApiResponse::success(response, Some(ctx.request_id.clone()))
        }
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn publish_entry(
    service: &CmsService,
    ctx: &CmsRequestContext,
    entry_id: CmsId,
    req: PublishRequest,
) -> ApiResponse<PublishSnapshotResponse> {
    let command = req_mapper::map_publish_request_to_command("entry", entry_id, req);
    match service.publish_entry(ctx, command).await {
        Ok(snapshot) => ApiResponse::success(res_mapper::map_publish_snapshot_to_response(snapshot), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn unpublish_entry(
    service: &CmsService,
    ctx: &CmsRequestContext,
    entry_id: CmsId,
    req: PublishRequest,
) -> ApiResponse<PublishSnapshotResponse> {
    let command = req_mapper::map_publish_request_to_command("entry", entry_id, req);
    match service.unpublish_entry(ctx, command).await {
        Ok(snapshot) => ApiResponse::success(res_mapper::map_publish_snapshot_to_response(snapshot), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn rollback_entry(
    service: &CmsService,
    ctx: &CmsRequestContext,
    entry_id: CmsId,
    req: RollbackRequest,
) -> ApiResponse<PublishSnapshotResponse> {
    let command = req_mapper::map_rollback_request_to_command("entry", entry_id, req);
    match service.rollback_entry(ctx, command).await {
        Ok(snapshot) => ApiResponse::success(res_mapper::map_publish_snapshot_to_response(snapshot), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}

pub async fn schedule_entry(
    service: &CmsService,
    ctx: &CmsRequestContext,
    entry_id: CmsId,
    req: ScheduleRequest,
) -> ApiResponse<EntryResponse> {
    let command = req_mapper::map_schedule_request_to_command(entry_id, req);
    match service.schedule_entry(ctx, command).await {
        Ok(entry) => ApiResponse::success(res_mapper::map_entry_to_response(entry), Some(ctx.request_id.clone())),
        Err(err) => ApiResponse::error(problem::map_cms_error_to_problem(&err, ctx.trace_id.clone()), Some(ctx.request_id.clone())),
    }
}
