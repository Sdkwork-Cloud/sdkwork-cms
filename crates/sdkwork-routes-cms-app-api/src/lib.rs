//! CMS app-api route manifest and adapter skeleton.

pub mod dto;
pub mod error;
pub mod handlers;
pub mod manifest;
pub mod mapper;
pub mod paths;
pub mod routes;

pub use manifest::cms_app_api_manifest;
pub use routes::build_sdkwork_cms_app_api_router;

pub fn gateway_mount() -> RouteManifest {
    build_sdkwork_cms_app_api_router()
}
