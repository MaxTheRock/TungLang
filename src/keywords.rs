use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::sync::RwLock;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeywordDefinition {
    pub original_name: String,
    pub keyword_type: String,
    pub description: String,
    pub aliases: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeywordConfig {
    pub keywords: Vec<KeywordDefinition>,
}

lazy_static! {
    // Use RwLock for thread-safe access to the keywords map
    static ref KEYWORDS_MAP: RwLock<HashMap<String, String>> = RwLock::new(HashMap::new());
}

/// Loads keywords from a JSON file
pub fn load_keywords_from_file(file_path: &str) -> Result<(), String> {
    let content = fs::read_to_string(file_path)
        .map_err(|err| format!("Failed to read keywords file: {}", err))?;

    load_keywords_from_json(&content)
}

/// Loads default keywords from the embedded JSON
pub fn load_default_keywords() -> Result<(), String> {
    let content = include_str!("keywords.json");
    load_keywords_from_json(content)
}

/// Parse JSON and build the keywords map
fn load_keywords_from_json(json_content: &str) -> Result<(), String> {
    let config: KeywordConfig = serde_json::from_str(json_content)
        .map_err(|err| format!("Failed to parse keywords JSON: {}", err))?;

    let mut keywords_map = KEYWORDS_MAP.write().unwrap();
    keywords_map.clear();

    for keyword in config.keywords {
        for alias in &keyword.aliases {
            if alias != &keyword.original_name {
                keywords_map.insert(alias.clone(), keyword.original_name.clone());
            }
        }
    }

    Ok(())
}

/// Resolve control keywords - maps aliases to original names
pub fn resolve_control_keyword(keyword: &str) -> String {
    let keywords_map = KEYWORDS_MAP.read().unwrap();
    match keywords_map.get(keyword) {
        Some(original) => original.clone(),
        None => keyword.to_string(), // If it's not an alias, return as is
    }
}

/// Resolve function name - maps aliases to original names
pub fn resolve_function_name(name: &str) -> String {
    let keywords_map = KEYWORDS_MAP.read().unwrap();
    match keywords_map.get(name) {
        Some(original) => original.clone(),
        None => name.to_string(), // If it's not an alias, return as is
    }
}

/// Get all aliases for a given original keyword
pub fn get_aliases_for(original: &str) -> Vec<String> {
    let mut result = Vec::new();
    let keywords_map = KEYWORDS_MAP.read().unwrap();
    for (alias, orig) in keywords_map.iter() {
        if orig == original {
            result.push(alias.clone());
        }
    }
    result
}

/// Preprocess source code to replace aliases with original keywords
pub fn preprocess_source(source: &str) -> String {
    // Get a read lock on the keywords map
    let mut result = String::from(source);

    // Check if map is empty and load defaults if needed
    {
        let keywords_map = KEYWORDS_MAP.read().unwrap();
        if keywords_map.is_empty() {
            // Drop the lock before calling load_default_keywords
            drop(keywords_map);
            let _ = load_default_keywords();
        }
    }

    // Get a new read lock for processing
    let keywords_map = KEYWORDS_MAP.read().unwrap();

    // Replace all aliases with their original keywords
    for (alias, original) in keywords_map.iter() {
        // Add word boundary checks to avoid replacing substrings
        let pattern = format!(r"\b{}\b", regex::escape(alias));
        let re = regex::Regex::new(&pattern).unwrap();
        result = re.replace_all(&result, original).to_string();
    }

    result
}

/// Check if a word is a recognized keyword or alias
pub fn is_keyword_or_alias(word: &str) -> bool {
    let keywords_map = KEYWORDS_MAP.read().unwrap();
    keywords_map.contains_key(word) || keywords_map.values().any(|orig| orig == word)
}
