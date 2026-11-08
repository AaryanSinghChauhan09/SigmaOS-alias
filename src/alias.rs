//! Alias definitions for shell alias management.

/// Alias type
#[derive(Debug, Clone, PartialEq)]
pub enum AliasType {
    /// Simple alias
    Simple,
    /// Parameterized alias
    Parameterized,
    /// Recursive alias
    Recursive,
    /// Suffix alias
    Suffix,
}

/// Alias definition
#[derive(Debug, Clone)]
pub struct AliasDefinition {
    /// Alias name
    pub name: String,
    /// Alias expansion
    pub expansion: String,
    /// Alias type
    pub alias_type: AliasType,
    /// Alias parameters
    pub parameters: Vec<String>,
    /// Alias description
    pub description: String,
    /// Alias enabled
    pub enabled: bool,
    /// Alias global
    pub global: bool,
    /// Alias created timestamp
    pub created_at: u64,
    /// Alias updated timestamp
    pub updated_at: u64,
    /// Alias use count
    pub use_count: u64,
    /// Alias last used timestamp
    pub last_used: Option<u64>,
}

/// Alias
#[derive(Debug, Clone)]
pub struct Alias {
    /// Alias name
    pub name: String,
    /// Alias expansion
    pub expansion: String,
    /// Alias type
    pub alias_type: AliasType,
    /// Alias parameters
    pub parameters: Vec<String>,
    /// Alias description
    pub description: String,
    /// Alias enabled
    pub enabled: bool,
    /// Alias global
    pub global: bool,
    /// Alias created timestamp
    pub created_at: u64,
    /// Alias updated timestamp
    pub updated_at: u64,
    /// Alias use count
    pub use_count: u64,
    /// Alias last used timestamp
    pub last_used: Option<u64>,
}

impl Alias {
    /// Create new alias
    pub fn new(name: String, expansion: String) -> Self {
        Self {
            name,
            expansion,
            alias_type: AliasType::Simple,
            parameters: vec![],
            description: String::new(),
            enabled: true,
            global: false,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            updated_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            use_count: 0,
            last_used: None,
        }
    }
    
    /// Set type
    pub fn set_type(&mut self, alias_type: AliasType) {
        self.alias_type = alias_type;
        self.update_timestamp();
    }
    
    /// Add parameter
    pub fn add_parameter(&mut self, parameter: &str) {
        self.parameters.push(parameter.to_string());
        self.update_timestamp();
    }
    
    /// Remove parameter
    pub fn remove_parameter(&mut self, parameter: &str) -> bool {
        if let Some(pos) = self.parameters.iter().position(|p| p == parameter) {
            self.parameters.remove(pos);
            self.update_timestamp();
            return true;
        }
        false
    }
    
    /// Set description
    pub fn set_description(&mut self, description: String) {
        self.description = description;
        self.update_timestamp();
    }
    
    /// Set expansion
    pub fn set_expansion(&mut self, expansion: String) {
        self.expansion = expansion;
        self.update_timestamp();
    }
    
    /// Enable alias
    pub fn enable(&mut self) {
        self.enabled = true;
        self.update_timestamp();
    }
    
    /// Disable alias
    pub fn disable(&mut self) {
        self.enabled = false;
        self.update_timestamp();
    }
    
    /// Set global
    pub fn set_global(&mut self, global: bool) {
        self.global = global;
        self.update_timestamp();
    }
    
    /// Record use
    pub fn record_use(&mut self) {
        self.use_count += 1;
        self.last_used = Some(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());
        self.update_timestamp();
    }
    
    /// Update timestamp
    fn update_timestamp(&mut self) {
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Check if alias is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Check if alias is disabled
    pub fn is_disabled(&self) -> bool {
        !self.enabled
    }
    
    /// Check if alias is global
    pub fn is_global(&self) -> bool {
        self.global
    }
    
    /// Check if alias is parameterized
    pub fn is_parameterized(&self) -> bool {
        self.alias_type == AliasType::Parameterized
    }
    
    /// Check if alias is recursive
    pub fn is_recursive(&self) -> bool {
        self.alias_type == AliasType::Recursive
    }
    
    /// Check if alias is suffix
    pub fn is_suffix(&self) -> bool {
        self.alias_type == AliasType::Suffix
    }
    
    /// Get parameter count
    pub fn get_parameter_count(&self) -> usize {
        self.parameters.len()
    }
    
    /// Check if alias has parameters
    pub fn has_parameters(&self) -> bool {
        !self.parameters.is_empty()
    }
    
    /// Check if alias has description
    pub fn has_description(&self) -> bool {
        !self.description.is_empty()
    }
    
    /// Get alias age in seconds
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
    
    /// Get time since last use in seconds
    pub fn get_time_since_last_use(&self) -> Option<u64> {
        self.last_used.map(|last| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() - last
        })
    }
    
    /// Get use frequency
    pub fn get_use_frequency(&self) -> f64 {
        let age_hours = self.get_age_seconds() as f64 / 3600.0;
        if age_hours == 0.0 {
            return 0.0;
        }
        self.use_count as f64 / age_hours
    }
}

/// Alias map for backward compatibility
#[derive(Debug, Clone, Default)]
pub struct AliasMap {
    pub aliases: std::collections::HashMap<String, String>,
}

impl AliasMap {
    /// Create new alias map
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add alias
    pub fn add(&mut self, name: &str, value: &str) {
        self.aliases.insert(name.to_string(), value.to_string());
    }
    
    /// Get alias
    pub fn get(&self, name: &str) -> Option<&String> {
        self.aliases.get(name)
    }
    
    /// Remove alias
    pub fn remove(&mut self, name: &str) -> bool {
        self.aliases.remove(name).is_some()
    }
    
    /// Check if alias exists
    pub fn contains(&self, name: &str) -> bool {
        self.aliases.contains_key(name)
    }
    
    /// Get all aliases
    pub fn get_all(&self) -> &std::collections::HashMap<String, String> {
        &self.aliases
    }
    
    /// Clear all aliases
    pub fn clear(&mut self) {
        self.aliases.clear();
    }
    
    /// Get alias count
    pub fn len(&self) -> usize {
        self.aliases.len()
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.aliases.is_empty()
    }
}
