import assert from "node:assert/strict";
import { readFileSync, existsSync } from "node:fs";
import { relative, resolve } from "node:path";
import test from "node:test";

const root = resolve(import.meta.dirname, "../..");

const rel = (path) => relative(root, resolve(root, path));
const read = (path) => readFileSync(resolve(root, path), "utf8");
const readJson = (path) => JSON.parse(read(path));

const parseMigrationColumns = (migration) => {
  const tables = new Map();
  const tableBlocks = migration.matchAll(/CREATE TABLE IF NOT EXISTS (cms_[a-z0-9_]+)\s*\(([\s\S]*?)\n\);/g);
  for (const [, tableName, body] of tableBlocks) {
    const columns = [];
    for (const line of body.split(/\r?\n/)) {
      const trimmed = line.trim().replace(/,$/, "");
      if (!trimmed) continue;
      const token = trimmed.split(/\s+/)[0];
      if (["CONSTRAINT", "PRIMARY", "FOREIGN", "UNIQUE", "CHECK"].includes(token)) continue;
      columns.push(token);
    }
    tables.set(tableName, columns);
  }
  return tables;
};

const requiredTables = [
  "cms_site",
  "cms_channel",
  "cms_content_type",
  "cms_content_field",
  "cms_taxonomy",
  "cms_taxonomy_term",
  "cms_entry",
  "cms_entry_body",
  "cms_entry_field_value",
  "cms_entry_version",
  "cms_entry_media",
  "cms_entry_term",
  "cms_page",
  "cms_page_block",
  "cms_publish_snapshot",
  "cms_feed",
  "cms_feed_rule",
  "cms_feed_item",
  "cms_feed_snapshot",
  "cms_audit_log",
  "cms_outbox_event",
  "cms_idempotency_key",
];

const apiAuthorities = [
  {
    authority: "sdkwork-cms.open",
    source: "apis/open-api/content/cms-open-api.openapi.json",
    sdkCopy: "sdks/sdkwork-cms-sdk/openapi/sdkwork-cms-open-api.openapi.json",
    assembly: "sdks/sdkwork-cms-sdk/.sdkwork-assembly.json",
    component: "sdks/sdkwork-cms-sdk/specs/component.spec.json",
    prefix: "/cms/v3/api",
    family: "sdkwork-cms-sdk",
  },
  {
    authority: "sdkwork-cms.app",
    source: "apis/app-api/content/cms-app-api.openapi.json",
    sdkCopy: "sdks/sdkwork-cms-app-sdk/openapi/sdkwork-cms-app-api.openapi.json",
    assembly: "sdks/sdkwork-cms-app-sdk/.sdkwork-assembly.json",
    component: "sdks/sdkwork-cms-app-sdk/specs/component.spec.json",
    prefix: "/app/v3/api/cms",
    family: "sdkwork-cms-app-sdk",
  },
  {
    authority: "sdkwork-cms.backend",
    source: "apis/backend-api/content/cms-backend-api.openapi.json",
    sdkCopy: "sdks/sdkwork-cms-backend-sdk/openapi/sdkwork-cms-backend-api.openapi.json",
    assembly: "sdks/sdkwork-cms-backend-sdk/.sdkwork-assembly.json",
    component: "sdks/sdkwork-cms-backend-sdk/specs/component.spec.json",
    prefix: "/backend/v3/api/cms",
    family: "sdkwork-cms-backend-sdk",
  },
];

