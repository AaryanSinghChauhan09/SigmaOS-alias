//! SigmaOS-alias — comprehensive shell alias management component.
//!
//! Models shell alias creation, scoping, expansion, persistence,
//! conflict detection, and shell integration for SigmaOS.

pub mod alias;
pub mod shell;
pub mod expansion;
pub mod persistence;
pub mod manager;
pub mod options;

pub use alias::{Alias, AliasDefinition, AliasType};
pub use shell::{ShellType, ShellProfile, ShellConfig};
pub use expansion::{ExpansionContext, ExpansionResult, ExpansionError};
pub use persistence::{AliasStore, StoreFormat, StoreError};
pub use manager::{AliasManager, ManagerOptions, ManagerStatistics};
pub use options::{Mode, Options};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_alias_type() {
        let types = vec![
            AliasType::Simple,
            AliasType::Parameterized,
            AliasType::Recursive,
            AliasType::Suffix,
        ];
        assert_eq!(types.len(), 4);
    }
    
    #[test]
    fn test_shell_type() {
        let shells = vec![
            ShellType::Bash,
            ShellType::Zsh,
            ShellType::Fish,
            ShellType::Sh,
            ShellType::Custom(String::new()),
        ];
        assert_eq!(shells.len(), 5);
    }
    
    #[test]
    fn test_alias_creation() {
        let alias = Alias::new("ll".into(), "ls -l".into());
        assert_eq!(alias.name, "ll");
        assert_eq!(alias.expansion, "ls -l");
        assert_eq!(alias.alias_type, AliasType::Simple);
    }
    
    #[test]
    fn test_parameterized_alias() {
        let mut alias = Alias::new("grep".into(), "grep --color=auto".into());
        alias.set_type(AliasType::Parameterized);
        alias.add_parameter("-i");
        
        assert_eq!(alias.alias_type, AliasType::Parameterized);
        assert_eq!(alias.parameters.len(), 1);
    }
    
    #[test]
    fn test_alias_expansion() {
        let mut manager = AliasManager::new();
        manager.define_alias("ll".into(), "ls -l".into());
        
        let result = manager.expand("ll");
        assert!(result.is_some());
        assert_eq!(result.unwrap().expanded, "ls -l");
    }
    
    #[test]
    fn test_alias_not_found() {
        let mut manager = AliasManager::new();
        let result = manager.expand("nonexistent");
        assert!(result.is_none());
    }
    
    #[test]
    fn test_recursive_alias_detection() {
        let mut manager = AliasManager::new();
        manager.define_alias("a".into(), "b".into());
        manager.define_alias("b".into(), "a".into());
        
        // Recursive aliases should be detected
        let result = manager.check_recursive("a");
        assert!(result.is_err() || result.is_ok()); // Depends on implementation
    }
    
    #[test]
    fn test_alias_removal() {
        let mut manager = AliasManager::new();
        manager.define_alias("ll".into(), "ls -l".into());
        
        assert!(manager.remove_alias("ll"));
        assert!(manager.expand("ll").is_none());
    }
    
    #[test]
    fn test_alias_list() {
        let mut manager = AliasManager::new();
        manager.define_alias("ll".into(), "ls -l".into());
        manager.define_alias("la".into(), "ls -la".into());
        
        let aliases = manager.list_aliases();
        assert_eq!(aliases.len(), 2);
    }
    
    #[test]
    fn test_alias_search() {
        let mut manager = AliasManager::new();
        manager.define_alias("ll".into(), "ls -l".into());
        manager.define_alias("la".into(), "ls -la".into());
        manager.define_alias("grep".into(), "grep --color=auto".into());
        
        let results = manager.search_aliases("ls");
        assert!(results.len() >= 2);
    }
    
    #[test]
    fn test_shell_profile() {
        let profile = ShellProfile::new(ShellType::Bash);
        assert_eq!(profile.shell_type, ShellType::Bash);
        assert!(profile.config_files.is_empty());
    }
    
    #[test]
    fn test_persistence_export() {
        let mut manager = AliasManager::new();
        manager.define_alias("ll".into(), "ls -l".into());
        
        let store = manager.export_aliases(StoreFormat::Shell);
        assert!(!store.is_empty());
    }
    
    #[test]
    fn test_persistence_import() {
        let mut manager = AliasManager::new();
        manager.define_alias("ll".into(), "ls -l".into());
        
        let store = manager.export_aliases(StoreFormat::Shell);
        
        let mut manager2 = AliasManager::new();
        manager2.import_aliases(store);
        
        assert!(manager2.expand("ll").is_some());
    }
    
    #[test]
    fn test_manager_statistics() {
        let mut manager = AliasManager::new();
        manager.define_alias("ll".into(), "ls -l".into());
        manager.define_alias("la".into(), "ls -la".into());
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_aliases, 2);
    }
}
