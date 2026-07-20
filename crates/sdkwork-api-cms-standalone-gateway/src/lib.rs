//! CMS standalone gateway library surface.

use axum::Router;

/// Business router from gateway assembly (route crates migrate to Router gateway_mount over time).
pub fn business_router_from_assembly() -> Router {
    sdkwork_api_cms_assembly::assemble_api_router().router
}
