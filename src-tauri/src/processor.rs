 use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessFunc {
    pub name: String,
    pub describe: String,
    #[serde(rename = "nextStep")]
    pub next_step: String,
}

pub struct TextProcessor;

impl TextProcessor {
    pub fn get_functions() -> Vec<ProcessFunc> {
        vec![
            ProcessFunc {
                name: "id-join".to_string(),
                describe: "ID拼接".to_string(),
                next_step: "id-join".to_string(),
            },
            ProcessFunc {
                name: "search".to_string(),
                describe: "快速搜索".to_string(),
                next_step: "search".to_string(),
            },
        ]
    }

    /// ID Join: transforms ID lists between formats
    /// - Comma: 1,2,3
    /// - Quoted: "1","2","3"
    /// - Regex: ^1$|^2$|^3$
    /// - Multi-line: joins with quoted format
    pub fn id_join(text: &str, lines: &[&str]) -> String {
        // Multi-line: join with quoted format
        if lines.len() > 1 {
            return format!(r#""{}""#, lines.join(r#"",""#));
        }

        // Single line: detect and transform
        if lines.len() == 1 {
            return Self::transform_single_line(text);
        }

        text.to_string()
    }

    fn transform_single_line(text: &str) -> String {
        if text.contains('"') {
            // Quoted -> Regex: "1","2","3" -> ^1$|^2$|^3$
            let result = text.replace(r#"",""#, "$|^");
            let result = result.replacen('"', "^", 1);
            if result.ends_with('"') {
                format!("{}$", &result[..result.len() - 1])
            } else {
                result
            }
        } else if text.contains('^') {
            // Regex -> Comma: ^1$|^2$|^3$ -> 1,2,3
            let result = text.replace("$|^", ",");
            let result = result.strip_prefix('^').unwrap_or(&result);
            result.strip_suffix('$').unwrap_or(result).to_string()
        } else {
            // Comma -> Quoted: 1,2,3 -> "1","2","3"
            format!(r#""{}""#, text.replace(',', r#"",""#))
        }
    }

    #[allow(dead_code)]
    pub fn check(_text: &str, lines: &[&str], is_json: bool) -> HashMap<String, i32> {
        let mut scores = HashMap::new();
        scores.insert(
            "id-join".to_string(),
            if !lines.is_empty() && !is_json { 100 } else { 0 },
        );
        scores
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comma_to_quoted() {
        let result = TextProcessor::id_join("1,2,3", &["1,2,3"]);
        assert_eq!(result, r#""1","2","3""#);
    }

    #[test]
    fn test_quoted_to_regex() {
        let result = TextProcessor::id_join(r#""1","2","3""#, &[r#""1","2","3""#]);
        assert_eq!(result, "^1$|^2$|^3$");
    }

    #[test]
    fn test_regex_to_comma() {
        let result = TextProcessor::id_join("^1$|^2$|^3$", &["^1$|^2$|^3$"]);
        assert_eq!(result, "1,2,3");
    }

    #[test]
    fn test_multiline_join() {
        let result = TextProcessor::id_join("1\n2\n3", &["1", "2", "3"]);
        assert_eq!(result, r#""1","2","3""#);
    }
}
