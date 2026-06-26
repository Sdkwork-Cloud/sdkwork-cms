use crate::manifest::{cms_open_api_manifest, RouteManifest};

/// Build the CMS open-api route manifest.
///
/// Returns the route manifest for SDK generation and contract validation.
/// When a framework router is needed, construct it here while preserving
/// the manifest export for SDK materialization.
pub fn build_sdkwork_cms_open_api_router() -> RouteManifest {
    cms_open_api_manifest()
}
