pub const IDX_CMS_ENTRY_LIST: &str = "idx_cms_entry_list";
pub const IDX_CMS_ENTRY_PUBLISHED: &str = "idx_cms_entry_published";
pub const IDX_CMS_PAGE_ROUTE: &str = "idx_cms_page_route";
pub const IDX_CMS_FEED_SITE_STATUS: &str = "idx_cms_feed_site_status";
pub const IDX_CMS_FEED_ITEM_FEED_ORDER: &str = "idx_cms_feed_item_feed_order";
pub const IDX_CMS_PUBLISH_SNAPSHOT_OWNER: &str = "idx_cms_publish_snapshot_owner";
pub const IDX_CMS_AUDIT_LOG_RESOURCE: &str = "idx_cms_audit_log_resource";
pub const IDX_CMS_OUTBOX_EVENT_PENDING: &str = "idx_cms_outbox_event_pending";

pub const CMS_V1_CRITICAL_INDEXES: &[&str] = &[
    IDX_CMS_ENTRY_LIST,
    IDX_CMS_ENTRY_PUBLISHED,
    IDX_CMS_PAGE_ROUTE,
    IDX_CMS_FEED_SITE_STATUS,
    IDX_CMS_FEED_ITEM_FEED_ORDER,
    IDX_CMS_PUBLISH_SNAPSHOT_OWNER,
    IDX_CMS_AUDIT_LOG_RESOURCE,
    IDX_CMS_OUTBOX_EVENT_PENDING,
];
