use markdown::mdast::Node;
use miette::Result;

use crate::{violation::Violation, Document};

use super::Rule;

#[derive(Default)]
pub struct MD022 {}

impl MD022 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Rule for MD022 {
    #[inline]
    fn name(&self) -> String {
        "MD022".to_string()
    }

    #[inline]
    fn description(&self) -> String {
        "Headers should be surrounded by blank lines".to_string()
    }

    #[inline]
    fn tags(&self) -> Vec<String> {
        vec!["headers".to_string(), "blank_lines".to_string()]
    }

    #[inline]
    fn aliases(&self) -> Vec<String> {
        vec!["blanks-around-headers".to_string()]
    }

    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        match doc.ast.children() {
            Some(children) => {
                let (violations, _) =
                    children
                        .iter()
                        .fold((vec![], None::<&Node>), |(mut acc, maybe_prev), node| {
                            match maybe_prev {
                                Some(prev) => {
                                    let prev_position =
                                        prev.position().expect("prev node must have position");
                                    let position =
                                        node.position().expect("node must have position");

                                    if let Node::Heading(_) = node {
                                        if position.start.line == prev_position.end.line + 1 {
                                            let violation = self
                                                .to_violation(doc.path.clone(), position.clone());
                                            acc.push(violation);
                                        }
                                    } else if let Node::Heading(_) = prev {
                                        if position.start.line == prev_position.end.line + 1 {
                                            let violation = self.to_violation(
                                                doc.path.clone(),
                                                prev_position.clone(),
                                            );
                                            acc.push(violation);
                                        }
                                    }

                                    (acc, Some(node))
                                }
                                None => (acc, Some(node)),
                            }
                        });
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
Some text

Some more text
## Header 2";
        let path = Path::new("test.md").to_path_buf();
        let ast = markdown::to_mdast(text, &ParseOptions::default()).unwrap();
        let doc = Document {
            path: path.clone(),
            ast,
            text: text.to_string(),
        };
        let rule = MD022::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![
            rule.to_violation(path.clone(), Position::new(1, 1, 0, 1, 11, 10)),
            rule.to_violation(path, Position::new(5, 1, 37, 5, 12, 48)),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors() {
        let text = "# Header 1

Some text

Some more text

## Header 2";
        let path = Path::new("test.md").to_path_buf();
        let ast = markdown::to_mdast(text, &ParseOptions::default()).unwrap();
        let doc = Document {
            path,
            ast,
            text: text.to_string(),
        };
        let rule = MD022::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}
