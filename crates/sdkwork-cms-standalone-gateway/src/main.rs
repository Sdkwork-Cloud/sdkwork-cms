use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, patch, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use sdkwork_content_cms_service::context::{CmsLoginScope, CmsRequestContext};
use sdkwork_content_cms_service::domain::*;
use sdkwork_content_cms_service::ports::{CmsEventPublisher, CmsIamAuthorizer, CmsRepository};
use sdkwork_content_cms_service::service::CmsService;
use sdkwork_content_cms_repository_sqlx::{connect_and_bootstrap_cms_database_from_env, CmsSqlxRepository};
use sdkwork_web_bootstrap::{service_router, ServiceRouterConfig};

mod handlers;
mod readiness;

#[derive(Clone)]
struct AppState {
    service: CmsService,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file if it exists
    let _ = dotenvy::dotenv();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    // Create database pool and apply application-root database/ lifecycle.
    let database_host = connect_and_bootstrap_cms_database_from_env()
        .await
        .map_err(|error| format!("CMS database bootstrap failed: {error}"))?;

    let pg_pool = database_host
        .pool()
        .as_postgres()
        .ok_or("Expected PostgreSQL pool for CMS service")?
        .clone();

    let readiness_pool = pg_pool.clone();
    let repository = CmsSqlxRepository::new(pg_pool);

    let repository: Arc<dyn CmsRepository + Send + Sync> = Arc::new(repository);
    let authorizer: Arc<dyn CmsIamAuthorizer + Send + Sync> = Arc::new(MockAuthorizer);
    let event_publisher: Arc<dyn CmsEventPublisher + Send + Sync> = Arc::new(MockEventPublisher);

    let service = CmsService::new(repository, authorizer, event_publisher);

    let state = AppState { service };

