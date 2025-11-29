use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub snippet: String,
}

#[tauri::command]
pub async fn search_duckduckgo(query: String) -> Result<Vec<SearchResult>, String> {
    let client = reqwest::Client::new();
    
    // DuckDuckGo Instant Answer API
    let url = format!(
        "https://api.duckduckgo.com/?q={}&format=json&no_html=1",
        urlencoding::encode(&query)
    );
    
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("搜索请求失败: {}", e))?;
    
    let data: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("解析响应失败: {}", e))?;
    
    let mut results = Vec::new();
    
    // 解析 RelatedTopics
    if let Some(related) = data["RelatedTopics"].as_array() {
        for topic in related.iter().take(10) {
            if let Some(text) = topic["Text"].as_str() {
                if let Some(url) = topic["FirstURL"].as_str() {
                    let title = text
                        .split(" - ")
                        .next()
                        .unwrap_or(text)
                        .to_string();
                    
                    results.push(SearchResult {
                        title,
                        url: url.to_string(),
                        snippet: text.to_string(),
                    });
                }
            }
        }
    }
    
    // 如果没有 RelatedTopics，尝试获取 Abstract
    if results.is_empty() {
        if let Some(abstract_text) = data["Abstract"].as_str() {
            if !abstract_text.is_empty() {
                if let Some(abstract_url) = data["AbstractURL"].as_str() {
                    results.push(SearchResult {
                        title: data["Heading"]
                            .as_str()
                            .unwrap_or("搜索结果")
                            .to_string(),
                        url: abstract_url.to_string(),
                        snippet: abstract_text.to_string(),
                    });
                }
            }
        }
    }
    
    Ok(results)
}