test("CMS v1 migration defines the required focused table catalog", () => {
  const migration = read("crates/sdkwork-content-cms-repository-sqlx/migrations/0001_cms_v1_foundation.sql");
  const tableRegistry = readJson("docs/database/cms-v1-table-registry.json");
  const tables = Array.from(migration.matchAll(/CREATE TABLE IF NOT EXISTS (cms_[a-z0-9_]+)/g), (match) => match[1]);

  assert.deepEqual([...new Set(tables)].sort(), [...requiredTables].sort());
  assert.deepEqual(tableRegistry.tables.map((table) => table.name).sort(), [...requiredTables].sort());
  assert.equal(tableRegistry.dependencyTables.reuseByReferenceOnly.length, 5);

  for (const table of requiredTables) {
    assert.match(migration, new RegExp(`CREATE TABLE IF NOT EXISTS ${table}\\s*\\(`));
  }

  assert.match(migration, /tenant_id BIGINT NOT NULL/);
  assert.match(migration, /uuid VARCHAR\(64\) NOT NULL UNIQUE/);
  assert.match(migration, /version BIGINT NOT NULL DEFAULT 0/);
  assert.match(migration, /cms_outbox_event/);
  assert.match(migration, /cms_idempotency_key/);
});

test("CMS column registry documents every migration column", () => {
  const migration = read("crates/sdkwork-content-cms-repository-sqlx/migrations/0001_cms_v1_foundation.sql");
  const migrationColumns = parseMigrationColumns(migration);
  const columnRegistry = readJson("docs/database/cms-v1-column-registry.json");

  assert.equal(columnRegistry.kind, "sdkwork.database.column-registry");
  assert.equal(columnRegistry.owner, "sdkwork-cms");
  assert.equal(columnRegistry.domain, "content");
  assert.equal(columnRegistry.capability, "cms");
  assert.equal(columnRegistry.tableCount, requiredTables.length);

  const registryTables = new Map(columnRegistry.tables.map((table) => [table.name, table]));
  assert.deepEqual([...registryTables.keys()].sort(), [...requiredTables].sort());

  for (const tableName of requiredTables) {
    const table = registryTables.get(tableName);
    const expectedColumns = migrationColumns.get(tableName);
    assert.ok(expectedColumns, `${tableName} must exist in migration`);
    assert.equal(table.columns.length, expectedColumns.length, `${tableName} column count must match migration`);
    assert.deepEqual(table.columns.map((column) => column.name), expectedColumns, `${tableName} column order must match migration`);
    assert.ok(table.purpose, `${tableName} must have table purpose`);
    assert.match(table.implementationTodo, /^TODO:/, `${tableName} must keep implementation TODO`);

    for (const column of table.columns) {
      assert.ok(column.logicalType, `${tableName}.${column.name} must declare logicalType`);
      assert.ok(column.dbType, `${tableName}.${column.name} must declare dbType`);
      assert.equal(typeof column.required, "boolean", `${tableName}.${column.name} must declare required`);
      assert.ok(column.purpose, `${tableName}.${column.name} must declare purpose`);
    }
  }
});

test("CMS OpenAPI authorities are valid JSON and use their SDKWork route prefixes", () => {
  for (const api of apiAuthorities) {
    const spec = readJson(api.source);
    assert.equal(spec.openapi, "3.1.2", rel(api.source));
    assert.equal(spec.info["x-sdkwork-owner"], "sdkwork-cms", rel(api.source));
    assert.equal(spec.info["x-sdkwork-api-authority"], api.authority, rel(api.source));

    const paths = Object.keys(spec.paths ?? {});
    assert.ok(paths.length > 0, `${rel(api.source)} must define at least one path`);
    for (const path of paths) {
      assert.ok(path.startsWith(api.prefix), `${rel(api.source)} path ${path} must start with ${api.prefix}`);
    }
  }
});

test("SDK family metadata points at mirrored owner-only OpenAPI specs", () => {
  for (const api of apiAuthorities) {
    assert.ok(existsSync(resolve(root, api.sdkCopy)), `${rel(api.sdkCopy)} is required for sdkgen input closure`);

    const assembly = readJson(api.assembly);
    const component = readJson(api.component);
    const mirrored = readJson(api.sdkCopy);
    const source = readJson(api.source);

    assert.equal(assembly.sdkOwner, "sdkwork-cms", rel(api.assembly));
    assert.equal(assembly.sdkFamily, api.family, rel(api.assembly));
    assert.equal(assembly.apiAuthority, api.authority, rel(api.assembly));
    assert.equal(assembly.derivedSpecs.default, `openapi/${api.sdkCopy.split("/").pop()}`, rel(api.assembly));
    assert.deepEqual(mirrored.paths, source.paths, `${rel(api.sdkCopy)} must mirror ${rel(api.source)} paths`);
    assert.equal(component.component.name, api.family, rel(api.component));
  }
});

