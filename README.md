# SDKWork CMS
repository-kind: application

SDKWork CMS is the content management application authority for structured content, page composition, publishing snapshots, and curated/rule-based feeds.

This repository contains the CMS backend implementation:
- Database contract and SQL migration.
- OpenAPI contracts for open-api, app-api, and backend-api.
- Rust service, repository, and route crate implementations.
- SDK family metadata and generation input placeholders.

Frontend `apps/` implementation is intentionally not included in this pass.

## Active Layout

| Directory | Purpose |
| --- | --- |
| `apis/` | Authored OpenAPI contracts and API review inputs |
| `crates/` | Rust service, SQLx repository, and route adapter crates |
| `sdks/` | SDK family workspaces, assembly metadata, and generation inputs |
| `docs/` | Architecture, database, API, SDK, and implementation handoff docs |
| `tests/` | Static and contract verification placeholders |
| `configs/` | Safe source-controlled config templates |
| `deployments/` | Deployment notes and future release handoff files |
| `scripts/` | Thin command entrypoints |
| `tools/` | Reusable validators and generators |

## Domain Boundary

CMS owns:
- `cms_*` tables.
- `/cms/v3/api/*`, `/app/v3/api/cms/*`, and `/backend/v3/api/cms/*` operations.
- `sdkwork-cms-sdk`, `sdkwork-cms-app-sdk`, and `sdkwork-cms-backend-sdk` SDK families.
- `sdkwork-content-cms-service` and `sdkwork-content-cms-repository-sqlx` Rust backend crates under the canonical `content` domain.

CMS depends on:
- IAM/appbase for users, tenants, organizations, roles, permissions, sessions, and audit/security primitives.
- Drive for file upload, storage, node lifecycle, and media resources.
- Search for search indexes, ranking, recommendations, embeddings, suggestions, and query analytics.
- Messaging for notifications.
- Comments/Engagement for public reader interactions.

## Documentation Canon

- [docs/README.md](docs/README.md)
- [docs/product/prd/PRD.md](docs/product/prd/PRD.md)
- [docs/architecture/tech/TECH_ARCHITECTURE.md](docs/architecture/tech/TECH_ARCHITECTURE.md)

## Application Roots

- [apps directory index](apps/README.md)
