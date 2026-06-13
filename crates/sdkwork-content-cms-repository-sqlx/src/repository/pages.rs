use sdkwork_content_cms_service::context::CmsRequestContext;
use sdkwork_content_cms_service::domain::*;
use sdkwork_content_cms_service::error::CmsResult;

use super::CmsSqlxRepository;

impl CmsSqlxRepository {
    pub async fn list_pages(
        &self,
        ctx: &CmsRequestContext,
        query: ListPagesQuery,
    ) -> CmsResult<CmsPagePage> {
        let limit = query.limit.min(100) as i64;
        let rows: Vec<(i64, i64, Option<i64>, String, String, String, i32, i64)> = sqlx::query_as(
            "SELECT id, site_id, channel_id, locale, path, title, publication_status, version 
             FROM cms_page WHERE tenant_id = $1 AND deleted_at IS NULL ORDER BY updated_at DESC, id DESC LIMIT $2"
        )
        .bind(ctx.tenant_id)
        .bind(limit)
        .fetch_all(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        let items = rows
            .into_iter()
            .map(|(id, site_id, channel_id, locale, path, title, pub_status, version)| {
                CmsPageModel {
                    id,
                    site_id,
                    channel_id,
                    locale,
                    path,
                    title,
                    publication_status: match pub_status {
                        10 => CmsPublicationStatus::Scheduled,
                        20 => CmsPublicationStatus::Published,
                        30 => CmsPublicationStatus::UnpublishedAfterPublish,
                        40 => CmsPublicationStatus::RolledBack,
                        _ => CmsPublicationStatus::Unpublished,
                    },
                    version,
                }
            })
            .collect();

        Ok(CmsPage {
            items,
            next_cursor: None,
        })
    }

    pub async fn create_page(
        &self,
        ctx: &CmsRequestContext,
        command: PageCommand,
    ) -> CmsResult<CmsPageModel> {
        let id = self.generate_id();
        let uuid = self.generate_uuid();
        let now = self.current_timestamp();

        let row: (i64, i64, Option<i64>, String, String, String, i32, i64) = sqlx::query_as(
            "INSERT INTO cms_page (id, uuid, tenant_id, organization_id, site_id, channel_id, locale, path, slug, title, publication_status, status, created_at, updated_at, created_by, updated_by, version) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, 0, 1, $11, $11, $12, $12, 0) 
             RETURNING id, site_id, channel_id, locale, path, title, publication_status, version"
        )
        .bind(id)
        .bind(&uuid)
        .bind(ctx.tenant_id)
        .bind(ctx.organization_id)
        .bind(command.site_id)
        .bind(command.channel_id)
        .bind(&command.locale)
        .bind(&command.path)
        .bind(&command.slug)
        .bind(&command.title)
        .bind(&now)
        .bind(ctx.user_id)
        .fetch_one(self.pool())
        .await
        .map_err(|e| {
            if e.to_string().contains("uk_cms_page_path") {
                sdkwork_content_cms_service::error::CmsError::conflict(format!(
                    "page path '{}' already exists in site/channel/locale",
                    command.path
                ))
            } else {
                sdkwork_content_cms_service::error::CmsError::internal(e.to_string())
            }
        })?;

        Ok(CmsPageModel {
            id: row.0,
            site_id: row.1,
            channel_id: row.2,
            locale: row.3,
            path: row.4,
            title: row.5,
            publication_status: CmsPublicationStatus::Unpublished,
            version: row.7,
        })
    }

    pub async fn retrieve_page(
        &self,
        ctx: &CmsRequestContext,
        page_id: CmsId,
    ) -> CmsResult<CmsPageModel> {
        let row: (i64, i64, Option<i64>, String, String, String, i32, i64) = sqlx::query_as(
            "SELECT id, site_id, channel_id, locale, path, title, publication_status, version 
             FROM cms_page WHERE tenant_id = $1 AND id = $2 AND deleted_at IS NULL"
        )
        .bind(ctx.tenant_id)
        .bind(page_id)
        .fetch_optional(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?
        .ok_or(sdkwork_content_cms_service::error::CmsError::not_found("page"))?;

        Ok(CmsPageModel {
            id: row.0,
            site_id: row.1,
            channel_id: row.2,
            locale: row.3,
            path: row.4,
            title: row.5,
            publication_status: match row.6 {
                10 => CmsPublicationStatus::Scheduled,
                20 => CmsPublicationStatus::Published,
                30 => CmsPublicationStatus::UnpublishedAfterPublish,
                40 => CmsPublicationStatus::RolledBack,
                _ => CmsPublicationStatus::Unpublished,
            },
            version: row.7,
        })
    }

    pub async fn update_page(
        &self,
        ctx: &CmsRequestContext,
        page_id: CmsId,
        command: PageCommand,
    ) -> CmsResult<CmsPageModel> {
        let now = self.current_timestamp();
        let version = command.version.unwrap_or(0);

        let row: (i64, i64, Option<i64>, String, String, String, i32, i64) = sqlx::query_as(
            "UPDATE cms_page SET 
                channel_id = COALESCE($4, channel_id),
                locale = COALESCE($5, locale),
                path = COALESCE($6, path),
                slug = COALESCE($7, slug),
                title = COALESCE($8, title),
                updated_at = $9,
                updated_by = $10,
                version = version + 1
             WHERE tenant_id = $1 AND id = $2 AND version = $3 AND deleted_at IS NULL
             RETURNING id, site_id, channel_id, locale, path, title, publication_status, version"
        )
        .bind(ctx.tenant_id)
        .bind(page_id)
        .bind(version)
        .bind(command.channel_id)
        .bind(if command.locale.is_empty() { None } else { Some(&command.locale) })
        .bind(if command.path.is_empty() { None } else { Some(&command.path) })
        .bind(if command.slug.is_empty() { None } else { Some(&command.slug) })
        .bind(if command.title.is_empty() { None } else { Some(&command.title) })
        .bind(&now)
        .bind(ctx.user_id)
        .fetch_optional(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?
        .ok_or(sdkwork_content_cms_service::error::CmsError::OptimisticLockConflict {
            resource: "page",
            resource_id: page_id,
            expected_version: version,
        })?;

        Ok(CmsPageModel {
            id: row.0,
            site_id: row.1,
            channel_id: row.2,
            locale: row.3,
            path: row.4,
            title: row.5,
            publication_status: match row.6 {
                10 => CmsPublicationStatus::Scheduled,
                20 => CmsPublicationStatus::Published,
                30 => CmsPublicationStatus::UnpublishedAfterPublish,
                40 => CmsPublicationStatus::RolledBack,
                _ => CmsPublicationStatus::Unpublished,
            },
            version: row.7,
        })
    }

    pub async fn delete_page(
        &self,
        ctx: &CmsRequestContext,
        page_id: CmsId,
    ) -> CmsResult<CommandResult> {
        let now = self.current_timestamp();
        let result = sqlx::query(
            "UPDATE cms_page SET status = 9, deleted_at = $3, deleted_by = $4 WHERE tenant_id = $1 AND id = $2 AND deleted_at IS NULL"
        )
        .bind(ctx.tenant_id)
        .bind(page_id)
        .bind(&now)
        .bind(ctx.user_id)
        .execute(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(sdkwork_content_cms_service::error::CmsError::not_found("page"));
        }

        Ok(CommandResult {
            ok: true,
            resource_id: Some(page_id),
            request_id: Some(ctx.request_id.clone()),
        })
    }

    pub async fn replace_page_blocks(
        &self,
        ctx: &CmsRequestContext,
        command: PageBlocksCommand,
    ) -> CmsResult<CmsPageModel> {
        let _ = command.blocks_json;
        self.retrieve_page(ctx, command.page_id).await
    }

    pub async fn publish_page(
        &self,
        ctx: &CmsRequestContext,
        command: PublishCommand,
    ) -> CmsResult<CmsPublishSnapshot> {
        let now = self.current_timestamp();
        let snapshot_id = self.generate_id();
        let snapshot_uuid = self.generate_uuid();

        let page = self.retrieve_page(ctx, command.owner_id).await?;

        let row = sqlx::query_as::<_, crate::db::rows::CmsPublishSnapshotRow>(
            "INSERT INTO cms_publish_snapshot (id, uuid, tenant_id, organization_id, site_id, owner_type, owner_id, channel_id, locale, snapshot_payload_json, status, published_at, published_by, created_at) 
             VALUES ($1, $2, $3, $4, $5, 'page', $6, $7, $8, '{}', 1, $9, $10, $9)
             RETURNING id, tenant_id, site_id, owner_type, owner_id, snapshot_payload_json, status, published_at"
        )
        .bind(snapshot_id)
        .bind(&snapshot_uuid)
        .bind(ctx.tenant_id)
        .bind(ctx.organization_id)
        .bind(page.site_id)
        .bind(command.owner_id)
        .bind(command.channel_id)
        .bind(command.locale.as_deref().unwrap_or("default"))
        .bind(&now)
        .bind(ctx.user_id)
        .fetch_one(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        sqlx::query(
            "UPDATE cms_page SET publication_status = 20, published_snapshot_id = $3, updated_at = $4, updated_by = $5, version = version + 1 WHERE tenant_id = $1 AND id = $2 AND deleted_at IS NULL"
        )
        .bind(ctx.tenant_id)
        .bind(command.owner_id)
        .bind(snapshot_id)
        .bind(&now)
        .bind(ctx.user_id)
        .execute(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        Ok(crate::mapper::row_mapper::map_publish_snapshot_row(row))
    }
}
