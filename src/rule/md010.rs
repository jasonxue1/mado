use comrak::nodes::Sourcepos;
use miette::IntoDiagnostic as _;
use miette::Result;
use regex::Regex;

use crate::violation::Violation;
use crate::Document;

use super::Rule;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct MD010;

impl MD010 {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Rule for MD010 {
    #[inline]
    fn name(&self) -> String {
        "MD010".to_owned()
    }

    #[inline]
    fn description(&self) -> String {
        "Hard tabs".to_owned()
    }

    #[inline]
    fn tags(&self) -> Vec<String> {
        vec!["whitespace".to_owned(), "hard_tab".to_owned()]
    }

    #[inline]
    fn aliases(&self) -> Vec<String> {
        vec!["no-hard-tabs".to_owned()]
    }

    #[inline]
    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        let re = Regex::new("\t").into_diagnostic()?;

        let mut violations = vec![];
        for (i, line) in doc.text.lines().enumerate() {
            let lineno = i + 1;
            let mut locs = re.capture_locations();
            re.captures_read(&mut locs, line);
            if let Some((start_column, end_column)) = locs.get(0) {
                let position = Sourcepos::from((lineno, start_column, lineno, end_column - 1));
                let violation = self.to_violation(doc.path.clone(), position);
                violations.push(violation);
            }
        }

        Ok(violations)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use comrak::{parse_document, Arena, Options};

    use super::*;

    #[test]
    fn check_errors() {
        let text = "Some text

	* hard tab character used to indent the list item";
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text: text.to_string(),
        };
        let rule = MD010::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(path, Sourcepos::from((3, 0, 3, 0)))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors() {
        let text = "Some text

    * Spaces used to indent the list item instead";
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, text, &Options::default());
        let doc = Document {
            path,
            ast,
            text: text.to_string(),
        };
        let rule = MD010::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}
