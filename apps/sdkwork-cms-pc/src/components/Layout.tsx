import { Outlet, NavLink, useLocation } from 'react-router-dom';
import {
  LayoutDashboard,
  Globe,
  FileText,
  Layers,
  Rss,
  Database,
  Tags,
  ScrollText,
  Settings,
} from 'lucide-react';

const navigation = [
  { name: '仪表盘', href: '/dashboard', icon: LayoutDashboard },
  { name: '站点管理', href: '/sites', icon: Globe },
  { name: '内容管理', href: '/entries', icon: FileText },
  { name: '页面管理', href: '/pages', icon: Layers },
  { name: '订阅源', href: '/feeds', icon: Rss },
  { name: '内容类型', href: '/content-types', icon: Database },
  { name: '分类管理', href: '/taxonomies', icon: Tags },
  { name: '审计日志', href: '/audit-logs', icon: ScrollText },
];

export default function Layout() {
  const location = useLocation();

  return (
    <div className="flex h-screen bg-gray-100">
      {/* Sidebar */}
      <aside className="w-64 bg-white border-r border-gray-200 flex flex-col">
        <div className="h-16 flex items-center px-6 border-b border-gray-200">
          <h1 className="text-xl font-bold text-gray-900">CMS 管理后台</h1>
        </div>
        <nav className="flex-1 px-3 py-4 space-y-1 overflow-y-auto">
          {navigation.map((item) => {
            const isActive = location.pathname.startsWith(item.href);
            return (
              <NavLink
                key={item.name}
                to={item.href}
                className={`flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium transition-colors ${
                  isActive
                    ? 'bg-primary-50 text-primary-700'
                    : 'text-gray-700 hover:bg-gray-50 hover:text-gray-900'
                }`}
              >
                <item.icon className="w-5 h-5" />
                {item.name}
              </NavLink>
            );
          })}
        </nav>
        <div className="p-4 border-t border-gray-200">
          <button className="flex items-center gap-3 w-full px-3 py-2 rounded-lg text-sm font-medium text-gray-700 hover:bg-gray-50">
            <Settings className="w-5 h-5" />
            设置
          </button>
        </div>
      </aside>

      {/* Main content */}
      <main className="flex-1 flex flex-col overflow-hidden">
        <header className="h-16 bg-white border-b border-gray-200 flex items-center px-6">
          <h2 className="text-lg font-semibold text-gray-900">
            {navigation.find((n) => location.pathname.startsWith(n.href))?.name || 'CMS'}
          </h2>
        </header>
        <div className="flex-1 overflow-auto p-6">
          <Outlet />
        </div>
      </main>
    </div>
  );
}
