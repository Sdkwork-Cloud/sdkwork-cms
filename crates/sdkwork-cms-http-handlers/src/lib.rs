//! Shared CMS HTTP handler state and Axum adapters.

pub mod handlers;

use axum::extract::FromRequestParts;
use axum::http::{request::Parts, StatusCode};
use sdkwork_content_cms_service::context::{CmsLoginScope, CmsRequestContext};
use sdkwork_web_core::{WebLoginScope, WebRequestContext};

#[derive(Clone)]
pub struct AppState {
    pub service: sdkwork_content_cms_service::CmsService,
}

impl AppState {
    pub fn new(service: sdkwork_content_cms_service::CmsService) -> Self {
        Self { service }
    }
}

pub struct CmsHttpRequestContext(pub CmsRequestContext);

impl<S> FromRequestParts<S> for CmsHttpRequestContext
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let web = parts
            .extensions
            .get::<WebRequestContext>()
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
        let principal = web.principal.as_ref();

        let tenant_id = parse_identity(
            principal.map(|value| value.tenant_id()),
            "SDKWORK_CMS_TENANT_ID",
            100_001,
        )?;
        let organization_id = parse_identity(
            principal.and_then(|value| value.organization_id()),
            "SDKWORK_CMS_ORGANIZATION_ID",
            0,
        )?;
        let user_id = principal
            .map(|value| value.user_id().parse::<i64>())
            .transpose()
            .map_err(|_| StatusCode::UNAUTHORIZED)?
            .unwrap_or(0);
        let login_scope = match principal.map(|value| value.login_scope()) {
            Some(WebLoginScope::Organization) => CmsLoginScope::Organization,
            _ => CmsLoginScope::Tenant,
        };

        Ok(Self(CmsRequestContext {
            request_id: web.request_id.0.clone(),
            trace_id: web.trace_id.clone(),
            tenant_id,
            organization_id,
            user_id,
            session_id: principal
                .and_then(|value| value.session_id())
                .map(str::to_owned),
            permissions: principal
                .map(|value| value.scopes.permission_scope.clone())
                .unwrap_or_default(),
            data_scope: 0,
            login_scope,
        }))
    }
}

fn parse_identity(
    principal_value: Option<&str>,
    environment_key: &str,
    default_value: i64,
) -> Result<i64, StatusCode> {
    let value = principal_value
        .map(str::to_owned)
        .or_else(|| std::env::var(environment_key).ok())
        .unwrap_or_else(|| default_value.to_string());
    value.parse().map_err(|_| StatusCode::UNAUTHORIZED)
}
