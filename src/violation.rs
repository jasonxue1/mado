use std::{
    cmp::Ordering,
    fmt::{Display, Error},
    path::PathBuf,
};

use colored::Colorize;
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

impl Display for Violation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path = self.path.to_str().ok_or(Error)?;
        write!(
            f,
            "{}{}{}{}{}{} {} {}",
            path.bold(),
            ":".blue(),
            self.position.start.line,
            ":".blue(),
            self.position.start.column,
            ":".blue(),
            self.name.red().bold(),
            self.description
        )
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn display_fmt() {
        let path = Path::new("file.md").to_path_buf();
        let position = Position::new(0, 1, 2, 3, 4, 5);
        let violation = Violation::new(
            path,
            "name".to_string(),
            "description".to_string(),
            position,
        );
        let actual = violation.to_string();
        let expected = "\u{1b}[1mfile.md\u{1b}[0m\u{1b}[34m:\u{1b}[0m0\u{1b}[34m:\u{1b}[0m1\u{1b}[34m:\u{1b}[0m \u{1b}[1;31mname\u{1b}[0m description";
        assert_eq!(actual, expected);
    }
}