test("Rust route crates keep SDKWork CMS naming and prefix boundaries", () => {
  const cargo = read("Cargo.toml");
  for (const crateName of [
    "sdkwork-content-cms-service",
    "sdkwork-content-cms-repository-sqlx",
    "sdkwork-routes-cms-open-api",
    "sdkwork-routes-cms-app-api",
    "sdkwork-routes-cms-backend-api",
  ]) {
    assert.ok(cargo.includes(`"crates/${crateName}"`), `${crateName} must be a workspace member`);
  }

  const routePrefixes = [
    ["crates/sdkwork-routes-cms-open-api/src/paths.rs", 'pub const PREFIX: &str = "/cms/v3/api";'],
    ["crates/sdkwork-routes-cms-app-api/src/paths.rs", 'pub const PREFIX: &str = "/app/v3/api/cms";'],
    ["crates/sdkwork-routes-cms-backend-api/src/paths.rs", 'pub const PREFIX: &str = "/backend/v3/api/cms";'],
  ];

  for (const [path, expected] of routePrefixes) {
    assert.ok(read(path).includes(expected), `${rel(path)} must keep ${expected}`);
  }
});

test("OpenAPI operationIds are declared by their Rust route manifests", () => {
  const manifests = {
    "sdkwork-cms.open": read("crates/sdkwork-routes-cms-open-api/src/manifest.rs"),
    "sdkwork-cms.app": read("crates/sdkwork-routes-cms-app-api/src/manifest.rs"),
    "sdkwork-cms.backend": read("crates/sdkwork-routes-cms-backend-api/src/manifest.rs"),
  };

  for (const api of apiAuthorities) {
    const spec = readJson(api.source);
    const manifest = manifests[api.authority];
    for (const pathItem of Object.values(spec.paths ?? {})) {
      for (const [method, operation] of Object.entries(pathItem)) {
        if (method.startsWith("x-")) continue;
        assert.ok(operation.operationId, `${rel(api.source)} ${method} must define operationId`);
        assert.ok(
          manifest.includes(`"${operation.operationId}"`),
          `${operation.operationId} from ${rel(api.source)} must be declared in Rust route manifest`,
        );
      }
    }
  }
});

test("Backend API keeps a professional but focused v1 operation surface", () => {
  const backend = readJson("apis/backend-api/content/cms-backend-api.openapi.json");
  const operationRegistry = readJson("docs/api/cms-v1-operation-registry.json");
  const operations = Object.values(backend.paths ?? {}).flatMap((pathItem) =>
    Object.entries(pathItem)
      .filter(([method]) => !method.startsWith("x-"))
      .map(([, operation]) => operation.operationId),
  );

  assert.equal(operations.length, 66);
  assert.equal(operationRegistry.operationCount, 76);
  assert.equal(operationRegistry.operations.length, 76);
  for (const required of [
    "cms.sites.list",
    "cms.contentTypes.create",
    "cms.entries.publish",
    "cms.entries.rollback",
    "cms.pages.publish",
    "cms.feeds.publish",
    "cms.auditLogs.list",
    "cms.outboxEvents.retry",
  ]) {
    assert.ok(operations.includes(required), `backend OpenAPI must include ${required}`);
  }
});

