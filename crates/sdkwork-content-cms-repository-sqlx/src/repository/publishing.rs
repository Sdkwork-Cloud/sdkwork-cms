use sdkwork_content_cms_service::context::CmsRequestContext;
use sdkwork_content_cms_service::domain::*;
use sdkwork_content_cms_service::error::CmsResult;

use super::CmsSqlxRepository;

impl CmsSqlxRepository {
    pub async fn create_publish_snapshot(
        &self,
        ctx: &CmsRequestContext,
        command: PublishCommand,
    ) -> CmsResult<CmsPublishSnapshot> {
        let id = self.generate_id();
        let uuid = self.generate_uuid();
        let now = self.current_timestamp();

        let row = sqlx::query_as::<_, crate::db::rows::CmsPublishSnapshotRow>(
            "INSERT INTO cms_publish_snapshot (id, uuid, tenant_id, organization_id, site_id, owner_type, owner_id, channel_id, locale, snapshot_payload_json, status, published_at, published_by, created_at) 
             VALUES ($1, $2, $3, $4, 0, $5, $6, $7, $8, '{}', 1, $9, $10, $9)
             RETURNING id, tenant_id, site_id, owner_type, owner_id, snapshot_payload_json, status, published_at"
        )
        .bind(id)
        .bind(&uuid)
        .bind(ctx.tenant_id)
        .bind(ctx.organization_id)
        .bind(&command.owner_type)
        .bind(command.owner_id)
        .bind(command.channel_id)
        .bind(command.locale.as_deref().unwrap_or("default"))
        .bind(&now)
        .bind(ctx.user_id)
        .fetch_one(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        Ok(crate::mapper::row_mapper::map_publish_snapshot_row(row))
    }
}
