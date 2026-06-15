import { useLocation, useNavigate } from 'react-router-dom';
import { useQuery } from '@tanstack/react-query';
import { cmsDelivery } from '@/sdk/client';
import { ArrowLeft } from 'lucide-react';

export default function PageViewerPage() {
  const location = useLocation();
  const navigate = useNavigate();
  const pagePath = location.pathname.replace('/pages', '') || '/';

  const { data: page, isLoading } = useQuery({
    queryKey: ['page', pagePath],
    queryFn: () => cmsDelivery.resolvePage('default', { path: pagePath }),
    enabled: pagePath !== '/',
  });

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-gray-500">加载中...</div>
      </div>
    );
  }

  if (!page?.data) {
    return (
      <div className="flex flex-col items-center justify-center h-64 gap-4">
        <div className="text-gray-500">页面不存在</div>
        <button
          onClick={() => navigate('/')}
          className="px-4 py-2 bg-primary-600 text-white rounded-lg text-sm"
        >
          返回首页
        </button>
      </div>
    );
  }

  return (
    <div className="pb-8">
      {/* Header */}
      <div className="sticky top-0 z-10 bg-white border-b border-gray-200 px-4 py-3 flex items-center gap-3">
        <button onClick={() => navigate(-1)} className="p-1">
          <ArrowLeft className="w-5 h-5 text-gray-600" />
        </button>
        <h1 className="text-base font-medium text-gray-900 truncate flex-1">{page.data.title}</h1>
      </div>

      {/* Page Content */}
      <div className="p-4">
        <h1 className="text-2xl font-bold text-gray-900 mb-4">{page.data.title}</h1>
        <div className="prose prose-sm max-w-none">
          <p className="text-gray-700 leading-relaxed">
            这里是页面的正文内容。在实际应用中，这里会显示从后端获取的页面区块内容。
          </p>
        </div>
      </div>
    </div>
  );
}
