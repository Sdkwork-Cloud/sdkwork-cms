# SDKWork CMS Agent Guide

This repository is the SDKWork CMS application root.

## Canonical Standards

- Workspace rules: `../sdkwork-specs/SDKWORK_WORKSPACE_SPEC.md`
- Agent execution: `../sdkwork-specs/SOUL.md`
- Standards entrypoint: `../sdkwork-specs/README.md`

## Required Reading By Task

- API work: `../sdkwork-specs/API_SPEC.md`
- SDK work: `../sdkwork-specs/SDK_SPEC.md` and `../sdkwork-specs/SDK_WORKSPACE_GENERATION_SPEC.md`
- Database work: `../sdkwork-specs/DATABASE_SPEC.md`
- Rust backend work: `../sdkwork-specs/RUST_CODE_SPEC.md` and `../sdkwork-specs/WEB_BACKEND_SPEC.md`
- Drive/media integration: `../sdkwork-specs/DRIVE_SPEC.md` and `../sdkwork-specs/MEDIA_RESOURCE_SPEC.md`
- IAM/security integration: `../sdkwork-specs/IAM_SPEC.md` and `../sdkwork-specs/SECURITY_SPEC.md`

## Local Boundaries

- CMS-owned database tables use the `cms_` prefix.
- CMS application-owned APIs live under `apis/`.
- SDK families live under `sdks/` and must remain owner-only.
- Rust service, repository, and route crates live under `crates/`.
- Frontend application work is intentionally out of scope for this repository pass.

## Cross-Domain Dependencies

CMS consumes IAM, Drive, Search, Messaging, Comments, and Engagement through their owning SDKs or service contracts. Do not copy dependency-owned tables, OpenAPI operations, generated SDK code, or route handlers into this application authority.

