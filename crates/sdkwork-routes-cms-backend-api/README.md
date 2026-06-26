# sdkwork-routes-cms-backend-api

Backend API route adapter for SDKWork CMS.

Prefix: `/backend/v3/api/cms`

Implementation:
- Routes bound to manifest with dual-token backend request context.
- IAM permissions enforced in services through typed request context.
- Idempotency preserved for create/publish/rollback/retry commands.
