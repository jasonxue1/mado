use markdown::mdast::Heading;
use markdown::mdast::Node;
use miette::Result;

use crate::violation::Violation;
use crate::Document;

use super::Rule;

pub struct MD002 {
    depth: u8,
}

impl MD002 {
    #[inline]
    pub fn new(depth: u8) -> Self {
        Self { depth }
    }
}

impl Default for MD002 {
    #[inline]
    fn default() -> Self {
        Self { depth: 1 }
    }
}

impl Rule for MD002 {
    #[inline]
    fn name(&self) -> String {
        "MD002".to_string()
    }

    #[inline]
    fn description(&self) -> String {
        "First header should be a top level header".to_string()
    }

    #[inline]
    fn tags(&self) -> Vec<String> {
        vec!["headers".to_string()]
    }

    #[inline]
    fn aliases(&self) -> Vec<String> {
        vec!["first-header-h1".to_string()]
    }

    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        match doc.ast.children() {
            Some(children) => {
                let maybe_first_heading = children.iter().find_map(|node| match node {
                    Node::Heading(heading) => Some(heading),
                    _ => None,
                });

                match maybe_first_heading {
                    Some(Heading {
                        depth, position, ..
                    }) if *depth != self.depth => Ok(vec![self.to_violation(
                        doc.path.clone(),
                        position.clone().expect("heading must have position"),
                    )]),
                    _ => Ok(vec![]),
                }
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
        let text = "## This isn't a H1 header

### Another header";
        let path = Path::new("test.md").to_path_buf();
        let ast = markdown::to_mdast(text, &ParseOptions::default()).unwrap();
        let doc = Document {
            path: path.clone(),
            ast,
            text: text.to_string(),
        };
        let rule = MD002::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(path, Position::new(1, 1, 0, 1, 26, 25))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors() {
        let text = "# Start with a H1 header

## Then use a H2 for subsections";
        let path = Path::new("test.md").to_path_buf();
        let ast = markdown::to_mdast(text, &ParseOptions::default()).unwrap();
        let doc = Document {
            path,
            ast,
            text: text.to_string(),
        };
        let rule = MD002::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}
