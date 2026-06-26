pub const PREFIX: &str = "/cms/v3/api";

pub const ENTRIES: &str = "/cms/v3/api/entries";
pub const ENTRY_BY_ID: &str = "/cms/v3/api/entries/{entryId}";
pub const ENTRIES_RESOLVE: &str = "/cms/v3/api/entries:resolve";
pub const PAGES_RESOLVE: &str = "/cms/v3/api/pages:resolve";
pub const FEED_ITEMS: &str = "/cms/v3/api/feeds/{feedCode}/items";

pub const ALL_PATHS: &[&str] = &[
    ENTRIES,
    ENTRY_BY_ID,
    ENTRIES_RESOLVE,
    PAGES_RESOLVE,
    FEED_ITEMS,
];
