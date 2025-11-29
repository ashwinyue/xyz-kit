import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

function SearchDialog({ isOpen, onClose }) {
  const [query, setQuery] = useState('');
  const [results, setResults] = useState([]);
  const [loading, setLoading] = useState(false);

  const handleSearch = async () => {
    if (!query.trim()) return;
    
    setLoading(true);
    try {
      const searchResults = await invoke('search_duckduckgo', { query });
      setResults(searchResults);
    } catch (error) {
      console.error('搜索失败:', error);
      alert('搜索失败: ' + error);
    } finally {
      setLoading(false);
    }
  };

  const handleKeyDown = (e) => {
    if (e.key === 'Enter') {
      handleSearch();
    } else if (e.key === 'Escape') {
      onClose();
    }
  };

  const openUrl = async (url) => {
    try {
      await invoke('tauri_plugin_opener::open', { url });
    } catch (error) {
      console.error('打开链接失败:', error);
    }
  };

  if (!isOpen) return null;

  return (
    <div 
      className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
      onClick={onClose}
    >
      <div 
        className="bg-white rounded-lg shadow-xl w-[600px] max-h-[80vh] flex flex-col"
        onClick={(e) => e.stopPropagation()}
      >
        {/* 搜索框 */}
        <div className="p-4 border-b">
          <div className="flex gap-2">
            <input
              type="text"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              onKeyDown={handleKeyDown}
              placeholder="搜索 DuckDuckGo..."
              className="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:border-[#8070d4]"
              autoFocus
            />
            <button
              onClick={handleSearch}
              disabled={loading}
              className="px-6 py-2 bg-[#8070d4] text-white rounded-lg hover:bg-[#6d5fc7] disabled:opacity-50"
            >
              {loading ? '搜索中...' : '搜索'}
            </button>
          </div>
        </div>

        {/* 搜索结果 */}
        <div className="flex-1 overflow-y-auto p-4">
          {results.length === 0 && !loading && (
            <div className="text-center text-gray-500 py-8">
              输入关键词开始搜索
            </div>
          )}
          
          {loading && (
            <div className="text-center text-gray-500 py-8">
              搜索中...
            </div>
          )}

          {results.map((result, index) => (
            <div
              key={index}
              className="mb-4 p-3 border border-gray-200 rounded-lg hover:bg-gray-50 cursor-pointer"
              onClick={() => openUrl(result.url)}
            >
              <h3 className="text-lg font-semibold text-[#8070d4] mb-1">
                {result.title}
              </h3>
              <p className="text-sm text-gray-600 mb-2">
                {result.snippet}
              </p>
              <a 
                href={result.url}
                className="text-xs text-gray-400 hover:underline"
                onClick={(e) => e.stopPropagation()}
              >
                {result.url}
              </a>
            </div>
          ))}
        </div>

        {/* 提示 */}
        <div className="p-3 border-t bg-gray-50 text-xs text-gray-500 text-center">
          按 Enter 搜索 · 按 Esc 关闭
        </div>
      </div>
    </div>
  );
}

export default SearchDialog;
