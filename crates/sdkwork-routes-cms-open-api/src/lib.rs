pub mod dto;
pub mod error;
pub mod handlers;
pub mod http_route_manifest;
pub mod manifest;
pub mod mapper;
pub mod paths;
pub mod routes;
pub mod web_bootstrap;

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

pub fn gateway_mount() -> RouteManifest {
    build_sdkwork_cms_open_api_router()
}
