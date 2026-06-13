#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CmsRequestContext {
    pub request_id: String,
    pub trace_id: Option<String>,
    pub tenant_id: i64,
    pub organization_id: i64,
    pub user_id: i64,
    pub session_id: Option<String>,
    pub permissions: Vec<String>,
    pub data_scope: i32,
    pub login_scope: CmsLoginScope,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CmsLoginScope {
    Tenant,
    Organization,
}

impl CmsRequestContext {
    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.iter().any(|value| value == permission)
    }

    pub fn require_permission(&self, permission: &'static str) -> Result<(), crate::error::CmsError> {
        if self.has_permission(permission) {
            Ok(())
        } else {
            Err(crate::error::CmsError::permission_denied(permission))
        }
    }
}
