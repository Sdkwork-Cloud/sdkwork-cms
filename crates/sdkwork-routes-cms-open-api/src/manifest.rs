use crate::paths;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RouteDefinition {
    pub method: &'static str,
    pub path: &'static str,
    pub operation_id: &'static str,
    pub permission: Option<&'static str>,
    pub auth_mode: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RouteManifest {
    pub schema_version: u32,
    pub kind: &'static str,
    pub package_name: &'static str,
    pub surface: &'static str,
    pub owner: &'static str,
    pub domain: &'static str,
    pub capability: &'static str,
    pub api_authority: &'static str,
    pub sdk_family: &'static str,
    pub prefix: &'static str,
    pub routes: Vec<RouteDefinition>,
}

pub fn cms_open_api_manifest() -> RouteManifest {
    RouteManifest {
        schema_version: 1,
        kind: "sdkwork.route.manifest",
        package_name: "sdkwork-routes-cms-open-api",
        surface: "open-api",
        owner: "sdkwork-cms",
        domain: "content",
        capability: "cms",
        api_authority: "sdkwork-cms.open",
        sdk_family: "sdkwork-cms-sdk",
        prefix: paths::PREFIX,
        routes: vec![
            RouteDefinition {
                method: "GET",
                path: paths::ENTRIES,
                operation_id: "cms.entries.list",
                permission: None,
                auth_mode: "api-key",
            },
            RouteDefinition {
                method: "GET",
                path: paths::ENTRY_BY_ID,
                operation_id: "cms.entries.retrieve",
                permission: None,
                auth_mode: "api-key",
            },
            RouteDefinition {
                method: "GET",
                path: paths::ENTRIES_RESOLVE,
                operation_id: "cms.entries.resolve",
                permission: None,
                auth_mode: "api-key",
            },
            RouteDefinition {
                method: "GET",
                path: paths::PAGES_RESOLVE,
                operation_id: "cms.pages.resolve",
                permission: None,
                auth_mode: "api-key",
            },
            RouteDefinition {
                method: "GET",
                path: paths::FEED_ITEMS,
                operation_id: "cms.feeds.items.list",
                permission: None,
                auth_mode: "api-key",
            },
        ],
    }
}
