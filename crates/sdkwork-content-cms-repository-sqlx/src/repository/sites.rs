use sdkwork_content_cms_service::context::CmsRequestContext;
use sdkwork_content_cms_service::domain::*;
use sdkwork_content_cms_service::error::CmsResult;

use super::CmsSqlxRepository;

impl CmsSqlxRepository {
    pub async fn list_sites(
        &self,
        ctx: &CmsRequestContext,
        query: ListSitesQuery,
    ) -> CmsResult<CmsSitePage> {
        let limit = query.limit.min(100) as i64;
        let rows = sqlx::query_as::<_, crate::db::rows::CmsSiteRow>(
            "SELECT id, uuid, tenant_id, organization_id, code, name, default_locale, settings_json, status, version 
             FROM cms_site WHERE tenant_id = $1 AND deleted_at IS NULL ORDER BY updated_at DESC, id DESC LIMIT $2"
        )
        .bind(ctx.tenant_id)
        .bind(limit)
        .fetch_all(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        let items = rows.into_iter().map(crate::mapper::row_mapper::map_site_row).collect();
        Ok(CmsPage {
            items,
            next_cursor: None,
        })
    }

    pub async fn create_site(
        &self,
        ctx: &CmsRequestContext,
        command: SiteCommand,
    ) -> CmsResult<CmsSite> {
        let id = self.generate_id();
        let uuid = self.generate_uuid();
        let now = self.current_timestamp();
        let code = command.code.ok_or_else(|| {
            sdkwork_content_cms_service::error::CmsError::validation("code is required")
        })?;
        let name = command.name.ok_or_else(|| {
            sdkwork_content_cms_service::error::CmsError::validation("name is required")
        })?;
        let default_locale = command.default_locale.unwrap_or_else(|| "en-US".to_string());
        let settings_json = command.settings_json.unwrap_or_else(|| "{}".to_string());

        let row = sqlx::query_as::<_, crate::db::rows::CmsSiteRow>(
            "INSERT INTO cms_site (id, uuid, tenant_id, organization_id, code, name, default_locale, settings_json, status, created_at, updated_at, created_by, updated_by, version) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8::jsonb, 1, $9, $9, $10, $10, 0) 
             RETURNING id, uuid, tenant_id, organization_id, code, name, default_locale, settings_json, status, version"
        )
        .bind(id)
        .bind(&uuid)
        .bind(ctx.tenant_id)
        .bind(ctx.organization_id)
        .bind(&code)
        .bind(&name)
        .bind(&default_locale)
        .bind(&settings_json)
        .bind(&now)
        .bind(ctx.user_id)
        .fetch_one(self.pool())
        .await
        .map_err(|e| {
            if e.to_string().contains("uk_cms_site_tenant_code") {
                sdkwork_content_cms_service::error::CmsError::conflict(format!(
                    "site code '{}' already exists",
                    code
                ))
            } else {
                sdkwork_content_cms_service::error::CmsError::internal(e.to_string())
            }
        })?;

        Ok(crate::mapper::row_mapper::map_site_row(row))
    }

    pub async fn retrieve_site(
        &self,
        ctx: &CmsRequestContext,
        site_id: CmsId,
    ) -> CmsResult<CmsSite> {
        let row = sqlx::query_as::<_, crate::db::rows::CmsSiteRow>(
            "SELECT id, uuid, tenant_id, organization_id, code, name, default_locale, settings_json, status, version 
             FROM cms_site WHERE tenant_id = $1 AND id = $2 AND deleted_at IS NULL"
        )
        .bind(ctx.tenant_id)
        .bind(site_id)
        .fetch_optional(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?
        .ok_or(sdkwork_content_cms_service::error::CmsError::not_found("site"))?;

        Ok(crate::mapper::row_mapper::map_site_row(row))
    }

    pub async fn update_site(
        &self,
        ctx: &CmsRequestContext,
        site_id: CmsId,
        command: SiteCommand,
    ) -> CmsResult<CmsSite> {
        let now = self.current_timestamp();
        let version = command.version.unwrap_or(0);

        let row = sqlx::query_as::<_, crate::db::rows::CmsSiteRow>(
            "UPDATE cms_site SET 
                code = COALESCE($4, code),
                name = COALESCE($5, name),
                default_locale = COALESCE($6, default_locale),
                settings_json = COALESCE($7::jsonb, settings_json),
                updated_at = $8,
                updated_by = $9,
                version = version + 1
             WHERE tenant_id = $1 AND id = $2 AND version = $3 AND deleted_at IS NULL
             RETURNING id, uuid, tenant_id, organization_id, code, name, default_locale, settings_json, status, version"
        )
        .bind(ctx.tenant_id)
        .bind(site_id)
        .bind(version)
        .bind(&command.code)
        .bind(&command.name)
        .bind(&command.default_locale)
        .bind(&command.settings_json)
        .bind(&now)
        .bind(ctx.user_id)
        .fetch_optional(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?
        .ok_or(sdkwork_content_cms_service::error::CmsError::OptimisticLockConflict {
            resource: "site",
            resource_id: site_id,
            expected_version: version,
        })?;

        Ok(crate::mapper::row_mapper::map_site_row(row))
    }

    pub async fn delete_site(
        &self,
        ctx: &CmsRequestContext,
        site_id: CmsId,
    ) -> CmsResult<CommandResult> {
        let now = self.current_timestamp();
        let result = sqlx::query(
            "UPDATE cms_site SET status = 9, deleted_at = $3, deleted_by = $4 WHERE tenant_id = $1 AND id = $2 AND deleted_at IS NULL"
        )
        .bind(ctx.tenant_id)
        .bind(site_id)
        .bind(&now)
        .bind(ctx.user_id)
        .execute(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(sdkwork_content_cms_service::error::CmsError::not_found("site"));
        }

        Ok(CommandResult {
            ok: true,
            resource_id: Some(site_id),
            request_id: Some(ctx.request_id.clone()),
        })
    }

    pub async fn list_channels(
        &self,
        ctx: &CmsRequestContext,
        query: ListBySiteQuery,
    ) -> CmsResult<CmsChannelPage> {
        let limit = query.limit.min(100) as i64;
        let rows: Vec<(i64, String, i64, String, String, String, i32)> = sqlx::query_as(
            "SELECT id, uuid, site_id, code, name, channel_kind, status 
             FROM cms_channel WHERE tenant_id = $1 AND site_id = $2 AND deleted_at IS NULL ORDER BY sort_order, id LIMIT $3"
        )
        .bind(ctx.tenant_id)
        .bind(query.site_id)
        .bind(limit)
        .fetch_all(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        let items = rows
            .into_iter()
            .map(|(id, _uuid, site_id, code, name, channel_kind, status)| CmsChannel {
                id,
                site_id,
                code,
                name,
                channel_kind,
                status,
            })
            .collect();

        Ok(CmsPage {
            items,
            next_cursor: None,
        })
    }

    pub async fn create_channel(
        &self,
        ctx: &CmsRequestContext,
        command: ChannelCommand,
    ) -> CmsResult<CmsChannel> {
        let id = self.generate_id();
        let uuid = self.generate_uuid();
        let now = self.current_timestamp();
        let code = command.code.ok_or_else(|| {
            sdkwork_content_cms_service::error::CmsError::validation("code is required")
        })?;
        let name = command.name.ok_or_else(|| {
            sdkwork_content_cms_service::error::CmsError::validation("name is required")
        })?;
        let channel_kind = command.channel_kind.unwrap_or_else(|| "web".to_string());

        let row: (i64, String, i64, String, String, String, i32) = sqlx::query_as(
            "INSERT INTO cms_channel (id, uuid, tenant_id, organization_id, site_id, code, name, channel_kind, status, created_at, updated_at, created_by, updated_by, version) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 1, $9, $9, $10, $10, 0) 
             RETURNING id, uuid, site_id, code, name, channel_kind, status"
        )
        .bind(id)
        .bind(&uuid)
        .bind(ctx.tenant_id)
        .bind(ctx.organization_id)
        .bind(command.site_id)
        .bind(&code)
        .bind(&name)
        .bind(&channel_kind)
        .bind(&now)
        .bind(ctx.user_id)
        .fetch_one(self.pool())
        .await
        .map_err(|e| {
            if e.to_string().contains("uk_cms_channel_site_code") {
                sdkwork_content_cms_service::error::CmsError::conflict(format!(
                    "channel code '{}' already exists in site",
                    code
                ))
            } else {
                sdkwork_content_cms_service::error::CmsError::internal(e.to_string())
            }
        })?;

        Ok(CmsChannel {
            id: row.0,
            site_id: row.2,
            code: row.3,
            name: row.4,
            channel_kind: row.5,
            status: row.6,
        })
    }

    pub async fn update_channel(
        &self,
        ctx: &CmsRequestContext,
        channel_id: CmsId,
        command: ChannelCommand,
    ) -> CmsResult<CmsChannel> {
        let now = self.current_timestamp();
        let version = command.version.unwrap_or(0);

        let row: (i64, String, i64, String, String, String, i32) = sqlx::query_as(
            "UPDATE cms_channel SET 
                code = COALESCE($4, code),
                name = COALESCE($5, name),
                channel_kind = COALESCE($6, channel_kind),
                updated_at = $7,
                updated_by = $8,
                version = version + 1
             WHERE tenant_id = $1 AND id = $2 AND version = $3 AND deleted_at IS NULL
             RETURNING id, uuid, site_id, code, name, channel_kind, status"
        )
        .bind(ctx.tenant_id)
        .bind(channel_id)
        .bind(version)
        .bind(&command.code)
        .bind(&command.name)
        .bind(&command.channel_kind)
        .bind(&now)
        .bind(ctx.user_id)
        .fetch_optional(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?
        .ok_or(sdkwork_content_cms_service::error::CmsError::OptimisticLockConflict {
            resource: "channel",
            resource_id: channel_id,
            expected_version: version,
        })?;

        Ok(CmsChannel {
            id: row.0,
            site_id: row.2,
            code: row.3,
            name: row.4,
            channel_kind: row.5,
            status: row.6,
        })
    }

    pub async fn delete_channel(
        &self,
        ctx: &CmsRequestContext,
        channel_id: CmsId,
    ) -> CmsResult<CommandResult> {
        let now = self.current_timestamp();
        let result = sqlx::query(
            "UPDATE cms_channel SET status = 9, deleted_at = $3, deleted_by = $4 WHERE tenant_id = $1 AND id = $2 AND deleted_at IS NULL"
        )
        .bind(ctx.tenant_id)
        .bind(channel_id)
        .bind(&now)
        .bind(ctx.user_id)
        .execute(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(sdkwork_content_cms_service::error::CmsError::not_found("channel"));
        }

        Ok(CommandResult {
            ok: true,
            resource_id: Some(channel_id),
            request_id: Some(ctx.request_id.clone()),
        })
    }
}
