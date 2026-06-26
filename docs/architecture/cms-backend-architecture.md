# SDKWork CMS Backend Architecture

This repository is the CMS application root for backend contracts and implementation skeletons. Frontend apps are intentionally out of scope for this pass.

## Naming Boundary

- SDKWork canonical domain: `content`
- CMS capability token: `cms`
- CMS-owned database prefix: `cms_`
- CMS API path segment and resource prefix: `cms`
- CMS SDK family stem: `sdkwork-cms`

This keeps the product recognizable as CMS while preserving `content` as the canonical SDKWork domain classification.

## Directory Ownership

| Path | Owner | Purpose |
| --- | --- | --- |
| `apis/open-api/content` | CMS API authority | Open/domain integration delivery OpenAPI contracts |
| `apis/app-api/content` | CMS API authority | App/H5/mini-program delivery OpenAPI contracts |
| `apis/backend-api/content` | CMS API authority | Backend-admin/operator management OpenAPI contracts |
| `crates/sdkwork-content-cms-service` | CMS service | Domain models, commands, results, ports, and service method implementations |
| `crates/sdkwork-content-cms-repository-sqlx` | CMS repository | SQLx migrations, schema constants, query plans, row mapping, and repository implementations |
| `crates/sdkwork-routes-cms-open-api` | Open route crate | Open API path constants, manifest, handler/mapping implementations |
| `crates/sdkwork-routes-cms-app-api` | App route crate | App API path constants, manifest, handler/mapping implementations |
| `crates/sdkwork-routes-cms-backend-api` | Backend route crate | Backend API path constants, manifest, handler/mapping implementations |
| `sdks/sdkwork-cms-sdk` | Open SDK family | Owner-only SDK generation metadata for CMS Open API |
| `sdks/sdkwork-cms-app-sdk` | App SDK family | Owner-only SDK generation metadata for CMS App API |
| `sdks/sdkwork-cms-backend-sdk` | Backend SDK family | Owner-only SDK generation metadata for CMS Backend API |
| `docs/database` | Contract docs | Human and machine-readable database table registry |
| `docs/api` | Contract docs | Human and machine-readable API operation registry |
| `docs/security` | Contract docs | Machine-readable CMS permission registry |
| `docs/integration` | Contract docs | Machine-readable CMS event, outbox, and dependency-port registry |
| `tests/static` | Contract verification | Static contract checks across database, API, SDK, and Rust modules |

## Service Module Plan

The service crate is split by CMS capability so follow-up agents can implement one area at a time:

| Module | Responsibility |
| --- | --- |
| `service/sites.rs` | Site and channel management |
| `service/content_modeling.rs` | Content type and field schema management |
| `service/taxonomy.rs` | Taxonomy and term tree management |
| `service/entries.rs` | Entry lifecycle, body, fields, media, terms, versions, publish, rollback, schedule |
| `service/pages.rs` | Page route, block composition, and page publish |
| `service/feeds.rs` | Feed definitions, rules, curated items, snapshots, and feed publish |
| `service/governance.rs` | Audit log and outbox operations |
| `service/delivery.rs` | Public/app delivery reads, preview token handling boundary |

## Repository Module Plan

The repository crate mirrors service capability modules:

| Module | Responsibility |
| --- | --- |
| `repository/sites.rs` | `cms_site`, `cms_channel` access |
| `repository/content_modeling.rs` | `cms_content_type`, `cms_content_field` access |
| `repository/taxonomy.rs` | `cms_taxonomy`, `cms_taxonomy_term` access |
| `repository/entries.rs` | `cms_entry`, body, field values, media, terms, versions, entry publish state |
| `repository/pages.rs` | `cms_page`, `cms_page_block`, page publish state |
| `repository/feeds.rs` | `cms_feed`, rules, items, feed snapshots |
| `repository/publishing.rs` | Shared `cms_publish_snapshot` writes |
| `repository/governance.rs` | `cms_audit_log`, `cms_outbox_event` access |
| `repository/port_impl.rs` | Thin adapter from `CmsRepository` trait to capability modules |

## Integration Boundaries

CMS depends on other SDKWork domains through SDK/service ports, not copied tables or copied OpenAPI:

- IAM: tenant, organization, user, permission, policy, and audit identity.
- Drive/MediaResource: file lifecycle, upload, storage object, and media metadata.
- Search: indexing, ranking, recommendations, suggestions, and embeddings.
- Messaging: notifications and workflow subscriber delivery.
- Comments/Engagement: public comments, likes, favorites, and engagement summaries.
- Scheduler: delayed publish/unpublish jobs.
- Webhook: external delivery retries and endpoint configuration.
- Sitemap/RSS projection: delivery projections requested by outbox events.

Machine-readable integration contract:

- `docs/integration/cms-v1-integration-registry.json` registers CMS-owned outbox events and dependency ports.
- Every event uses `cms_outbox_event`; dependency-owned tables are never written directly by CMS.
- Search, cache, notification, webhook, sitemap, and RSS work must be requested asynchronously through the registered ports.

## Permission Boundary

Machine-readable permission contract:

- `docs/security/cms-v1-permission-registry.json` registers all 22 backend permissions used by CMS v1.
- Backend OpenAPI operations must keep `x-sdkwork-permission` aligned with this registry.
- `CmsIamAuthorizer` is the required enforcement boundary before repository writes or management reads.

## Route Adapter Plan

Route crates expose focused handler and DTO skeleton modules:

| Crate | Handler modules | DTO modules |
| --- | --- | --- |
| `sdkwork-routes-cms-open-api` | `entries`, `pages`, `feeds` | `dto/request.rs`, `dto/response.rs` |
| `sdkwork-routes-cms-app-api` | `delivery` | `dto/request.rs`, `dto/response.rs` |
| `sdkwork-routes-cms-backend-api` | `sites`, `content_modeling`, `taxonomy`, `entries`, `pages`, `feeds`, `governance` | `dto/request.rs`, `dto/response.rs` |

## Implementation Status

All service methods, repository implementations, route handlers, DTOs, and mappers are implemented. Methods use typed request context for permission checks, tenant isolation, and idempotency. Integration events are emitted through the outbox pattern.
