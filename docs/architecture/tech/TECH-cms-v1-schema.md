> Migrated from `docs/database/cms-v1-schema.md` on 2026-06-24.
> Owner: SDKWork maintainers

本文档是 SDKWork CMS v1 的数据库契约说明。DDL 草案位于：

`crates/sdkwork-content-cms-repository-sqlx/migrations/0001_cms_v1_foundation.sql`

结构化表目录位于：

`docs/database/cms-v1-table-registry.json`

Column-level machine-readable contract:
`docs/database/cms-v1-column-registry.json`

后续 ORM、DTO、OpenAPI schema、SDK DTO 或数据库校验工具应优先读取该 registry，再反查 DDL。DDL 是执行草案，registry 是面向工程协作和契约检查的清单。

## 设计边界

CMS 自有表统一使用 `cms_` 前缀。`content` 可作为 SDKWork 上层领域归类，但数据库、API、SDK、权限、事件均使用 `cms` 作为能力前缀。

CMS 拥有内容模型、内容条目、页面、发布快照、feeds、审计和 outbox。CMS 不复制 IAM、Drive、Search、Messaging、Comments、Engagement 的事实表。

## 可复用表检查结论

| 来源项目 | 可复用能力 | CMS 处理方式 |
| --- | --- | --- |
| `sdkwork-appbase` | `iam_tenant`, `iam_user`, `iam_organization`, `iam_role`, `iam_permission`, `iam_policy`, `iam_audit_event`, `iam_security_event` 等 | CMS 保存 `tenant_id`, `organization_id`, `created_by`, `updated_by`, `author_user_id` 等引用，不建用户、角色、权限表 |
| `sdkwork-drive` | `dr_drive_space`, `dr_drive_node`, `dr_drive_storage_object`, `dr_drive_upload_session`, `dr_drive_node_version` 等 | CMS 通过 Drive 管理文件和媒体生命周期，只在 `cms_entry_media` 中保存 Drive 引用和媒体快照 |
| `sdkwork-search` | `search_index`, `search_document`, `search_ranking_profile`, `search_recommendation_strategy`, `search_query_suggestion`, `search_embedding_job` 等 | CMS 通过 `cms_outbox_event` 请求搜索同步，不在 v1 主库建立搜索索引/向量表 |
| `sdkwork-news` | `news_item`, `news_channel`, `news_feed_stream`, `news_feed_candidate`, `news_search_projection` 等 | 仅作为内容/feeds 设计参考，不直接复用 `news_*` 表 |
| `sdkwork-knowledgebase` | `kb_document`, `kb_chunk`, `kb_index`, `kb_embedding` 等 | 未来知识内容接入通过 KB/Search 域实现，不把 chunk/embedding 放进 CMS |
| `sdkwork-comments` / engagement | `comments_*`, `engagement_*` | 公开评论、点赞、收藏优先复用互动域；CMS v1 仅保留后台审计 |
| `sdkwork-messaging` | `messaging_*` 通知和发送能力 | CMS 通过事件或 Messaging SDK 发送通知，不建通用通知发送表 |

## 表目录

| 表名 | 表画像 | 事实来源 | 用途 |
| --- | --- | --- | --- |
| `cms_site` | tenant_entity | 是 | CMS 站点/工作区。保存站点编码、名称、默认语言、域名配置、站点配置、发布配置 |
| `cms_channel` | dictionary_entity | 是 | 发布频道。支持 web、app、h5、mini_program、rss、api 等交付渠道 |
| `cms_content_type` | dictionary_entity | 是 | 内容类型定义。比如 article、page、banner、notice、topic |
| `cms_content_field` | dictionary_entity | 是 | 内容字段定义。保存字段类型、校验、枚举选项、搜索/筛选/排序标记和 UI 配置 |
| `cms_taxonomy` | dictionary_entity | 是 | 分类体系定义。比如 category、tag、topic、audience |
| `cms_taxonomy_term` | tree_entity | 是 | 分类项/标签项。支持树形结构、slug、path、排序、启停 |
| `cms_entry` | tenant_entity | 是 | 内容条目主表。保存高频列表、路由、发布筛选字段 |
| `cms_entry_body` | core_entity | 是 | 内容正文。保存富文本、Markdown、HTML、块结构 JSON、纯文本投影 |
| `cms_entry_field_value` | relation_entity | 是 | 动态字段值。支持内容模型自定义字段筛选、排序和搜索投影 |
| `cms_entry_version` | snapshot | 是 | 内容版本快照。用于历史版本、发布版本、回滚 |
| `cms_entry_media` | relation_entity | 是 | 内容与 Drive/MediaResource 关系。保存媒体角色、顺序、alt、caption 和快照 |
| `cms_entry_term` | relation_entity | 是 | 内容与分类/标签关系 |
| `cms_page` | tenant_entity | 是 | 页面路由。保存 path、模板、SEO、发布状态 |
| `cms_page_block` | relation_entity | 是 | 页面区块编排。保存 slot、block 类型、排序、配置和引用对象 |
| `cms_publish_snapshot` | snapshot | 是 | 发布快照。内容、页面、feed 发布后形成稳定交付版本 |
| `cms_feed` | tenant_entity | 是 | Feed 定义。支持首页流、频道流、专题流、运营精选流 |
| `cms_feed_rule` | dictionary_entity | 是 | Feed 自动规则。按内容类型、分类、标签、发布时间、状态等聚合内容 |
| `cms_feed_item` | relation_entity | 是 | Feed 人工编排项。支持置顶、固定位置、运营标题、运营图和有效期 |
| `cms_feed_snapshot` | snapshot | 是 | Feed 发布快照。前台读取稳定 feed 结果 |
| `cms_audit_log` | audit_log | 是 | CMS 业务审计。记录内容、模型、页面、feed、发布、回滚等动作 |
| `cms_outbox_event` | outbox_event | 是 | 可靠事件 outbox。用于搜索同步、缓存失效、通知、Webhook 等 |
| `cms_idempotency_key` | event_log | 是 | 幂等记录。保护创建、发布、回滚、导入等可重试命令 |

