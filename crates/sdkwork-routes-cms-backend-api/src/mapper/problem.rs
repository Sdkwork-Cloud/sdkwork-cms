use crate::dto::response::ProblemDetail;
use sdkwork_content_cms_service::error::CmsError;

pub fn map_cms_error_to_problem(err: &CmsError, trace_id: Option<String>) -> ProblemDetail {
    match err {
        CmsError::NotImplemented(scope) => ProblemDetail {
            problem_type: "https://sdkwork.com/errors/not-implemented".to_string(),
            title: "Not Implemented".to_string(),
            status: 501,
            detail: format!("Not implemented: {}", scope),
            instance: None,
            trace_id,
        },
        CmsError::Validation(msg) => ProblemDetail::validation(msg, trace_id),
        CmsError::PermissionDenied(perm) => ProblemDetail::permission_denied(perm, trace_id),
        CmsError::NotFound(resource) => ProblemDetail::not_found(resource, trace_id),
        CmsError::Conflict(msg) => ProblemDetail::conflict(msg, trace_id),
        CmsError::DependencyUnavailable(dep) => ProblemDetail {
            problem_type: "https://sdkwork.com/errors/dependency-unavailable".to_string(),
            title: "Dependency Unavailable".to_string(),
            status: 503,
            detail: format!("Dependency unavailable: {}", dep),
            instance: None,
            trace_id,
        },
        CmsError::OptimisticLockConflict {
            resource,
            resource_id,
            expected_version,
        } => ProblemDetail::optimistic_lock_conflict(
            resource,
            *resource_id,
            *expected_version,
            trace_id,
        ),
        CmsError::PreconditionFailed(msg) => ProblemDetail {
            problem_type: "https://sdkwork.com/errors/precondition-failed".to_string(),
            title: "Precondition Failed".to_string(),
            status: 412,
            detail: msg.clone(),
            instance: None,
            trace_id,
        },
        CmsError::Internal(msg) => ProblemDetail::internal(msg, trace_id),
    }
}
