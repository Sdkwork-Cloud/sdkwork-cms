import { useQuery } from '@tanstack/react-query';
import { Link } from 'react-router-dom';
import { cmsDelivery } from '@/sdk/client';
import { FileText, ArrowRight } from 'lucide-react';

const SITE_CODE = 'default';

export default function HomePage() {
  const { data: bootstrap } = useQuery({
    queryKey: ['bootstrap', SITE_CODE],
    queryFn: () => cmsDelivery.bootstrap(SITE_CODE),
  });

  const { data: entries } = useQuery({
    queryKey: ['entries', SITE_CODE],
    queryFn: () => cmsDelivery.listFeedItems(SITE_CODE, 'latest', { limit: 10 }),
  });

  return (
    <div className="p-4 space-y-6">
      {/* Welcome Section */}
      <div className="bg-gradient-to-r from-primary-600 to-primary-700 rounded-xl p-6 text-white">
        <h2 className="text-xl font-bold mb-2">
          {bootstrap?.data?.site?.name || '欢迎访问'}
        </h2>
        <p className="text-primary-100 text-sm">
          {bootstrap?.data?.site?.defaultLocale === 'zh-CN' ? '发现精彩内容' : 'Discover amazing content'}
        </p>
      </div>

      {/* Channels */}
      {bootstrap?.data?.channels && bootstrap.data.channels.length > 0 && (
        <div>
          <h3 className="text-sm font-medium text-gray-500 mb-3">频道</h3>
          <div className="flex gap-2 overflow-x-auto pb-2">
            {bootstrap.data.channels.map((channel) => (
              <div
                key={channel.id}
                className="flex-shrink-0 px-4 py-2 bg-white rounded-full border border-gray-200 text-sm font-medium text-gray-700"
              >
                {channel.name}
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Latest Articles */}
      <div>
        <div className="flex items-center justify-between mb-3">
          <h3 className="text-sm font-medium text-gray-500">最新文章</h3>
          <Link to="/articles" className="text-sm text-primary-600 flex items-center gap-1">
            查看全部 <ArrowRight className="w-4 h-4" />
          </Link>
        </div>
        <div className="space-y-3">
          {entries?.data?.items.map((item) => (
            <Link
              key={item.id}
              to={`/articles/${item.entryId || item.id}`}
              className="block bg-white rounded-lg p-4 border border-gray-200 hover:border-primary-300 transition-colors"
            >
              <div className="flex items-start gap-3">
                <div className="w-10 h-10 bg-primary-50 rounded-lg flex items-center justify-center flex-shrink-0">
                  <FileText className="w-5 h-5 text-primary-600" />
                </div>
                <div className="flex-1 min-w-0">
                  <h4 className="font-medium text-gray-900 truncate">{item.itemKind}</h4>
                  <p className="text-sm text-gray-500 mt-1 line-clamp-2">
                    {item.externalUrl || '查看详情'}
                  </p>
                </div>
              </div>
            </Link>
          ))}
          {(!entries?.data?.items || entries.data.items.length === 0) && (
            <div className="text-center py-8 text-gray-500">暂无内容</div>
          )}
        </div>
      </div>
    </div>
  );
}
