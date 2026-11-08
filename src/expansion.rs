//! Alias expansion for shell alias management.

use crate::alias::Alias;
use std::collections::HashMap;

/// Expansion context
#[derive(Debug, Clone)]
pub struct ExpansionContext {
    /// Context variables
    pub variables: HashMap<String, String>,
    /// Context arguments
    pub arguments: Vec<String>,
    /// Context working directory
    pub working_directory: String,
    /// Context environment variables
    pub environment: HashMap<String, String>,
    /// Context shell type
    pub shell_type: String,
    /// Context expansion depth
    pub depth: u32,
    /// Context max depth
    pub max_depth: u32,
}

impl Default for ExpansionContext {
    fn default() -> Self {
        Self {
            variables: HashMap::new(),
            arguments: vec![],
            working_directory: "/".into(),
            environment: HashMap::new(),
            shell_type: "bash".into(),
            depth: 0,
            max_depth: 10,
        }
    }
}

impl ExpansionContext {
    /// Create new expansion context
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set variable
    pub fn set_variable(&mut self, name: String, value: String) {
        self.variables.insert(name, value);
    }
    
    /// Get variable
    pub fn get_variable(&self, name: &str) -> Option<&String> {
        self.variables.get(name)
    }
    
    /// Add argument
    pub fn add_argument(&mut self, argument: String) {
        self.arguments.push(argument);
    }
    
    /// Set working directory
    pub fn set_working_directory(&mut self, directory: String) {
        self.working_directory = directory;
    }
    
    /// Set environment variable
    pub fn set_environment(&mut self, name: String, value: String) {
        self.environment.insert(name, value);
    }
    
    /// Get environment variable
    pub fn get_environment(&self, name: &str) -> Option<&String> {
        self.environment.get(name)
    }
    
    /// Set shell type
    pub fn set_shell_type(&mut self, shell_type: String) {
        self.shell_type = shell_type;
    }
    
    /// Increment depth
    pub fn increment_depth(&mut self) {
        self.depth += 1;
    }
    
    /// Check if max depth reached
    pub fn is_max_depth(&self) -> bool {
        self.depth >= self.max_depth
    }
    
    /// Reset depth
    pub fn reset_depth(&mut self) {
        self.depth = 0;
    }
}

/// Expansion result
#[derive(Debug, Clone)]
pub struct ExpansionResult {
    /// Expanded command
    pub expanded: String,
    /// Expansion steps
    pub steps: Vec<ExpansionStep>,
    /// Expansion successful
    pub success: bool,
    /// Expansion error
    pub error: Option<String>,
    /// Expansion depth
    pub depth: u32,
}

/// Expansion step
#[derive(Debug, Clone)]
pub struct ExpansionStep {
    /// Step alias name
    pub alias_name: String,
    /// Step expansion
    pub expansion: String,
    /// Step timestamp
    pub timestamp: u64,
}

/// Expansion error
#[derive(Debug, Clone, PartialEq)]
pub enum ExpansionError {
    /// Alias not found
    AliasNotFound,
    /// Recursive alias
    RecursiveAlias,
    /// Max depth exceeded
    MaxDepthExceeded,
    /// Invalid expansion
    InvalidExpansion,
    /// Custom error
    Custom(String),
}

/// Expander
#[derive(Debug, Clone)]
pub struct Expander {
    /// Alias map
    pub aliases: HashMap<String, Alias>,
    /// Expansion context
    pub context: ExpansionContext,
}

impl Expander {
    /// Create new expander
    pub fn new() -> Self {
        Self {
            aliases: HashMap::new(),
            context: ExpansionContext::new(),
        }
    }
    
    /// Add alias
    pub fn add_alias(&mut self, alias: Alias) {
        self.aliases.insert(alias.name.clone(), alias);
    }
    
    /// Remove alias
    pub fn remove_alias(&mut self, name: &str) -> bool {
        self.aliases.remove(name).is_some()
    }
    
    /// Get alias
    pub fn get_alias(&self, name: &str) -> Option<&Alias> {
        self.aliases.get(name)
    }
    
    /// Expand command
    pub fn expand(&mut self, command: &str) -> Result<ExpansionResult, ExpansionError> {
        let mut result = ExpansionResult {
            expanded: command.to_string(),
            steps: vec![],
            success: true,
            error: None,
            depth: 0,
        };
        
        self.context.reset_depth();
        self.expand_recursive(command, &mut result)?;
        
        Ok(result)
    }
    
    /// Expand command recursively
    fn expand_recursive(&mut self, command: &str, result: &mut ExpansionResult) -> Result<(), ExpansionError> {
        if self.context.is_max_depth() {
            result.success = false;
            result.error = Some("Max depth exceeded".into());
            return Err(ExpansionError::MaxDepthExceeded);
        }
        
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(());
        }
        
        let command_name = parts[0];
        
        if let Some(alias) = self.aliases.get(command_name) {
            if !alias.is_enabled() {
                return Ok(());
            }
            
            // Check for recursive alias
            if self.is_recursive(command_name, &mut vec![]) {
                result.success = false;
                result.error = Some("Recursive alias detected".into());
                return Err(ExpansionError::RecursiveAlias);
            }
            
            // Expand alias
            let expansion = self.expand_alias(alias, &parts[1..]);
            
            // Record step
            result.steps.push(ExpansionStep {
                alias_name: alias.name.clone(),
                expansion: expansion.clone(),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            });
            
            // Recursively expand
            self.context.increment_depth();
            result.depth = self.context.depth;
            self.expand_recursive(&expansion, result)?;
            
            result.expanded = expansion;
        }
        
        Ok(())
    }
    
    /// Expand alias with arguments
    fn expand_alias(&self, alias: &Alias, args: &[&str]) -> String {
        let mut expansion = alias.expansion.clone();
        
        // Replace parameters
        for (i, param) in alias.parameters.iter().enumerate() {
            let placeholder = format!("${}", i + 1);
            expansion = expansion.replace(&placeholder, args.get(i).unwrap_or(&""));
        }
        
        // Replace $@ with all arguments
        expansion = expansion.replace("$@", &args.join(" "));
        
        // Replace $# with argument count
        expansion = expansion.replace("$#", &args.len().to_string());
        
        expansion
    }
    
    /// Check if alias is recursive
    fn is_recursive(&self, name: &str, visited: &mut Vec<String>) -> bool {
        if visited.contains(&name.to_string()) {
            return true;
        }
        
        visited.push(name.to_string());
        
        if let Some(alias) = self.aliases.get(name) {
            let parts: Vec<&str> = alias.expansion.split_whitespace().collect();
            if !parts.is_empty() {
                let first = parts[0];
                if self.aliases.contains_key(first) {
                    return self.is_recursive(first, visited);
                }
            }
        }
        
        visited.pop();
        false
    }
    
    /// Check if command is alias
    pub fn is_alias(&self, command: &str) -> bool {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return false;
        }
        self.aliases.contains_key(parts[0])
    }
    
    /// Get alias count
    pub fn get_alias_count(&self) -> usize {
        self.aliases.len()
    }
    
    /// Clear all aliases
    pub fn clear(&mut self) {
        self.aliases.clear();
    }
}

impl Default for Expander {
    fn default() -> Self {
        Self::new()
    }
}
