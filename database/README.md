# CMS Database Module

Canonical lifecycle assets for `sdkwork-cms` per `DATABASE_FRAMEWORK_SPEC.md`.

- moduleId: `cms`
- serviceCode: `CMS`
- tablePrefix: `cms_`

## Commands

```bash
pnpm run db:validate
pnpm run db:plan
pnpm run db:init
pnpm run db:migrate
pnpm run db:seed
pnpm run db:status
pnpm run db:drift:check
pnpm run db:materialize:contract
```

## Migration status

Legacy SQL was consolidated into `ddl/baseline/postgres/0001_cms_legacy_baseline.sql` for bootstrap review.
Author contract-first tables in `contract/schema.yaml`, then split baseline into versioned `migrations/` pairs.

Imported legacy sources:
- `crates/sdkwork-content-cms-repository-sqlx/migrations/0001_cms_v1_foundation.sql`

Runtime services MUST create pools through `sdkwork-database-sqlx` and register `DefaultDatabaseModule` at bootstrap via `sdkwork-cms-database-host`.

```rust
use sdkwork_content_cms_repository_sqlx::connect_and_bootstrap_cms_database_from_env;
```
