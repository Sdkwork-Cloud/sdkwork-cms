use crate::manifest::{cms_backend_api_manifest, RouteManifest};

/// Build the CMS backend-api route manifest.
///
/// Returns the route manifest for SDK generation and contract validation.
/// When a framework router is needed, construct it here while preserving
/// the manifest export for SDK materialization.
pub fn build_sdkwork_cms_backend_api_router() -> RouteManifest {
    cms_backend_api_manifest()
}
