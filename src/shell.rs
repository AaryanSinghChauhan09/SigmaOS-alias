//! Shell integration for alias management.

/// Shell type
#[derive(Debug, Clone, PartialEq)]
pub enum ShellType {
    /// Bash shell
    Bash,
    /// Zsh shell
    Zsh,
    /// Fish shell
    Fish,
    /// Sh shell
    Sh,
    /// Custom shell
    Custom(String),
}

impl ShellType {
    /// Get shell name
    pub fn name(&self) -> &str {
        match self {
            ShellType::Bash => "bash",
            ShellType::Zsh => "zsh",
            ShellType::Fish => "fish",
            ShellType::Sh => "sh",
            ShellType::Custom(name) => name,
        }
    }
    
    /// Get config file path
    pub fn config_file(&self) -> String {
        match self {
            ShellType::Bash => ".bashrc".into(),
            ShellType::Zsh => ".zshrc".into(),
            ShellType::Fish => ".config/fish/config.fish".into(),
            ShellType::Sh => ".profile".into(),
            ShellType::Custom(_) => ".shellrc".into(),
        }
    }
}

/// Shell profile
#[derive(Debug, Clone)]
pub struct ShellProfile {
    /// Shell type
    pub shell_type: ShellType,
    /// Config files
    pub config_files: Vec<String>,
    /// Alias files
    pub alias_files: Vec<String>,
    /// Profile name
    pub name: String,
    /// Profile description
    pub description: String,
    /// Profile enabled
    pub enabled: bool,
}

impl ShellProfile {
    /// Create new shell profile
    pub fn new(shell_type: ShellType) -> Self {
        let name = shell_type.name().to_string();
        Self {
            shell_type,
            config_files: vec![],
            alias_files: vec![],
            name,
            description: String::new(),
            enabled: true,
        }
    }
    
    /// Add config file
    pub fn add_config_file(&mut self, file: String) {
        if !self.config_files.contains(&file) {
            self.config_files.push(file);
        }
    }
    
    /// Add alias file
    pub fn add_alias_file(&mut self, file: String) {
        if !self.alias_files.contains(&file) {
            self.alias_files.push(file);
        }
    }
    
    /// Set name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    
    /// Set description
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
    
    /// Enable profile
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    
    /// Disable profile
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    
    /// Check if profile is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// Shell config
#[derive(Debug, Clone)]
pub struct ShellConfig {
    /// Shell type
    pub shell_type: ShellType,
    /// Config file path
    pub config_path: String,
    /// Alias file path
    pub alias_path: String,
    /// Auto-load aliases
    pub auto_load_aliases: bool,
    /// Alias expansion enabled
    pub alias_expansion: bool,
    /// Alias completion enabled
    pub alias_completion: bool,
    /// Alias history enabled
    pub alias_history: bool,
}

impl ShellConfig {
    /// Create new shell config
    pub fn new(shell_type: ShellType) -> Self {
        Self {
            shell_type: shell_type.clone(),
            config_path: shell_type.config_file(),
            alias_path: ".aliases".into(),
            auto_load_aliases: true,
            alias_expansion: true,
            alias_completion: true,
            alias_history: true,
        }
    }
    
    /// Set config path
    pub fn set_config_path(&mut self, path: String) {
        self.config_path = path;
    }
    
    /// Set alias path
    pub fn set_alias_path(&mut self, path: String) {
        self.alias_path = path;
    }
    
    /// Enable auto-load
    pub fn enable_auto_load(&mut self) {
        self.auto_load_aliases = true;
    }
    
    /// Disable auto-load
    pub fn disable_auto_load(&mut self) {
        self.auto_load_aliases = false;
    }
    
    /// Enable alias expansion
    pub fn enable_expansion(&mut self) {
        self.alias_expansion = true;
    }
    
    /// Disable alias expansion
    pub fn disable_expansion(&mut self) {
        self.alias_expansion = false;
    }
    
    /// Enable alias completion
    pub fn enable_completion(&mut self) {
        self.alias_completion = true;
    }
    
    /// Disable alias completion
    pub fn disable_completion(&mut self) {
        self.alias_completion = false;
    }
    
    /// Enable alias history
    pub fn enable_history(&mut self) {
        self.alias_history = true;
    }
    
    /// Disable alias history
    pub fn disable_history(&mut self) {
        self.alias_history = false;
    }
}
