use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaginationParams {
    pub cursor: Option<String>,
    #[serde(default = "default_limit")]
    pub limit: u32,
}

fn default_limit() -> u32 {
    20
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SiteCreateRequest {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(default = "default_locale")]
    pub default_locale: String,
    pub settings_json: Option<String>,
}

fn default_locale() -> String {
    "en-US".to_string()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SiteUpdateRequest {
    pub code: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub default_locale: Option<String>,
    pub settings_json: Option<String>,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChannelCreateRequest {
    pub code: String,
    pub name: String,
    #[serde(default = "default_channel_kind")]
    pub channel_kind: String,
    pub delivery_config_json: Option<String>,
}

fn default_channel_kind() -> String {
    "web".to_string()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChannelUpdateRequest {
    pub code: Option<String>,
    pub name: Option<String>,
    pub channel_kind: Option<String>,
    pub delivery_config_json: Option<String>,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContentTypeCreateRequest {
    pub code: String,
    pub name: String,
    #[serde(default = "default_content_kind")]
    pub content_kind: String,
    pub description: Option<String>,
    pub settings_json: Option<String>,
}

fn default_content_kind() -> String {
    "entry".to_string()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContentTypeUpdateRequest {
    pub code: Option<String>,
    pub name: Option<String>,
    pub content_kind: Option<String>,
    pub description: Option<String>,
    pub settings_json: Option<String>,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContentFieldCreateRequest {
    pub code: String,
    pub name: String,
    #[serde(default = "default_field_kind")]
    pub field_kind: String,
    pub required: Option<bool>,
    pub localized: Option<bool>,
    pub multiple: Option<bool>,
    pub searchable: Option<bool>,
    pub filterable: Option<bool>,
    pub sortable: Option<bool>,
    pub validation_json: Option<String>,
    pub options_json: Option<String>,
    pub ui_json: Option<String>,
    pub default_value_json: Option<String>,
}

fn default_field_kind() -> String {
    "text".to_string()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContentFieldUpdateRequest {
    pub code: Option<String>,
    pub name: Option<String>,
    pub field_kind: Option<String>,
    pub required: Option<bool>,
    pub searchable: Option<bool>,
    pub filterable: Option<bool>,
    pub sortable: Option<bool>,
    pub validation_json: Option<String>,
    pub options_json: Option<String>,
    pub ui_json: Option<String>,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaxonomyCreateRequest {
    pub code: String,
    pub name: String,
    #[serde(default = "default_taxonomy_kind")]
    pub taxonomy_kind: String,
    pub description: Option<String>,
    pub settings_json: Option<String>,
}

fn default_taxonomy_kind() -> String {
    "category".to_string()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaxonomyUpdateRequest {
    pub code: Option<String>,
    pub name: Option<String>,
    pub taxonomy_kind: Option<String>,
    pub description: Option<String>,
    pub settings_json: Option<String>,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaxonomyTermCreateRequest {
    pub parent_id: Option<i64>,
    pub code: String,
    pub slug: Option<String>,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaxonomyTermUpdateRequest {
    pub parent_id: Option<i64>,
    pub code: Option<String>,
    pub slug: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntryCreateRequest {
    pub site_id: i64,
    pub content_type_id: i64,
    pub channel_id: Option<i64>,
    #[serde(default = "default_locale")]
    pub locale: String,
    pub title: String,
    pub slug: String,
    pub summary: Option<String>,
    pub seo_json: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntryUpdateRequest {
    pub channel_id: Option<i64>,
    pub locale: Option<String>,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub summary: Option<String>,
    pub seo_json: Option<String>,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntryBodyRequest {
    #[serde(default = "default_locale")]
    pub locale: String,
    #[serde(default = "default_body_format")]
    pub body_format: String,
    pub body_text: Option<String>,
    pub body_html: Option<String>,
    pub block_tree_json: String,
    pub version: Option<i64>,
}

fn default_body_format() -> String {
    "blocks".to_string()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntryFieldsRequest {
    #[serde(default = "default_locale")]
    pub locale: String,
    pub fields_json: String,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntryMediaAttachRequest {
    pub field_id: Option<i64>,
    pub media_role: String,
    pub drive_space_id: Option<String>,
    pub drive_node_id: Option<String>,
    pub drive_uri: Option<String>,
    pub media_resource_id: Option<String>,
    pub media_snapshot_json: String,
    pub alt_text: Option<String>,
    pub caption: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntryTermsRequest {
    pub term_ids: Vec<i64>,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublishRequest {
    pub channel_id: Option<i64>,
    pub locale: Option<String>,
    pub note: Option<String>,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RollbackRequest {
    pub target_version_id: i64,
    pub note: Option<String>,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScheduleRequest {
    pub scheduled_publish_at: Option<String>,
    pub scheduled_unpublish_at: Option<String>,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PageCreateRequest {
    pub site_id: i64,
    pub channel_id: Option<i64>,
    #[serde(default = "default_locale")]
    pub locale: String,
    pub path: String,
    pub slug: String,
    pub title: String,
    pub page_kind: Option<String>,
    pub template_code: Option<String>,
    pub seo_json: Option<String>,
    pub settings_json: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PageUpdateRequest {
    pub channel_id: Option<i64>,
    pub locale: Option<String>,
    pub path: Option<String>,
    pub slug: Option<String>,
    pub title: Option<String>,
    pub seo_json: Option<String>,
    pub settings_json: Option<String>,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PageBlocksRequest {
    pub blocks_json: String,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeedCreateRequest {
    pub site_id: i64,
    pub channel_id: Option<i64>,
    pub code: String,
    pub name: String,
    #[serde(default = "default_feed_kind")]
    pub feed_kind: String,
    #[serde(default = "default_locale")]
    pub locale: String,
    pub default_page_size: Option<i32>,
    pub rule_config_json: Option<String>,
}

fn default_feed_kind() -> String {
    "hybrid".to_string()
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeedUpdateRequest {
    pub code: Option<String>,
    pub name: Option<String>,
    pub feed_kind: Option<String>,
    pub locale: Option<String>,
    pub default_page_size: Option<i32>,
    pub rule_config_json: Option<String>,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeedRuleCreateRequest {
    pub rule_kind: String,
    pub content_type_id: Option<i64>,
    pub taxonomy_id: Option<i64>,
    pub taxonomy_term_id: Option<i64>,
    pub q: Option<String>,
    pub condition_json: String,
    pub sort_json: String,
    #[serde(default = "default_limit_count")]
    pub limit_count: u32,
    #[serde(default = "default_true")]
    pub enabled: bool,
}

fn default_limit_count() -> u32 {
    50
}

fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeedRuleUpdateRequest {
    pub rule_kind: Option<String>,
    pub condition_json: Option<String>,
    pub sort_json: Option<String>,
    pub limit_count: Option<u32>,
    pub enabled: Option<bool>,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeedItemsUpsertRequest {
    pub items_json: String,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListSitesQueryParams {
    pub cursor: Option<String>,
    pub page_size: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListBySiteQueryParams {
    pub cursor: Option<String>,
    pub page_size: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListEntriesQueryParams {
    pub site_id: Option<i64>,
    pub content_type_id: Option<i64>,
    pub channel_id: Option<i64>,
    pub locale: Option<String>,
    pub entry_status: Option<i32>,
    pub publication_status: Option<i32>,
    pub author_user_id: Option<i64>,
    pub cursor: Option<String>,
    pub page_size: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListPagesQueryParams {
    pub site_id: Option<i64>,
    pub channel_id: Option<i64>,
    pub locale: Option<String>,
    pub status: Option<i32>,
    pub cursor: Option<String>,
    pub page_size: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListFeedsQueryParams {
    pub site_id: Option<i64>,
    pub channel_id: Option<i64>,
    pub locale: Option<String>,
    pub status: Option<i32>,
    pub cursor: Option<String>,
    pub page_size: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListFeedRulesQueryParams {
    pub enabled: Option<bool>,
    pub cursor: Option<String>,
    pub page_size: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListFeedItemsQueryParams {
    pub status: Option<i32>,
    pub cursor: Option<String>,
    pub page_size: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListAuditLogsQueryParams {
    pub site_id: Option<i64>,
    pub resource_type: Option<String>,
    pub resource_id: Option<i64>,
    pub actor_user_id: Option<i64>,
    pub cursor: Option<String>,
    pub page_size: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListOutboxEventsQueryParams {
    pub aggregate_type: Option<String>,
    pub aggregate_id: Option<i64>,
    pub status: Option<i32>,
    pub cursor: Option<String>,
    pub page_size: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RetryOutboxEventRequest {
    pub reason: Option<String>,
}
