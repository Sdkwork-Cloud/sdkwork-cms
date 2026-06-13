use sdkwork_content_cms_service::domain::{
    CmsEntry, CmsEntryStatus, CmsPublicationStatus, CmsPublishSnapshot, CmsSite,
};

use crate::db::rows::{CmsEntryRow, CmsPublishSnapshotRow, CmsSiteRow};

fn map_status_to_i32(status: i32) -> i32 {
    status
}

fn map_entry_status(status: i32) -> CmsEntryStatus {
    match status {
        0 => CmsEntryStatus::Draft,
        10 => CmsEntryStatus::Reviewing,
        20 => CmsEntryStatus::Approved,
        30 => CmsEntryStatus::Published,
        40 => CmsEntryStatus::Archived,
        9 => CmsEntryStatus::Deleted,
        _ => CmsEntryStatus::Draft,
    }
}

fn map_publication_status(status: i32) -> CmsPublicationStatus {
    match status {
        0 => CmsPublicationStatus::Unpublished,
        10 => CmsPublicationStatus::Scheduled,
        20 => CmsPublicationStatus::Published,
        30 => CmsPublicationStatus::UnpublishedAfterPublish,
        40 => CmsPublicationStatus::RolledBack,
        _ => CmsPublicationStatus::Unpublished,
    }
}

pub fn map_site_row(row: CmsSiteRow) -> CmsSite {
    CmsSite {
        id: row.id,
        uuid: row.uuid,
        tenant_id: row.tenant_id,
        organization_id: row.organization_id,
        code: row.code,
        name: row.name,
        default_locale: row.default_locale,
        settings_json: row.settings_json,
        status: map_status_to_i32(row.status),
        version: row.version,
    }
}

pub fn map_entry_row(row: CmsEntryRow) -> CmsEntry {
    CmsEntry {
        id: row.id,
        uuid: row.uuid,
        site_id: row.site_id,
        content_type_id: row.content_type_id,
        channel_id: row.channel_id,
        locale: row.locale,
        title: row.title,
        slug: row.slug,
        summary: row.summary,
        entry_status: map_entry_status(row.entry_status),
        publication_status: map_publication_status(row.publication_status),
        published_at: row.published_at,
        version: row.version,
    }
}

pub fn map_publish_snapshot_row(row: CmsPublishSnapshotRow) -> CmsPublishSnapshot {
    CmsPublishSnapshot {
        id: row.id,
        owner_type: row.owner_type,
        owner_id: row.owner_id,
        snapshot_payload_json: row.snapshot_payload_json,
        status: map_status_to_i32(row.status),
        published_at: row.published_at,
    }
}
