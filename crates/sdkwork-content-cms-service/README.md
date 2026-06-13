# sdkwork-content-cms-service

Business service crate for SDKWork CMS.

Responsibilities:
- Own CMS domain models, commands, results, service ports, authorization hooks, idempotency boundaries, publish orchestration, and outbox event intent.
- Keep HTTP framework and SQLx implementation details out of the service contract.

Implementation:
- Use cases implemented with IAM permission checks through typed request context.
- Drive references integrated through service/provider ports.
- Search, cache, Messaging, and Webhook work emitted through outbox events.