test("Service and repository contracts expose method boundaries for every CMS API group", () => {
  const serviceFiles = [
    "crates/sdkwork-content-cms-service/src/service/sites.rs",
    "crates/sdkwork-content-cms-service/src/service/content_modeling.rs",
    "crates/sdkwork-content-cms-service/src/service/taxonomy.rs",
    "crates/sdkwork-content-cms-service/src/service/entries.rs",
    "crates/sdkwork-content-cms-service/src/service/pages.rs",
    "crates/sdkwork-content-cms-service/src/service/feeds.rs",
    "crates/sdkwork-content-cms-service/src/service/governance.rs",
    "crates/sdkwork-content-cms-service/src/service/delivery.rs",
  ];
  const repositoryFiles = [
    "crates/sdkwork-content-cms-repository-sqlx/src/repository/sites.rs",
    "crates/sdkwork-content-cms-repository-sqlx/src/repository/content_modeling.rs",
    "crates/sdkwork-content-cms-repository-sqlx/src/repository/taxonomy.rs",
    "crates/sdkwork-content-cms-repository-sqlx/src/repository/entries.rs",
    "crates/sdkwork-content-cms-repository-sqlx/src/repository/pages.rs",
    "crates/sdkwork-content-cms-repository-sqlx/src/repository/feeds.rs",
    "crates/sdkwork-content-cms-repository-sqlx/src/repository/governance.rs",
    "crates/sdkwork-content-cms-repository-sqlx/src/repository/publishing.rs",
  ];
  for (const path of [...serviceFiles, ...repositoryFiles]) {
    assert.ok(existsSync(resolve(root, path)), `${rel(path)} must exist as a focused capability module`);
  }

  const service = serviceFiles.map(read).join("\n");
  const repository = read("crates/sdkwork-content-cms-service/src/ports/repository.rs");
  const operationRegistry = readJson("docs/api/cms-v1-operation-registry.json");

  const expectedServiceMethods = [
    "list_sites",
    "create_site",
    "retrieve_site",
    "update_site",
    "delete_site",
    "list_channels",
    "create_channel",
    "update_channel",
    "delete_channel",
    "list_content_types",
    "create_content_type",
    "retrieve_content_type",
    "update_content_type",
    "delete_content_type",
    "list_content_fields",
    "create_content_field",
    "update_content_field",
    "delete_content_field",
    "list_taxonomies",
    "create_taxonomy",
    "update_taxonomy",
    "delete_taxonomy",
    "list_taxonomy_terms",
    "create_taxonomy_term",
    "update_taxonomy_term",
    "delete_taxonomy_term",
    "list_entries",
    "create_entry",
    "retrieve_entry",
    "update_entry",
    "delete_entry",
    "replace_entry_body",
    "replace_entry_fields",
    "list_entry_media",
    "attach_entry_media",
    "delete_entry_media",
    "replace_entry_terms",
    "list_entry_versions",
    "publish_entry",
    "unpublish_entry",
    "rollback_entry",
    "schedule_entry",
    "list_pages",
    "create_page",
    "retrieve_page",
    "update_page",
    "delete_page",
    "replace_page_blocks",
    "publish_page",
    "list_feeds",
    "create_feed",
    "retrieve_feed",
    "update_feed",
    "delete_feed",
    "list_feed_rules",
    "create_feed_rule",
    "update_feed_rule",
    "delete_feed_rule",
    "list_feed_items",
    "upsert_feed_items",
    "delete_feed_item",
    "publish_feed",
    "retrieve_feed_snapshot",
    "list_audit_logs",
    "list_outbox_events",
    "retry_outbox_event",
    "delivery_bootstrap",
    "delivery_list_entries",
    "delivery_resolve_entry",
    "delivery_retrieve_entry",
    "delivery_resolve_page",
    "delivery_list_feed_items",
  ];

  const expectedRepositoryMethods = expectedServiceMethods.filter(
    (method) => !method.startsWith("delivery_"),
  );

  for (const method of expectedServiceMethods) {
    assert.match(service, new RegExp(`pub fn ${method}\\s*\\(`), `CmsService must expose ${method}`);
    assert.match(service, new RegExp(`TODO:[^\\n]+${method.replaceAll("_", "[._]")}|${method}`), `${method} must keep TODO implementation guidance`);
  }

  for (const method of expectedRepositoryMethods) {
    assert.match(repository, new RegExp(`fn ${method}\\s*\\(`), `CmsRepository must expose ${method}`);
  }

  for (const operation of operationRegistry.operations) {
    assert.ok(operation.serviceMethod, `${operation.operationId} must map to a CmsService method`);
    assert.match(service, new RegExp(`pub fn ${operation.serviceMethod}\\s*\\(`), `${operation.operationId} service method ${operation.serviceMethod} must exist`);
  }
});

