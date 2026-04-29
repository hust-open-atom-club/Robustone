//! Xfail registry for known compatibility deviations.

use std::collections::HashSet;

/// A single xfail entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XfailEntry {
    pub id: String,
    pub reason: String,
    pub issue: Option<String>,
    pub expires: Option<String>,
}

/// Registry of expected failures.
#[derive(Debug, Default)]
pub struct XfailRegistry {
    entries: HashSet<String>,
    details: Vec<XfailEntry>,
}

impl XfailRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a known xfail by fixture identifier.
    pub fn register(&mut self, entry: XfailEntry) {
        self.entries.insert(entry.id.clone());
        self.details.push(entry);
    }

    /// Check whether a fixture is a known xfail.
    pub fn is_xfail(&self, id: &str) -> bool {
        self.entries.contains(id)
    }

    /// Return all registered xfails.
    pub fn entries(&self) -> &[XfailEntry] {
        &self.details
    }

    /// Count of registered xfails.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// Build a default xfail registry for LoongArch Capstone YAML tests.
pub fn loongarch_default_xfails() -> XfailRegistry {
    // No documented xfails yet – all failures are treated as real bugs.
    XfailRegistry::new()
}
