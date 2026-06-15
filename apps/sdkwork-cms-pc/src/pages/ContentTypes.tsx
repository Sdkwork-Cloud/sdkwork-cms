import { useQuery } from '@tanstack/react-query';
import { cmsApi } from '@/sdk/client';
import { Plus } from 'lucide-react';

export default function ContentTypesPage() {
  const { data: contentTypes, isLoading } = useQuery({
    queryKey: ['contentTypes'],
    queryFn: () => cmsApi.listContentTypes('', { limit: 100 }),
  });

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h2 className="text-2xl font-bold text-gray-900">内容类型</h2>
        <button className="flex items-center gap-2 px-4 py-2 bg-primary-600 text-white rounded-lg hover:bg-primary-700 transition-colors">
          <Plus className="w-5 h-5" />
          创建内容类型
        </button>
      </div>

      <div className="bg-white rounded-xl shadow-sm border border-gray-200 overflow-hidden">
        <table className="w-full">
          <thead className="bg-gray-50 border-b border-gray-200">
            <tr>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">名称</th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">编码</th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">类型</th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">Schema版本</th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">状态</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-200">
            {isLoading ? (
              <tr>
                <td colSpan={5} className="px-6 py-12 text-center text-gray-500">加载中...</td>
              </tr>
            ) : contentTypes?.data?.items.length === 0 ? (
              <tr>
                <td colSpan={5} className="px-6 py-12 text-center text-gray-500">暂无内容类型</td>
              </tr>
            ) : (
              contentTypes?.data?.items.map((ct) => (
                <tr key={ct.id} className="hover:bg-gray-50">
                  <td className="px-6 py-4 font-medium text-gray-900">{ct.name}</td>
                  <td className="px-6 py-4 text-sm text-gray-500 font-mono">{ct.code}</td>
                  <td className="px-6 py-4 text-sm text-gray-500">{ct.contentKind}</td>
                  <td className="px-6 py-4 text-sm text-gray-500">v{ct.schemaVersion}</td>
                  <td className="px-6 py-4">
                    <span className={`px-2 py-1 text-xs font-medium rounded-full ${
                      ct.status === 1 ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-800'
                    }`}>
                      {ct.status === 1 ? '启用' : '禁用'}
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
