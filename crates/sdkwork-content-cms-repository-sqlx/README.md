# sdkwork-content-cms-repository-sqlx

SQLx repository implementation crate for SDKWork CMS.

Responsibilities:
- Own CMS table, column, and index constants.
- Own SQL query text and row-to-domain mapping.
- Implement repository traits from `sdkwork-content-cms-service`.

Implementation:
- SQLx dependency and pool abstraction configured.
- Migration runner integrated.
- Repository methods implement tenant scope, optimistic locking, and query shape.
