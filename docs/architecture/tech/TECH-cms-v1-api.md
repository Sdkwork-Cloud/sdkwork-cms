> Migrated from `docs/api/cms-v1-api.md` on 2026-06-24.
> Owner: SDKWork maintainers

CMS v1 exposes three SDKWork API surfaces.

Machine-readable operation registry:

`docs/api/cms-v1-operation-registry.json`

The registry maps each OpenAPI operation to surface, auth mode, SDK family, permission, resource, and `CmsService` method. Follow-up route handlers should implement against that registry instead of inventing local operation names.

| Surface | Prefix | Audience | Auth |
| --- | --- | --- | --- |
| Open API | `/cms/v3/api` | External integrations and public/domain clients | API key for protected integration reads |
| App API | `/app/v3/api/cms` | App, H5, mini program, desktop, and user-facing delivery clients | Anonymous for public delivery; dual-token for future preview/private delivery |
| Backend API | `/backend/v3/api/cms` | Backend-admin UI, operators, automation, and trusted backend clients | Dual-token with IAM permissions |

## Open API Operations

| Operation ID | Method | Path | Purpose |
| --- | --- | --- | --- |
| `cms.entries.list` | `GET` | `/cms/v3/api/entries` | List published entries for integrations |
| `cms.entries.retrieve` | `GET` | `/cms/v3/api/entries/{entryId}` | Retrieve one published entry |
| `cms.entries.resolve` | `GET` | `/cms/v3/api/entries:resolve` | Resolve published entry by site/channel/locale/slug |
| `cms.pages.resolve` | `GET` | `/cms/v3/api/pages:resolve` | Resolve published page by site/channel/locale/path |
| `cms.feeds.items.list` | `GET` | `/cms/v3/api/feeds/{feedCode}/items` | Read a published feed snapshot |

## App API Operations

| Operation ID | Method | Path | Purpose |
| --- | --- | --- | --- |
| `cms.delivery.bootstrap.retrieve` | `GET` | `/app/v3/api/cms/sites/{siteCode}/bootstrap` | Read site delivery bootstrap config |
| `cms.delivery.entries.resolve` | `GET` | `/app/v3/api/cms/sites/{siteCode}/entries:resolve` | Resolve entry by slug |
| `cms.delivery.entries.retrieve` | `GET` | `/app/v3/api/cms/entries/{entryId}` | Retrieve published entry |
| `cms.delivery.pages.resolve` | `GET` | `/app/v3/api/cms/sites/{siteCode}/pages:resolve` | Resolve page by path |
| `cms.delivery.feeds.items.list` | `GET` | `/app/v3/api/cms/sites/{siteCode}/feeds/{feedCode}/items` | Read feed items |

## Backend API Operations

### Sites And Channels

| Operation ID | Method | Path |
| --- | --- | --- |
| `cms.sites.list` | `GET` | `/backend/v3/api/cms/sites` |
| `cms.sites.create` | `POST` | `/backend/v3/api/cms/sites` |
| `cms.sites.retrieve` | `GET` | `/backend/v3/api/cms/sites/{siteId}` |
| `cms.sites.update` | `PATCH` | `/backend/v3/api/cms/sites/{siteId}` |
| `cms.sites.delete` | `DELETE` | `/backend/v3/api/cms/sites/{siteId}` |
| `cms.channels.list` | `GET` | `/backend/v3/api/cms/sites/{siteId}/channels` |
| `cms.channels.create` | `POST` | `/backend/v3/api/cms/sites/{siteId}/channels` |
| `cms.channels.update` | `PATCH` | `/backend/v3/api/cms/channels/{channelId}` |
| `cms.channels.delete` | `DELETE` | `/backend/v3/api/cms/channels/{channelId}` |

### Content Modeling

| Operation ID | Method | Path |
| --- | --- | --- |
| `cms.contentTypes.list` | `GET` | `/backend/v3/api/cms/sites/{siteId}/content_types` |
| `cms.contentTypes.create` | `POST` | `/backend/v3/api/cms/sites/{siteId}/content_types` |
| `cms.contentTypes.retrieve` | `GET` | `/backend/v3/api/cms/content_types/{contentTypeId}` |
| `cms.contentTypes.update` | `PATCH` | `/backend/v3/api/cms/content_types/{contentTypeId}` |
| `cms.contentTypes.delete` | `DELETE` | `/backend/v3/api/cms/content_types/{contentTypeId}` |
| `cms.contentFields.list` | `GET` | `/backend/v3/api/cms/content_types/{contentTypeId}/fields` |
| `cms.contentFields.create` | `POST` | `/backend/v3/api/cms/content_types/{contentTypeId}/fields` |
| `cms.contentFields.update` | `PATCH` | `/backend/v3/api/cms/content_fields/{fieldId}` |
| `cms.contentFields.delete` | `DELETE` | `/backend/v3/api/cms/content_fields/{fieldId}` |

### Taxonomy

| Operation ID | Method | Path |
| --- | --- | --- |
| `cms.taxonomies.list` | `GET` | `/backend/v3/api/cms/sites/{siteId}/taxonomies` |
| `cms.taxonomies.create` | `POST` | `/backend/v3/api/cms/sites/{siteId}/taxonomies` |
| `cms.taxonomies.update` | `PATCH` | `/backend/v3/api/cms/taxonomies/{taxonomyId}` |
| `cms.taxonomies.delete` | `DELETE` | `/backend/v3/api/cms/taxonomies/{taxonomyId}` |
| `cms.taxonomyTerms.list` | `GET` | `/backend/v3/api/cms/taxonomies/{taxonomyId}/terms` |
| `cms.taxonomyTerms.create` | `POST` | `/backend/v3/api/cms/taxonomies/{taxonomyId}/terms` |
| `cms.taxonomyTerms.update` | `PATCH` | `/backend/v3/api/cms/taxonomy_terms/{termId}` |
| `cms.taxonomyTerms.delete` | `DELETE` | `/backend/v3/api/cms/taxonomy_terms/{termId}` |

