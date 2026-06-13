use async_trait::async_trait;

use crate::context::CmsRequestContext;
use crate::domain::CmsOutboxEventDraft;
use crate::error::CmsResult;

#[async_trait]
pub trait CmsEventPublisher: Send + Sync {
    async fn enqueue(&self, ctx: &CmsRequestContext, event: CmsOutboxEventDraft) -> CmsResult<()>;
}
