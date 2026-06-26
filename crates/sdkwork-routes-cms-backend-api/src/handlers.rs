pub mod content_modeling;
pub mod entries;
pub mod feeds;
pub mod governance;
pub mod pages;
pub mod sites;
pub mod taxonomy;

use sdkwork_content_cms_service::context::CmsRequestContext;
use sdkwork_content_cms_service::service::CmsService;

pub struct HandlerState {
    pub service: CmsService,
    pub context: CmsRequestContext,
}

impl HandlerState {
    pub fn new(service: CmsService, context: CmsRequestContext) -> Self {
        Self { service, context }
    }
}
