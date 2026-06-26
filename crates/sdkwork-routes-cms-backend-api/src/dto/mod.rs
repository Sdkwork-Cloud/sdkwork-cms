//! DTO boundary for CMS backend-api route adapters.
//!
//! Backend-api DTOs are aligned with:
//! - Dual-token context (AuthToken + Access-Token)
//! - Permission checks via `CmsIamAuthorizer`
//! - Idempotency-Key support for create/publish/schedule commands
//! - Audit metadata (request_id, trace_id, actor_user_id)

pub mod request;
pub mod response;

pub use request::*;
pub use response::*;
