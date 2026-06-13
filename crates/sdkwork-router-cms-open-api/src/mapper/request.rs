use sdkwork_content_cms_service::context::{CmsLoginScope, CmsRequestContext};

use super::super::dto::request::*;

pub fn build_open_api_context(
    tenant_id: i64,
    organization_id: i64,
    request_id: String,
) -> CmsRequestContext {
    CmsRequestContext {
        request_id,
        trace_id: None,
        tenant_id,
        organization_id,
        user_id: 0,
        session_id: None,
        permissions: vec![],
        data_scope: 2,
        login_scope: CmsLoginScope::Tenant,
    }
}

pub fn map_list_entries_query(req: &DeliveryListEntriesRequest) -> sdkwork_content_cms_service::domain::DeliveryListEntriesQuery {
    sdkwork_content_cms_service::domain::DeliveryListEntriesQuery {
        site_code: req.site_code.clone(),
        channel_code: req.channel_code.clone(),
        locale: req.locale.clone(),
        content_type_code: req.content_type_code.clone(),
        term_code: req.term_code.clone(),
        cursor: req.cursor.clone(),
        limit: req.limit.unwrap_or(20).min(100),
    }
}

pub fn map_resolve_entry_query(req: &DeliveryResolveEntryRequest) -> sdkwork_content_cms_service::domain::DeliveryResolveEntryQuery {
    sdkwork_content_cms_service::domain::DeliveryResolveEntryQuery {
        site_code: req.site_code.clone(),
        channel_code: req.channel_code.clone(),
        locale: req.locale.clone(),
        slug: req.slug.clone(),
        preview_token: None,
    }
}

pub fn map_retrieve_entry_query(req: &DeliveryRetrieveEntryRequest) -> sdkwork_content_cms_service::domain::DeliveryRetrieveEntryQuery {
    sdkwork_content_cms_service::domain::DeliveryRetrieveEntryQuery {
        entry_id: req.entry_id,
        preview_token: None,
    }
}

pub fn map_resolve_page_query(req: &DeliveryResolvePageRequest) -> sdkwork_content_cms_service::domain::DeliveryResolvePageQuery {
    sdkwork_content_cms_service::domain::DeliveryResolvePageQuery {
        site_code: req.site_code.clone(),
        channel_code: req.channel_code.clone(),
        locale: req.locale.clone(),
        path: req.path.clone(),
        preview_token: None,
    }
}

pub fn map_feed_items_query(req: &DeliveryListFeedItemsRequest) -> sdkwork_content_cms_service::domain::DeliveryFeedItemsQuery {
    sdkwork_content_cms_service::domain::DeliveryFeedItemsQuery {
        site_code: req.site_code.clone(),
        feed_code: req.feed_code.clone(),
        channel_code: req.channel_code.clone(),
        locale: req.locale.clone(),
        cursor: req.cursor.clone(),
        limit: req.limit.unwrap_or(20).min(100),
    }
}
