import { Outlet, NavLink } from 'react-router-dom';
import { Home, FileText } from 'lucide-react';

const tabs = [
  { name: '首页', href: '/', icon: Home },
  { name: '文章', href: '/articles', icon: FileText },
];

export default function Layout() {
  return (
    <div className="flex flex-col min-h-screen bg-gray-50">
      {/* Header */}
      <header className="sticky top-0 z-50 bg-white border-b border-gray-200 px-4 py-3" style={{ paddingTop: 'var(--safe-area-top, 0px)' }}>
        <h1 className="text-lg font-bold text-gray-900 text-center">CMS 内容平台</h1>
      </header>

      {/* Content */}
      <main className="flex-1 overflow-auto">
        <Outlet />
      </main>

      {/* Bottom Tab Bar */}
      <nav className="sticky bottom-0 bg-white border-t border-gray-200 flex" style={{ paddingBottom: 'var(--safe-area-bottom, 0px)' }}>
        {tabs.map((tab) => (
          <NavLink
            key={tab.name}
            to={tab.href}
            end={tab.href === '/'}
            className={({ isActive }) =>
              `flex-1 flex flex-col items-center py-2 text-xs font-medium transition-colors ${
                isActive ? 'text-primary-600' : 'text-gray-500'
              }`
            }
          >
            <tab.icon className="w-5 h-5 mb-1" />
            {tab.name}
          </NavLink>
        ))}
      </nav>
    </div>
  );
}
