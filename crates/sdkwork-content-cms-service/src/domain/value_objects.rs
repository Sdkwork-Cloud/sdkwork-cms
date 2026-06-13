pub type CmsId = i64;
pub type CmsJson = String;
pub type CmsInstant = String;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CmsPageCursor {
    pub cursor: Option<String>,
    pub limit: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CmsPage<T> {
    pub items: Vec<T>,
    pub next_cursor: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CmsVersion {
    pub value: i64,
}
