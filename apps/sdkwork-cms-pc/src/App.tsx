import { Routes, Route, Navigate } from 'react-router-dom';
import Layout from './components/Layout';
import Dashboard from './pages/Dashboard';
import SitesPage from './pages/Sites';
import EntriesPage from './pages/Entries';
import PagesPage from './pages/Pages';
import FeedsPage from './pages/Feeds';
import ContentTypesPage from './pages/ContentTypes';
import TaxonomiesPage from './pages/Taxonomies';
import AuditLogsPage from './pages/AuditLogs';

export default function App() {
  return (
    <Routes>
      <Route path="/" element={<Layout />}>
        <Route index element={<Navigate to="/dashboard" replace />} />
        <Route path="dashboard" element={<Dashboard />} />
        <Route path="sites" element={<SitesPage />} />
        <Route path="entries" element={<EntriesPage />} />
        <Route path="pages" element={<PagesPage />} />
        <Route path="feeds" element={<FeedsPage />} />
        <Route path="content-types" element={<ContentTypesPage />} />
        <Route path="taxonomies" element={<TaxonomiesPage />} />
        <Route path="audit-logs" element={<AuditLogsPage />} />
      </Route>
    </Routes>
  );
}
