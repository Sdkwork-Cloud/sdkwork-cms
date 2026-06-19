use sdkwork_router_cms_open_api::{cms_open_api_manifest, open_route_manifest};
use sdkwork_web_core::RouteAuth;

#[test]
fn cms_open_route_manifest_matches_metadata_routes() {
    let manifest = open_route_manifest();
    let metadata = cms_open_api_manifest();
    assert_eq!(metadata.routes.len(), 5);
    for entry in metadata.routes {
        let matched = manifest
            .match_route(entry.method, entry.path)
            .unwrap_or_else(|| {
                panic!(
                    "missing http route manifest for {} {}",
                    entry.method, entry.path
                )
            });
        assert_eq!(matched.auth, RouteAuth::ApiKey);
        assert_eq!(matched.operation_id, entry.operation_id);
    }
}
