import type {
  ApiResponse,
  PaginatedResponse,
  CommandResponse,
  CmsSite,
  CmsChannel,
  CmsContentType,
  CmsContentField,
  CmsTaxonomy,
  CmsTaxonomyTerm,
  CmsEntry,
  CmsPage,
  CmsFeed,
  CmsFeedRule,
  CmsFeedItem,
  CmsPublishSnapshot,
  CmsAuditLog,
  CmsOutboxEvent,
  CmsMediaRef,
  CmsEntryVersion,
  ListSitesQueryParams,
  ListBySiteQueryParams,
  ListEntriesQueryParams,
  SiteCreateRequest,
  SiteUpdateRequest,
  EntryCreateRequest,
  EntryUpdateRequest,
  PublishRequest,
  RollbackRequest,
  ScheduleRequest,
} from '@/types';

const BASE_URL = '/backend/v3/api/cms';

class CmsApiClient {
  private accessToken: string = '';

  setAccessToken(token: string) {
    this.accessToken = token;
  }

  private async request<T>(
    method: string,
    path: string,
    body?: unknown,
    params?: Record<string, string | number | undefined>
  ): Promise<ApiResponse<T>> {
    const url = new URL(`${BASE_URL}${path}`, window.location.origin);
    if (params) {
      Object.entries(params).forEach(([key, value]) => {
        if (value !== undefined) {
          url.searchParams.set(key, String(value));
        }
      });
    }

    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
    };
    if (this.accessToken) {
      headers['Authorization'] = `Bearer ${this.accessToken}`;
    }

    const response = await fetch(url.toString(), {
      method,
      headers,
      body: body ? JSON.stringify(body) : undefined,
    });