### Entries

| Operation ID | Method | Path |
| --- | --- | --- |
| `cms.entries.management.list` | `GET` | `/backend/v3/api/cms/entries` |
| `cms.entries.create` | `POST` | `/backend/v3/api/cms/entries` |
| `cms.entries.management.retrieve` | `GET` | `/backend/v3/api/cms/entries/{entryId}` |
| `cms.entries.update` | `PATCH` | `/backend/v3/api/cms/entries/{entryId}` |
| `cms.entries.delete` | `DELETE` | `/backend/v3/api/cms/entries/{entryId}` |
| `cms.entries.body.update` | `PUT` | `/backend/v3/api/cms/entries/{entryId}/body` |
| `cms.entries.fields.replace` | `PUT` | `/backend/v3/api/cms/entries/{entryId}/fields` |
| `cms.entries.media.list` | `GET` | `/backend/v3/api/cms/entries/{entryId}/media` |
| `cms.entries.media.attach` | `POST` | `/backend/v3/api/cms/entries/{entryId}/media` |
| `cms.entries.media.delete` | `DELETE` | `/backend/v3/api/cms/entries/{entryId}/media/{mediaId}` |
| `cms.entries.terms.replace` | `PUT` | `/backend/v3/api/cms/entries/{entryId}/terms` |
| `cms.entries.versions.list` | `GET` | `/backend/v3/api/cms/entries/{entryId}/versions` |
| `cms.entries.publish` | `POST` | `/backend/v3/api/cms/entries/{entryId}:publish` |
| `cms.entries.unpublish` | `POST` | `/backend/v3/api/cms/entries/{entryId}:unpublish` |
| `cms.entries.rollback` | `POST` | `/backend/v3/api/cms/entries/{entryId}:rollback` |
| `cms.entries.schedule` | `POST` | `/backend/v3/api/cms/entries/{entryId}:schedule` |

### Pages

| Operation ID | Method | Path |
| --- | --- | --- |
| `cms.pages.management.list` | `GET` | `/backend/v3/api/cms/pages` |
| `cms.pages.create` | `POST` | `/backend/v3/api/cms/pages` |
| `cms.pages.management.retrieve` | `GET` | `/backend/v3/api/cms/pages/{pageId}` |
| `cms.pages.update` | `PATCH` | `/backend/v3/api/cms/pages/{pageId}` |
| `cms.pages.delete` | `DELETE` | `/backend/v3/api/cms/pages/{pageId}` |
| `cms.pages.blocks.replace` | `PUT` | `/backend/v3/api/cms/pages/{pageId}/blocks` |
| `cms.pages.publish` | `POST` | `/backend/v3/api/cms/pages/{pageId}:publish` |

### Feeds

| Operation ID | Method | Path |
| --- | --- | --- |
| `cms.feeds.management.list` | `GET` | `/backend/v3/api/cms/feeds` |
| `cms.feeds.create` | `POST` | `/backend/v3/api/cms/feeds` |
| `cms.feeds.management.retrieve` | `GET` | `/backend/v3/api/cms/feeds/{feedId}` |
| `cms.feeds.update` | `PATCH` | `/backend/v3/api/cms/feeds/{feedId}` |
| `cms.feeds.delete` | `DELETE` | `/backend/v3/api/cms/feeds/{feedId}` |
| `cms.feedRules.list` | `GET` | `/backend/v3/api/cms/feeds/{feedId}/rules` |
| `cms.feedRules.create` | `POST` | `/backend/v3/api/cms/feeds/{feedId}/rules` |
| `cms.feedRules.update` | `PATCH` | `/backend/v3/api/cms/feed_rules/{ruleId}` |
| `cms.feedRules.delete` | `DELETE` | `/backend/v3/api/cms/feed_rules/{ruleId}` |
| `cms.feedItems.list` | `GET` | `/backend/v3/api/cms/feeds/{feedId}/items` |
| `cms.feedItems.upsert` | `PUT` | `/backend/v3/api/cms/feeds/{feedId}/items` |
| `cms.feedItems.delete` | `DELETE` | `/backend/v3/api/cms/feed_items/{itemId}` |
| `cms.feeds.publish` | `POST` | `/backend/v3/api/cms/feeds/{feedId}:publish` |
| `cms.feeds.snapshots.retrieve` | `GET` | `/backend/v3/api/cms/feeds/{feedId}/snapshots/{snapshotId}` |

### Governance

| Operation ID | Method | Path |
| --- | --- | --- |
| `cms.auditLogs.list` | `GET` | `/backend/v3/api/cms/audit_logs` |
| `cms.outboxEvents.list` | `GET` | `/backend/v3/api/cms/outbox_events` |
| `cms.outboxEvents.retry` | `POST` | `/backend/v3/api/cms/outbox_events/{eventId}:retry` |

## Implementation Status

- [x] Backend handlers consume typed SDKWork request context and never parse raw auth headers.
- [x] Backend services enforce IAM permissions before repository calls.
- [x] Commands that create, publish, unpublish, rollback, schedule, or import support `Idempotency-Key`.
- [x] Media attach commands accept Drive/MediaResource references only, never raw file bytes or provider object keys.
- [x] Search sync, cache invalidation, messaging, and webhooks are emitted through `cms_outbox_event`.

