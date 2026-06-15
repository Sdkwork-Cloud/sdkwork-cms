import { useParams } from 'react-router-dom';
import { useQuery } from '@tanstack/react-query';
import { cmsDelivery } from '@/sdk/client';
import { ArrowLeft, Calendar, User } from 'lucide-react';
import { useNavigate } from 'react-router-dom';

export default function ArticleDetailPage() {
  const { slug } = useParams<{ slug: string }>();
  const navigate = useNavigate();

  const { data: entry, isLoading } = useQuery({
    queryKey: ['entry', slug],
    queryFn: () => cmsDelivery.resolveEntry('default', { slug: slug || '' }),
    enabled: !!slug,
  });

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-gray-500">加载中...</div>
      </div>
    );
  }

  if (!entry?.data) {
    return (
      <div className="flex flex-col items-center justify-center h-64 gap-4">
        <div className="text-gray-500">文章不存在</div>
        <button
          onClick={() => navigate('/articles')}
          className="px-4 py-2 bg-primary-600 text-white rounded-lg text-sm"
        >
          返回列表
        </button>
      </div>
    );
  }

  const article = entry.data;

  return (
    <div className="pb-8">
      {/* Header */}
      <div className="sticky top-0 z-10 bg-white border-b border-gray-200 px-4 py-3 flex items-center gap-3">
        <button onClick={() => navigate(-1)} className="p-1">
          <ArrowLeft className="w-5 h-5 text-gray-600" />
        </button>
        <h1 className="text-base font-medium text-gray-900 truncate flex-1">文章详情</h1>
      </div>

      {/* Article Content */}
      <article className="p-4">
        <h1 className="text-2xl font-bold text-gray-900 mb-4">{article.title}</h1>

        <div className="flex items-center gap-4 text-sm text-gray-500 mb-6">
          <div className="flex items-center gap-1">
            <Calendar className="w-4 h-4" />
            <span>{article.publishedAt ? new Date(article.publishedAt).toLocaleDateString() : '未发布'}</span>
          </div>
          <div className="flex items-center gap-1">
            <User className="w-4 h-4" />
            <span>{article.locale}</span>
          </div>
        </div>

        {article.summary && (
          <div className="bg-gray-50 rounded-lg p-4 mb-6">
            <p className="text-gray-700 text-sm leading-relaxed">{article.summary}</p>
          </div>
        )}

        <div className="prose prose-sm max-w-none">
          <p className="text-gray-700 leading-relaxed">
            这里是文章的正文内容。在实际应用中，这里会显示从后端获取的富文本内容。
          </p>
        </div>
      </article>
    </div>
  );
}
