use super::value_objects::{CmsId, CmsJson};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CmsEventType {
    EntryCreated,
    EntryUpdated,
    EntryPublished,
    EntryUnpublished,
    EntryRolledBack,
    PagePublished,
    FeedPublished,
    SearchSyncRequested,
    CacheInvalidateRequested,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CmsOutboxEventDraft {
    pub aggregate_type: String,
    pub aggregate_id: CmsId,
    pub event_type: CmsEventType,
    pub payload_json: CmsJson,
}