test("CMS permission registry aligns backend OpenAPI and operation registry permissions", () => {
  const permissionRegistry = readJson("docs/security/cms-v1-permission-registry.json");
  const operationRegistry = readJson("docs/api/cms-v1-operation-registry.json");
  const backend = readJson("apis/backend-api/content/cms-backend-api.openapi.json");

  assert.equal(permissionRegistry.kind, "sdkwork.permission.registry");
  assert.equal(permissionRegistry.owner, "sdkwork-cms");
  assert.equal(permissionRegistry.domain, "content");
  assert.equal(permissionRegistry.capability, "cms");

  const expectedPermissions = [
    ...new Set(operationRegistry.operations.map((operation) => operation.permission).filter(Boolean)),
  ].sort();
  const registeredPermissions = permissionRegistry.permissions.map((permission) => permission.code).sort();
  assert.deepEqual(registeredPermissions, expectedPermissions);

  const permissionsByCode = new Map(
    permissionRegistry.permissions.map((permission) => [permission.code, permission]),
  );

  for (const permission of permissionRegistry.permissions) {
    assert.match(permission.code, /^cms\.[a-z0-9_]+\.[a-z0-9_]+$/);
    assert.ok(permission.description, `${permission.code} must describe the authorization boundary`);
    assert.ok(permission.resource, `${permission.code} must declare its CMS resource`);
    assert.ok(permission.action, `${permission.code} must declare its permission action`);
    assert.ok(permission.tenantScope, `${permission.code} must declare tenant scope`);
    assert.ok(permission.operationIds.length > 0, `${permission.code} must map to operations`);
    assert.match(permission.implementationTodo, /^TODO:/, `${permission.code} must keep a handoff TODO`);
  }

  for (const operation of operationRegistry.operations.filter((operation) => operation.permission)) {
    assert.ok(
      permissionsByCode.get(operation.permission)?.operationIds.includes(operation.operationId),
      `${operation.operationId} must be mapped by ${operation.permission}`,
    );
  }

  for (const pathItem of Object.values(backend.paths ?? {})) {
    for (const [method, operation] of Object.entries(pathItem)) {
      if (method.startsWith("x-")) continue;
      const permission = operation["x-sdkwork-permission"];
      assert.ok(permission, `${operation.operationId} must declare x-sdkwork-permission`);
      assert.ok(permissionsByCode.has(permission), `${operation.operationId} permission ${permission} must be registered`);
    }
  }
});

