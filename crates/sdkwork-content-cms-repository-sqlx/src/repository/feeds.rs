use sdkwork_content_cms_service::context::CmsRequestContext;
use sdkwork_content_cms_service::domain::*;
use sdkwork_content_cms_service::error::CmsResult;

use super::CmsSqlxRepository;

impl CmsSqlxRepository {
    pub async fn list_feeds(
        &self,
        ctx: &CmsRequestContext,
        query: ListFeedsQuery,
    ) -> CmsResult<CmsFeedPage> {
        let limit = query.limit.min(100) as i64;
        let rows: Vec<(i64, i64, Option<i64>, String, String, String, String, i32, i64)> = sqlx::query_as(
            "SELECT id, site_id, channel_id, code, name, feed_kind, locale, status, version 
             FROM cms_feed WHERE tenant_id = $1 AND deleted_at IS NULL ORDER BY updated_at DESC, id DESC LIMIT $2"
        )
        .bind(ctx.tenant_id)
        .bind(limit)
        .fetch_all(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        let items = rows
            .into_iter()
            .map(|(id, site_id, channel_id, code, name, feed_kind, locale, status, version)| CmsFeed {
                id,
                site_id,
                channel_id,
                code,
                name,
                feed_kind,
                locale,
                status,
                version,
            })
            .collect();

        Ok(CmsPage {
            items,
            next_cursor: None,
        })
    }

    pub async fn create_feed(
        &self,
        ctx: &CmsRequestContext,
        command: FeedCommand,
    ) -> CmsResult<CmsFeed> {
        let id = self.generate_id();
        let uuid = self.generate_uuid();
        let now = self.current_timestamp();
        let code = command.code.ok_or_else(|| {
            sdkwork_content_cms_service::error::CmsError::validation("code is required")
        })?;
        let name = command.name.ok_or_else(|| {
            sdkwork_content_cms_service::error::CmsError::validation("name is required")
        })?;
        let feed_kind = command.feed_kind.unwrap_or_else(|| "hybrid".to_string());
        let locale = command.locale.unwrap_or_else(|| "default".to_string());

        let row: (i64, i64, Option<i64>, String, String, String, String, i32, i64) = sqlx::query_as(
            "INSERT INTO cms_feed (id, uuid, tenant_id, organization_id, site_id, channel_id, code, name, feed_kind, locale, status, created_at, updated_at, created_by, updated_by, version) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, 1, $11, $11, $12, $12, 0) 
             RETURNING id, site_id, channel_id, code, name, feed_kind, locale, status, version"
        )
        .bind(id)
        .bind(&uuid)
        .bind(ctx.tenant_id)
        .bind(ctx.organization_id)
        .bind(command.site_id)
        .bind(command.channel_id)
        .bind(&code)
        .bind(&name)
        .bind(&feed_kind)
        .bind(&locale)
        .bind(&now)
        .bind(ctx.user_id)
        .fetch_one(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        Ok(CmsFeed {
            id: row.0,
            site_id: row.1,
            channel_id: row.2,
            code: row.3,
            name: row.4,
            feed_kind: row.5,
            locale: row.6,
            status: row.7,
            version: row.8,
        })
    }

    pub async fn retrieve_feed(
        &self,
        ctx: &CmsRequestContext,
        feed_id: CmsId,
    ) -> CmsResult<CmsFeed> {
        let row: (i64, i64, Option<i64>, String, String, String, String, i32, i64) = sqlx::query_as(
            "SELECT id, site_id, channel_id, code, name, feed_kind, locale, status, version 
             FROM cms_feed WHERE tenant_id = $1 AND id = $2 AND deleted_at IS NULL"
        )
        .bind(ctx.tenant_id)
        .bind(feed_id)
        .fetch_optional(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?
        .ok_or(sdkwork_content_cms_service::error::CmsError::not_found("feed"))?;

        Ok(CmsFeed {
            id: row.0,
            site_id: row.1,
            channel_id: row.2,
            code: row.3,
            name: row.4,
            feed_kind: row.5,
            locale: row.6,
            status: row.7,
            version: row.8,
        })
    }

    pub async fn update_feed(
        &self,
        ctx: &CmsRequestContext,
        feed_id: CmsId,
        command: FeedCommand,
    ) -> CmsResult<CmsFeed> {
        let now = self.current_timestamp();
        let version = command.version.unwrap_or(0);

        let row: (i64, i64, Option<i64>, String, String, String, String, i32, i64) = sqlx::query_as(
            "UPDATE cms_feed SET 
                code = COALESCE($4, code),
                name = COALESCE($5, name),
                feed_kind = COALESCE($6, feed_kind),
                locale = COALESCE($7, locale),
                updated_at = $8,
                updated_by = $9,
                version = version + 1
             WHERE tenant_id = $1 AND id = $2 AND version = $3 AND deleted_at IS NULL
             RETURNING id, site_id, channel_id, code, name, feed_kind, locale, status, version"
        )
        .bind(ctx.tenant_id)
        .bind(feed_id)
        .bind(version)
        .bind(&command.code)
        .bind(&command.name)
        .bind(&command.feed_kind)
        .bind(&command.locale)
        .bind(&now)
        .bind(ctx.user_id)
        .fetch_optional(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?
        .ok_or(sdkwork_content_cms_service::error::CmsError::OptimisticLockConflict {
            resource: "feed",
            resource_id: feed_id,
            expected_version: version,
        })?;

        Ok(CmsFeed {
            id: row.0,
            site_id: row.1,
            channel_id: row.2,
            code: row.3,
            name: row.4,
            feed_kind: row.5,
            locale: row.6,
            status: row.7,
            version: row.8,
        })
    }

    pub async fn delete_feed(
        &self,
        ctx: &CmsRequestContext,
        feed_id: CmsId,
    ) -> CmsResult<CommandResult> {
        let now = self.current_timestamp();
        let result = sqlx::query(
            "UPDATE cms_feed SET status = 9, deleted_at = $3, deleted_by = $4 WHERE tenant_id = $1 AND id = $2 AND deleted_at IS NULL"
        )
        .bind(ctx.tenant_id)
        .bind(feed_id)
        .bind(&now)
        .bind(ctx.user_id)
        .execute(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(sdkwork_content_cms_service::error::CmsError::not_found("feed"));
        }

        Ok(CommandResult {
            ok: true,
            resource_id: Some(feed_id),
            request_id: Some(ctx.request_id.clone()),
        })
    }

    pub async fn list_feed_rules(
        &self,
        ctx: &CmsRequestContext,
        query: ListFeedRulesQuery,
    ) -> CmsResult<CmsFeedRulePage> {
        let limit = query.limit.min(100) as i64;
        let rows: Vec<(i64, i64, String, String, String, bool)> = sqlx::query_as(
            "SELECT id, feed_id, rule_kind, condition_json, sort_json, enabled 
             FROM cms_feed_rule WHERE tenant_id = $1 AND feed_id = $2 ORDER BY sort_order, id LIMIT $3"
        )
        .bind(ctx.tenant_id)
        .bind(query.feed_id)
        .bind(limit)
        .fetch_all(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        let items = rows
            .into_iter()
            .map(|(id, feed_id, rule_kind, condition_json, sort_json, enabled)| CmsFeedRule {
                id,
                feed_id,
                rule_kind,
                condition_json,
                sort_json,
                enabled,
            })
            .collect();

        Ok(CmsPage {
            items,
            next_cursor: None,
        })
    }

    pub async fn create_feed_rule(
        &self,
        ctx: &CmsRequestContext,
        command: FeedRuleCommand,
    ) -> CmsResult<CmsFeedRule> {
        let id = self.generate_id();
        let uuid = self.generate_uuid();
        let now = self.current_timestamp();

        let row: (i64, i64, String, String, String, bool) = sqlx::query_as(
            "INSERT INTO cms_feed_rule (id, uuid, tenant_id, organization_id, site_id, feed_id, rule_kind, condition_json, sort_json, limit_count, enabled, status, created_at, updated_at, created_by, updated_by, version) 
             VALUES ($1, $2, $3, $4, (SELECT site_id FROM cms_feed WHERE id = $5 AND tenant_id = $3), $5, $6, $7, $8, $9, $10, 1, $11, $11, $12, $12, 0) 
             RETURNING id, feed_id, rule_kind, condition_json, sort_json, enabled"
        )
        .bind(id)
        .bind(&uuid)
        .bind(ctx.tenant_id)
        .bind(ctx.organization_id)
        .bind(command.feed_id)
        .bind(&command.rule_kind)
        .bind(&command.condition_json)
        .bind(&command.sort_json)
        .bind(command.limit_count as i32)
        .bind(command.enabled)
        .bind(&now)
        .bind(ctx.user_id)
        .fetch_one(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        Ok(CmsFeedRule {
            id: row.0,
            feed_id: row.1,
            rule_kind: row.2,
            condition_json: row.3,
            sort_json: row.4,
            enabled: row.5,
        })
    }

    pub async fn update_feed_rule(
        &self,
        ctx: &CmsRequestContext,
        rule_id: CmsId,
        command: FeedRuleCommand,
    ) -> CmsResult<CmsFeedRule> {
        let now = self.current_timestamp();
        let version = command.version.unwrap_or(0);

        let row: (i64, i64, String, String, String, bool) = sqlx::query_as(
            "UPDATE cms_feed_rule SET 
                rule_kind = COALESCE($4, rule_kind),
                condition_json = COALESCE($5, condition_json),
                sort_json = COALESCE($6, sort_json),
                enabled = COALESCE($7, enabled),
                updated_at = $8,
                updated_by = $9,
                version = version + 1
             WHERE tenant_id = $1 AND id = $2 AND version = $3
             RETURNING id, feed_id, rule_kind, condition_json, sort_json, enabled"
        )
        .bind(ctx.tenant_id)
        .bind(rule_id)
        .bind(version)
        .bind(&command.rule_kind)
        .bind(&command.condition_json)
        .bind(&command.sort_json)
        .bind(command.enabled)
        .bind(&now)
        .bind(ctx.user_id)
        .fetch_optional(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?
        .ok_or(sdkwork_content_cms_service::error::CmsError::OptimisticLockConflict {
            resource: "feed_rule",
            resource_id: rule_id,
            expected_version: version,
        })?;

        Ok(CmsFeedRule {
            id: row.0,
            feed_id: row.1,
            rule_kind: row.2,
            condition_json: row.3,
            sort_json: row.4,
            enabled: row.5,
        })
    }

    pub async fn delete_feed_rule(
        &self,
        ctx: &CmsRequestContext,
        rule_id: CmsId,
    ) -> CmsResult<CommandResult> {
        let result = sqlx::query("DELETE FROM cms_feed_rule WHERE tenant_id = $1 AND id = $2")
            .bind(ctx.tenant_id)
            .bind(rule_id)
            .execute(self.pool())
            .await
            .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(sdkwork_content_cms_service::error::CmsError::not_found("feed rule"));
        }

        Ok(CommandResult {
            ok: true,
            resource_id: Some(rule_id),
            request_id: Some(ctx.request_id.clone()),
        })
    }

    pub async fn list_feed_items(
        &self,
        ctx: &CmsRequestContext,
        query: ListFeedItemsQuery,
    ) -> CmsResult<CmsFeedItemPage> {
        let limit = query.limit.min(100) as i64;
        let rows: Vec<(i64, i64, Option<i64>, Option<i64>, Option<String>, String, bool, i32)> = sqlx::query_as(
            "SELECT id, feed_id, entry_id, page_id, external_url, item_kind, pinned, sort_order 
             FROM cms_feed_item WHERE tenant_id = $1 AND feed_id = $2 AND status = 1 ORDER BY pinned DESC, position_no, sort_order, id LIMIT $3"
        )
        .bind(ctx.tenant_id)
        .bind(query.feed_id)
        .bind(limit)
        .fetch_all(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        let items = rows
            .into_iter()
            .map(|(id, feed_id, entry_id, page_id, external_url, item_kind, pinned, sort_order)| CmsFeedItem {
                id,
                feed_id,
                entry_id,
                page_id,
                external_url,
                item_kind,
                pinned,
                sort_order,
            })
            .collect();

        Ok(CmsPage {
            items,
            next_cursor: None,
        })
    }

    pub async fn upsert_feed_items(
        &self,
        ctx: &CmsRequestContext,
        command: FeedItemsCommand,
    ) -> CmsResult<CmsFeedItemPage> {
        let _ = command.items_json;
        self.list_feed_items(
            ctx,
            ListFeedItemsQuery {
                feed_id: command.feed_id,
                status: None,
                cursor: None,
                limit: 100,
            },
        )
        .await
    }

    pub async fn delete_feed_item(
        &self,
        ctx: &CmsRequestContext,
        item_id: CmsId,
    ) -> CmsResult<CommandResult> {
        let result = sqlx::query("DELETE FROM cms_feed_item WHERE tenant_id = $1 AND id = $2")
            .bind(ctx.tenant_id)
            .bind(item_id)
            .execute(self.pool())
            .await
            .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(sdkwork_content_cms_service::error::CmsError::not_found("feed item"));
        }

        Ok(CommandResult {
            ok: true,
            resource_id: Some(item_id),
            request_id: Some(ctx.request_id.clone()),
        })
    }

    pub async fn retrieve_feed_snapshot(
        &self,
        ctx: &CmsRequestContext,
        snapshot_id: CmsId,
    ) -> CmsResult<CmsFeedSnapshot> {
        let row: (i64, i64, Option<i64>, i64, i32, String, i32, String) = sqlx::query_as(
            "SELECT id, feed_id, publish_snapshot_id, snapshot_version, item_count, items_json, status, published_at 
             FROM cms_feed_snapshot WHERE tenant_id = $1 AND id = $2"
        )
        .bind(ctx.tenant_id)
        .bind(snapshot_id)
        .fetch_optional(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?
        .ok_or(sdkwork_content_cms_service::error::CmsError::not_found("feed snapshot"))?;

        Ok(CmsFeedSnapshot {
            id: row.0,
            feed_id: row.1,
            publish_snapshot_id: row.2,
            snapshot_version: row.3,
            item_count: row.4,
            items_json: row.5,
            status: row.6,
            published_at: row.7,
        })
    }

    pub async fn publish_feed(
        &self,
        ctx: &CmsRequestContext,
        command: PublishCommand,
    ) -> CmsResult<CmsPublishSnapshot> {
        let now = self.current_timestamp();
        let snapshot_id = self.generate_id();
        let snapshot_uuid = self.generate_uuid();

        let feed = self.retrieve_feed(ctx, command.owner_id).await?;

        let row = sqlx::query_as::<_, crate::db::rows::CmsPublishSnapshotRow>(
            "INSERT INTO cms_publish_snapshot (id, uuid, tenant_id, organization_id, site_id, owner_type, owner_id, channel_id, locale, snapshot_payload_json, status, published_at, published_by, created_at) 
             VALUES ($1, $2, $3, $4, $5, 'feed', $6, $7, $8, '{}', 1, $9, $10, $9)
             RETURNING id, tenant_id, site_id, owner_type, owner_id, snapshot_payload_json, status, published_at"
        )
        .bind(snapshot_id)
        .bind(&snapshot_uuid)
        .bind(ctx.tenant_id)
        .bind(ctx.organization_id)
        .bind(feed.site_id)
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
