use sdkwork_content_cms_service::error::CmsError;

use super::super::dto::response::ProblemDetailResponse;

pub fn map_error_to_problem_detail(
    err: &CmsError,
    request_id: Option<String>,
) -> ProblemDetailResponse {
    let (status, title, detail) = match err {
        CmsError::NotImplemented(scope) => (
            501,
            "Not Implemented",
            format!("Not implemented: {}", scope),
        ),
        CmsError::Validation(msg) => (400, "Validation Error", msg.clone()),
        CmsError::PermissionDenied(perm) => (
            403,
            "Permission Denied",
            format!("Missing permission: {}", perm),
        ),
        CmsError::NotFound(resource) => (404, "Not Found", format!("{} not found", resource)),
        CmsError::Conflict(msg) => (409, "Conflict", msg.clone()),
        CmsError::DependencyUnavailable(dep) => (
            503,
            "Service Unavailable",
            format!("Dependency unavailable: {}", dep),
        ),
        CmsError::OptimisticLockConflict {
            resource,
            resource_id,
            expected_version,
        } => (
            409,
            "Conflict",
            format!(
                "{} with id {} has been modified (expected version {})",
                resource, resource_id, expected_version
            ),
        ),
        CmsError::PreconditionFailed(msg) => (412, "Precondition Failed", msg.clone()),
        CmsError::Internal(msg) => (500, "Internal Server Error", msg.clone()),
    };

    ProblemDetailResponse {
        r#type: format!(
            "https://sdkwork.com/errors/{}",
            title.to_lowercase().replace(' ', "-")
        ),
        title: title.to_string(),
        status,
        detail,
        instance: None,
        request_id,
    }
}
