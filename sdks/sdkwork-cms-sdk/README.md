# sdkwork-cms-sdk

Public/open SDK family for SDKWork CMS.

| Field | Value |
| --- | --- |
| SDK family | `sdkwork-cms-sdk` |
| API authority | `sdkwork-cms.open` |
| API prefix | `/cms/v3/api` |
| Audience | External integrations and public/domain clients |
| Auth mode | API key |

SDK generation notes:
- Materialize `openapi/sdkwork-cms-open-api.openapi.json` from `apis/open-api/content/cms-open-api.openapi.json`.
- Run canonical `sdkgen --standard-profile sdkwork-v3`.
- Do not hand-edit `generated/server-openapi`.
