use std::cmp::Ordering;

use markdown::unist::Position;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Violation {
    name: String,
    description: String,
    position: Position,
}

impl Violation {
    pub fn new(name: String, description: String, position: Position) -> Self {
        Self {
            position,
            name,
            description,
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }

    pub fn position(&self) -> Position {
        self.position.clone()
    }
}

impl PartialOrd for Violation {
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
