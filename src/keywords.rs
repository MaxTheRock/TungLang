use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::sync::RwLock;

// Define keyword types for categorization
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeywordType {
    Function,
    Control,
    Statement,
    Operator,
}

// Structure to represent a keyword
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyword {
    pub original_name: String,     // Canonical name (like "print")
    pub keyword_type: KeywordType, // Type of keyword
    pub description: String,       // Optional description
}

// Main keyword registry - a single source of truth
lazy_static! {
    // Maps all aliases to their canonical keywords
    pub static ref KEYWORD_REGISTRY: RwLock<HashMap<String, Keyword>> = {
        let mut registry = HashMap::new();

        // === FUNCTION KEYWORDS ===
        // Print function
        register_keyword_internal(
            &mut registry,
            "print",
            KeywordType::Function,
            "Print values to the console",
            &["print", "tung"]
        );

        // Input function
        register_keyword_internal(
            &mut registry,
            "input",
            KeywordType::Function,
            "Read user input from the console",
            &["input", "sahur"]
        );

        // Integer conversion
        register_keyword_internal(
            &mut registry,
            "int",
            KeywordType::Function,
            "Convert value to integer",
            &["int", "tripi"]
        );

        // Quit function
        register_keyword_internal(
            &mut registry,
            "quit",
            KeywordType::Function,
            "Exit the program",
            &["quit", "tralalelo"]
        );

        // === CONTROL FLOW KEYWORDS ===
        // If statement
        register_keyword_internal(
            &mut registry,
            "if",
            KeywordType::Control,
            "Conditional if statement",
            &["if", "la_vaca"]
        );

        // End if statement
        register_keyword_internal(
            &mut registry,
            "endif",
            KeywordType::Control,
            "End of if block",
            &["endif", "fine_vaca"]
        );

        RwLock::new(registry)
    };
}

// Helper function to register keywords internally during initialization
fn register_keyword_internal(
    registry: &mut HashMap<String, Keyword>,
    original_name: &str,
    keyword_type: KeywordType,
    description: &str,
    aliases: &[&str],
) {
    let keyword = Keyword {
        original_name: original_name.to_string(),
        keyword_type,
        description: description.to_string(),
    };

    // Add the canonical name
    registry.insert(original_name.to_string(), keyword.clone());

    // Add all aliases
    for alias in aliases {
        if *alias != original_name {
            registry.insert(alias.to_string(), keyword.clone());
        }
    }
}

// Public API

/// Resolve an alias to its canonical keyword
pub fn resolve_keyword(alias: &str) -> Option<Keyword> {
    KEYWORD_REGISTRY.read().unwrap().get(alias).cloned()
}

/// Get the canonical name of a function alias
pub fn resolve_function_name(alias: &str) -> String {
    match resolve_keyword(alias) {
        Some(keyword) if keyword.keyword_type == KeywordType::Function => keyword.original_name,
        _ => alias.to_string(),
    }
}

/// Get the canonical name of a control keyword
pub fn resolve_control_keyword(alias: &str) -> String {
    match resolve_keyword(alias) {
        Some(keyword) if keyword.keyword_type == KeywordType::Control => keyword.original_name,
        _ => alias.to_string(),
    }
}

/// Register a new keyword alias
#[allow(dead_code)]
pub fn register_keyword(
    original_name: &str,
    keyword_type: KeywordType,
    description: &str,
    aliases: &[&str],
) -> Result<(), String> {
    let mut registry = KEYWORD_REGISTRY.write().unwrap();

    let keyword = Keyword {
        original_name: original_name.to_string(),
        keyword_type,
        description: description.to_string(),
    };

    // Add all aliases
    for alias in aliases {
        registry.insert(alias.to_string(), keyword.clone());
    }

    Ok(())
}

// Configuration file structures for loading/saving keywords
#[derive(Serialize, Deserialize, Debug)]
pub struct KeywordDefinition {
    pub original_name: String,
    pub keyword_type: KeywordType,
    pub description: String,
    pub aliases: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeywordConfig {
    pub keywords: Vec<KeywordDefinition>,
}

/// Load keywords from a JSON configuration file
pub fn load_keywords_from_file(config_path: &str) -> Result<(), String> {
    let config_content = fs::read_to_string(config_path)
        .map_err(|e| format!("Failed to read config file: {}", e))?;

    let config: KeywordConfig = serde_json::from_str(&config_content)
        .map_err(|e| format!("Failed to parse config: {}", e))?;

    let mut registry = KEYWORD_REGISTRY.write().unwrap();
    registry.clear(); // Remove existing keywords

    // Add all keywords from config
    for kw_def in config.keywords {
        let keyword = Keyword {
            original_name: kw_def.original_name.clone(),
            keyword_type: kw_def.keyword_type,
            description: kw_def.description,
        };

        // Add canonical name
        registry.insert(kw_def.original_name.clone(), keyword.clone());

        // Add all aliases
        for alias in kw_def.aliases {
            if alias != kw_def.original_name {
                registry.insert(alias, keyword.clone());
            }
        }
    }

    Ok(())
}

/// Save current keywords to a JSON configuration file
#[allow(dead_code)]
pub fn save_keywords_to_file(config_path: &str) -> Result<(), String> {
    // Build a unique list of keywords
    let registry = KEYWORD_REGISTRY.read().unwrap();

    // Group keywords by original_name
    let mut keyword_map: HashMap<String, Vec<String>> = HashMap::new();
    for (alias, keyword) in registry.iter() {
        keyword_map
            .entry(keyword.original_name.clone())
            .or_insert_with(Vec::new)
            .push(alias.clone());
    }

    // Create keyword definitions
    let mut keyword_defs = Vec::new();
    for (original_name, aliases) in keyword_map {
        // Find the original keyword entry
        if let Some(keyword) = registry.get(&original_name) {
            keyword_defs.push(KeywordDefinition {
                original_name: keyword.original_name.clone(),
                keyword_type: keyword.keyword_type.clone(),
                description: keyword.description.clone(),
                aliases,
            });
        }
    }

    let config = KeywordConfig {
        keywords: keyword_defs,
    };

    let json = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;

    fs::write(config_path, json).map_err(|e| format!("Failed to write config file: {}", e))?;

    Ok(())
}

// Get all aliases for a given canonical name
#[allow(dead_code)]
pub fn get_aliases_for(canonical_name: &str) -> Vec<String> {
    let registry = KEYWORD_REGISTRY.read().unwrap();
    registry
        .iter()
        .filter_map(|(alias, keyword)| {
            if keyword.original_name == canonical_name {
                Some(alias.clone())
            } else {
                None
            }
        })
        .collect()
}

// Utility to list all keywords by type
#[allow(dead_code)]
pub fn list_keywords_by_type(keyword_type: KeywordType) -> Vec<(String, Vec<String>)> {
    let registry = KEYWORD_REGISTRY.read().unwrap();

    // Find all canonical keywords of the requested type
    let mut result = Vec::new();

    // Track canonical names we've already processed
    let mut processed = std::collections::HashSet::new();

    // Process all entries
    for (_, keyword) in registry.iter() {
        if keyword.keyword_type == keyword_type && !processed.contains(&keyword.original_name) {
            processed.insert(keyword.original_name.clone());

            // Get all aliases for this canonical name
            let aliases = get_aliases_for(&keyword.original_name);

            result.push((keyword.original_name.clone(), aliases));
        }
    }

    result
}
