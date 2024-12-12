use core::cmp::Ordering;
use core::fmt::{Display, Error, Formatter, Result};
use std::path::PathBuf;

use colored::Colorize as _;
use comrak::nodes::Sourcepos;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Violation {
    path: PathBuf,
    name: String,
    description: String,
    position: Sourcepos,
}

impl Violation {
    #[inline]
    #[must_use]
    pub fn new(path: PathBuf, name: String, description: String, position: Sourcepos) -> Self {
        Self {
            path,
            name,
            description,
            position,
        }
    }

    #[inline]
    #[must_use]
    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }

    #[inline]
    #[must_use]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[inline]
    #[must_use]
    pub fn description(&self) -> String {
        self.description.clone()
    }

    #[inline]
    #[must_use]
    pub fn position(&self) -> Sourcepos {
        self.position
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

impl Display for Violation {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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
        let position = Sourcepos::from((0, 1, 3, 5));
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
