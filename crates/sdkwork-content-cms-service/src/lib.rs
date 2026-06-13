//! SDKWork CMS business service contract.
//!
//! This crate owns CMS domain models, commands, results, and service ports.
//! Business rules are implemented in `src/service/` with IAM permission checks
//! and outbox-backed integration events.

pub mod context;
pub mod domain;
pub mod error;
pub mod ports;
pub mod service;

pub use context::CmsRequestContext;
pub use error::{CmsError, CmsResult};
pub use service::CmsService;