    let app = Router::new()
        // Backend API routes
        .route("/backend/v3/api/cms/sites", get(handlers::list_sites).post(handlers::create_site))
        .route("/backend/v3/api/cms/sites/{site_id}", get(handlers::retrieve_site).patch(handlers::update_site).delete(handlers::delete_site))
        .route("/backend/v3/api/cms/sites/{site_id}/channels", get(handlers::list_channels).post(handlers::create_channel))
        .route("/backend/v3/api/cms/channels/{channel_id}", patch(handlers::update_channel).delete(handlers::delete_channel))
        .route("/backend/v3/api/cms/sites/{site_id}/content_types", get(handlers::list_content_types).post(handlers::create_content_type))
        .route("/backend/v3/api/cms/content_types/{content_type_id}", get(handlers::retrieve_content_type).patch(handlers::update_content_type).delete(handlers::delete_content_type))
        .route("/backend/v3/api/cms/content_types/{content_type_id}/fields", get(handlers::list_content_fields).post(handlers::create_content_field))
        .route("/backend/v3/api/cms/content_fields/{field_id}", patch(handlers::update_content_field).delete(handlers::delete_content_field))
        .route("/backend/v3/api/cms/sites/{site_id}/taxonomies", get(handlers::list_taxonomies).post(handlers::create_taxonomy))
        .route("/backend/v3/api/cms/taxonomies/{taxonomy_id}", patch(handlers::update_taxonomy).delete(handlers::delete_taxonomy))
        .route("/backend/v3/api/cms/taxonomies/{taxonomy_id}/terms", get(handlers::list_taxonomy_terms).post(handlers::create_taxonomy_term))
        .route("/backend/v3/api/cms/taxonomy_terms/{term_id}", patch(handlers::update_taxonomy_term).delete(handlers::delete_taxonomy_term))
        .route("/backend/v3/api/cms/entries", get(handlers::list_entries).post(handlers::create_entry))
        .route("/backend/v3/api/cms/entries/{entry_id}", get(handlers::retrieve_entry).patch(handlers::update_entry).delete(handlers::delete_entry))
        .route("/backend/v3/api/cms/entries/{entry_id}/body", put(handlers::replace_entry_body))
        .route("/backend/v3/api/cms/entries/{entry_id}/fields", put(handlers::replace_entry_fields))
        .route("/backend/v3/api/cms/entries/{entry_id}/media", get(handlers::list_entry_media).post(handlers::attach_entry_media))
        .route("/backend/v3/api/cms/entries/{entry_id}/terms", put(handlers::replace_entry_terms))
        .route("/backend/v3/api/cms/entries/{entry_id}/versions", get(handlers::list_entry_versions))
        .route("/backend/v3/api/cms/entries/{entry_id}/publish", post(handlers::publish_entry))
        .route("/backend/v3/api/cms/entries/{entry_id}/unpublish", post(handlers::unpublish_entry))
        .route("/backend/v3/api/cms/entries/{entry_id}/rollback", post(handlers::rollback_entry))
        .route("/backend/v3/api/cms/entries/{entry_id}/schedule", post(handlers::schedule_entry))
        .route("/backend/v3/api/cms/pages", get(handlers::list_pages).post(handlers::create_page))
        .route("/backend/v3/api/cms/pages/{page_id}", get(handlers::retrieve_page).patch(handlers::update_page).delete(handlers::delete_page))
        .route("/backend/v3/api/cms/pages/{page_id}/blocks", put(handlers::replace_page_blocks))
        .route("/backend/v3/api/cms/pages/{page_id}/publish", post(handlers::publish_page))
        .route("/backend/v3/api/cms/feeds", get(handlers::list_feeds).post(handlers::create_feed))
        .route("/backend/v3/api/cms/feeds/{feed_id}", get(handlers::retrieve_feed).patch(handlers::update_feed).delete(handlers::delete_feed))
        .route("/backend/v3/api/cms/feeds/{feed_id}/rules", get(handlers::list_feed_rules).post(handlers::create_feed_rule))
        .route("/backend/v3/api/cms/feeds/{feed_id}/items", get(handlers::list_feed_items))
        .route("/backend/v3/api/cms/feeds/{feed_id}/publish", post(handlers::publish_feed))
        .route("/backend/v3/api/cms/feeds/{feed_id}/snapshots/{snapshot_id}", get(handlers::retrieve_feed_snapshot))
        .route("/backend/v3/api/cms/audit_logs", get(handlers::list_audit_logs))
        .route("/backend/v3/api/cms/outbox_events", get(handlers::list_outbox_events))
        // App API routes (delivery)
        .route("/app/v3/api/cms/sites/{site_code}/bootstrap", get(handlers::delivery_bootstrap))
        .route("/app/v3/api/cms/sites/{site_code}/entries/resolve", get(handlers::delivery_resolve_entry))
        .route("/app/v3/api/cms/entries/{entry_id}", get(handlers::delivery_retrieve_entry))
        .route("/app/v3/api/cms/sites/{site_code}/pages/resolve", get(handlers::delivery_resolve_page))
        .route("/app/v3/api/cms/sites/{site_code}/feeds/{feed_code}/items", get(handlers::delivery_list_feed_items))
        // Open API routes (delivery)
        .route("/cms/v3/api/entries", get(handlers::open_list_entries))
        .route("/cms/v3/api/entries/{entry_id}", get(handlers::open_retrieve_entry))
        .route("/cms/v3/api/entries/resolve", get(handlers::open_resolve_entry))
        .route("/cms/v3/api/pages/resolve", get(handlers::open_resolve_page))
        .route("/cms/v3/api/feeds/{feed_code}/items", get(handlers::open_list_feed_items))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let app = service_router(
        app,
        ServiceRouterConfig::default().with_readiness_check(Arc::new(
            readiness::CmsPostgresReadinessCheck::new(readiness_pool),
        )),
    );

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("Starting CMS API server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// Mock implementations for testing
struct MockAuthorizer;

#[async_trait::async_trait]
impl CmsIamAuthorizer for MockAuthorizer {
    async fn require_permission(
        &self,
        _ctx: &CmsRequestContext,
        _permission: &'static str,
    ) -> sdkwork_content_cms_service::error::CmsResult<()> {
        Ok(())
    }
}

struct MockEventPublisher;

#[async_trait::async_trait]
impl CmsEventPublisher for MockEventPublisher {
    async fn enqueue(
        &self,
        _ctx: &CmsRequestContext,
        _event: CmsOutboxEventDraft,
    ) -> sdkwork_content_cms_service::error::CmsResult<()> {
        Ok(())
    }
}
