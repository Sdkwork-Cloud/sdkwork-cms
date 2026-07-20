use sdkwork_web_core::{HttpMethod, HttpRoute, HttpRouteManifest};

use crate::paths;

const HTTP_ROUTES: &[HttpRoute] = &[
    HttpRoute::api_key(HttpMethod::Get, paths::ENTRIES, "cms", "cms.entries.list"),
    HttpRoute::api_key(
        HttpMethod::Get,
        paths::ENTRY_BY_ID,
        "cms",
        "cms.entries.retrieve",
    ),
    HttpRoute::api_key(
        HttpMethod::Get,
        paths::ENTRIES_RESOLVE,
        "cms",
        "cms.entries.resolve",
    ),
    HttpRoute::api_key(
        HttpMethod::Get,
        paths::PAGES_RESOLVE,
        "cms",
        "cms.pages.resolve",
    ),
    HttpRoute::api_key(
        HttpMethod::Get,
        paths::FEED_ITEMS,
        "cms",
        "cms.feeds.items.list",
    ),
];

pub fn open_route_manifest() -> HttpRouteManifest {
    HttpRouteManifest::new(HTTP_ROUTES)
}
