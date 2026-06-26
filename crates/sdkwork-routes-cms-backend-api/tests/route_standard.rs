use sdkwork_routes_cms_backend_api::cms_backend_api_manifest;

#[test]
fn backend_api_manifest_uses_backend_cms_prefix() {
    let manifest = cms_backend_api_manifest();
    assert_eq!(manifest.prefix, "/backend/v3/api/cms");
    assert!(manifest
        .routes
        .iter()
        .all(|route| route.path.starts_with(manifest.prefix)));
}

#[test]
fn backend_api_manifest_declares_management_routes() {
    let manifest = cms_backend_api_manifest();
    assert!(manifest.routes.len() >= 40);
    assert!(manifest
        .routes
        .iter()
        .any(|route| route.operation_id == "cms.entries.publish"));
    assert!(manifest
        .routes
        .iter()
        .any(|route| route.operation_id == "cms.feeds.publish"));
}
