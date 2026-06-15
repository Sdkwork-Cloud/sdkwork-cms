export interface DeliverySite {
  id: string;
  uuid: string;
  code: string;
  name: string;
  defaultLocale: string;
}

export interface DeliveryChannel {
  id: string;
  code: string;
  name: string;
  channelKind: string;
}

export interface DeliveryEntry {
  id: string;
  uuid: string;
  siteId: string;
  contentTypeId: string;
  locale: string;
  title: string;
  slug: string;
  summary?: string;
  publishedAt?: string;
}

export interface DeliveryPage {
  id: string;
  siteId: string;
  locale: string;
  path: string;
  title: string;
}

export interface DeliveryFeedItem {
  id: string;
  feedId: string;
  entryId?: string;
  pageId?: string;
  externalUrl?: string;
  itemKind: string;
  pinned: boolean;
  sortOrder: number;
}

export interface DeliveryBootstrap {
  site: DeliverySite;
  channels: DeliveryChannel[];
}

export interface PaginatedResponse<T> {
  items: T[];
  nextCursor?: string;
}

export interface ApiResponse<T> {
  ok: boolean;
  data?: T;
  error?: {
    detail: string;
  };
}
