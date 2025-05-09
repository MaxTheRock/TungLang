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
        register_keyword_internal(&mut registry, "print", KeywordType::Function, "Print values to the console", &["print", "tung"]);
        register_keyword_internal(&mut registry, "input", KeywordType::Function, "Read user input from the console", &["input", "sahur"]);
        register_keyword_internal(&mut registry, "int", KeywordType::Function, "Convert value to integer", &["int", "tripi"]);
        register_keyword_internal(&mut registry, "quit", KeywordType::Function, "Exit the program", &["quit", "tralalelo"]);

        // === CONTROL FLOW KEYWORDS ===
        register_keyword_internal(&mut registry, "if", KeywordType::Control, "Conditional if statement", &["if", "la_vaca"]);
        register_keyword_internal(&mut registry, "endif", KeywordType::Control, "End of if block", &["endif", "fine_vaca"]);

        // === STATEMENTS ===
        register_keyword_internal(&mut registry, "var", KeywordType::Statement, "Variable declaration", &["var", "babbo"]);

        // === OPERATORS ===
        register_keyword_internal(&mut registry, "+", KeywordType::Operator, "Addition operator", &["+", "piu"]);
        register_keyword_internal(&mut registry, "-", KeywordType::Operator, "Subtraction operator", &["-", "meno"]);
        register_keyword_internal(&mut registry, "*", KeywordType::Operator, "Multiplication operator", &["*", "per"]);
        register_keyword_internal(&mut registry, "/", KeywordType::Operator, "Division operator", &["/", "diviso"]);

        register_keyword_internal(&mut registry, "==", KeywordType::Operator, "Equal to", &["==", "uguale"]);
        register_keyword_internal(&mut registry, "!=", KeywordType::Operator, "Not equal to", &["!=", "diverso"]);
        register_keyword_internal(&mut registry, ">", KeywordType::Operator, "Greater than", &[">", "maggiore"]);
        register_keyword_internal(&mut registry, "<", KeywordType::Operator, "Less than", &["<", "minore"]);
        register_keyword_internal(&mut registry, ">=", KeywordType::Operator, "Greater than or equal to", &[">=", "maggiore_uguale"]);
        register_keyword_internal(&mut registry, "<=", KeywordType::Operator, "Less than or equal to", &["<=", "minore_uguale"]);

        // === BOOLEAN VALUES ===
        register_keyword_internal(&mut registry, "true", KeywordType::Statement, "Boolean true value", &["true", "vero"]);
        register_keyword_internal(&mut registry, "false", KeywordType::Statement, "Boolean false value", &["false", "falso"]);

        RwLock::new(registry)
    };
}

// Helper function to register keywords internally
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

    for alias in aliases {
        registry.insert(alias.to_string(), keyword.clone());
    }

    Ok(())
}
