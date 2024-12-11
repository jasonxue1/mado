use comrak::nodes::NodeValue;
use miette::Result;

use crate::violation::Violation;
use crate::Document;

use super::Rule;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct MD019;

impl MD019 {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Rule for MD019 {
    #[inline]
    fn name(&self) -> String {
        "MD019".to_owned()
    }

    #[inline]
    fn description(&self) -> String {
        "Multiple spaces after hash on atx style header".to_owned()
    }

    #[inline]
    fn tags(&self) -> Vec<String> {
        vec!["headers".to_owned(), "atx".to_owned(), "spaces".to_owned()]
    }

    #[inline]
    fn aliases(&self) -> Vec<String> {
        vec!["no-multiple-space-atx".to_owned()]
    }

    #[inline]
    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        let mut violations = vec![];

        for node in doc.ast.children() {
            if let NodeValue::Heading(heading) = node.data.borrow().value.clone() {
                if let Some(text_node) = node.first_child() {
                    if let NodeValue::Text(_) = text_node.data.borrow().value {
                        let heading_position = node.data.borrow().sourcepos;
                        let text_position = text_node.data.borrow().sourcepos;
                        let expected_text_offset =
                            heading_position.start.column + (heading.level as usize) + 1;
                        if !heading.setext && text_position.start.column > expected_text_offset {
                            let violation = self.to_violation(doc.path.clone(), heading_position);
                            violations.push(violation);
                        }
                    }
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
        let text = "#  Header 1

##  Header 2";
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text: text.to_string(),
        };
        let rule = MD019::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![
            rule.to_violation(path.clone(), Sourcepos::from((1, 1, 1, 11))),
            rule.to_violation(path, Sourcepos::from((3, 1, 3, 12))),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_for_multiple_children_nodes() {
        let text = "# Header with `code` and text";
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text: text.to_string(),
        };
        let rule = MD019::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors() {
        let text = "# Header 1

## Header 2";
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, text, &Options::default());
        let doc = Document {
            path,
            ast,
            text: text.to_string(),
        };
        let rule = MD019::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}
