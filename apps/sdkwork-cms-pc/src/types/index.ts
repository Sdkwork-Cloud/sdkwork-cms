export interface CmsSite {
  id: string;
  uuid: string;
  tenantId: string;
  organizationId: string;
  code: string;
  name: string;
  defaultLocale: string;
  settingsJson: string;
  status: number;
  version: string;
}

export interface CmsChannel {
  id: string;
  siteId: string;
  code: string;
  name: string;
  channelKind: string;
  status: number;
}

export interface CmsContentType {
  id: string;
  siteId: string;
  code: string;
  name: string;
  contentKind: string;
  schemaVersion: string;
  status: number;
}

export interface CmsContentField {
  id: string;
  contentTypeId: string;
  code: string;
  name: string;
  fieldKind: string;
  required: boolean;
  searchable: boolean;
  filterable: boolean;
  sortable: boolean;
}

export interface CmsTaxonomy {
  id: string;
  siteId: string;
  code: string;
  name: string;
  taxonomyKind: string;
  status: number;
}

export interface CmsTaxonomyTerm {
  id: string;
  taxonomyId: string;
  parentId?: string;
  code: string;
  slug: string;
  name: string;
  path: string;
  status: number;
}

export interface CmsEntry {
  id: string;
  uuid: string;
  siteId: string;
  contentTypeId: string;
  channelId?: string;
  locale: string;
  title: string;
  slug: string;
  summary?: string;
  entryStatus: EntryStatus;
  publicationStatus: PublicationStatus;
  publishedAt?: string;
  version: string;
}

export interface CmsPage {
  id: string;
  siteId: string;
  channelId?: string;
  locale: string;
  path: string;
  title: string;
  publicationStatus: PublicationStatus;
  version: string;
}

export interface CmsFeed {
  id: string;
  siteId: string;
  channelId?: string;
  code: string;
  name: string;
  feedKind: string;
  locale: string;
  status: number;
  version: string;
}

export interface CmsFeedRule {
  id: string;
  feedId: string;
  ruleKind: string;
  conditionJson: string;
  sortJson: string;
  enabled: boolean;
}

export interface CmsFeedItem {
  id: string;
  feedId: string;
  entryId?: string;
  pageId?: string;
  externalUrl?: string;
  itemKind: string;
  pinned: boolean;
  sortOrder: number;
}

export interface CmsPublishSnapshot {
  id: string;
  ownerType: string;
  ownerId: string;
  snapshotPayloadJson: string;
  status: number;
  publishedAt: string;
}

export interface CmsAuditLog {
  id: string;
  siteId?: string;
  actorUserId: string;
  action: string;
  resourceType: string;
  resourceId?: string;
  beforeJson: string;
  afterJson: string;
  createdAt: string;
}

export interface CmsOutboxEvent {
  id: string;
  aggregateType: string;
  aggregateId: string;
  eventType: string;
  payloadJson: string;
  status: number;
  attemptCount: number;
  nextAttemptAt?: string;
  createdAt: string;
}

export interface CmsMediaRef {
  id: string;
  role: string;
  driveSpaceId?: string;
  driveNodeId?: string;
  driveUri?: string;
  mediaResourceId?: string;
  mediaSnapshotJson: string;
}

export interface CmsEntryVersion {
  id: string;
  entryId: string;
  versionNo: string;
  versionKind: string;
  checksum?: string;
}

export type EntryStatus = 'draft' | 'reviewing' | 'approved' | 'published' | 'archived' | 'deleted';
export type PublicationStatus = 'unpublished' | 'scheduled' | 'published' | 'unpublished_after_publish' | 'rolled_back';

export interface PaginatedResponse<T> {
  items: T[];
  nextCursor?: string;
  totalCount?: number;
}

export interface CommandResponse {
  ok: boolean;
  resourceId?: string;
  requestId?: string;
}

export interface ApiResponse<T> {
  ok: boolean;
  data?: T;
  error?: ProblemDetail;
  requestId?: string;
}

export interface ProblemDetail {
  type: string;
  title: string;
  status: number;
  detail: string;
  instance?: string;
  requestId?: string;
  code?: string;
}

export interface ListQueryParams {
  cursor?: string;
  limit?: number;
}

export interface ListSitesQueryParams extends ListQueryParams {}

export interface ListBySiteQueryParams extends ListQueryParams {}

export interface ListEntriesQueryParams extends ListQueryParams {
  siteId?: string;
  contentTypeId?: string;
  channelId?: string;
  locale?: string;
  entryStatus?: number;
  publicationStatus?: number;
  authorUserId?: string;
}

export interface SiteCreateRequest {
  code: string;
  name: string;
  description?: string;
  defaultLocale?: string;
  settingsJson?: string;
}

export interface SiteUpdateRequest {
  code?: string;
  name?: string;
  description?: string;
  defaultLocale?: string;
  settingsJson?: string;
  version?: string;
}

export interface EntryCreateRequest {
  siteId: string;
  contentTypeId: string;
  channelId?: string;
  locale: string;
  title: string;
  slug: string;
  summary?: string;
  seoJson?: string;
}

export interface EntryUpdateRequest {
  channelId?: string;
  locale?: string;
  title?: string;
  slug?: string;
  summary?: string;
  seoJson?: string;
  version?: string;
}

export interface PublishRequest {
  channelId?: string;
  locale?: string;
  note?: string;
  version?: string;
}

export interface RollbackRequest {
  targetVersionId: string;
  note?: string;
  version?: string;
}

export interface ScheduleRequest {
  scheduledPublishAt?: string;
  scheduledUnpublishAt?: string;
  version?: string;
}
