use sdkwork_content_cms_service::domain::*;
use crate::dto::response::*;

fn id_to_string(id: i64) -> String {
    id.to_string()
}

pub fn map_site_to_response(site: CmsSite) -> SiteResponse {
    SiteResponse {
        id: id_to_string(site.id),
        uuid: site.uuid,
        tenant_id: id_to_string(site.tenant_id),
        organization_id: id_to_string(site.organization_id),
        code: site.code,
        name: site.name,
        default_locale: site.default_locale,
        settings_json: site.settings_json,
        status: site.status,
        version: id_to_string(site.version),
    }
}

pub fn map_channel_to_response(channel: CmsChannel) -> ChannelResponse {
    ChannelResponse {
        id: id_to_string(channel.id),
        site_id: id_to_string(channel.site_id),
        code: channel.code,
        name: channel.name,
        channel_kind: channel.channel_kind,
        status: channel.status,
    }
}

pub fn map_content_type_to_response(ct: CmsContentType) -> ContentTypeResponse {
    ContentTypeResponse {
        id: id_to_string(ct.id),
        site_id: id_to_string(ct.site_id),
        code: ct.code,
        name: ct.name,
        content_kind: ct.content_kind,
        schema_version: id_to_string(ct.schema_version),
        status: ct.status,
    }
}

pub fn map_content_field_to_response(field: CmsContentField) -> ContentFieldResponse {
    ContentFieldResponse {
        id: id_to_string(field.id),
        content_type_id: id_to_string(field.content_type_id),
        code: field.code,
        name: field.name,
        field_kind: field.field_kind,
        required: field.required,
        searchable: field.searchable,
        filterable: field.filterable,
        sortable: field.sortable,
    }
}

pub fn map_taxonomy_to_response(taxonomy: CmsTaxonomy) -> TaxonomyResponse {
    TaxonomyResponse {
        id: id_to_string(taxonomy.id),
        site_id: id_to_string(taxonomy.site_id),
        code: taxonomy.code,
        name: taxonomy.name,
        taxonomy_kind: taxonomy.taxonomy_kind,
        status: taxonomy.status,
    }
}

pub fn map_taxonomy_term_to_response(term: CmsTaxonomyTerm) -> TaxonomyTermResponse {
    TaxonomyTermResponse {
        id: id_to_string(term.id),
        taxonomy_id: id_to_string(term.taxonomy_id),
        parent_id: term.parent_id.map(id_to_string),
        code: term.code,
        slug: term.slug,
        name: term.name,
        path: term.path,
        status: term.status,
    }
}

pub fn entry_status_to_string(status: &CmsEntryStatus) -> String {
    match status {
        CmsEntryStatus::Draft => "draft".to_string(),
        CmsEntryStatus::Reviewing => "reviewing".to_string(),
        CmsEntryStatus::Approved => "approved".to_string(),
        CmsEntryStatus::Published => "published".to_string(),
        CmsEntryStatus::Archived => "archived".to_string(),
        CmsEntryStatus::Deleted => "deleted".to_string(),
    }
}

pub fn publication_status_to_string(status: &CmsPublicationStatus) -> String {
    match status {
        CmsPublicationStatus::Unpublished => "unpublished".to_string(),
        CmsPublicationStatus::Scheduled => "scheduled".to_string(),
        CmsPublicationStatus::Published => "published".to_string(),
        CmsPublicationStatus::UnpublishedAfterPublish => "unpublished_after_publish".to_string(),
        CmsPublicationStatus::RolledBack => "rolled_back".to_string(),
    }
}

pub fn map_entry_to_response(entry: CmsEntry) -> EntryResponse {
    EntryResponse {
        id: id_to_string(entry.id),
        uuid: entry.uuid,
        site_id: id_to_string(entry.site_id),
        content_type_id: id_to_string(entry.content_type_id),
        channel_id: entry.channel_id.map(id_to_string),
        locale: entry.locale,
        title: entry.title,
        slug: entry.slug,
        summary: entry.summary,
        entry_status: entry_status_to_string(&entry.entry_status),
        publication_status: publication_status_to_string(&entry.publication_status),
        published_at: entry.published_at,
        version: id_to_string(entry.version),
    }
}

pub fn map_entry_version_to_response(version: CmsEntryVersion) -> EntryVersionResponse {
    EntryVersionResponse {
        id: id_to_string(version.id),
        entry_id: id_to_string(version.entry_id),
        version_no: id_to_string(version.version_no),
        version_kind: version.version_kind,
        checksum: version.checksum,
    }
}

