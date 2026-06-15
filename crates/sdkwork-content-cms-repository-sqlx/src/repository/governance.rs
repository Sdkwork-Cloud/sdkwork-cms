use sdkwork_content_cms_service::context::CmsRequestContext;
use sdkwork_content_cms_service::domain::*;
use sdkwork_content_cms_service::error::CmsResult;

use super::CmsSqlxRepository;

impl CmsSqlxRepository {
    pub async fn list_audit_logs(
        &self,
        ctx: &CmsRequestContext,
        query: ListAuditLogsQuery,
    ) -> CmsResult<CmsAuditLogPage> {
        let limit = query.limit.min(100) as i64;
        let rows: Vec<(i64, Option<i64>, i64, String, String, Option<i64>, String, String, String)> = sqlx::query_as(
            "SELECT id, site_id, actor_user_id, action, resource_type, resource_id, before_json, after_json, created_at 
             FROM cms_audit_log WHERE tenant_id = $1 ORDER BY created_at DESC, id DESC LIMIT $2"
        )
        .bind(ctx.tenant_id)
        .bind(limit)
        .fetch_all(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        let items = rows
            .into_iter()
            .map(|(id, site_id, actor_user_id, action, resource_type, resource_id, before_json, after_json, created_at)| {
                CmsAuditLog {
                    id,
                    site_id,
                    actor_user_id,
                    action,
                    resource_type,
                    resource_id,
                    before_json,
                    after_json,
                    created_at,
                }
            })
            .collect();

        Ok(CmsPage {
            items,
            next_cursor: None,
        })
    }

    pub async fn list_outbox_events(
        &self,
        ctx: &CmsRequestContext,
        query: ListOutboxEventsQuery,
    ) -> CmsResult<CmsOutboxEventPage> {
        let limit = query.limit.min(100) as i64;
        let rows: Vec<(i64, String, i64, String, String, i32, i32, Option<String>, String)> = sqlx::query_as(
            "SELECT id, aggregate_type, aggregate_id, event_type, payload_json, status, attempt_count, next_attempt_at, created_at 
             FROM cms_outbox_event WHERE tenant_id = $1 ORDER BY created_at DESC, id DESC LIMIT $2"
        )
        .bind(ctx.tenant_id)
        .bind(limit)
        .fetch_all(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        let items = rows
            .into_iter()
            .map(|(id, aggregate_type, aggregate_id, event_type, payload_json, status, attempt_count, next_attempt_at, created_at)| {
                CmsOutboxEvent {
                    id,
                    aggregate_type,
                    aggregate_id,
                    event_type,
                    payload_json,
                    status,
                    attempt_count,
                    next_attempt_at,
                    created_at,
                }
            })
            .collect();

        Ok(CmsPage {
            items,
            next_cursor: None,
        })
    }

    pub async fn retry_outbox_event(
        &self,
        ctx: &CmsRequestContext,
        command: RetryOutboxEventCommand,
    ) -> CmsResult<CommandResult> {
        let now = self.current_timestamp();
        let result = sqlx::query(
            "UPDATE cms_outbox_event SET status = 0, attempt_count = 0, next_attempt_at = $3, updated_at = $3, error_message = NULL WHERE tenant_id = $1 AND id = $2"
        )
        .bind(ctx.tenant_id)
        .bind(command.event_id)
        .bind(&now)
        .execute(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(sdkwork_content_cms_service::error::CmsError::not_found("outbox event"));
        }

        Ok(CommandResult {
            ok: true,
            resource_id: Some(command.event_id),
            request_id: Some(ctx.request_id.clone()),
        })
    }

    pub async fn create_outbox_event(
        &self,
        ctx: &CmsRequestContext,
        event: CmsOutboxEventDraft,
    ) -> CmsResult<CommandResult> {
        let id = self.generate_id();
        let uuid = self.generate_uuid();
        let now = self.current_timestamp();
        let event_type_str = match event.event_type {
            CmsEventType::SiteCreated => "cms.site.created",
            CmsEventType::SiteUpdated => "cms.site.updated",
            CmsEventType::ChannelCreated => "cms.channel.created",
            CmsEventType::ChannelUpdated => "cms.channel.updated",
            CmsEventType::ContentTypeCreated => "cms.content_type.created",
            CmsEventType::ContentTypeUpdated => "cms.content_type.updated",
            CmsEventType::EntryCreated => "cms.entry.created",
            CmsEventType::EntryUpdated => "cms.entry.updated",
            CmsEventType::EntryPublished => "cms.entry.published",
            CmsEventType::EntryUnpublished => "cms.entry.unpublished",
            CmsEventType::EntryRolledBack => "cms.entry.rolled_back",
            CmsEventType::PagePublished => "cms.page.published",
            CmsEventType::FeedPublished => "cms.feed.published",
            CmsEventType::SearchSyncRequested => "cms.search.sync_requested",
            CmsEventType::CacheInvalidateRequested => "cms.cache.invalidate_requested",
        };

        sqlx::query(
            "INSERT INTO cms_outbox_event (id, uuid, tenant_id, organization_id, aggregate_type, aggregate_id, event_type, event_version, payload_json, status, attempt_count, created_at, updated_at, request_id, trace_id) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, 1, $8, 0, 0, $9, $9, $10, $11)"
        )
        .bind(id)
        .bind(&uuid)
        .bind(ctx.tenant_id)
        .bind(ctx.organization_id)
        .bind(&event.aggregate_type)
        .bind(event.aggregate_id)
        .bind(event_type_str)
        .bind(&event.payload_json)
        .bind(&now)
        .bind(&ctx.request_id)
        .bind(&ctx.trace_id)
        .execute(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        Ok(CommandResult {
            ok: true,
            resource_id: Some(id),
            request_id: Some(ctx.request_id.clone()),
        })
    }
}