test("CMS integration registry aligns outbox events, service metadata, and dependency ports", () => {
  const integrationRegistry = readJson("docs/integration/cms-v1-integration-registry.json");
  const serviceComponent = readJson("crates/sdkwork-content-cms-service/specs/component.spec.json");

  assert.equal(integrationRegistry.kind, "sdkwork.integration.registry");
  assert.equal(integrationRegistry.owner, "sdkwork-cms");
  assert.equal(integrationRegistry.domain, "content");
  assert.equal(integrationRegistry.capability, "cms");

  const eventsByName = new Map(integrationRegistry.events.map((event) => [event.name, event]));
  const serviceEvents = serviceComponent.contracts.events;
  for (const eventName of serviceEvents) {
    assert.ok(eventsByName.has(eventName), `${eventName} from service component spec must be registered`);
  }

  for (const requiredEvent of [
    "cms.entry.created",
    "cms.entry.updated",
    "cms.entry.published",
    "cms.entry.unpublished",
    "cms.entry.rolled_back",
    "cms.page.published",
    "cms.feed.published",
    "cms.search.sync_requested",
    "cms.cache.invalidate_requested",
    "cms.notification.requested",
    "cms.webhook.delivery_requested",
    "cms.sitemap.projection_requested",
    "cms.rss.projection_requested",
  ]) {
    assert.ok(eventsByName.has(requiredEvent), `${requiredEvent} must be registered`);
  }

  for (const event of integrationRegistry.events) {
    assert.equal(event.owner, "sdkwork-cms", `${event.name} must stay CMS-owned`);
    assert.equal(event.outboxTable, "cms_outbox_event", `${event.name} must use the CMS outbox table`);
    assert.ok(event.producer, `${event.name} must declare a producer`);
    assert.ok(event.consumerBoundary, `${event.name} must declare the consumer boundary`);
    assert.match(event.implementationTodo, /^TODO:/, `${event.name} must keep a handoff TODO`);
  }

  const portTraits = new Set(integrationRegistry.dependencyPorts.map((port) => port.trait));
  for (const requiredTrait of [
    "CmsIamAuthorizer",
    "CmsDriveMediaPort",
    "CmsSearchSyncPort",
    "CmsCacheInvalidationPort",
    "CmsSchedulerPort",
    "CmsNotificationPort",
    "CmsWebhookPort",
    "CmsEngagementPort",
    "CmsSitemapProjectionPort",
  ]) {
    assert.ok(portTraits.has(requiredTrait), `${requiredTrait} must be registered as an integration boundary`);
  }
});

test("Route crates expose focused handler and DTO skeleton modules for every API operation", () => {
  const operationRegistry = readJson("docs/api/cms-v1-operation-registry.json");
  const routeCrates = [
    {
      root: "crates/sdkwork-routes-cms-open-api",
      surface: "open-api",
      handlerModules: ["entries", "pages", "feeds"],
    },
    {
      root: "crates/sdkwork-routes-cms-app-api",
      surface: "app-api",
      handlerModules: ["delivery"],
    },
    {
      root: "crates/sdkwork-routes-cms-backend-api",
      surface: "backend-api",
      handlerModules: [
        "sites",
        "content_modeling",
        "taxonomy",
        "entries",
        "pages",
        "feeds",
        "governance",
      ],
    },
  ];

  for (const routeCrate of routeCrates) {
    const lib = read(`${routeCrate.root}/src/lib.rs`);
    assert.ok(lib.includes("pub mod dto;"), `${routeCrate.root} must expose DTO skeletons`);

    for (const dtoPath of ["src/dto/mod.rs", "src/dto/request.rs", "src/dto/response.rs"]) {
      assert.ok(existsSync(resolve(root, routeCrate.root, dtoPath)), `${routeCrate.root}/${dtoPath} must exist`);
      assert.match(read(`${routeCrate.root}/${dtoPath}`), /TODO:/, `${routeCrate.root}/${dtoPath} must keep TODO guidance`);
    }

    const handlersMod = read(`${routeCrate.root}/src/handlers.rs`);
    for (const moduleName of routeCrate.handlerModules) {
      const modulePath = `${routeCrate.root}/src/handlers/${moduleName}.rs`;
      assert.ok(existsSync(resolve(root, modulePath)), `${modulePath} must exist`);
      assert.ok(handlersMod.includes(`pub mod ${moduleName};`), `${routeCrate.root} must export ${moduleName}`);
      assert.match(read(modulePath), /TODO:/, `${modulePath} must keep TODO guidance`);
    }

    const handlerSource = routeCrate.handlerModules
      .map((moduleName) => read(`${routeCrate.root}/src/handlers/${moduleName}.rs`))
      .join("\n");
    const serviceMethods = [
      ...new Set(
        operationRegistry.operations
          .filter((operation) => operation.surface === routeCrate.surface)
          .map((operation) => operation.serviceMethod),
      ),
    ];

    for (const serviceMethod of serviceMethods) {
      assert.match(
        handlerSource,
        new RegExp(`pub fn ${serviceMethod}\\s*\\([^]*?TODO:`),
        `${routeCrate.root} handlers must expose ${serviceMethod} with TODO guidance`,
      );
    }
  }
});
