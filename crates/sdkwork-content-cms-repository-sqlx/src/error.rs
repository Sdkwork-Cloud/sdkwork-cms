use thiserror::Error;

#[derive(Debug, Error)]
pub enum CmsRepositoryError {
    #[error("not implemented: {0}")]
    NotImplemented(&'static str),

    #[error("query error: {0}")]
    Query(String),

    #[error("mapping error: {0}")]
    Mapping(String),

    #[error("database error: {0}")]
    Database(String),

    #[error("optimistic lock conflict for {resource} id={resource_id}, expected version={expected_version}")]
    OptimisticLockConflict {
        resource: &'static str,
        resource_id: i64,
        expected_version: i64,
    },

    #[error("not found: {0}")]
    NotFound(&'static str),

    #[error("conflict: {0}")]
    Conflict(String),
}

pub type CmsRepositoryResult<T> = Result<T, CmsRepositoryError>;

impl CmsRepositoryError {
    pub fn not_implemented(scope: &'static str) -> Self {
        CmsRepositoryError::NotImplemented(scope)
    }
}

impl From<CmsRepositoryError> for sdkwork_content_cms_service::error::CmsError {
    fn from(err: CmsRepositoryError) -> Self {
        match err {
            CmsRepositoryError::NotImplemented(s) => {
                sdkwork_content_cms_service::error::CmsError::NotImplemented(s)
            }
            CmsRepositoryError::Query(msg) => {
                sdkwork_content_cms_service::error::CmsError::Internal(msg)
            }
            CmsRepositoryError::Mapping(msg) => {
                sdkwork_content_cms_service::error::CmsError::Internal(msg)
            }
            CmsRepositoryError::Database(msg) => {
                sdkwork_content_cms_service::error::CmsError::Internal(msg)
            }
            CmsRepositoryError::OptimisticLockConflict {
                resource,
                resource_id,
                expected_version,
            } => sdkwork_content_cms_service::error::CmsError::OptimisticLockConflict {
                resource,
                resource_id,
                expected_version,
            },
            CmsRepositoryError::NotFound(s) => {
                sdkwork_content_cms_service::error::CmsError::NotFound(s)
            }
            CmsRepositoryError::Conflict(msg) => {
                sdkwork_content_cms_service::error::CmsError::Conflict(msg)
            }
        }
    }
}
