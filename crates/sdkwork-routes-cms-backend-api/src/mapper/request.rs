use sdkwork_content_cms_service::domain::*;
use crate::dto::request::*;

pub fn map_site_create_request_to_command(req: SiteCreateRequest) -> SiteCommand {
    SiteCommand {
        code: Some(req.code),
        name: Some(req.name),
        description: req.description,
        default_locale: Some(req.default_locale),
        settings_json: req.settings_json,
        version: None,
    }
}

pub fn map_site_update_request_to_command(req: SiteUpdateRequest) -> SiteCommand {
    SiteCommand {
        code: req.code,
        name: req.name,
        description: req.description,
        default_locale: req.default_locale,
        settings_json: req.settings_json,
        version: req.version,
    }
}

pub fn map_channel_create_request_to_command(site_id: CmsId, req: ChannelCreateRequest) -> ChannelCommand {
    ChannelCommand {
        site_id,
        code: Some(req.code),
        name: Some(req.name),
        channel_kind: Some(req.channel_kind),
        delivery_config_json: req.delivery_config_json,
        version: None,
    }
}

pub fn map_channel_update_request_to_command(req: ChannelUpdateRequest) -> ChannelCommand {
    ChannelCommand {
        site_id: 0,
        code: req.code,
        name: req.name,
        channel_kind: req.channel_kind,
        delivery_config_json: req.delivery_config_json,
        version: req.version,
    }
}

pub fn map_content_type_create_request_to_command(site_id: CmsId, req: ContentTypeCreateRequest) -> ContentTypeCommand {
    ContentTypeCommand {
        site_id,
        code: Some(req.code),
        name: Some(req.name),
        content_kind: Some(req.content_kind),
        settings_json: req.settings_json,
        version: None,
    }
}

pub fn map_content_type_update_request_to_command(req: ContentTypeUpdateRequest) -> ContentTypeCommand {
    ContentTypeCommand {
        site_id: 0,
        code: req.code,
        name: req.name,
        content_kind: req.content_kind,
        settings_json: req.settings_json,
        version: req.version,
    }
}

pub fn map_content_field_create_request_to_command(content_type_id: CmsId, req: ContentFieldCreateRequest) -> ContentFieldCommand {
    ContentFieldCommand {
        content_type_id,
        code: Some(req.code),
        name: Some(req.name),
        field_kind: Some(req.field_kind),
        validation_json: req.validation_json,
        options_json: req.options_json,
        ui_json: req.ui_json,
        version: None,
    }
}

pub fn map_content_field_update_request_to_command(req: ContentFieldUpdateRequest) -> ContentFieldCommand {
    ContentFieldCommand {
        content_type_id: 0,
        code: req.code,
        name: req.name,
        field_kind: req.field_kind,
        validation_json: req.validation_json,
        options_json: req.options_json,
        ui_json: req.ui_json,
        version: req.version,
    }
}

pub fn map_taxonomy_create_request_to_command(site_id: CmsId, req: TaxonomyCreateRequest) -> TaxonomyCommand {
    TaxonomyCommand {
        site_id,
        code: Some(req.code),
        name: Some(req.name),
        taxonomy_kind: Some(req.taxonomy_kind),
        settings_json: req.settings_json,
        version: None,
    }
}

pub fn map_taxonomy_update_request_to_command(req: TaxonomyUpdateRequest) -> TaxonomyCommand {
    TaxonomyCommand {
        site_id: 0,
        code: req.code,
        name: req.name,
        taxonomy_kind: req.taxonomy_kind,
        settings_json: req.settings_json,
        version: req.version,
    }
}

pub fn map_taxonomy_term_create_request_to_command(taxonomy_id: CmsId, req: TaxonomyTermCreateRequest) -> TaxonomyTermCommand {
    TaxonomyTermCommand {
        taxonomy_id,
        parent_id: req.parent_id,
        code: Some(req.code),
        slug: req.slug,
        name: Some(req.name),
        version: None,
    }
}

pub fn map_taxonomy_term_update_request_to_command(req: TaxonomyTermUpdateRequest) -> TaxonomyTermCommand {
    TaxonomyTermCommand {
        taxonomy_id: 0,
        parent_id: req.parent_id,
        code: req.code,
        slug: req.slug,
        name: req.name,
        version: req.version,
    }
}

pub fn map_entry_create_request_to_command(req: EntryCreateRequest) -> EntryCommand {
    EntryCommand {
        site_id: req.site_id,
        content_type_id: req.content_type_id,
        channel_id: req.channel_id,
        locale: req.locale,
        title: req.title,
        slug: req.slug,
        summary: req.summary,
        seo_json: req.seo_json,
        version: None,
    }
}

