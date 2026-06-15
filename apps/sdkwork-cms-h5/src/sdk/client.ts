import type {
  ApiResponse,
  PaginatedResponse,
  DeliveryBootstrap,
  DeliveryEntry,
  DeliveryPage,
  DeliveryFeedItem,
} from '@/types';

const BASE_URL = '/app/v3/api/cms';

class CmsDeliveryClient {
  private async request<T>(
    method: string,
    path: string,
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

    const response = await fetch(url.toString(), {
      method,
      headers: {
        'Content-Type': 'application/json',
      },
    });

    return response.json();
  }

  async bootstrap(siteCode: string, params?: { channelCode?: string; locale?: string }): Promise<ApiResponse<DeliveryBootstrap>> {
    return this.request('GET', `/sites/${siteCode}/bootstrap`, params as Record<string, string>);
  }

  async resolveEntry(siteCode: string, params: { slug: string; channelCode?: string; locale?: string; previewToken?: string }): Promise<ApiResponse<DeliveryEntry>> {
    return this.request('GET', `/sites/${siteCode}/entries:resolve`, params as Record<string, string>);
  }

  async retrieveEntry(entryId: string, params?: { previewToken?: string }): Promise<ApiResponse<DeliveryEntry>> {
    return this.request('GET', `/entries/${entryId}`, params as Record<string, string>);
  }

  async resolvePage(siteCode: string, params: { path: string; channelCode?: string; locale?: string; previewToken?: string }): Promise<ApiResponse<DeliveryPage>> {
    return this.request('GET', `/sites/${siteCode}/pages:resolve`, params as Record<string, string>);
  }

  async listFeedItems(siteCode: string, feedCode: string, params?: { channelCode?: string; locale?: string; cursor?: string; limit?: number }): Promise<ApiResponse<PaginatedResponse<DeliveryFeedItem>>> {
    return this.request('GET', `/sites/${siteCode}/feeds/${feedCode}/items`, params as Record<string, string>);
  }
}

export const cmsDelivery = new CmsDeliveryClient();
export default cmsDelivery;
