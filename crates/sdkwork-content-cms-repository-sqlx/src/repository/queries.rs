pub const LIST_SITES: &str = "SELECT id, uuid, tenant_id, organization_id, code, name, default_locale, settings_json, status, version FROM cms_site WHERE tenant_id = $1 AND deleted_at IS NULL ORDER BY updated_at DESC, id DESC LIMIT $2";
pub const RETRIEVE_SITE: &str = "SELECT id, uuid, tenant_id, organization_id, code, name, default_locale, settings_json, status, version FROM cms_site WHERE tenant_id = $1 AND id = $2 AND deleted_at IS NULL";
pub const CREATE_SITE: &str = "INSERT INTO cms_site (id, uuid, tenant_id, organization_id, code, name, default_locale, settings_json, status, created_at, updated_at, created_by, updated_by, version) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 1, $9, $9, $10, $10, 0) RETURNING id, uuid, tenant_id, organization_id, code, name, default_locale, settings_json, status, version";
pub const UPDATE_SITE: &str = "UPDATE cms_site SET code = COALESCE($4, code), name = COALESCE($5, name), default_locale = COALESCE($6, default_locale), settings_json = COALESCE($7, settings_json), updated_at = $8, updated_by = $9, version = version + 1 WHERE tenant_id = $1 AND id = $2 AND version = $3 AND deleted_at IS NULL RETURNING id, uuid, tenant_id, organization_id, code, name, default_locale, settings_json, status, version";
pub const SOFT_DELETE_SITE: &str = "UPDATE cms_site SET status = 9, deleted_at = CURRENT_TIMESTAMP, deleted_by = $3 WHERE tenant_id = $1 AND id = $2";

pub const LIST_CHANNELS: &str = "SELECT id, uuid, site_id, code, name, channel_kind, status FROM cms_channel WHERE tenant_id = $1 AND site_id = $2 AND deleted_at IS NULL ORDER BY sort_order, id";
pub const LIST_CONTENT_TYPES: &str = "SELECT id, uuid, site_id, code, name, content_kind, schema_version, status FROM cms_content_type WHERE tenant_id = $1 AND site_id = $2 AND deleted_at IS NULL ORDER BY updated_at DESC, id DESC";
pub const LIST_CONTENT_FIELDS: &str = "SELECT id, content_type_id, code, name, field_kind, required, searchable, filterable, sortable FROM cms_content_field WHERE tenant_id = $1 AND content_type_id = $2 AND deleted_at IS NULL ORDER BY sort_order, id";
pub const LIST_TAXONOMIES: &str = "SELECT id, site_id, code, name, taxonomy_kind, status FROM cms_taxonomy WHERE tenant_id = $1 AND site_id = $2 AND deleted_at IS NULL ORDER BY updated_at DESC, id DESC";
pub const LIST_TAXONOMY_TERMS: &str = "SELECT id, taxonomy_id, parent_id, code, slug, name, path, status FROM cms_taxonomy_term WHERE tenant_id = $1 AND taxonomy_id = $2 AND deleted_at IS NULL ORDER BY path, sort_order, id";

pub const LIST_ENTRIES: &str = "SELECT id, uuid, tenant_id, organization_id, site_id, content_type_id, channel_id, locale, title, slug, summary, entry_status, publication_status, published_at, version FROM cms_entry WHERE tenant_id = $1 AND deleted_at IS NULL ORDER BY updated_at DESC, id DESC LIMIT $2";
pub const RETRIEVE_ENTRY: &str = "SELECT id, uuid, tenant_id, organization_id, site_id, content_type_id, channel_id, locale, title, slug, summary, entry_status, publication_status, published_at, version FROM cms_entry WHERE tenant_id = $1 AND id = $2 AND deleted_at IS NULL";
pub const LIST_ENTRY_MEDIA: &str = "SELECT id, media_role, drive_space_id, drive_node_id, drive_uri, media_resource_id, media_snapshot_json FROM cms_entry_media WHERE tenant_id = $1 AND entry_id = $2 AND status = 1 ORDER BY sort_order, id";
pub const LIST_ENTRY_VERSIONS: &str = "SELECT id, entry_id, version_no, version_kind, checksum FROM cms_entry_version WHERE tenant_id = $1 AND entry_id = $2 ORDER BY version_no DESC, id DESC";

pub const LIST_PAGES: &str = "SELECT id, site_id, channel_id, locale, path, title, publication_status, version FROM cms_page WHERE tenant_id = $1 AND deleted_at IS NULL ORDER BY updated_at DESC, id DESC";

pub const LIST_FEEDS: &str = "SELECT id, site_id, channel_id, code, name, feed_kind, locale, status, version FROM cms_feed WHERE tenant_id = $1 AND deleted_at IS NULL ORDER BY updated_at DESC, id DESC";
pub const LIST_FEED_RULES: &str = "SELECT id, feed_id, rule_kind, condition_json, sort_json, enabled FROM cms_feed_rule WHERE tenant_id = $1 AND feed_id = $2 ORDER BY sort_order, id";
pub const LIST_FEED_ITEMS: &str = "SELECT id, feed_id, entry_id, page_id, external_url, item_kind, pinned, sort_order FROM cms_feed_item WHERE tenant_id = $1 AND feed_id = $2 AND status = 1 ORDER BY pinned DESC, position_no, sort_order, id";
pub const RETRIEVE_FEED_SNAPSHOT: &str = "SELECT id, feed_id, publish_snapshot_id, snapshot_version, item_count, items_json, status, published_at FROM cms_feed_snapshot WHERE tenant_id = $1 AND id = $2";

pub const LIST_AUDIT_LOGS: &str = "SELECT id, site_id, actor_user_id, action, resource_type, resource_id, before_json, after_json, created_at FROM cms_audit_log WHERE tenant_id = $1 ORDER BY created_at DESC, id DESC LIMIT $2";
pub const LIST_OUTBOX_EVENTS: &str = "SELECT id, aggregate_type, aggregate_id, event_type, payload_json, status, attempt_count, next_attempt_at, created_at FROM cms_outbox_event WHERE tenant_id = $1 ORDER BY created_at DESC, id DESC LIMIT $2";
pub const RETRY_OUTBOX_EVENT: &str = "UPDATE cms_outbox_event SET status = 0, attempt_count = 0, next_attempt_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP, error_message = NULL WHERE tenant_id = $1 AND id = $2";
pub const CREATE_OUTBOX_EVENT: &str = "INSERT INTO cms_outbox_event (id, uuid, tenant_id, organization_id, aggregate_type, aggregate_id, event_type, event_version, payload_json, status, attempt_count, created_at, updated_at, request_id, trace_id) VALUES ($1, $2, $3, $4, $5, $6, $7, 1, $8, 0, 0, $9, $9, $10, $11)";