## 表关系概览

- `cms_site` 是站点边界。
- `cms_channel`, `cms_content_type`, `cms_taxonomy`, `cms_feed`, `cms_page` 挂在 `cms_site` 下。
- `cms_content_field` 挂在 `cms_content_type` 下。
- `cms_entry` 是内容聚合根，关联 `cms_entry_body`, `cms_entry_field_value`, `cms_entry_version`, `cms_entry_media`, `cms_entry_term`。
- `cms_page` 通过 `cms_page_block` 引用 entry、feed、media 或配置型 block。
- `cms_publish_snapshot` 对 entry、page、feed 统一建快照。
- `cms_feed_snapshot` 保存 feed 的稳定 items；`cms_publish_snapshot` 保存统一发布事实。
- `cms_audit_log` 和 `cms_outbox_event` 记录跨聚合副作用。

## 标准字段

核心业务表使用：

- `id BIGINT`：内部主键，API/SDK 以 string 序列化。
- `uuid VARCHAR(64)`：外部稳定 ID。
- `tenant_id BIGINT`：IAM 租户 ID。
- `organization_id BIGINT DEFAULT 0`：IAM 组织 ID。
- `site_id BIGINT`：CMS 站点 ID。
- `data_scope INTEGER`：数据范围。
- `created_at`, `updated_at`：UTC 时间。
- `created_by`, `updated_by`：IAM 用户 ID。
- `version BIGINT`：乐观锁。
- `status INTEGER`：业务状态。
- `deleted_at`, `deleted_by`：软删除。

关系表可简化，但必须保留租户/站点隔离字段和唯一约束。

## 状态约定

通用 `status`：

| 值 | 含义 |
| ---: | --- |
| `0` | disabled / inactive |
| `1` | active |
| `2` | archived |
| `9` | deleted |

内容 `entry_status`：

| 值 | 含义 |
| ---: | --- |
| `0` | draft |
| `10` | reviewing |
| `20` | approved |
| `30` | published |
| `40` | archived |

发布 `publication_status`：

| 值 | 含义 |
| ---: | --- |
| `0` | unpublished |
| `10` | scheduled |
| `20` | published |
| `30` | unpublished_after_publish |
| `40` | rolled_back |

## 高频查询列化要求

以下字段不能只存在 JSON 中：

- 内容列表：`tenant_id`, `organization_id`, `site_id`, `content_type_id`, `channel_id`, `locale`, `entry_status`, `publication_status`, `author_user_id`, `published_at`
- 路由解析：`site_id`, `channel_id`, `locale`, `slug`, `path`, `status`
- Feed 读取：`feed_id`, `channel_id`, `locale`, `published_snapshot_id`, `status`
- 审计查询：`tenant_id`, `site_id`, `actor_user_id`, `resource_type`, `resource_id`, `action`, `created_at`
- 事件发布：`event_type`, `aggregate_type`, `aggregate_id`, `status`, `next_attempt_at`

## 延后到 v2/v3 的表

以下能力不在 v1 物理建表：

- 多级工作流：`cms_workflow`, `cms_review_task`, `cms_review_decision`
- Webhook 配置：`cms_webhook_endpoint`, `cms_webhook_delivery`
- 导入导出：`cms_import_job`, `cms_import_item`, `cms_export_job`
- 搜索投影：`cms_search_projection`, `cms_search_rebuild_job`
- RSS/Sitemap：`cms_rss_feed_snapshot`, `cms_sitemap_snapshot`
- 指标投影：`cms_feed_metric_projection`
- 导航树：`cms_navigation`, `cms_navigation_item`
- 精细编辑锁/批注：`cms_entry_lock`, `cms_entry_annotation`

这些能力可通过 v1 的 JSON 配置、审计日志、outbox 事件或上游依赖 SDK 先行支撑，等产品行为稳定后再拆表。

