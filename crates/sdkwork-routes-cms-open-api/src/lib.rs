pub mod dto;
pub mod error;
pub mod handlers;
pub mod http_route_manifest;
pub mod manifest;
pub mod mapper;
pub mod paths;
pub mod routes;
pub mod web_bootstrap;

use sdkwork_web_core::HttpRouteManifest;

pub use http_route_manifest::open_route_manifest;
pub use manifest::cms_open_api_manifest;
pub use routes::build_sdkwork_cms_open_api_router;
pub use web_bootstrap::{
    cms_open_api_prefixes, cms_open_api_public_path_prefixes, wrap_router_with_web_framework,
    wrap_router_with_web_framework_from_env,
};

pub fn gateway_route_manifest() -> HttpRouteManifest {
    open_route_manifest()
}

pub fn gateway_mount(state: sdkwork_cms_http_handlers::AppState) -> axum::Router {
    use axum::routing::get;
    use sdkwork_cms_http_handlers::handlers;

    axum::Router::new()
        .route(paths::ENTRIES, get(handlers::open_list_entries))
        .route(paths::ENTRY_BY_ID, get(handlers::open_retrieve_entry))
        .route(paths::ENTRIES_RESOLVE, get(handlers::open_resolve_entry))
        .route(paths::PAGES_RESOLVE, get(handlers::open_resolve_page))
        .route(paths::FEED_ITEMS, get(handlers::open_list_feed_items))
        .with_state(state)
}
