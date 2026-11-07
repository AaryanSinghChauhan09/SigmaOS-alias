//! Options model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    Temporary,
    Permanent,
}

#[derive(Debug, Clone)]
pub struct AliasOptions {
    pub scope: Scope,
    pub list_all: bool,
}

impl Default for AliasOptions {
    fn default() -> Self {
        Self {
            scope: Scope::Temporary,
            list_all: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alias_options() {
        let o = AliasOptions::default();
        assert_eq!(o.scope, Scope::Temporary);
    }
}
