//! Application API assembly bootstrap for sdkwork-cms.

use std::sync::Arc;

use async_trait::async_trait;
use axum::Router;
use sdkwork_cms_http_handlers::AppState;
use sdkwork_content_cms_repository_sqlx::{
    connect_and_bootstrap_cms_database_from_env, CmsSqlxRepository,
};
use sdkwork_content_cms_service::context::CmsRequestContext;
use sdkwork_content_cms_service::domain::CmsOutboxEventDraft;
use sdkwork_content_cms_service::error::CmsResult;
use sdkwork_content_cms_service::ports::{CmsEventPublisher, CmsIamAuthorizer, CmsRepository};
use sdkwork_content_cms_service::CmsService;
use sdkwork_web_core::{WebEnvironment, WebRequestContextProfile};

pub struct ApiAssembly {
    pub router: Router,
    pub readiness_pool: sqlx::PgPool,
}

pub async fn assemble_api_router() -> Result<ApiAssembly, String> {
    let database_host = connect_and_bootstrap_cms_database_from_env()
        .await
        .map_err(|error| format!("CMS database bootstrap failed: {error}"))?;
    let readiness_pool = database_host
        .pool()
        .as_postgres()
        .ok_or_else(|| "CMS requires a PostgreSQL database profile".to_owned())?
        .clone();

    let repository: Arc<dyn CmsRepository + Send + Sync> =
        Arc::new(CmsSqlxRepository::new(readiness_pool.clone()));
    let authorizer: Arc<dyn CmsIamAuthorizer + Send + Sync> = Arc::new(ContextAuthorizer);
    let event_publisher: Arc<dyn CmsEventPublisher + Send + Sync> =
        Arc::new(RepositoryEventPublisher {
            repository: Arc::clone(&repository),
        });
    let state = AppState::new(CmsService::new(repository, authorizer, event_publisher));

    let business_router = Router::new()
        .merge(sdkwork_routes_cms_app_api::gateway_mount(state.clone()))
        .merge(sdkwork_routes_cms_backend_api::gateway_mount(state.clone()))
        .merge(sdkwork_routes_cms_open_api::gateway_mount(state));
    let resolver = sdkwork_iam_web_adapter::iam_web_request_context_resolver_from_env().await;
    let profile = WebRequestContextProfile {
        open_api_prefixes: vec!["/cms/v3/api".to_owned()],
        public_path_prefixes: vec![
            "/healthz".to_owned(),
            "/readyz".to_owned(),
            "/metrics".to_owned(),
            "/app/v3/api/cms".to_owned(),
        ],
        environment: web_environment(),
        ..WebRequestContextProfile::default()
    };
    let router = sdkwork_web_axum::with_web_request_context(
        business_router,
        sdkwork_web_axum::WebFrameworkLayer::new(resolver).with_profile(profile),
    );

    Ok(ApiAssembly {
        router,
        readiness_pool,
    })
}

fn web_environment() -> WebEnvironment {
    match std::env::var("SDKWORK_CMS_ENVIRONMENT")
        .unwrap_or_else(|_| "development".to_owned())
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "production" | "prod" => WebEnvironment::Prod,
        "test" | "testing" => WebEnvironment::Test,
        _ => WebEnvironment::Dev,
    }
}

struct ContextAuthorizer;

#[async_trait]
impl CmsIamAuthorizer for ContextAuthorizer {
    async fn require_permission(
        &self,
        context: &CmsRequestContext,
        permission: &'static str,
    ) -> CmsResult<()> {
        context.require_permission(permission)
    }
}

struct RepositoryEventPublisher {
    repository: Arc<dyn CmsRepository + Send + Sync>,
}

#[async_trait]
impl CmsEventPublisher for RepositoryEventPublisher {
    async fn enqueue(
        &self,
        context: &CmsRequestContext,
        event: CmsOutboxEventDraft,
    ) -> CmsResult<()> {
        self.repository
            .create_outbox_event(context, event)
            .await
            .map(|_| ())
    }
}
