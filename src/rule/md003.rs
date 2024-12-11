use comrak::nodes::NodeValue;
use miette::Result;

use crate::violation::Violation;
use crate::Document;

use super::Rule;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeadingStyle {
    Consistent,
    Atx,
    // AtxClosed, // TODO
    Setext,
    SetextWithAtx,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct MD003 {
    style: HeadingStyle,
}

impl MD003 {
    #[inline]
    #[must_use]
    pub fn new(style: HeadingStyle) -> Self {
        Self { style }
    }
}

impl Default for MD003 {
    #[inline]
    fn default() -> Self {
        Self {
            style: HeadingStyle::Consistent,
        }
    }
}

impl Rule for MD003 {
    #[inline]
    fn name(&self) -> String {
        "MD003".to_owned()
    }

    #[inline]
    fn description(&self) -> String {
        "Header style".to_owned()
    }

    #[inline]
    fn tags(&self) -> Vec<String> {
        vec!["headers".to_owned()]
    }

    #[inline]
    fn aliases(&self) -> Vec<String> {
        vec!["header-style".to_owned()]
    }

    #[inline]
    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        let mut violations = vec![];
        let mut maybe_first_heading_style = None;

        for node in doc.ast.children() {
            if let NodeValue::Heading(heading) = node.data.borrow().value {
                match self.style {
                    HeadingStyle::Consistent => match maybe_first_heading_style {
                        Some(first_heading_style) => {
                            if heading.setext != first_heading_style {
                                let position = node.data.borrow().sourcepos;
                                let violation = self.to_violation(doc.path.clone(), position);
                                violations.push(violation);
                            }
                        }
                        None => maybe_first_heading_style = Some(heading.setext),
                    },
                    HeadingStyle::Atx if heading.setext => {
                        let position = node.data.borrow().sourcepos;
                        let violation = self.to_violation(doc.path.clone(), position);
                        violations.push(violation);
                    }
                    HeadingStyle::Setext if !heading.setext => {
                        let position = node.data.borrow().sourcepos;
                        let violation = self.to_violation(doc.path.clone(), position);
                        violations.push(violation);
                    }
                    HeadingStyle::SetextWithAtx if heading.level < 3 && !heading.setext => {
                        let position = node.data.borrow().sourcepos;
                        let violation = self.to_violation(doc.path.clone(), position);
                        violations.push(violation);
                    }
                    _ => {}
                }
            }
        }

        Ok(violations)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use comrak::{nodes::Sourcepos, parse_document, Arena, Options};

    use super::*;

    #[test]
    fn check_errors() {
        let text = "# ATX style H1

## Closed ATX style H2 ##

Setext style H1
===============";
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text: text.to_string(),
        };
        let rule = MD003::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(path, Sourcepos::from((5, 1, 6, 15)))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_for_consistent() {
        let text = "# ATX style H1

## ATX style H2";
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, text, &Options::default());
        let doc = Document {
            path,
            ast,
            text: text.to_string(),
        };
        let rule = MD003::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_for_atx() {
        let text = "# ATX style H1

## ATX style H2";
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, text, &Options::default());
        let doc = Document {
            path,
            ast,
            text: text.to_string(),
        };
        let rule = MD003::new(HeadingStyle::Atx);
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_for_setext() {
        let text = "Setext style H1
===============

Setext style H2
---------------";
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, text, &Options::default());
        let doc = Document {
            path,
            ast,
            text: text.to_string(),
        };
        let rule = MD003::new(HeadingStyle::Setext);
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_for_setext_with_atx() {
        let text = "Setext style H1
===============

Setext style H2
---------------

### ATX style H3";
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, text, &Options::default());
        let doc = Document {
            path,
            ast,
            text: text.to_string(),
        };
        let rule = MD003::new(HeadingStyle::SetextWithAtx);
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_with_front_matter() {
        let text = r#"---
author: "John Smith"
---

# Header 1"#;
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let mut options = Options::default();
        options.extension.front_matter_delimiter = Some("---".to_owned());
        let ast = parse_document(&arena, text, &options);
        let doc = Document {
            path,
            ast,
            text: text.to_string(),
        };
        let rule = MD003::new(HeadingStyle::Consistent);
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}
