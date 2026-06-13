# sdkwork-cms-app-sdk

App SDK family for SDKWork CMS delivery APIs.

| Field | Value |
| --- | --- |
| SDK family | `sdkwork-cms-app-sdk` |
| API authority | `sdkwork-cms.app` |
| API prefix | `/app/v3/api/cms` |
| Audience | App, H5, mini program, desktop, and user-facing delivery clients |
| Auth mode | Anonymous for public delivery; future dual-token for preview/private delivery |

SDK generation notes:
- Materialize `openapi/sdkwork-cms-app-api.openapi.json` from `apis/app-api/content/cms-app-api.openapi.json`.
- Run canonical `sdkgen --standard-profile sdkwork-v3`.
- Keep IAM login/session APIs in appbase SDK, not this SDK family.
