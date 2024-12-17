use core::cmp::Ordering;
use std::path::PathBuf;

use comrak::nodes::Sourcepos;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Violation {
    path: PathBuf,
    name: String,
    alias: String,
    description: String,
    position: Sourcepos,
}

impl Violation {
    #[inline]
    #[must_use]
    pub fn new(
        path: PathBuf,
        name: String,
        alias: String,
        description: String,
        position: Sourcepos,
    ) -> Self {
        Self {
            path,
            name,
            alias,
            description,
            position,
        }
    }

    #[inline]
    #[must_use]
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    #[must_use]
    pub fn alias(&self) -> &str {
        &self.alias
    }

    #[inline]
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    #[inline]
    #[must_use]
    pub fn position(&self) -> &Sourcepos {
        &self.position
    }

    #[inline]
    pub fn update_position(&mut self, position: Sourcepos) {
        self.position = position;
    }
}

impl PartialOrd for Violation {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Violation {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        match self.position().start.line.cmp(&other.position().start.line) {
            Ordering::Equal => self.name.cmp(&other.name),
            ord => ord,
        }
    }
}
