import { useQuery } from '@tanstack/react-query';
import { cmsApi } from '@/sdk/client';
import { Globe, FileText, Layers, Rss } from 'lucide-react';

export default function Dashboard() {
  const { data: sites } = useQuery({
    queryKey: ['sites'],
    queryFn: () => cmsApi.listSites({ limit: 100 }),
  });

  const { data: entries } = useQuery({
    queryKey: ['entries'],
    queryFn: () => cmsApi.listEntries({ limit: 100 }),
  });

  const { data: pages } = useQuery({
    queryKey: ['pages'],
    queryFn: () => cmsApi.listPages({ limit: 100 }),
  });

  const { data: feeds } = useQuery({
    queryKey: ['feeds'],
    queryFn: () => cmsApi.listFeeds({ limit: 100 }),
  });

  const stats = [
    { name: '站点', value: sites?.data?.items.length ?? 0, icon: Globe, color: 'bg-blue-500' },
    { name: '内容', value: entries?.data?.items.length ?? 0, icon: FileText, color: 'bg-green-500' },
    { name: '页面', value: pages?.data?.items.length ?? 0, icon: Layers, color: 'bg-purple-500' },
    { name: '订阅源', value: feeds?.data?.items.length ?? 0, icon: Rss, color: 'bg-orange-500' },
  ];

  return (
    <div className="space-y-6">
      <div className="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-4">
        {stats.map((stat) => (
          <div
            key={stat.name}
            className="bg-white rounded-xl shadow-sm border border-gray-200 p-6"
          >
            <div className="flex items-center gap-4">
              <div className={`${stat.color} p-3 rounded-lg`}>
                <stat.icon className="w-6 h-6 text-white" />
              </div>
              <div>
                <p className="text-sm font-medium text-gray-500">{stat.name}</p>
                <p className="text-2xl font-bold text-gray-900">{stat.value}</p>
              </div>
            </div>
          </div>
        ))}
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">最近内容</h3>
          <div className="space-y-3">
            {entries?.data?.items.slice(0, 5).map((entry) => (
              <div key={entry.id} className="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
                <div>
                  <p className="font-medium text-gray-900">{entry.title}</p>
                  <p className="text-sm text-gray-500">{entry.slug}</p>
                </div>
                <span className={`px-2 py-1 text-xs font-medium rounded-full ${
                  entry.publicationStatus === 'published' ? 'bg-green-100 text-green-800' :
                  entry.publicationStatus === 'unpublished' ? 'bg-gray-100 text-gray-800' :
                  'bg-yellow-100 text-yellow-800'
                }`}>
                  {entry.publicationStatus}
                </span>
              </div>
            ))}
            {(!entries?.data?.items || entries.data.items.length === 0) && (
              <p className="text-gray-500 text-center py-4">暂无内容</p>
            )}
          </div>
        </div>

        <div className="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">站点列表</h3>
          <div className="space-y-3">
            {sites?.data?.items.slice(0, 5).map((site) => (
              <div key={site.id} className="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
                <div>
                  <p className="font-medium text-gray-900">{site.name}</p>
                  <p className="text-sm text-gray-500">{site.code}</p>
                </div>
                <span className="text-sm text-gray-500">{site.defaultLocale}</span>
              </div>
            ))}
            {(!sites?.data?.items || sites.data.items.length === 0) && (
              <p className="text-gray-500 text-center py-4">暂无站点</p>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
