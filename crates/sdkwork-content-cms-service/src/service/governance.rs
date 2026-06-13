use crate::context::CmsRequestContext;
use crate::domain::*;
use crate::error::CmsResult;
use crate::service::CmsService;

impl CmsService {
    pub async fn list_audit_logs(
        &self,
        ctx: &CmsRequestContext,
        query: ListAuditLogsQuery,
    ) -> CmsResult<CmsAuditLogPage> {
        ctx.require_permission("cms.audit.read")?;
        self.repository().list_audit_logs(ctx, query).await
    }

    pub async fn list_outbox_events(
        &self,
        ctx: &CmsRequestContext,
        query: ListOutboxEventsQuery,
    ) -> CmsResult<CmsOutboxEventPage> {
        ctx.require_permission("cms.audit.read")?;
        self.repository().list_outbox_events(ctx, query).await
    }

    pub async fn retry_outbox_event(
        &self,
        ctx: &CmsRequestContext,
        command: RetryOutboxEventCommand,
    ) -> CmsResult<CommandResult> {
        ctx.require_permission("cms.audit.manage")?;
        self.repository().retry_outbox_event(ctx, command).await
    }
}
