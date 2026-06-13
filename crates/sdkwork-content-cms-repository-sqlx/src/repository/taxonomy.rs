use sdkwork_content_cms_service::context::CmsRequestContext;
use sdkwork_content_cms_service::domain::*;
use sdkwork_content_cms_service::error::CmsResult;

use super::CmsSqlxRepository;

impl CmsSqlxRepository {
    pub async fn list_taxonomies(
        &self,
        ctx: &CmsRequestContext,
        query: ListBySiteQuery,
    ) -> CmsResult<CmsTaxonomyPage> {
        let limit = query.limit.min(100) as i64;
        let rows: Vec<(i64, i64, String, String, String, i32)> = sqlx::query_as(
            "SELECT id, site_id, code, name, taxonomy_kind, status 
             FROM cms_taxonomy WHERE tenant_id = $1 AND site_id = $2 AND deleted_at IS NULL ORDER BY updated_at DESC, id DESC LIMIT $3"
        )
        .bind(ctx.tenant_id)
        .bind(query.site_id)
        .bind(limit)
        .fetch_all(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        let items = rows
            .into_iter()
            .map(|(id, site_id, code, name, taxonomy_kind, status)| CmsTaxonomy {
                id,
                site_id,
                code,
                name,
                taxonomy_kind,
                status,
            })
            .collect();

        Ok(CmsPage {
            items,
            next_cursor: None,
        })
    }

    pub async fn create_taxonomy(
        &self,
        ctx: &CmsRequestContext,
        command: TaxonomyCommand,
    ) -> CmsResult<CmsTaxonomy> {
        let id = self.generate_id();
        let uuid = self.generate_uuid();
        let now = self.current_timestamp();
        let code = command.code.ok_or_else(|| {
            sdkwork_content_cms_service::error::CmsError::validation("code is required")
        })?;
        let name = command.name.ok_or_else(|| {
            sdkwork_content_cms_service::error::CmsError::validation("name is required")
        })?;
        let taxonomy_kind = command.taxonomy_kind.unwrap_or_else(|| "category".to_string());

        let row: (i64, i64, String, String, String, i32) = sqlx::query_as(
            "INSERT INTO cms_taxonomy (id, uuid, tenant_id, organization_id, site_id, code, name, taxonomy_kind, status, created_at, updated_at, created_by, updated_by, version) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 1, $9, $9, $10, $10, 0) 
             RETURNING id, site_id, code, name, taxonomy_kind, status"
        )
        .bind(id)
        .bind(&uuid)
        .bind(ctx.tenant_id)
        .bind(ctx.organization_id)
        .bind(command.site_id)
        .bind(&code)
        .bind(&name)
        .bind(&taxonomy_kind)
        .bind(&now)
        .bind(ctx.user_id)
        .fetch_one(self.pool())
        .await
        .map_err(|e| {
            if e.to_string().contains("uk_cms_taxonomy_site_code") {
                sdkwork_content_cms_service::error::CmsError::conflict(format!(
                    "taxonomy code '{}' already exists in site",
                    code
                ))
            } else {
                sdkwork_content_cms_service::error::CmsError::internal(e.to_string())
            }
        })?;

        Ok(CmsTaxonomy {
            id: row.0,
            site_id: row.1,
            code: row.2,
            name: row.3,
            taxonomy_kind: row.4,
            status: row.5,
        })
    }

    pub async fn update_taxonomy(
        &self,
        ctx: &CmsRequestContext,
        taxonomy_id: CmsId,
        command: TaxonomyCommand,
    ) -> CmsResult<CmsTaxonomy> {
        let now = self.current_timestamp();
        let version = command.version.unwrap_or(0);

        let row: (i64, i64, String, String, String, i32) = sqlx::query_as(
            "UPDATE cms_taxonomy SET 
                code = COALESCE($4, code),
                name = COALESCE($5, name),
                taxonomy_kind = COALESCE($6, taxonomy_kind),
                updated_at = $7,
                updated_by = $8,
                version = version + 1
             WHERE tenant_id = $1 AND id = $2 AND version = $3 AND deleted_at IS NULL
             RETURNING id, site_id, code, name, taxonomy_kind, status"
        )
        .bind(ctx.tenant_id)
        .bind(taxonomy_id)
        .bind(version)
        .bind(&command.code)
        .bind(&command.name)
        .bind(&command.taxonomy_kind)
        .bind(&now)
        .bind(ctx.user_id)
        .fetch_optional(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?
        .ok_or(sdkwork_content_cms_service::error::CmsError::OptimisticLockConflict {
            resource: "taxonomy",
            resource_id: taxonomy_id,
            expected_version: version,
        })?;

        Ok(CmsTaxonomy {
            id: row.0,
            site_id: row.1,
            code: row.2,
            name: row.3,
            taxonomy_kind: row.4,
            status: row.5,
        })
    }

    pub async fn delete_taxonomy(
        &self,
        ctx: &CmsRequestContext,
        taxonomy_id: CmsId,
    ) -> CmsResult<CommandResult> {
        let now = self.current_timestamp();
        let result = sqlx::query(
            "UPDATE cms_taxonomy SET status = 9, deleted_at = $3, deleted_by = $4 WHERE tenant_id = $1 AND id = $2 AND deleted_at IS NULL"
        )
        .bind(ctx.tenant_id)
        .bind(taxonomy_id)
        .bind(&now)
        .bind(ctx.user_id)
        .execute(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(sdkwork_content_cms_service::error::CmsError::not_found("taxonomy"));
        }

        Ok(CommandResult {
            ok: true,
            resource_id: Some(taxonomy_id),
            request_id: Some(ctx.request_id.clone()),
        })
    }

    pub async fn list_taxonomy_terms(
        &self,
        ctx: &CmsRequestContext,
        query: ListTaxonomyTermsQuery,
    ) -> CmsResult<CmsTaxonomyTermPage> {
        let limit = query.limit.min(100) as i64;
        let rows: Vec<(i64, i64, Option<i64>, String, String, String, String, i32)> = sqlx::query_as(
            "SELECT id, taxonomy_id, parent_id, code, slug, name, path, status 
             FROM cms_taxonomy_term WHERE tenant_id = $1 AND taxonomy_id = $2 AND deleted_at IS NULL ORDER BY path, sort_order, id LIMIT $3"
        )
        .bind(ctx.tenant_id)
        .bind(query.taxonomy_id)
        .bind(limit)
        .fetch_all(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        let items = rows
            .into_iter()
            .map(|(id, taxonomy_id, parent_id, code, slug, name, path, status)| CmsTaxonomyTerm {
                id,
                taxonomy_id,
                parent_id,
                code,
                slug,
                name,
                path,
                status,
            })
            .collect();

        Ok(CmsPage {
            items,
            next_cursor: None,
        })
    }

    pub async fn create_taxonomy_term(
        &self,
        ctx: &CmsRequestContext,
        command: TaxonomyTermCommand,
    ) -> CmsResult<CmsTaxonomyTerm> {
        let id = self.generate_id();
        let uuid = self.generate_uuid();
        let now = self.current_timestamp();
        let code = command.code.ok_or_else(|| {
            sdkwork_content_cms_service::error::CmsError::validation("code is required")
        })?;
        let slug = command.slug.unwrap_or_else(|| code.clone());
        let name = command.name.ok_or_else(|| {
            sdkwork_content_cms_service::error::CmsError::validation("name is required")
        })?;

        let (parent_path, level_no) = if let Some(parent_id) = command.parent_id {
            let parent: (String, i32) = sqlx::query_as(
                "SELECT path, level_no FROM cms_taxonomy_term WHERE tenant_id = $1 AND id = $2 AND deleted_at IS NULL"
            )
            .bind(ctx.tenant_id)
            .bind(parent_id)
            .fetch_one(self.pool())
            .await
            .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;
            (parent.0, parent.1 + 1)
        } else {
            ("/".to_string(), 0)
        };
        let path = format!("{}/{}/", parent_path.trim_end_matches('/'), id);

        let row: (i64, i64, Option<i64>, String, String, String, String, i32) = sqlx::query_as(
            "INSERT INTO cms_taxonomy_term (id, uuid, tenant_id, organization_id, site_id, taxonomy_id, parent_id, code, slug, name, path, level_no, status, created_at, updated_at, created_by, updated_by, version) 
             VALUES ($1, $2, $3, $4, (SELECT site_id FROM cms_taxonomy WHERE id = $5 AND tenant_id = $3), $5, $6, $7, $8, $9, $10, $11, 1, $12, $12, $13, $13, 0) 
             RETURNING id, taxonomy_id, parent_id, code, slug, name, path, status"
        )
        .bind(id)
        .bind(&uuid)
        .bind(ctx.tenant_id)
        .bind(ctx.organization_id)
        .bind(command.taxonomy_id)
        .bind(command.parent_id)
        .bind(&code)
        .bind(&slug)
        .bind(&name)
        .bind(&path)
        .bind(level_no)
        .bind(&now)
        .bind(ctx.user_id)
        .fetch_one(self.pool())
        .await
        .map_err(|e| {
            if e.to_string().contains("uk_cms_taxonomy_term_code") {
                sdkwork_content_cms_service::error::CmsError::conflict(format!(
                    "taxonomy term code '{}' already exists",
                    code
                ))
            } else if e.to_string().contains("uk_cms_taxonomy_term_slug") {
                sdkwork_content_cms_service::error::CmsError::conflict(format!(
                    "taxonomy term slug '{}' already exists",
                    slug
                ))
            } else {
                sdkwork_content_cms_service::error::CmsError::internal(e.to_string())
            }
        })?;

        Ok(CmsTaxonomyTerm {
            id: row.0,
            taxonomy_id: row.1,
            parent_id: row.2,
            code: row.3,
            slug: row.4,
            name: row.5,
            path: row.6,
            status: row.7,
        })
    }

    pub async fn update_taxonomy_term(
        &self,
        ctx: &CmsRequestContext,
        term_id: CmsId,
        command: TaxonomyTermCommand,
    ) -> CmsResult<CmsTaxonomyTerm> {
        let now = self.current_timestamp();
        let version = command.version.unwrap_or(0);

        let row: (i64, i64, Option<i64>, String, String, String, String, i32) = sqlx::query_as(
            "UPDATE cms_taxonomy_term SET 
                code = COALESCE($4, code),
                slug = COALESCE($5, slug),
                name = COALESCE($6, name),
                updated_at = $7,
                updated_by = $8,
                version = version + 1
             WHERE tenant_id = $1 AND id = $2 AND version = $3 AND deleted_at IS NULL
             RETURNING id, taxonomy_id, parent_id, code, slug, name, path, status"
        )
        .bind(ctx.tenant_id)
        .bind(term_id)
        .bind(version)
        .bind(&command.code)
        .bind(&command.slug)
        .bind(&command.name)
        .bind(&now)
        .bind(ctx.user_id)
        .fetch_optional(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?
        .ok_or(sdkwork_content_cms_service::error::CmsError::OptimisticLockConflict {
            resource: "taxonomy_term",
            resource_id: term_id,
            expected_version: version,
        })?;

        Ok(CmsTaxonomyTerm {
            id: row.0,
            taxonomy_id: row.1,
            parent_id: row.2,
            code: row.3,
            slug: row.4,
            name: row.5,
            path: row.6,
            status: row.7,
        })
    }

    pub async fn delete_taxonomy_term(
        &self,
        ctx: &CmsRequestContext,
        term_id: CmsId,
    ) -> CmsResult<CommandResult> {
        let now = self.current_timestamp();
        let result = sqlx::query(
            "UPDATE cms_taxonomy_term SET status = 9, deleted_at = $3, deleted_by = $4 WHERE tenant_id = $1 AND id = $2 AND deleted_at IS NULL"
        )
        .bind(ctx.tenant_id)
        .bind(term_id)
        .bind(&now)
        .bind(ctx.user_id)
        .execute(self.pool())
        .await
        .map_err(|e| sdkwork_content_cms_service::error::CmsError::internal(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(sdkwork_content_cms_service::error::CmsError::not_found("taxonomy term"));
        }

        Ok(CommandResult {
            ok: true,
            resource_id: Some(term_id),
            request_id: Some(ctx.request_id.clone()),
        })
    }
}
