use core::cmp::Ordering;
use core::fmt::{Display, Error, Formatter, Result};

use colored::Colorize as _;

use crate::Violation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Markdownlint<'a> {
    violation: &'a Violation,
}

impl<'a> Markdownlint<'a> {
    pub fn new(violation: &'a Violation) -> Self {
        Self { violation }
    }
}

impl Display for Markdownlint<'_> {
    // TODO: Add `expected` and `actual`
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let path = self.violation.path().to_str().ok_or(Error)?;
        write!(
            f,
            "{}",
            format!(
                "{}:{}:{} {}/{} {}",
                path,
                self.violation.position().start.line,
                self.violation.position().start.column,
                self.violation.name(),
                self.violation.alias(),
                self.violation.description()
            )
            .red()
        )
    }
}

impl PartialOrd for Markdownlint<'_> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Markdownlint<'_> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.violation.cmp(other.violation)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use comrak::nodes::Sourcepos;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn display_fmt() {
        let path = Path::new("file.md").to_path_buf();
        let position = Sourcepos::from((0, 1, 3, 5));
        let violation = Violation::new(
            path,
            "name".to_owned(),
            "alias".to_owned(),
            "description".to_owned(),
            position,
        );
        let actual = Markdownlint::new(&violation).to_string();
        let expected = "\u{1b}[31mfile.md:0:1 name/alias description\u{1b}[0m";
        assert_eq!(actual, expected);
    }
}
