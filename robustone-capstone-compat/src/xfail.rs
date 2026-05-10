//! Xfail registry for known compatibility deviations.

use std::collections::HashSet;

/// A single xfail entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XfailEntry {
    pub id: String,
    pub reason: String,
    /// Substring pattern to match against an error message.
    pub pattern: Option<String>,
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

    /// Match an error message against registered xfail patterns.
    pub fn match_error(&self, msg: &str) -> Option<&XfailEntry> {
        self.details.iter().find(|e| {
            if let Some(ref pat) = e.pattern {
                msg.contains(pat)
            } else {
                msg.contains(&e.id)
            }
        })
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
    let mut reg = XfailRegistry::new();

    // Capstone renders `orn` where we render `nor` for some encodings.
    reg.register(XfailEntry {
        id: "nor-orn-alias".into(),
        reason: "Capstone uses orn alias, we use nor".into(),
        pattern: Some("orn".into()),
        issue: None,
        expires: None,
    });

    // screl syntax differences.
    reg.register(XfailEntry {
        id: "screl-syntax".into(),
        reason: "screl syntax difference".into(),
        pattern: Some("screl".into()),
        issue: None,
        expires: None,
    });

    // Vector immediate 0x9 vs 9 rendering.
    reg.register(XfailEntry {
        id: "vec-immed-0x9".into(),
        reason: "Capstone YAML expects 0x9 but cstool renders 9".into(),
        pattern: Some("0x9\"".into()),
        issue: None,
        expires: None,
    });

    reg
}