pub fn map_entry_update_request_to_command(_entry_id: CmsId, req: EntryUpdateRequest) -> EntryCommand {
    EntryCommand {
        site_id: 0,
        content_type_id: 0,
        channel_id: req.channel_id,
        locale: req.locale.unwrap_or_default(),
        title: req.title.unwrap_or_default(),
        slug: req.slug.unwrap_or_default(),
        summary: req.summary,
        seo_json: req.seo_json,
        version: req.version,
    }
}

pub fn map_entry_body_request_to_command(entry_id: CmsId, req: EntryBodyRequest) -> EntryBodyCommand {
    EntryBodyCommand {
        entry_id,
        locale: req.locale,
        body_format: req.body_format,
        body_text: req.body_text,
        body_html: req.body_html,
        block_tree_json: req.block_tree_json,
        version: req.version,
    }
}

pub fn map_entry_fields_request_to_command(entry_id: CmsId, req: EntryFieldsRequest) -> EntryFieldsCommand {
    EntryFieldsCommand {
        entry_id,
        locale: req.locale,
        fields_json: req.fields_json,
        version: req.version,
    }
}

pub fn map_entry_media_attach_request_to_command(entry_id: CmsId, req: EntryMediaAttachRequest) -> EntryMediaCommand {
    EntryMediaCommand {
        entry_id,
        field_id: req.field_id,
        media_role: req.media_role,
        drive_space_id: req.drive_space_id,
        drive_node_id: req.drive_node_id,
        drive_uri: req.drive_uri,
        media_resource_id: req.media_resource_id,
        media_snapshot_json: req.media_snapshot_json,
        alt_text: req.alt_text,
        caption: req.caption,
    }
}

pub fn map_entry_terms_request_to_command(entry_id: CmsId, req: EntryTermsRequest) -> ReplaceEntryTermsCommand {
    ReplaceEntryTermsCommand {
        entry_id,
        term_ids: req.term_ids,
        version: req.version,
    }
}

pub fn map_publish_request_to_command(owner_type: &str, owner_id: CmsId, req: PublishRequest) -> PublishCommand {
    PublishCommand {
        owner_type: owner_type.to_string(),
        owner_id,
        channel_id: req.channel_id,
        locale: req.locale,
        note: req.note,
        version: req.version,
    }
}

pub fn map_rollback_request_to_command(owner_type: &str, owner_id: CmsId, req: RollbackRequest) -> RollbackCommand {
    RollbackCommand {
        owner_type: owner_type.to_string(),
        owner_id,
        target_version_id: req.target_version_id,
        note: req.note,
        version: req.version,
    }
}

pub fn map_schedule_request_to_command(entry_id: CmsId, req: ScheduleRequest) -> ScheduleCommand {
    ScheduleCommand {
        entry_id,
        scheduled_publish_at: req.scheduled_publish_at,
        scheduled_unpublish_at: req.scheduled_unpublish_at,
        version: req.version,
    }
}

pub fn map_page_create_request_to_command(req: PageCreateRequest) -> PageCommand {
    PageCommand {
        site_id: req.site_id,
        channel_id: req.channel_id,
        locale: req.locale,
        path: req.path,
        slug: req.slug,
        title: req.title,
        seo_json: req.seo_json,
        settings_json: req.settings_json,
        version: None,
    }
}

pub fn map_page_update_request_to_command(_page_id: CmsId, req: PageUpdateRequest) -> PageCommand {
    PageCommand {
        site_id: 0,
        channel_id: req.channel_id,
        locale: req.locale.unwrap_or_default(),
        path: req.path.unwrap_or_default(),
        slug: req.slug.unwrap_or_default(),
        title: req.title.unwrap_or_default(),
        seo_json: req.seo_json,
        settings_json: req.settings_json,
        version: req.version,
    }
}

pub fn map_page_blocks_request_to_command(page_id: CmsId, req: PageBlocksRequest) -> PageBlocksCommand {
    PageBlocksCommand {
        page_id,
        blocks_json: req.blocks_json,
        version: req.version,
    }
}

pub fn map_feed_create_request_to_command(req: FeedCreateRequest) -> FeedCommand {
    FeedCommand {
        site_id: req.site_id,
        channel_id: req.channel_id,
        code: Some(req.code),
        name: Some(req.name),
        feed_kind: Some(req.feed_kind),
        locale: Some(req.locale),
        rule_config_json: req.rule_config_json,
        version: None,
    }
}

