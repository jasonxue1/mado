use comrak::nodes::Sourcepos;
use miette::IntoDiagnostic as _;
use miette::Result;
use regex::Regex;

use crate::{violation::Violation, Document};

use super::Rule;

#[derive(Default)]
#[non_exhaustive]
pub struct MD009;

impl MD009 {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Rule for MD009 {
    #[inline]
    fn name(&self) -> String {
        "MD009".to_owned()
    }

    #[inline]
    fn description(&self) -> String {
        "Trailing spaces".to_owned()
    }

    #[inline]
    fn tags(&self) -> Vec<String> {
        vec!["whitespace".to_owned()]
    }

    #[inline]
    fn aliases(&self) -> Vec<String> {
        vec!["no-trailing-spaces".to_owned()]
    }

    #[inline]
    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        let re = Regex::new(" +$").into_diagnostic()?;

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
        let text = "Text with a trailing space 
And text with some trailing spaces   ";
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text: text.to_string(),
        };
        let rule = MD009::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![
            rule.to_violation(path.clone(), Sourcepos::from((1, 26, 1, 26))),
            rule.to_violation(path, Sourcepos::from((2, 34, 2, 36))),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors() {
        let text = "Text with no trailing spaces";
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, text, &Options::default());
        let doc = Document {
            path,
            ast,
            text: text.to_string(),
        };
        let rule = MD009::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}
