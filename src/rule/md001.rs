use markdown::mdast::Node;
use miette::Result;

use crate::{violation::Violation, Document};

use super::Rule;

#[derive(Default)]
pub struct MD001 {}

impl MD001 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Rule for MD001 {
    #[inline]
    fn name(&self) -> String {
        "MD001".to_string()
    }

    #[inline]
    fn description(&self) -> String {
        "Header levels should only increment by one level at a time".to_string()
    }

    #[inline]
    fn tags(&self) -> Vec<String> {
        vec!["headers".to_string()]
    }

    #[inline]
    fn aliases(&self) -> Vec<String> {
        vec!["header-increment".to_string()]
    }

    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        match doc.ast.children() {
            Some(children) => {
                let (violations, _) = children.iter().fold(
                    (vec![], None),
                    |(mut acc, maybe_old_depth), node| match (node, maybe_old_depth) {
                        (Node::Heading(heading), Some(old_depth))
                            if heading.depth > old_depth + 1 =>
                        {
                            let violation = self.to_violation(
                                doc.path.clone(),
                                heading
                                    .position
                                    .clone()
                                    .expect("heading must have position"),
                            );
                            acc.push(violation);

                            (acc, Some(heading.depth))
                        }
                        (Node::Heading(heading), _) => (acc, Some(heading.depth)),
                        _ => (acc, maybe_old_depth),
                    },
                );
                Ok(violations)
            }
            None => Ok(vec![]),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use markdown::{unist::Position, ParseOptions};

    use super::*;

    #[test]
    fn check_errors() {
        let text = "# Header 1

### Header 3

We skipped out a 2nd level header in this document";
        let path = Path::new("test.md").to_path_buf();
        let ast = markdown::to_mdast(text, &ParseOptions::default()).unwrap();
        let doc = Document {
            path: path.clone(),
            ast,
            text: text.to_string(),
        };
        let rule = MD001::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(path, Position::new(3, 1, 12, 3, 13, 24))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors() {
        let text = "# Header 1

## Header 2

### Header 3

#### Header 4

## Another Header 2

### Another Header 3";
        let path = Path::new("test.md").to_path_buf();
        let ast = markdown::to_mdast(text, &ParseOptions::default()).unwrap();
        let doc = Document {
            path,
            ast,
            text: text.to_string(),
        };
        let rule = MD001::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_no_top_level() {
        let text = "## This isn't a H1 header";
        let path = Path::new("test.md").to_path_buf();
        let ast = markdown::to_mdast(text, &ParseOptions::default()).unwrap();
        let doc = Document {
            path,
            ast,
            text: text.to_string(),
        };
        let rule = MD001::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}
