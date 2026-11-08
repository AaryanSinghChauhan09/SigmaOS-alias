//! Alias manager for shell alias management.

use std::collections::HashMap;
use crate::alias::{Alias, AliasType};
use crate::expansion::{Expander, ExpansionResult, ExpansionError};
use crate::persistence::{AliasStore, StoreFormat};
use crate::shell::{ShellType, ShellProfile, ShellConfig};

/// Manager options
#[derive(Debug, Clone)]
pub struct ManagerOptions {
    /// Default shell type
    pub default_shell: ShellType,
    /// Auto-load aliases on startup
    pub auto_load: bool,
    /// Auto-save aliases on change
    pub auto_save: bool,
    /// Alias file path
    pub alias_file: String,
    /// Enable alias expansion
    pub enable_expansion: bool,
    /// Enable alias completion
    pub enable_completion: bool,
    /// Enable alias history
    pub enable_history: bool,
    /// Max alias expansion depth
    pub max_expansion_depth: u32,
}

impl Default for ManagerOptions {
    fn default() -> Self {
        Self {
            default_shell: ShellType::Bash,
            auto_load: true,
            auto_save: false,
            alias_file: ".aliases".into(),
            enable_expansion: true,
            enable_completion: true,
            enable_history: true,
            max_expansion_depth: 10,
        }
    }
}

/// Manager statistics
#[derive(Debug, Clone)]
pub struct ManagerStatistics {
    /// Total aliases
    pub total_aliases: usize,
    /// Enabled aliases
    pub enabled_aliases: usize,
    /// Disabled aliases
    pub disabled_aliases: usize,
    /// Global aliases
    pub global_aliases: usize,
    /// Parameterized aliases
    pub parameterized_aliases: usize,
    /// Total expansions
    pub total_expansions: usize,
    /// Successful expansions
    pub successful_expansions: usize,
    /// Failed expansions
    pub failed_expansions: usize,
    /// Manager uptime in seconds
    pub uptime: u64,
}

/// Alias manager
#[derive(Debug, Clone)]
pub struct AliasManager {
    /// Aliases
    aliases: HashMap<String, Alias>,
    /// Expander
    expander: Expander,
    /// Shell profiles
    shell_profiles: HashMap<String, ShellProfile>,
    /// Shell configs
    shell_configs: HashMap<String, ShellConfig>,
    /// Options
    options: ManagerOptions,
    /// Statistics
    statistics: ManagerStatistics,
    /// Manager created timestamp
    pub created_at: u64,
}

