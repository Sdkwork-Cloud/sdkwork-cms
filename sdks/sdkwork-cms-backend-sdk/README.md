# sdkwork-cms-backend-sdk

Backend SDK family for SDKWork CMS management APIs.

| Field | Value |
| --- | --- |
| SDK family | `sdkwork-cms-backend-sdk` |
| API authority | `sdkwork-cms.backend` |
| API prefix | `/backend/v3/api/cms` |
| Audience | Backend-admin UI, operators, automation, and trusted backend clients |
| Auth mode | Dual-token |

SDK generation notes:
- Materialize `openapi/sdkwork-cms-backend-api.openapi.json` from `apis/backend-api/content/cms-backend-api.openapi.json`.
- Run canonical `sdkgen --standard-profile sdkwork-v3`.
- Keep backend IAM management in appbase backend SDK, not copied into CMS.
