use sqlx::FromRow;

#[derive(Clone, Debug, Eq, PartialEq, FromRow)]
pub struct CmsSiteRow {
    pub id: i64,
    pub uuid: String,
    pub tenant_id: i64,
    pub organization_id: i64,
    pub code: String,
    pub name: String,
    pub default_locale: String,
    pub settings_json: String,
    pub status: i32,
    pub version: i64,
}

#[derive(Clone, Debug, Eq, PartialEq, FromRow)]
pub struct CmsEntryRow {
    pub id: i64,
    pub uuid: String,
    pub tenant_id: i64,
    pub organization_id: i64,
    pub site_id: i64,
    pub content_type_id: i64,
    pub channel_id: Option<i64>,
    pub locale: String,
    pub title: String,
    pub slug: String,
    pub summary: Option<String>,
    pub entry_status: i32,
    pub publication_status: i32,
    pub published_at: Option<String>,
    pub version: i64,
}

#[derive(Clone, Debug, Eq, PartialEq, FromRow)]
pub struct CmsPublishSnapshotRow {
    pub id: i64,
    pub tenant_id: i64,
    pub site_id: i64,
    pub owner_type: String,
    pub owner_id: i64,
    pub snapshot_payload_json: String,
    pub status: i32,
    pub published_at: String,
}