pub fn map_feed_update_request_to_command(_feed_id: CmsId, req: FeedUpdateRequest) -> FeedCommand {
    FeedCommand {
        site_id: 0,
        channel_id: None,
        code: req.code,
        name: req.name,
        feed_kind: req.feed_kind,
        locale: req.locale,
        rule_config_json: req.rule_config_json,
        version: req.version,
    }
}

pub fn map_feed_rule_create_request_to_command(feed_id: CmsId, req: FeedRuleCreateRequest) -> FeedRuleCommand {
    FeedRuleCommand {
        feed_id,
        rule_kind: req.rule_kind,
        condition_json: req.condition_json,
        sort_json: req.sort_json,
        limit_count: req.limit_count,
        enabled: req.enabled,
        version: None,
    }
}

pub fn map_feed_rule_update_request_to_command(req: FeedRuleUpdateRequest) -> FeedRuleCommand {
    FeedRuleCommand {
        feed_id: 0,
        rule_kind: req.rule_kind.unwrap_or_default(),
        condition_json: req.condition_json.unwrap_or_default(),
        sort_json: req.sort_json.unwrap_or_default(),
        limit_count: req.limit_count.unwrap_or(50),
        enabled: req.enabled.unwrap_or(true),
        version: req.version,
    }
}

pub fn map_feed_items_upsert_request_to_command(feed_id: CmsId, req: FeedItemsUpsertRequest) -> FeedItemsCommand {
    FeedItemsCommand {
        feed_id,
        items_json: req.items_json,
        version: req.version,
    }
}

pub fn map_list_sites_params_to_query(params: ListSitesQueryParams) -> ListSitesQuery {
    ListSitesQuery {
        cursor: params.cursor,
        limit: params.limit.unwrap_or(20).min(100),
    }
}

pub fn map_list_by_site_params_to_query(site_id: CmsId, params: ListBySiteQueryParams) -> ListBySiteQuery {
    ListBySiteQuery {
        site_id,
        cursor: params.cursor,
        limit: params.limit.unwrap_or(20).min(100),
    }
}

pub fn map_list_entries_params_to_query(params: ListEntriesQueryParams) -> ListEntriesQuery {
    ListEntriesQuery {
        site_id: params.site_id,
        content_type_id: params.content_type_id,
        channel_id: params.channel_id,
        locale: params.locale,
        entry_status: params.entry_status,
        publication_status: params.publication_status,
        author_user_id: params.author_user_id,
        cursor: params.cursor,
        limit: params.limit.unwrap_or(20).min(100),
    }
}

pub fn map_list_pages_params_to_query(params: ListPagesQueryParams) -> ListPagesQuery {
    ListPagesQuery {
        site_id: params.site_id,
        channel_id: params.channel_id,
        locale: params.locale,
        status: params.status,
        cursor: params.cursor,
        limit: params.limit.unwrap_or(20).min(100),
    }
}

pub fn map_list_feeds_params_to_query(params: ListFeedsQueryParams) -> ListFeedsQuery {
    ListFeedsQuery {
        site_id: params.site_id,
        channel_id: params.channel_id,
        locale: params.locale,
        status: params.status,
        cursor: params.cursor,
        limit: params.limit.unwrap_or(20).min(100),
    }
}

pub fn map_list_feed_rules_params_to_query(feed_id: CmsId, params: ListFeedRulesQueryParams) -> ListFeedRulesQuery {
    ListFeedRulesQuery {
        feed_id,
        enabled: params.enabled,
        cursor: params.cursor,
        limit: params.limit.unwrap_or(20).min(100),
    }
}

pub fn map_list_feed_items_params_to_query(feed_id: CmsId, params: ListFeedItemsQueryParams) -> ListFeedItemsQuery {
    ListFeedItemsQuery {
        feed_id,
        status: params.status,
        cursor: params.cursor,
        limit: params.limit.unwrap_or(20).min(100),
    }
}

pub fn map_list_audit_logs_params_to_query(params: ListAuditLogsQueryParams) -> ListAuditLogsQuery {
    ListAuditLogsQuery {
        site_id: params.site_id,
        resource_type: params.resource_type,
        resource_id: params.resource_id,
        actor_user_id: params.actor_user_id,
        cursor: params.cursor,
        limit: params.limit.unwrap_or(20).min(100),
    }
}

pub fn map_list_outbox_events_params_to_query(params: ListOutboxEventsQueryParams) -> ListOutboxEventsQuery {
    ListOutboxEventsQuery {
        aggregate_type: params.aggregate_type,
        aggregate_id: params.aggregate_id,
        status: params.status,
        cursor: params.cursor,
        limit: params.limit.unwrap_or(20).min(100),
    }
}
