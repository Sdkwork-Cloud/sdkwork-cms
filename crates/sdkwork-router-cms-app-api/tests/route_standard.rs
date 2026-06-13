use sdkwork_router_cms_app_api::cms_app_api_manifest;

#[test]
fn app_api_manifest_uses_app_cms_prefix() {
    let manifest = cms_app_api_manifest();
    assert_eq!(manifest.prefix, "/app/v3/api/cms");
    assert!(manifest
        .routes
        .iter()
        .all(|route| route.path.starts_with(manifest.prefix)));
}