pub fn map_media_ref_to_response(media: CmsMediaRef) -> MediaRefResponse {
    MediaRefResponse {
        id: id_to_string(media.id),
        role: media.role,
        drive_space_id: media.drive_space_id,
        drive_node_id: media.drive_node_id,
        drive_uri: media.drive_uri,
        media_resource_id: media.media_resource_id,
        media_snapshot_json: media.media_snapshot_json,
    }
}

pub fn map_publish_snapshot_to_response(snapshot: CmsPublishSnapshot) -> PublishSnapshotResponse {
    PublishSnapshotResponse {
        id: id_to_string(snapshot.id),
        owner_type: snapshot.owner_type,
        owner_id: id_to_string(snapshot.owner_id),
        snapshot_payload_json: snapshot.snapshot_payload_json,
        status: snapshot.status,
        published_at: snapshot.published_at,
    }
}

pub fn map_page_to_response(page: CmsPageModel) -> PageResponse {
    PageResponse {
        id: id_to_string(page.id),
        site_id: id_to_string(page.site_id),
        channel_id: page.channel_id.map(id_to_string),
        locale: page.locale,
        path: page.path,
        title: page.title,
        publication_status: publication_status_to_string(&page.publication_status),
        version: id_to_string(page.version),
    }
}

pub fn map_feed_to_response(feed: CmsFeed) -> FeedResponse {
    FeedResponse {
        id: id_to_string(feed.id),
        site_id: id_to_string(feed.site_id),
        channel_id: feed.channel_id.map(id_to_string),
        code: feed.code,
        name: feed.name,
        feed_kind: feed.feed_kind,
        locale: feed.locale,
        status: feed.status,
        version: id_to_string(feed.version),
    }
}

pub fn map_feed_rule_to_response(rule: CmsFeedRule) -> FeedRuleResponse {
    FeedRuleResponse {
        id: id_to_string(rule.id),
        feed_id: id_to_string(rule.feed_id),
        rule_kind: rule.rule_kind,
        condition_json: rule.condition_json,
        sort_json: rule.sort_json,
        enabled: rule.enabled,
    }
}

pub fn map_feed_item_to_response(item: CmsFeedItem) -> FeedItemResponse {
    FeedItemResponse {
        id: id_to_string(item.id),
        feed_id: id_to_string(item.feed_id),
        entry_id: item.entry_id.map(id_to_string),
        page_id: item.page_id.map(id_to_string),
        external_url: item.external_url,
        item_kind: item.item_kind,
        pinned: item.pinned,
        sort_order: item.sort_order,
    }
}

pub fn map_feed_snapshot_to_response(snapshot: CmsFeedSnapshot) -> FeedSnapshotResponse {
    FeedSnapshotResponse {
        id: id_to_string(snapshot.id),
        feed_id: id_to_string(snapshot.feed_id),
        publish_snapshot_id: snapshot.publish_snapshot_id.map(id_to_string),
        snapshot_version: id_to_string(snapshot.snapshot_version),
        item_count: snapshot.item_count,
        items_json: snapshot.items_json,
        status: snapshot.status,
        published_at: snapshot.published_at,
    }
}

pub fn map_audit_log_to_response(log: CmsAuditLog) -> AuditLogResponse {
    AuditLogResponse {
        id: id_to_string(log.id),
        site_id: log.site_id.map(id_to_string),
        actor_user_id: id_to_string(log.actor_user_id),
        action: log.action,
        resource_type: log.resource_type,
        resource_id: log.resource_id.map(id_to_string),
        before_json: log.before_json,
        after_json: log.after_json,
        created_at: log.created_at,
    }
}

pub fn map_outbox_event_to_response(event: CmsOutboxEvent) -> OutboxEventResponse {
    OutboxEventResponse {
        id: id_to_string(event.id),
        aggregate_type: event.aggregate_type,
        aggregate_id: id_to_string(event.aggregate_id),
        event_type: event.event_type,
        payload_json: event.payload_json,
        status: event.status,
        attempt_count: event.attempt_count,
        next_attempt_at: event.next_attempt_at,
        created_at: event.created_at,
    }
}

pub fn map_page_to_paginated_response<T, R: serde::Serialize>(page: CmsPage<T>, map_fn: fn(T) -> R) -> PaginatedResponse<R> {
    PaginatedResponse {
        items: page.items.into_iter().map(map_fn).collect(),
        next_cursor: page.next_cursor,
        total_count: None,
    }
}

pub fn map_command_result_to_response(result: CommandResult) -> CommandResponse {
    CommandResponse {
        ok: result.ok,
        resource_id: result.resource_id.map(id_to_string),
        request_id: result.request_id,
    }
}
