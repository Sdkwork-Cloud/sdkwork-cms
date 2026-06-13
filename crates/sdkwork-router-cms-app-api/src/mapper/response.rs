use sdkwork_content_cms_service::domain::*;

use super::super::dto::response::*;

pub fn map_site_to_response(site: &CmsSite) -> DeliverySiteResponse {
    DeliverySiteResponse {
        id: site.id.to_string(),
        uuid: site.uuid.clone(),
        code: site.code.clone(),
        name: site.name.clone(),
        default_locale: site.default_locale.clone(),
    }
}

pub fn map_channel_to_response(channel: &CmsChannel) -> DeliveryChannelResponse {
    DeliveryChannelResponse {
        id: channel.id.to_string(),
        code: channel.code.clone(),
        name: channel.name.clone(),
        channel_kind: channel.channel_kind.clone(),
    }
}

pub fn map_bootstrap_to_response(bootstrap: &CmsDeliveryBootstrap) -> DeliveryBootstrapResponse {
    DeliveryBootstrapResponse {
        site: map_site_to_response(&bootstrap.site),
        channels: bootstrap.channels.iter().map(|c| map_channel_to_response(c)).collect(),
    }
}

pub fn map_entry_to_response(entry: &CmsEntry) -> DeliveryEntryResponse {
    DeliveryEntryResponse {
        id: entry.id.to_string(),
        uuid: entry.uuid.clone(),
        site_id: entry.site_id.to_string(),
        content_type_id: entry.content_type_id.to_string(),
        channel_id: entry.channel_id.map(|id| id.to_string()),
        locale: entry.locale.clone(),
        title: entry.title.clone(),
        slug: entry.slug.clone(),
        summary: entry.summary.clone(),
        published_at: entry.published_at.clone(),
    }
}

pub fn map_page_to_response(page: &CmsPageModel) -> DeliveryPageResponse {
    DeliveryPageResponse {
        id: page.id.to_string(),
        site_id: page.site_id.to_string(),
        channel_id: page.channel_id.map(|id| id.to_string()),
        locale: page.locale.clone(),
        path: page.path.clone(),
        title: page.title.clone(),
        publication_status: format!("{:?}", page.publication_status),
    }
}

pub fn map_feed_item_to_response(item: &CmsFeedItem) -> DeliveryFeedItemResponse {
    DeliveryFeedItemResponse {
        id: item.id.to_string(),
        feed_id: item.feed_id.to_string(),
        entry_id: item.entry_id.map(|id| id.to_string()),
        page_id: item.page_id.map(|id| id.to_string()),
        external_url: item.external_url.clone(),
        item_kind: item.item_kind.clone(),
        pinned: item.pinned,
        sort_order: item.sort_order,
    }
}

pub fn map_feed_items_page_to_response(page: &CmsFeedItemPage) -> DeliveryListResponse<DeliveryFeedItemResponse> {
    DeliveryListResponse {
        items: page.items.iter().map(|i| map_feed_item_to_response(i)).collect(),
        next_cursor: page.next_cursor.clone(),
        total: None,
    }
}
