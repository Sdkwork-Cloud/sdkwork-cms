import { Routes, Route } from 'react-router-dom';
import Layout from './components/Layout';
import HomePage from './pages/Home';
import ArticleListPage from './pages/ArticleList';
import ArticleDetailPage from './pages/ArticleDetail';
import PageViewerPage from './pages/PageViewer';

export default function App() {
  return (
    <Routes>
      <Route path="/" element={<Layout />}>
        <Route index element={<HomePage />} />
        <Route path="articles" element={<ArticleListPage />} />
        <Route path="articles/:slug" element={<ArticleDetailPage />} />
        <Route path="pages/*" element={<PageViewerPage />} />
      </Route>
    </Routes>
  );
}
