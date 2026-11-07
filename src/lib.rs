//! SigmaOS-alias — Shell alias model.
//!
//! Models creating and managing shell aliases.
//! Deterministic and std-only.

pub mod alias;
pub mod options;

pub use alias::{Alias, AliasMap};
pub use options::{AliasOptions, Scope};
