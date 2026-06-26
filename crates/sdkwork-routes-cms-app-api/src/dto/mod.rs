//! DTO boundary for CMS app-api route adapters.
//!
//! App-api DTOs are aligned with:
//! - Anonymous delivery for public content
//! - Preview-token delivery for draft content via `CmsPreviewTokenPort`
//! - Site-code based resolution (not site-id)

pub mod request;
pub mod response;

pub use request::*;
pub use response::*;
