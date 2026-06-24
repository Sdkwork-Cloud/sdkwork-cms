> Migrated from `docs/implementation-todos.md` on 2026-06-24.
> Owner: SDKWork maintainers

This document tracks implementation status for the CMS crate.

## Hard Boundaries

- Do not build apps or frontend code in this repository pass.
- Do not copy IAM, Drive, Search, Messaging, Comments, Engagement, KB, or provider-owned tables into CMS.
- Do not copy dependency-owned OpenAPI operations into CMS SDK authority specs.
- Do not hand-edit generated SDK output under `generated/server-openapi`.
- Keep `cms_` for CMS-owned tables, permissions, events, API resources, and SDK family identity.
- Keep `content` as the SDKWork canonical domain classification from `sdkwork-specs/DOMAIN_SPEC.md`.

## Database

- [x] Wire `crates/sdkwork-content-cms-repository-sqlx/migrations/0001_cms_v1_foundation.sql` into the repository migration runner.
- [x] Keep `docs/database/cms-v1-column-registry.json` synchronized with migration columns before changing row mappers or DTOs.
- [x] Implement id/uuid allocation through SDKWork ID Provider before any insert.
- [x] Implement tenant, organization, site, optimistic `version`, and soft-delete predicates for every repository query.
- [x] Implement transactional writes for entry body/fields/media/terms/version, publish snapshots, feed snapshots, audit logs, outbox events, and idempotency records.
- [ ] Add SDKWork database linter integration when the workspace validator is available. *(blocked: external tooling)*
- [ ] Add migration tests against PostgreSQL once a test database profile exists. *(blocked: test infrastructure)*

## Service

- [x] Replace `CmsService::new()` with constructor injection for `CmsRepository`, `CmsIamAuthorizer`, Drive, Search, cache, event, preview token, scheduler, notification, webhook, engagement, and projection ports.
- [x] Implement IAM permission checks before repository calls.
- [x] Implement schema validation for `cms_content_type` and `cms_content_field`.
- [x] Implement entry lifecycle rules: draft, review, approve, publish, unpublish, rollback, schedule, archive, soft delete.
- [x] Implement publish snapshot rendering for entries, pages, and feeds.
- [x] Implement feed composition for curated, rule, hybrid, search-assisted, recommendation-assisted, and RSS-like feeds without owning recommendation/search tables.
- [x] Implement outbox-backed integration events for search sync, cache invalidation, messaging, webhooks, sitemap/RSS projection, and future automation.

## API

- [x] Route handlers extract typed SDKWork request contexts and idempotency keys through shared middleware.
- [x] Implement handler functions in `src/handlers/*.rs` according to `docs/api/cms-v1-operation-registry.json`.
- [x] Implement request/response DTO mapping modules in each route crate without embedding business logic in handlers.
- [x] Backend API handlers map every operation in `apis/backend-api/content/cms-backend-api.openapi.json` to the matching `CmsService` method.
- [x] Open API handlers use API-key context and expose published, delivery-safe payloads only.
- [x] App API handlers support anonymous public delivery and future preview-token/private delivery through `CmsPreviewTokenPort`.
- [x] Errors map to RFC 9457 problem detail without leaking SQL, provider, or dependency internals.
- [ ] Add generated route-manifest-to-OpenAPI materialization after SDKWork route tooling is available. *(blocked: external tooling)*

## Permission

- [x] Enforce every permission from `docs/security/cms-v1-permission-registry.json` through `CmsIamAuthorizer`.
- [x] Keep backend OpenAPI `x-sdkwork-permission`, route manifests, and permission registry in sync.
- [ ] Add IAM ABAC conditions for tenant, organization, site, owner, state, and scheduled publish windows. *(future enhancement)*

## Integration

- [x] Implement every event and dependency port in `docs/integration/cms-v1-integration-registry.json`.
- [x] Persist outbox payloads in `cms_outbox_event` inside the same transaction as CMS aggregate changes.
- [x] Dispatch Search, cache, notification, webhook, sitemap, and RSS work asynchronously after commit.

## SDK

- [ ] Run canonical `sdkgen --standard-profile sdkwork-v3` after OpenAPI review. *(blocked: SDK generator)*
- [ ] Keep SDK generation owner-only: CMS SDKs generate only CMS authority APIs. *(already enforced by architecture)*
- [ ] Add typed composed facades outside `generated/server-openapi` when product ergonomics need higher-level workflows. *(future enhancement)*
- [ ] Verify `sdkDependencies` resolve against sibling workspace repositories before release packaging. *(blocked: release infrastructure)*

## Verification

- [ ] Keep `tests/static/cms-contract-boundary.test.mjs` in CI. *(already in CI)*
- [ ] Add OpenAPI schema validation with the SDKWork official API validator when available. *(blocked: external tooling)*
- [ ] Add SQL parser/linter validation with SDKWork database tooling when available. *(blocked: external tooling)*
- [ ] Add service unit tests before implementing each business use case. *(future enhancement)*
- [ ] Add repository integration tests before executing migrations against shared environments. *(blocked: test infrastructure)*

