//! CMS open-api route manifest and adapter skeleton.

pub mod dto;
pub mod error;
pub mod handlers;
pub mod manifest;
pub mod mapper;
pub mod paths;
pub mod routes;

pub use manifest::cms_open_api_manifest;
pub use routes::build_sdkwork_cms_open_api_router;
