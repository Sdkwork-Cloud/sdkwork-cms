use thiserror::Error;

#[derive(Clone, Debug, Error, PartialEq)]
pub enum CmsError {
    #[error("not implemented: {0}")]
    NotImplemented(&'static str),

    #[error("validation error: {0}")]
    Validation(String),

    #[error("permission denied: {0}")]
    PermissionDenied(&'static str),

    #[error("not found: {0}")]
    NotFound(&'static str),

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("dependency unavailable: {0}")]
    DependencyUnavailable(&'static str),

    #[error("optimistic lock conflict for {resource} id={resource_id}, expected version={expected_version}")]
    OptimisticLockConflict {
        resource: &'static str,
        resource_id: i64,
        expected_version: i64,
    },

    #[error("precondition failed: {0}")]
    PreconditionFailed(String),

    #[error("internal error: {0}")]
    Internal(String),
}

pub type CmsResult<T> = Result<T, CmsError>;

impl CmsError {
    pub fn not_implemented(scope: &'static str) -> Self {
        CmsError::NotImplemented(scope)
    }

    pub fn validation(msg: impl Into<String>) -> Self {
        CmsError::Validation(msg.into())
    }

    pub fn conflict(msg: impl Into<String>) -> Self {
        CmsError::Conflict(msg.into())
    }

    pub fn not_found(resource: &'static str) -> Self {
        CmsError::NotFound(resource)
    }

    pub fn permission_denied(permission: &'static str) -> Self {
        CmsError::PermissionDenied(permission)
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        CmsError::Internal(msg.into())
    }
}
