import { useQuery } from '@tanstack/react-query';
import { cmsApi } from '@/sdk/client';
import { Plus } from 'lucide-react';

export default function FeedsPage() {
  const { data: feeds, isLoading } = useQuery({
    queryKey: ['feeds'],
    queryFn: () => cmsApi.listFeeds({ limit: 100 }),
  });

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h2 className="text-2xl font-bold text-gray-900">订阅源管理</h2>
        <button className="flex items-center gap-2 px-4 py-2 bg-primary-600 text-white rounded-lg hover:bg-primary-700 transition-colors">
          <Plus className="w-5 h-5" />
          创建订阅源
        </button>
      </div>

      <div className="bg-white rounded-xl shadow-sm border border-gray-200 overflow-hidden">
        <table className="w-full">
          <thead className="bg-gray-50 border-b border-gray-200">
            <tr>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">名称</th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">编码</th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">类型</th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">语言</th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">状态</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-200">
            {isLoading ? (
              <tr>
                <td colSpan={5} className="px-6 py-12 text-center text-gray-500">加载中...</td>
              </tr>
            ) : feeds?.data?.items.length === 0 ? (
              <tr>
                <td colSpan={5} className="px-6 py-12 text-center text-gray-500">暂无订阅源</td>
              </tr>
            ) : (
              feeds?.data?.items.map((feed) => (
                <tr key={feed.id} className="hover:bg-gray-50">
                  <td className="px-6 py-4 font-medium text-gray-900">{feed.name}</td>
                  <td className="px-6 py-4 text-sm text-gray-500 font-mono">{feed.code}</td>
                  <td className="px-6 py-4 text-sm text-gray-500">{feed.feedKind}</td>
                  <td className="px-6 py-4 text-sm text-gray-500">{feed.locale}</td>
                  <td className="px-6 py-4">
                    <span className={`px-2 py-1 text-xs font-medium rounded-full ${
                      feed.status === 1 ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-800'
                    }`}>
                      {feed.status === 1 ? '启用' : '禁用'}
                    </span>
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
}