    return response.json();
  }

  // Sites
  async listSites(params?: ListSitesQueryParams): Promise<ApiResponse<PaginatedResponse<CmsSite>>> {
    return this.request('GET', '/sites', undefined, params as Record<string, string>);
  }

  async createSite(data: SiteCreateRequest): Promise<ApiResponse<CmsSite>> {
    return this.request('POST', '/sites', data);
  }

  async retrieveSite(siteId: string): Promise<ApiResponse<CmsSite>> {
    return this.request('GET', `/sites/${siteId}`);
  }

  async updateSite(siteId: string, data: SiteUpdateRequest): Promise<ApiResponse<CmsSite>> {
    return this.request('PATCH', `/sites/${siteId}`, data);
  }

  async deleteSite(siteId: string): Promise<ApiResponse<CommandResponse>> {
    return this.request('DELETE', `/sites/${siteId}`);
  }

  // Channels
  async listChannels(siteId: string, params?: ListBySiteQueryParams): Promise<ApiResponse<PaginatedResponse<CmsChannel>>> {
    return this.request('GET', `/sites/${siteId}/channels`, undefined, params as Record<string, string>);
  }

  async createChannel(siteId: string, data: { code: string; name: string; channelKind?: string }): Promise<ApiResponse<CmsChannel>> {
    return this.request('POST', `/sites/${siteId}/channels`, data);
  }

  // Content Types
  async listContentTypes(siteId: string, params?: ListBySiteQueryParams): Promise<ApiResponse<PaginatedResponse<CmsContentType>>> {
    return this.request('GET', `/sites/${siteId}/content_types`, undefined, params as Record<string, string>);
  }

  async createContentType(siteId: string, data: { code: string; name: string; contentKind?: string }): Promise<ApiResponse<CmsContentType>> {
    return this.request('POST', `/sites/${siteId}/content_types`, data);
  }

  async retrieveContentType(contentTypeId: string): Promise<ApiResponse<CmsContentType>> {
    return this.request('GET', `/content_types/${contentTypeId}`);
  }

  // Content Fields
  async listContentFields(contentTypeId: string, params?: ListBySiteQueryParams): Promise<ApiResponse<PaginatedResponse<CmsContentField>>> {
    return this.request('GET', `/content_types/${contentTypeId}/fields`, undefined, params as Record<string, string>);
  }

  async createContentField(contentTypeId: string, data: { code: string; name: string; fieldKind: string }): Promise<ApiResponse<CmsContentField>> {
    return this.request('POST', `/content_types/${contentTypeId}/fields`, data);
  }

  // Taxonomies
  async listTaxonomies(siteId: string, params?: ListBySiteQueryParams): Promise<ApiResponse<PaginatedResponse<CmsTaxonomy>>> {
    return this.request('GET', `/sites/${siteId}/taxonomies`, undefined, params as Record<string, string>);
  }

  async createTaxonomy(siteId: string, data: { code: string; name: string; taxonomyKind?: string }): Promise<ApiResponse<CmsTaxonomy>> {
    return this.request('POST', `/sites/${siteId}/taxonomies`, data);
  }

  // Taxonomy Terms
  async listTaxonomyTerms(taxonomyId: string, params?: ListBySiteQueryParams): Promise<ApiResponse<PaginatedResponse<CmsTaxonomyTerm>>> {
    return this.request('GET', `/taxonomies/${taxonomyId}/terms`, undefined, params as Record<string, string>);
  }

  async createTaxonomyTerm(taxonomyId: string, data: { code: string; name: string; slug?: string; parentId?: string }): Promise<ApiResponse<CmsTaxonomyTerm>> {
    return this.request('POST', `/taxonomies/${taxonomyId}/terms`, data);
  }

  // Entries
  async listEntries(params?: ListEntriesQueryParams): Promise<ApiResponse<PaginatedResponse<CmsEntry>>> {
    return this.request('GET', '/entries', undefined, params as Record<string, string>);
  }

  async createEntry(data: EntryCreateRequest): Promise<ApiResponse<CmsEntry>> {
    return this.request('POST', '/entries', data);
  }

  async retrieveEntry(entryId: string): Promise<ApiResponse<CmsEntry>> {
    return this.request('GET', `/entries/${entryId}`);
  }

  async updateEntry(entryId: string, data: EntryUpdateRequest): Promise<ApiResponse<CmsEntry>> {
    return this.request('PATCH', `/entries/${entryId}`, data);
  }

  async deleteEntry(entryId: string): Promise<ApiResponse<CommandResponse>> {
    return this.request('DELETE', `/entries/${entryId}`);
  }

  async replaceEntryBody(entryId: string, data: { locale: string; bodyFormat: string; bodyText?: string; bodyHtml?: string; blockTreeJson: string }): Promise<ApiResponse<CmsEntry>> {
    return this.request('PUT', `/entries/${entryId}/body`, data);
  }

  async replaceEntryFields(entryId: string, data: { locale: string; fieldsJson: string }): Promise<ApiResponse<CmsEntry>> {
    return this.request('PUT', `/entries/${entryId}/fields`, data);
  }

  async listEntryMedia(entryId: string, params?: ListBySiteQueryParams): Promise<ApiResponse<PaginatedResponse<CmsMediaRef>>> {
    return this.request('GET', `/entries/${entryId}/media`, undefined, params as Record<string, string>);
  }

  async attachEntryMedia(entryId: string, data: { mediaRole: string; driveNodeId?: string; driveUri?: string }): Promise<ApiResponse<CmsMediaRef>> {
    return this.request('POST', `/entries/${entryId}/media`, data);
  }

  async deleteEntryMedia(mediaId: string): Promise<ApiResponse<CommandResponse>> {
    return this.request('DELETE', `/entries/media/${mediaId}`);
  }

  async replaceEntryTerms(entryId: string, data: { termIds: string[] }): Promise<ApiResponse<CmsEntry>> {
    return this.request('PUT', `/entries/${entryId}/terms`, data);
  }

  async listEntryVersions(entryId: string, params?: ListBySiteQueryParams): Promise<ApiResponse<PaginatedResponse<CmsEntryVersion>>> {
    return this.request('GET', `/entries/${entryId}/versions`, undefined, params as Record<string, string>);
  }

  async publishEntry(entryId: string, data: PublishRequest): Promise<ApiResponse<CmsPublishSnapshot>> {
    return this.request('POST', `/entries/${entryId}:publish`, data);
  }

  async unpublishEntry(entryId: string, data: PublishRequest): Promise<ApiResponse<CmsPublishSnapshot>> {
    return this.request('POST', `/entries/${entryId}:unpublish`, data);
  }

  async rollbackEntry(entryId: string, data: RollbackRequest): Promise<ApiResponse<CmsPublishSnapshot>> {
    return this.request('POST', `/entries/${entryId}:rollback`, data);
  }

  async scheduleEntry(entryId: string, data: ScheduleRequest): Promise<ApiResponse<CmsEntry>> {
    return this.request('POST', `/entries/${entryId}:schedule`, data);
  }

  // Pages
  async listPages(params?: { siteId?: string; channelId?: string; locale?: string; status?: number; cursor?: string; limit?: number }): Promise<ApiResponse<PaginatedResponse<CmsPage>>> {
    return this.request('GET', '/pages', undefined, params as Record<string, string>);
  }

  async createPage(data: { siteId: string; channelId?: string; locale: string; path: string; slug: string; title: string }): Promise<ApiResponse<CmsPage>> {
    return this.request('POST', '/pages', data);
  }

  async retrievePage(pageId: string): Promise<ApiResponse<CmsPage>> {
    return this.request('GET', `/pages/${pageId}`);
  }

  async updatePage(pageId: string, data: { channelId?: string; locale?: string; path?: string; slug?: string; title?: string; version?: string }): Promise<ApiResponse<CmsPage>> {
    return this.request('PATCH', `/pages/${pageId}`, data);
  }

  async deletePage(pageId: string): Promise<ApiResponse<CommandResponse>> {
    return this.request('DELETE', `/pages/${pageId}`);
  }

  async replacePageBlocks(pageId: string, data: { blocksJson: string; version?: string }): Promise<ApiResponse<CmsPage>> {
    return this.request('PUT', `/pages/${pageId}/blocks`, data);
  }

  async publishPage(pageId: string, data: PublishRequest): Promise<ApiResponse<CmsPublishSnapshot>> {
    return this.request('POST', `/pages/${pageId}:publish`, data);
  }

  // Feeds
  async listFeeds(params?: { siteId?: string; channelId?: string; locale?: string; status?: number; cursor?: string; limit?: number }): Promise<ApiResponse<PaginatedResponse<CmsFeed>>> {
    return this.request('GET', '/feeds', undefined, params as Record<string, string>);
  }

  async createFeed(data: { siteId: string; channelId?: string; code: string; name: string; feedKind?: string; locale?: string }): Promise<ApiResponse<CmsFeed>> {
    return this.request('POST', '/feeds', data);
  }

  async retrieveFeed(feedId: string): Promise<ApiResponse<CmsFeed>> {
    return this.request('GET', `/feeds/${feedId}`);
  }

  async updateFeed(feedId: string, data: { code?: string; name?: string; feedKind?: string; locale?: string; version?: string }): Promise<ApiResponse<CmsFeed>> {
    return this.request('PATCH', `/feeds/${feedId}`, data);
  }

  async deleteFeed(feedId: string): Promise<ApiResponse<CommandResponse>> {
    return this.request('DELETE', `/feeds/${feedId}`);
  }

  // Feed Rules
  async listFeedRules(feedId: string, params?: { enabled?: boolean; cursor?: string; limit?: number }): Promise<ApiResponse<PaginatedResponse<CmsFeedRule>>> {
    return this.request('GET', `/feeds/${feedId}/rules`, undefined, params as Record<string, string>);
  }

  async createFeedRule(feedId: string, data: { ruleKind: string; conditionJson: string; sortJson: string; limitCount?: number; enabled?: boolean }): Promise<ApiResponse<CmsFeedRule>> {
    return this.request('POST', `/feeds/${feedId}/rules`, data);
  }

  // Feed Items
  async listFeedItems(feedId: string, params?: { status?: number; cursor?: string; limit?: number }): Promise<ApiResponse<PaginatedResponse<CmsFeedItem>>> {
    return this.request('GET', `/feeds/${feedId}/items`, undefined, params as Record<string, string>);
  }

  async upsertFeedItems(feedId: string, data: { itemsJson: string }): Promise<ApiResponse<PaginatedResponse<CmsFeedItem>>> {
    return this.request('PUT', `/feeds/${feedId}/items`, data);
  }

  async deleteFeedItem(itemId: string): Promise<ApiResponse<CommandResponse>> {
    return this.request('DELETE', `/feeds/items/${itemId}`);
  }

  async publishFeed(feedId: string, data: PublishRequest): Promise<ApiResponse<CmsPublishSnapshot>> {
    return this.request('POST', `/feeds/${feedId}:publish`, data);
  }

  // Governance
  async listAuditLogs(params?: { siteId?: string; resourceType?: string; resourceId?: string; actorUserId?: string; cursor?: string; limit?: number }): Promise<ApiResponse<PaginatedResponse<CmsAuditLog>>> {
    return this.request('GET', '/audit_logs', undefined, params as Record<string, string>);
  }

  async listOutboxEvents(params?: { aggregateType?: string; aggregateId?: string; status?: number; cursor?: string; limit?: number }): Promise<ApiResponse<PaginatedResponse<CmsOutboxEvent>>> {
    return this.request('GET', '/outbox_events', undefined, params as Record<string, string>);
  }

  async retryOutboxEvent(eventId: string, data?: { reason?: string }): Promise<ApiResponse<CommandResponse>> {
    return this.request('POST', `/outbox_events/${eventId}:retry`, data);
  }
}

export const cmsApi = new CmsApiClient();
export default cmsApi;
