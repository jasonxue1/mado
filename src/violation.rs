use std::{cmp::Ordering, path::PathBuf};

use markdown::unist::Position;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Violation {
    path: PathBuf,
    name: String,
    description: String,
    position: Position,
}

impl Violation {
    pub fn new(path: PathBuf, name: String, description: String, position: Position) -> Self {
        Self {
            path,
            position,
            name,
            description,
        }
    }

    #[inline]
    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }

    #[inline]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[inline]
    pub fn description(&self) -> String {
        self.description.clone()
    }

    #[inline]
    pub fn position(&self) -> Position {
        self.position.clone()
    }
}

impl PartialOrd for Violation {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Violation {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.position().start.line.cmp(&other.position().start.line) {
            Ordering::Equal => self.name.cmp(&other.name),
            ord => ord,
        }
    }
}
