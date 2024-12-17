use core::fmt::{Display, Error, Formatter, Result};

use colored::Colorize as _;

use crate::Violation;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mdl {
    violation: Violation,
}

impl Mdl {
    pub fn new(violation: Violation) -> Self {
        Self { violation }
    }
}

impl Display for Mdl {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let path = self.violation.path().to_str().ok_or(Error)?;
        write!(
            f,
            "{}{}{}{} {} {}",
            path.bold(),
            ":".blue(),
            self.violation.position().start.line,
            ":".blue(),
            self.violation.name().red().bold(),
            self.violation.description()
        )
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use comrak::nodes::Sourcepos;

    use super::*;

    #[test]
    fn display_fmt() {
        let path = Path::new("file.md").to_path_buf();
        let position = Sourcepos::from((0, 1, 3, 5));
        let violation = Violation::new(
            path,
            "name".to_string(),
            "alias".to_string(),
            "description".to_string(),
            position,
        );
        let actual = Mdl::new(violation).to_string();
        let expected = "\u{1b}[1mfile.md\u{1b}[0m\u{1b}[34m:\u{1b}[0m0\u{1b}[34m:\u{1b}[0m \u{1b}[1;31mname\u{1b}[0m description";
        assert_eq!(actual, expected);
    }
}
