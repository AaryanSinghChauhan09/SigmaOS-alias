//! Persistence for alias management.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Store format
#[derive(Debug, Clone, PartialEq)]
pub enum StoreFormat {
    /// Shell format
    Shell,
    /// JSON format
    Json,
    /// YAML format
    Yaml,
    /// Custom format
    Custom(String),
}

/// Store error
#[derive(Debug, Clone, PartialEq)]
pub enum StoreError {
    /// File not found
    FileNotFound,
    /// Parse error
    ParseError(String),
    /// Write error
    WriteError(String),
    /// Invalid format
    InvalidFormat,
    /// Custom error
    Custom(String),
}

/// Alias store
#[derive(Debug, Clone)]
pub struct AliasStore {
    /// Store format
    pub format: StoreFormat,
    /// Store path
    pub path: String,
    /// Store aliases
    pub aliases: HashMap<String, String>,
    /// Store created timestamp
    pub created_at: u64,
    /// Store updated timestamp
    pub updated_at: u64,
}

impl AliasStore {
    /// Create new alias store
    pub fn new(format: StoreFormat, path: String) -> Self {
        Self {
            format,
            path,
            aliases: HashMap::new(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            updated_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Add alias
    pub fn add_alias(&mut self, name: String, expansion: String) {
        self.aliases.insert(name, expansion);
        self.update_timestamp();
    }
    
    /// Remove alias
    pub fn remove_alias(&mut self, name: &str) -> bool {
        if self.aliases.remove(name).is_some() {
            self.update_timestamp();
            return true;
        }
        false
    }
    
    /// Get alias
    pub fn get_alias(&self, name: &str) -> Option<&String> {
        self.aliases.get(name)
    }
    
    /// Set alias
    pub fn set_alias(&mut self, name: String, expansion: String) {
        self.aliases.insert(name, expansion);
        self.update_timestamp();
    }
    
    /// Update timestamp
    fn update_timestamp(&mut self) {
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Save to file
    pub fn save(&self) -> Result<(), StoreError> {
        let content = self.serialize()?;
        fs::write(&self.path, content)
            .map_err(|e| StoreError::WriteError(e.to_string()))
    }
    
    /// Load from file
    pub fn load(&mut self) -> Result<(), StoreError> {
        if !Path::new(&self.path).exists() {
            return Err(StoreError::FileNotFound);
        }
        
        let content = fs::read_to_string(&self.path)
            .map_err(|e| StoreError::WriteError(e.to_string()))?;
        
        self.deserialize(&content)
    }
    
    /// Serialize store
    pub(crate) fn serialize(&self) -> Result<String, StoreError> {
        match self.format {
            StoreFormat::Shell => {
                let mut output = String::new();
                for (name, expansion) in &self.aliases {
                    output.push_str(&format!("alias {}='{}'\n", name, expansion));
                }
                Ok(output)
            }
            StoreFormat::Json => {
                // Simple JSON serialization
                let mut output = String::from("{\n");
                for (i, (name, expansion)) in self.aliases.iter().enumerate() {
                    output.push_str(&format!("  \"{}\": \"{}\"", name, expansion));
                    if i < self.aliases.len() - 1 {
                        output.push(',');
                    }
                    output.push('\n');
                }
                output.push_str("}\n");
                Ok(output)
            }
            StoreFormat::Yaml => {
                // Simple YAML serialization
                let mut output = String::new();
                for (name, expansion) in &self.aliases {
                    output.push_str(&format!("{}: {}\n", name, expansion));
                }
                Ok(output)
            }
            StoreFormat::Custom(_) => {
                // Custom format - just return shell format for now
                let mut output = String::new();
                for (name, expansion) in &self.aliases {
                    output.push_str(&format!("alias {}='{}'\n", name, expansion));
                }
                Ok(output)
            }
        }
    }
    
    /// Deserialize store
    pub(crate) fn deserialize(&mut self, content: &str) -> Result<(), StoreError> {
        self.aliases.clear();
        
        match self.format {
            StoreFormat::Shell => {
                for line in content.lines() {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with('#') {
                        continue;
                    }
                    
                    if let Some(rest) = line.strip_prefix("alias ") {
                        if let Some(eq_pos) = rest.find('=') {
                            let name = rest[..eq_pos].trim().to_string();
                            let expansion = rest[eq_pos + 1..].trim()
                                .trim_matches('\'')
                                .trim_matches('"')
                                .to_string();
                            self.aliases.insert(name, expansion);
                        }
                    }
                }
            }
            StoreFormat::Json => {
                // Simple JSON parsing
                for line in content.lines() {
                    let line = line.trim();
                    if line.is_empty() || line == "{" || line == "}" {
                        continue;
                    }
                    
                    if let Some(colon_pos) = line.find(':') {
                        let name = line[..colon_pos].trim()
                            .trim_matches('"')
                            .to_string();
                        let expansion = line[colon_pos + 1..].trim()
                            .trim_matches(',')
                            .trim_matches('"')
                            .to_string();
                        self.aliases.insert(name, expansion);
                    }
                }
            }
            StoreFormat::Yaml => {
                for line in content.lines() {
                    let line = line.trim();
                    if line.is_empty() {
                        continue;
                    }
                    
                    if let Some(colon_pos) = line.find(':') {
                        let name = line[..colon_pos].trim().to_string();
                        let expansion = line[colon_pos + 1..].trim().to_string();
                        self.aliases.insert(name, expansion);
                    }
                }
            }
            StoreFormat::Custom(_) => {
                // Custom format - parse as shell format
                for line in content.lines() {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with('#') {
                        continue;
                    }
                    
                    if let Some(rest) = line.strip_prefix("alias ") {
                        if let Some(eq_pos) = rest.find('=') {
                            let name = rest[..eq_pos].trim().to_string();
                            let expansion = rest[eq_pos + 1..].trim()
                                .trim_matches('\'')
                                .trim_matches('"')
                                .to_string();
                            self.aliases.insert(name, expansion);
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Get alias count
    pub fn len(&self) -> usize {
        self.aliases.len()
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.aliases.is_empty()
    }
    
    /// Clear all aliases
    pub fn clear(&mut self) {
        self.aliases.clear();
        self.update_timestamp();
    }
    
    /// Get all aliases
    pub fn get_all(&self) -> &HashMap<String, String> {
        &self.aliases
    }
    
    /// Get store age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
    
    /// Get time since update in seconds
    pub fn get_time_since_update(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.updated_at
    }
}
