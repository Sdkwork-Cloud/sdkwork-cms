import { useQuery } from '@tanstack/react-query';
import { Link } from 'react-router-dom';
import { cmsDelivery } from '@/sdk/client';
import { FileText, Calendar } from 'lucide-react';

const SITE_CODE = 'default';

export default function ArticleListPage() {
  const { data: entries, isLoading } = useQuery({
    queryKey: ['entries', SITE_CODE],
    queryFn: () => cmsDelivery.listFeedItems(SITE_CODE, 'latest', { limit: 50 }),
  });

  return (
    <div className="p-4">
      <h2 className="text-lg font-bold text-gray-900 mb-4">文章列表</h2>

      {isLoading ? (
        <div className="text-center py-12 text-gray-500">加载中...</div>
      ) : entries?.data?.items.length === 0 ? (
        <div className="text-center py-12 text-gray-500">暂无文章</div>
      ) : (
        <div className="space-y-3">
          {entries?.data?.items.map((item) => (
            <Link
              key={item.id}
              to={`/articles/${item.entryId || item.id}`}
              className="block bg-white rounded-lg p-4 border border-gray-200 hover:border-primary-300 transition-colors"
            >
              <div className="flex items-start gap-3">
                <div className="w-12 h-12 bg-primary-50 rounded-lg flex items-center justify-center flex-shrink-0">
                  <FileText className="w-6 h-6 text-primary-600" />
                </div>
                <div className="flex-1 min-w-0">
                  <h3 className="font-medium text-gray-900">{item.itemKind}</h3>
                  <div className="flex items-center gap-2 mt-2 text-xs text-gray-500">
                    <Calendar className="w-3 h-3" />
                    <span>最近更新</span>
                  </div>
                </div>
              </div>
            </Link>
          ))}
        </div>
      )}
    </div>
  );
}
