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
