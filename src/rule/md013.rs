use comrak::nodes::Sourcepos;
use miette::IntoDiagnostic as _;
use miette::Result;
use regex::Regex;

use crate::{violation::Violation, Document};

use super::RuleLike;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct MD013 {
    line_length: usize,
}

impl MD013 {
    pub const DEFAULT_LINE_LENGTH: usize = 80;

    #[inline]
    #[must_use]
    pub fn new(line_length: usize) -> Self {
        Self { line_length }
    }
}

impl Default for MD013 {
    #[inline]
    fn default() -> Self {
        Self {
            line_length: Self::DEFAULT_LINE_LENGTH,
        }
    }
}

impl RuleLike for MD013 {
    #[inline]
    fn name(&self) -> String {
        "MD013".to_owned()
    }

    #[inline]
    fn description(&self) -> String {
        "Line length".to_owned()
    }

    #[inline]
    fn tags(&self) -> Vec<String> {
        vec!["line_length".to_owned()]
    }

    #[inline]
    fn aliases(&self) -> Vec<String> {
        vec!["line-length".to_owned()]
    }

    #[inline]
    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        let mut violations = vec![];
        let re = Regex::new(&format!(r#"^.{{{}}}.*\s"#, self.line_length)).into_diagnostic()?;

        for (i, line) in doc.text.lines().enumerate() {
            let lineno = i + 1;

            if re.is_match(line) {
                let position = Sourcepos::from((lineno, self.line_length + 1, lineno, line.len()));
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
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn check_errors() {
        let text = "
IF THIS LINE IS THE MAXIMUM LENGTH
This line is a violation because there are spaces beyond that length"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text,
        };
        let rule = MD013::new(34);
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(path, Sourcepos::from((3, 35, 3, 68)))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_errors_with_other_nodes() {
        let text = "
IF THIS LINE IS THE MAXIMUM LENGTH
This line is a violation because [there are spaces beyond that](https://example.com)
This line is a violation because `there are spaces beyond that`"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text,
        };
        let rule = MD013::new(34);
        let actual = rule.check(&doc).unwrap();
        let expected = vec![
            rule.to_violation(path.clone(), Sourcepos::from((3, 35, 3, 84))),
            rule.to_violation(path, Sourcepos::from((4, 35, 4, 63))),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_errors_table() {
        let text = "
IF THIS LINE IS THE MAXIMUM LENGTH
| foo | bar | baz | foo | bar | baz |
|-----|-----|-----|-----|-----|-----|
| foo | bar | baz | foo | bar | baz |"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text,
        };
        let rule = MD013::new(34);
        let actual = rule.check(&doc).unwrap();
        let expected = vec![
            rule.to_violation(path.clone(), Sourcepos::from((3, 35, 3, 37))),
            rule.to_violation(path, Sourcepos::from((5, 35, 5, 37))),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_errors_codeblock() {
        let text = "
IF THIS LINE IS THE MAXIMUM LENGTH

```ruby
puts 'This line is a violation because there are spaces beyond that length'
```"
        .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text,
        };
        let rule = MD013::new(34);
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(path, Sourcepos::from((5, 35, 5, 75)))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors() {
        let text = "
IF THIS LINE IS THE MAXIMUM LENGTH
This line is okay because there are-no-spaces-beyond-that-length
This-line-is-okay-because-there-are-no-spaces-anywhere-within"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD013::new(34);
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_with_other_nodes() {
        let text = "
IF THIS LINE IS THE MAXIMUM LENGTH
This line is okay because there [are-no-spaces-beyond-that-length](https://example.com)
This line is okay because there `are-no-spaces-beyond-that-length`"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD013::new(34);
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_table() {
        let text = "
IF THIS LINE IS THE MAXIMUM LENGTH
|foo|bar|baz|foo|bar|baz|foo|bar|baz|foo|bar|baz|foo|bar|baz|foo|bar|baz|foo|bar|baz|
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
|foo|bar|baz|foo|bar|baz|foo|bar|baz|foo|bar|baz|foo|bar|baz|foo|bar|baz|foo|bar|baz|"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD013::new(34);
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_codeblock() {
        let text = "
IF THIS LINE IS THE MAXIMUM LENGTH

```ruby
puts 'This line is okay because there-are-no-spaces-beyond-that-length'
puts 'This-line-is-okay-because-there-are-no-spaces-anywhere-within'
```"
        .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD013::new(34);
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}
