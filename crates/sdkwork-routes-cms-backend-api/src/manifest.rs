use crate::paths;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RouteDefinition {
    pub method: &'static str,
    pub path: &'static str,
    pub operation_id: &'static str,
    pub permission: &'static str,
    pub auth_mode: &'static str,
    pub idempotent: bool,
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

pub fn cms_backend_api_manifest() -> RouteManifest {
    let mut routes = Vec::new();
    push_crud_routes(&mut routes);
    push_entry_routes(&mut routes);
    push_page_routes(&mut routes);
    push_feed_routes(&mut routes);
    push_governance_routes(&mut routes);

    RouteManifest {
        schema_version: 1,
        kind: "sdkwork.route.manifest",
        package_name: "sdkwork-routes-cms-backend-api",
        surface: "backend-api",
        owner: "sdkwork-cms",
        domain: "content",
        capability: "cms",
        api_authority: "sdkwork-cms.backend",
        sdk_family: "sdkwork-cms-backend-sdk",
        prefix: paths::PREFIX,
        routes,
    }
}

fn route(
    method: &'static str,
    path: &'static str,
    operation_id: &'static str,
    permission: &'static str,
    idempotent: bool,
) -> RouteDefinition {
    RouteDefinition {
        method,
        path,
        operation_id,
        permission,
        auth_mode: "dual-token",
        idempotent,
    }
}

fn push_crud_routes(routes: &mut Vec<RouteDefinition>) {
    routes.extend([
        route(
            "GET",
            paths::SITES,
            "cms.sites.list",
            "cms.site.read",
            false,
        ),
        route(
            "POST",
            paths::SITES,
            "cms.sites.create",
            "cms.site.manage",
            true,
        ),
        route(
            "GET",
            paths::SITE_BY_ID,
            "cms.sites.retrieve",
            "cms.site.read",
            false,
        ),
        route(
            "PATCH",
            paths::SITE_BY_ID,
            "cms.sites.update",
            "cms.site.manage",
            false,
        ),
        route(
            "DELETE",
            paths::SITE_BY_ID,
            "cms.sites.delete",
            "cms.site.manage",
            false,
        ),
        route(
            "GET",
            paths::SITE_CHANNELS,
            "cms.channels.list",
            "cms.channel.read",
            false,
        ),
        route(
            "POST",
            paths::SITE_CHANNELS,
            "cms.channels.create",
            "cms.channel.manage",
            true,
        ),
        route(
            "PATCH",
            paths::CHANNEL_BY_ID,
            "cms.channels.update",
            "cms.channel.manage",
            false,
        ),
        route(
            "DELETE",
            paths::CHANNEL_BY_ID,
            "cms.channels.delete",
            "cms.channel.manage",
            false,
        ),
        route(
            "GET",
            paths::SITE_CONTENT_TYPES,
            "cms.contentTypes.list",
            "cms.content_type.read",
            false,
        ),
        route(
            "POST",
            paths::SITE_CONTENT_TYPES,
            "cms.contentTypes.create",
            "cms.content_type.manage",
            true,
        ),
        route(
            "GET",
            paths::CONTENT_TYPE_BY_ID,
            "cms.contentTypes.retrieve",
            "cms.content_type.read",
            false,
        ),
        route(
            "PATCH",
            paths::CONTENT_TYPE_BY_ID,
            "cms.contentTypes.update",
            "cms.content_type.manage",
            false,
        ),
        route(
            "DELETE",
            paths::CONTENT_TYPE_BY_ID,
            "cms.contentTypes.delete",
            "cms.content_type.manage",
            false,
        ),
        route(
            "GET",
            paths::CONTENT_TYPE_FIELDS,
            "cms.contentFields.list",
            "cms.content_type.read",
            false,
        ),
        route(
            "POST",
            paths::CONTENT_TYPE_FIELDS,
            "cms.contentFields.create",
            "cms.content_type.manage",
            true,
        ),
        route(
            "PATCH",
            paths::CONTENT_FIELD_BY_ID,
            "cms.contentFields.update",
            "cms.content_type.manage",
            false,
        ),
        route(
            "DELETE",
            paths::CONTENT_FIELD_BY_ID,
            "cms.contentFields.delete",
            "cms.content_type.manage",
            false,
        ),
        route(
            "GET",
            paths::SITE_TAXONOMIES,
            "cms.taxonomies.list",
            "cms.taxonomy.read",
            false,
        ),
        route(
            "POST",
            paths::SITE_TAXONOMIES,
            "cms.taxonomies.create",
            "cms.taxonomy.manage",
            true,
        ),
        route(
            "PATCH",
            paths::TAXONOMY_BY_ID,
            "cms.taxonomies.update",
            "cms.taxonomy.manage",
            false,
        ),
        route(
            "DELETE",
            paths::TAXONOMY_BY_ID,
            "cms.taxonomies.delete",
            "cms.taxonomy.manage",
            false,
        ),
        route(
            "GET",
            paths::TAXONOMY_TERMS,
            "cms.taxonomyTerms.list",
            "cms.taxonomy.read",
            false,
        ),
        route(
            "POST",
            paths::TAXONOMY_TERMS,
            "cms.taxonomyTerms.create",
            "cms.taxonomy.manage",
            true,
        ),
        route(
            "PATCH",
            paths::TAXONOMY_TERM_BY_ID,
            "cms.taxonomyTerms.update",
            "cms.taxonomy.manage",
            false,
        ),
        route(
            "DELETE",
            paths::TAXONOMY_TERM_BY_ID,
            "cms.taxonomyTerms.delete",
            "cms.taxonomy.manage",
            false,
        ),
    ]);
}

fn push_entry_routes(routes: &mut Vec<RouteDefinition>) {
    routes.extend([
        route(
            "GET",
            paths::ENTRIES,
            "cms.entries.management.list",
            "cms.entry.read",
            false,
        ),
        route(
            "POST",
            paths::ENTRIES,
            "cms.entries.create",
            "cms.entry.create",
            true,
        ),
        route(
            "GET",
            paths::ENTRY_BY_ID,
            "cms.entries.management.retrieve",
            "cms.entry.read",
            false,
        ),
        route(
            "PATCH",
            paths::ENTRY_BY_ID,
            "cms.entries.update",
            "cms.entry.update",
            false,
        ),
        route(
            "DELETE",
            paths::ENTRY_BY_ID,
            "cms.entries.delete",
            "cms.entry.delete",
            false,
        ),
        route(
            "PUT",
            paths::ENTRY_BODY,
            "cms.entries.body.update",
            "cms.entry.update",
            false,
        ),
        route(
            "PUT",
            paths::ENTRY_FIELDS,
            "cms.entries.fields.replace",
            "cms.entry.update",
            false,
        ),
        route(
            "GET",
            paths::ENTRY_MEDIA,
            "cms.entries.media.list",
            "cms.entry.read",
            false,
        ),
        route(
            "POST",
            paths::ENTRY_MEDIA,
            "cms.entries.media.attach",
            "cms.entry.update",
            true,
        ),
        route(
            "DELETE",
            paths::ENTRY_MEDIA_BY_ID,
            "cms.entries.media.delete",
            "cms.entry.update",
            false,
        ),
        route(
            "PUT",
            paths::ENTRY_TERMS,
            "cms.entries.terms.replace",
            "cms.entry.update",
            false,
        ),
        route(
            "GET",
            paths::ENTRY_VERSIONS,
            "cms.entries.versions.list",
            "cms.entry.read",
            false,
        ),
        route(
            "POST",
            paths::ENTRY_PUBLISH,
            "cms.entries.publish",
            "cms.entry.publish",
            true,
        ),
        route(
            "POST",
            paths::ENTRY_UNPUBLISH,
            "cms.entries.unpublish",
            "cms.entry.publish",
            true,
        ),
        route(
            "POST",
            paths::ENTRY_ROLLBACK,
            "cms.entries.rollback",
            "cms.entry.rollback",
            true,
        ),
        route(
            "POST",
            paths::ENTRY_SCHEDULE,
            "cms.entries.schedule",
            "cms.entry.publish",
            true,
        ),
    ]);
}

fn push_page_routes(routes: &mut Vec<RouteDefinition>) {
    routes.extend([
        route(
            "GET",
            paths::PAGES,
            "cms.pages.management.list",
            "cms.page.read",
            false,
        ),
        route(
            "POST",
            paths::PAGES,
            "cms.pages.create",
            "cms.page.manage",
            true,
        ),
        route(
            "GET",
            paths::PAGE_BY_ID,
            "cms.pages.management.retrieve",
            "cms.page.read",
            false,
        ),
        route(
            "PATCH",
            paths::PAGE_BY_ID,
            "cms.pages.update",
            "cms.page.manage",
            false,
        ),
        route(
            "DELETE",
            paths::PAGE_BY_ID,
            "cms.pages.delete",
            "cms.page.manage",
            false,
        ),
        route(
            "PUT",
            paths::PAGE_BLOCKS,
            "cms.pages.blocks.replace",
            "cms.page.manage",
            false,
        ),
        route(
            "POST",
            paths::PAGE_PUBLISH,
            "cms.pages.publish",
            "cms.page.publish",
            true,
        ),
    ]);
}

fn push_feed_routes(routes: &mut Vec<RouteDefinition>) {
    routes.extend([
        route(
            "GET",
            paths::FEEDS,
            "cms.feeds.management.list",
            "cms.feed.read",
            false,
        ),
        route(
            "POST",
            paths::FEEDS,
            "cms.feeds.create",
            "cms.feed.manage",
            true,
        ),
        route(
            "GET",
            paths::FEED_BY_ID,
            "cms.feeds.management.retrieve",
            "cms.feed.read",
            false,
        ),
        route(
            "PATCH",
            paths::FEED_BY_ID,
            "cms.feeds.update",
            "cms.feed.manage",
            false,
        ),
        route(
            "DELETE",
            paths::FEED_BY_ID,
            "cms.feeds.delete",
            "cms.feed.manage",
            false,
        ),
        route(
            "GET",
            paths::FEED_RULES,
            "cms.feedRules.list",
            "cms.feed.read",
            false,
        ),
        route(
            "POST",
            paths::FEED_RULES,
            "cms.feedRules.create",
            "cms.feed.manage",
            true,
        ),
        route(
            "PATCH",
            paths::FEED_RULE_BY_ID,
            "cms.feedRules.update",
            "cms.feed.manage",
            false,
        ),
        route(
            "DELETE",
            paths::FEED_RULE_BY_ID,
            "cms.feedRules.delete",
            "cms.feed.manage",
            false,
        ),
        route(
            "GET",
            paths::FEED_ITEMS,
            "cms.feedItems.list",
            "cms.feed.read",
            false,
        ),
        route(
            "PUT",
            paths::FEED_ITEMS,
            "cms.feedItems.upsert",
            "cms.feed.manage",
            true,
        ),
        route(
            "DELETE",
            paths::FEED_ITEM_BY_ID,
            "cms.feedItems.delete",
            "cms.feed.manage",
            false,
        ),
        route(
            "POST",
            paths::FEED_PUBLISH,
            "cms.feeds.publish",
            "cms.feed.publish",
            true,
        ),
        route(
            "GET",
            paths::FEED_SNAPSHOT_BY_ID,
            "cms.feeds.snapshots.retrieve",
            "cms.feed.read",
            false,
        ),
    ]);
}

fn push_governance_routes(routes: &mut Vec<RouteDefinition>) {
    routes.extend([
        route(
            "GET",
            paths::AUDIT_LOGS,
            "cms.auditLogs.list",
            "cms.audit.read",
            false,
        ),
        route(
            "GET",
            paths::OUTBOX_EVENTS,
            "cms.outboxEvents.list",
            "cms.audit.read",
            false,
        ),
        route(
            "POST",
            paths::OUTBOX_EVENT_RETRY,
            "cms.outboxEvents.retry",
            "cms.audit.manage",
            true,
        ),
    ]);
}