impl Default for AliasManager {
    fn default() -> Self {
        Self {
            aliases: HashMap::new(),
            expander: Expander::new(),
            shell_profiles: HashMap::new(),
            shell_configs: HashMap::new(),
            options: ManagerOptions::default(),
            statistics: ManagerStatistics {
                total_aliases: 0,
                enabled_aliases: 0,
                disabled_aliases: 0,
                global_aliases: 0,
                parameterized_aliases: 0,
                total_expansions: 0,
                successful_expansions: 0,
                failed_expansions: 0,
                uptime: 0,
            },
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

impl AliasManager {
    /// Create new alias manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create manager with options
    pub fn with_options(options: ManagerOptions) -> Self {
        Self {
            aliases: HashMap::new(),
            expander: Expander::new(),
            shell_profiles: HashMap::new(),
            shell_configs: HashMap::new(),
            options,
            statistics: ManagerStatistics {
                total_aliases: 0,
                enabled_aliases: 0,
                disabled_aliases: 0,
                global_aliases: 0,
                parameterized_aliases: 0,
                total_expansions: 0,
                successful_expansions: 0,
                failed_expansions: 0,
                uptime: 0,
            },
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Initialize manager
    pub fn initialize(&mut self) {
        // Add default shell profile
        let default_profile = ShellProfile::new(self.options.default_shell.clone());
        self.shell_profiles.insert("default".into(), default_profile);
        
        // Add default shell config
        let default_config = ShellConfig::new(self.options.default_shell.clone());
        self.shell_configs.insert("default".into(), default_config);
        
        // Load aliases if auto-load enabled
        if self.options.auto_load {
            self.load_aliases();
        }
        
        self.update_statistics();
    }
    
    // Alias methods
    
    /// Define alias
    pub fn define_alias(&mut self, name: String, expansion: String) -> bool {
        let alias = Alias::new(name.clone(), expansion);
        self.aliases.insert(name.clone(), alias.clone());
        self.expander.add_alias(alias);
        self.update_statistics();
        true
    }
    
    /// Define alias with type
    pub fn define_alias_with_type(&mut self, name: String, expansion: String, alias_type: AliasType) -> bool {
        let mut alias = Alias::new(name.clone(), expansion);
        alias.set_type(alias_type);
        self.aliases.insert(name.clone(), alias.clone());
        self.expander.add_alias(alias);
        self.update_statistics();
        true
    }
    
    /// Get alias
    pub fn get_alias(&self, name: &str) -> Option<&Alias> {
        self.aliases.get(name)
    }
    
    /// Get alias mutably
    pub fn get_alias_mut(&mut self, name: &str) -> Option<&mut Alias> {
        self.aliases.get_mut(name)
    }
    
    /// Get all aliases
    pub fn list_aliases(&self) -> Vec<&Alias> {
        self.aliases.values().collect()
    }
    
    /// Remove alias
    pub fn remove_alias(&mut self, name: &str) -> bool {
        if self.aliases.remove(name).is_some() {
            self.expander.remove_alias(name);
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Enable alias
    pub fn enable_alias(&mut self, name: &str) -> bool {
        if let Some(alias) = self.aliases.get_mut(name) {
            alias.enable();
            self.expander.add_alias(alias.clone());
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Disable alias
    pub fn disable_alias(&mut self, name: &str) -> bool {
        if let Some(alias) = self.aliases.get_mut(name) {
            alias.disable();
            self.expander.add_alias(alias.clone());
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Set alias expansion
    pub fn set_alias_expansion(&mut self, name: &str, expansion: String) -> bool {
        if let Some(alias) = self.aliases.get_mut(name) {
            alias.set_expansion(expansion);
            self.expander.add_alias(alias.clone());
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Set alias description
    pub fn set_alias_description(&mut self, name: &str, description: String) -> bool {
        if let Some(alias) = self.aliases.get_mut(name) {
            alias.set_description(description);
            self.expander.add_alias(alias.clone());
            return true;
        }
        false
    }
    
    /// Add alias parameter
    pub fn add_alias_parameter(&mut self, name: &str, parameter: &str) -> bool {
        if let Some(alias) = self.aliases.get_mut(name) {
            alias.add_parameter(parameter);
            self.expander.add_alias(alias.clone());
            return true;
        }
        false
    }
    
    /// Remove alias parameter
    pub fn remove_alias_parameter(&mut self, name: &str, parameter: &str) -> bool {
        if let Some(alias) = self.aliases.get_mut(name) {
            if alias.remove_parameter(parameter) {
                self.expander.add_alias(alias.clone());
                return true;
            }
        }
        false
    }
    
    /// Set alias global
    pub fn set_alias_global(&mut self, name: &str, global: bool) -> bool {
        if let Some(alias) = self.aliases.get_mut(name) {
            alias.set_global(global);
            self.expander.add_alias(alias.clone());
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Get aliases by type
    pub fn get_aliases_by_type(&self, alias_type: &AliasType) -> Vec<&Alias> {
        self.aliases.values()
            .filter(|a| a.alias_type == *alias_type)
            .collect()
    }
    
    /// Get enabled aliases
    pub fn get_enabled_aliases(&self) -> Vec<&Alias> {
        self.aliases.values()
            .filter(|a| a.is_enabled())
            .collect()
    }
    
    /// Get disabled aliases
    pub fn get_disabled_aliases(&self) -> Vec<&Alias> {
        self.aliases.values()
            .filter(|a| a.is_disabled())
            .collect()
    }
    
    /// Get global aliases
    pub fn get_global_aliases(&self) -> Vec<&Alias> {
        self.aliases.values()
            .filter(|a| a.is_global())
            .collect()
    }
    
    /// Get parameterized aliases
    pub fn get_parameterized_aliases(&self) -> Vec<&Alias> {
        self.aliases.values()
            .filter(|a| a.is_parameterized())
            .collect()
    }
    
    /// Search aliases
    pub fn search_aliases(&self, query: &str) -> Vec<&Alias> {
        let query_lower = query.to_lowercase();
        self.aliases.values()
            .filter(|a| a.name.to_lowercase().contains(&query_lower) ||
                       a.expansion.to_lowercase().contains(&query_lower) ||
                       a.description.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    /// Get most used aliases
    pub fn get_most_used_aliases(&self, limit: usize) -> Vec<&Alias> {
        let mut aliases: Vec<&Alias> = self.aliases.values().collect();
        aliases.sort_by(|a, b| b.use_count.cmp(&a.use_count));
        aliases.truncate(limit);
        aliases
    }
    
    /// Get least used aliases
    pub fn get_least_used_aliases(&self, limit: usize) -> Vec<&Alias> {
        let mut aliases: Vec<&Alias> = self.aliases.values().collect();
        aliases.sort_by(|a, b| a.use_count.cmp(&b.use_count));
        aliases.truncate(limit);
        aliases
    }
    
    // Expansion methods
    
    /// Expand command
    pub fn expand(&mut self, command: &str) -> Option<ExpansionResult> {
        self.statistics.total_expansions += 1;
        
        match self.expander.expand(command) {
            Ok(result) => {
                if result.steps.is_empty() {
                    self.statistics.failed_expansions += 1;
                    None
                } else {
                    self.statistics.successful_expansions += 1;
                    Some(result)
                }
            }
            Err(_) => {
                self.statistics.failed_expansions += 1;
                None
            }
        }
    }
    
    /// Check if command is alias
    pub fn is_alias(&self, command: &str) -> bool {
        self.expander.is_alias(command)
    }
    
    /// Check if alias is recursive
    pub fn check_recursive(&self, name: &str) -> Result<(), ExpansionError> {
        // Simple check - would need more sophisticated detection
        if self.aliases.contains_key(name) {
            Ok(())
        } else {
            Err(ExpansionError::AliasNotFound)
        }
    }
    
    // Shell profile methods
    
    /// Add shell profile
    pub fn add_shell_profile(&mut self, name: String, profile: ShellProfile) -> bool {
        self.shell_profiles.insert(name, profile);
        true
    }
    
    /// Get shell profile
    pub fn get_shell_profile(&self, name: &str) -> Option<&ShellProfile> {
        self.shell_profiles.get(name)
    }
    
    /// Get shell profile mutably
    pub fn get_shell_profile_mut(&mut self, name: &str) -> Option<&mut ShellProfile> {
        self.shell_profiles.get_mut(name)
    }
    
    /// Get all shell profiles
    pub fn list_shell_profiles(&self) -> Vec<&ShellProfile> {
        self.shell_profiles.values().collect()
    }
    
    /// Delete shell profile
    pub fn delete_shell_profile(&mut self, name: &str) -> bool {
        self.shell_profiles.remove(name).is_some()
    }
    
    // Shell config methods
    
    /// Add shell config
    pub fn add_shell_config(&mut self, name: String, config: ShellConfig) -> bool {
        self.shell_configs.insert(name, config);
        true
    }
    
    /// Get shell config
    pub fn get_shell_config(&self, name: &str) -> Option<&ShellConfig> {
        self.shell_configs.get(name)
    }
    
    /// Get shell config mutably
    pub fn get_shell_config_mut(&mut self, name: &str) -> Option<&mut ShellConfig> {
        self.shell_configs.get_mut(name)
    }
    
    /// Get all shell configs
    pub fn list_shell_configs(&self) -> Vec<&ShellConfig> {
        self.shell_configs.values().collect()
    }
    
    /// Delete shell config
    pub fn delete_shell_config(&mut self, name: &str) -> bool {
        self.shell_configs.remove(name).is_some()
    }
    
    // Persistence methods
    
    /// Export aliases
    pub fn export_aliases(&self, format: StoreFormat) -> String {
        let mut store = AliasStore::new(format.clone(), self.options.alias_file.clone());
        
        for alias in self.aliases.values() {
            store.add_alias(alias.name.clone(), alias.expansion.clone());
        }
        
        match store.serialize() {
            Ok(content) => content,
            Err(_) => String::new(),
        }
    }
    
    /// Import aliases
    pub fn import_aliases(&mut self, content: String) {
        let mut store = AliasStore::new(StoreFormat::Shell, self.options.alias_file.clone());
        
        // Parse content and add aliases
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
                    store.add_alias(name, expansion);
                }
            }
        }
        
        // Add aliases to manager
        for (name, expansion) in store.get_all() {
            self.define_alias(name.clone(), expansion.clone());
        }
    }
    
    /// Save aliases to file
    pub fn save_aliases(&self) -> Result<(), crate::persistence::StoreError> {
        let mut store = AliasStore::new(StoreFormat::Shell, self.options.alias_file.clone());
        
        for alias in self.aliases.values() {
            store.add_alias(alias.name.clone(), alias.expansion.clone());
        }
        
        store.save()
    }
    
    /// Load aliases from file
    pub fn load_aliases(&mut self) -> Result<(), crate::persistence::StoreError> {
        let mut store = AliasStore::new(StoreFormat::Shell, self.options.alias_file.clone());
        store.load()?;
        
        // Clear existing aliases
        self.aliases.clear();
        self.expander.clear();
        
        // Add loaded aliases
        for (name, expansion) in store.get_all() {
            self.define_alias(name.clone(), expansion.clone());
        }
        
        Ok(())
    }
    
    /// Get manager uptime
    pub fn get_uptime(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
    
    /// Update statistics
    fn update_statistics(&mut self) {
        self.statistics.total_aliases = self.aliases.len();
        self.statistics.enabled_aliases = self.aliases.values()
            .filter(|a| a.is_enabled())
            .count();
        self.statistics.disabled_aliases = self.aliases.values()
            .filter(|a| a.is_disabled())
            .count();
        self.statistics.global_aliases = self.aliases.values()
            .filter(|a| a.is_global())
            .count();
        self.statistics.parameterized_aliases = self.aliases.values()
            .filter(|a| a.is_parameterized())
            .count();
        self.statistics.uptime = self.get_uptime();
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &ManagerStatistics {
        &self.statistics
    }
}
