//! Alias model.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AliasMap {
    pub aliases: HashMap<String, String>,
}

impl Default for AliasMap {
    fn default() -> Self {
        Self {
            aliases: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Alias {
    pub temporary: bool,
}

impl Default for Alias {
    fn default() -> Self {
        Self { temporary: true }
    }
}

impl Alias {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&self, map: &mut AliasMap, name: &str, value: &str) {
        map.aliases.insert(name.to_string(), value.to_string());
    }

    pub fn get<'a>(&self, map: &'a AliasMap, name: &str) -> Option<&'a String> {
        map.aliases.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alias() {
        let a = Alias::new();
        let mut m = AliasMap::default();
        a.add(&mut m, "ll", "ls -l");
        assert_eq!(a.get(&m, "ll"), Some(&"ls -l".to_string()));
    }
}
