use sdkwork_routes_cms_open_api::cms_open_api_manifest;

#[test]
fn open_api_manifest_uses_cms_prefix() {
    let manifest = cms_open_api_manifest();
    assert_eq!(manifest.prefix, "/cms/v3/api");
    assert!(manifest
        .routes
        .iter()
        .all(|route| route.path.starts_with(manifest.prefix)));
}
