//! CMS backend-api route manifest and adapter skeleton.

pub mod dto;
pub mod error;
pub mod handlers;
pub mod manifest;
pub mod mapper;
pub mod paths;
pub mod routes;

pub use manifest::cms_backend_api_manifest;
pub use routes::build_sdkwork_cms_backend_api_router;

pub fn gateway_mount(state: sdkwork_cms_http_handlers::AppState) -> axum::Router {
    use axum::routing::{delete, get, patch, post, put};
    use sdkwork_cms_http_handlers::handlers;

    axum::Router::new()
        .route(
            paths::SITES,
            get(handlers::list_sites).post(handlers::create_site),
        )
        .route(
            paths::SITE_BY_ID,
            get(handlers::retrieve_site)
                .patch(handlers::update_site)
                .delete(handlers::delete_site),
        )
        .route(
            paths::SITE_CHANNELS,
            get(handlers::list_channels).post(handlers::create_channel),
        )
        .route(
            paths::CHANNEL_BY_ID,
            patch(handlers::update_channel).delete(handlers::delete_channel),
        )
        .route(
            paths::SITE_CONTENT_TYPES,
            get(handlers::list_content_types).post(handlers::create_content_type),
        )
        .route(
            paths::CONTENT_TYPE_BY_ID,
            get(handlers::retrieve_content_type)
                .patch(handlers::update_content_type)
                .delete(handlers::delete_content_type),
        )
        .route(
            paths::CONTENT_TYPE_FIELDS,
            get(handlers::list_content_fields).post(handlers::create_content_field),
        )
        .route(
            paths::CONTENT_FIELD_BY_ID,
            patch(handlers::update_content_field).delete(handlers::delete_content_field),
        )
        .route(
            paths::SITE_TAXONOMIES,
            get(handlers::list_taxonomies).post(handlers::create_taxonomy),
        )
        .route(
            paths::TAXONOMY_BY_ID,
            patch(handlers::update_taxonomy).delete(handlers::delete_taxonomy),
        )
        .route(
            paths::TAXONOMY_TERMS,
            get(handlers::list_taxonomy_terms).post(handlers::create_taxonomy_term),
        )
        .route(
            paths::TAXONOMY_TERM_BY_ID,
            patch(handlers::update_taxonomy_term).delete(handlers::delete_taxonomy_term),
        )
        .route(
            paths::ENTRIES,
            get(handlers::list_entries).post(handlers::create_entry),
        )
        .route(
            paths::ENTRY_BY_ID,
            get(handlers::retrieve_entry)
                .patch(handlers::update_entry)
                .delete(handlers::delete_entry),
        )
        .route(paths::ENTRY_BODY, put(handlers::replace_entry_body))
        .route(paths::ENTRY_FIELDS, put(handlers::replace_entry_fields))
        .route(
            paths::ENTRY_MEDIA,
            get(handlers::list_entry_media).post(handlers::attach_entry_media),
        )
        .route(
            paths::ENTRY_MEDIA_BY_ID,
            delete(handlers::delete_entry_media),
        )
        .route(paths::ENTRY_TERMS, put(handlers::replace_entry_terms))
        .route(paths::ENTRY_VERSIONS, get(handlers::list_entry_versions))
        .route(paths::ENTRY_PUBLISH, post(handlers::publish_entry))
        .route(paths::ENTRY_UNPUBLISH, post(handlers::unpublish_entry))
        .route(paths::ENTRY_ROLLBACK, post(handlers::rollback_entry))
        .route(paths::ENTRY_SCHEDULE, post(handlers::schedule_entry))
        .route(
            paths::PAGES,
            get(handlers::list_pages).post(handlers::create_page),
        )
        .route(
            paths::PAGE_BY_ID,
            get(handlers::retrieve_page)
                .patch(handlers::update_page)
                .delete(handlers::delete_page),
        )
        .route(paths::PAGE_BLOCKS, put(handlers::replace_page_blocks))
        .route(paths::PAGE_PUBLISH, post(handlers::publish_page))
        .route(
            paths::FEEDS,
            get(handlers::list_feeds).post(handlers::create_feed),
        )
        .route(
            paths::FEED_BY_ID,
            get(handlers::retrieve_feed)
                .patch(handlers::update_feed)
                .delete(handlers::delete_feed),
        )
        .route(
            paths::FEED_RULES,
            get(handlers::list_feed_rules).post(handlers::create_feed_rule),
        )
        .route(
            paths::FEED_RULE_BY_ID,
            patch(handlers::update_feed_rule).delete(handlers::delete_feed_rule),
        )
        .route(
            paths::FEED_ITEMS,
            get(handlers::list_feed_items).put(handlers::upsert_feed_items),
        )
        .route(paths::FEED_ITEM_BY_ID, delete(handlers::delete_feed_item))
        .route(paths::FEED_PUBLISH, post(handlers::publish_feed))
        .route(
            paths::FEED_SNAPSHOT_BY_ID,
            get(handlers::retrieve_feed_snapshot),
        )
        .route(paths::AUDIT_LOGS, get(handlers::list_audit_logs))
        .route(paths::OUTBOX_EVENTS, get(handlers::list_outbox_events))
        .route(
            paths::OUTBOX_EVENT_RETRY,
            post(handlers::retry_outbox_event),
        )
        .with_state(state)
}
