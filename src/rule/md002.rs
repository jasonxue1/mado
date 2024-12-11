use comrak::nodes::NodeValue;
use miette::Result;

use crate::violation::Violation;
use crate::Document;

use super::Rule;

pub struct MD002 {
    depth: u8,
}

impl MD002 {
    #[inline]
    #[must_use]
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
        for node in doc.ast.children() {
            if let NodeValue::Heading(heading) = node.data.borrow().value {
                if heading.level != self.depth {
                    let position = node.data.borrow().sourcepos;
                    let violation = self.to_violation(doc.path.clone(), position);

                    return Ok(vec![violation]);
                }

                break;
            }
        }

        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use comrak::{nodes::Sourcepos, parse_document, Arena, Options};

    use super::*;

    #[test]
    fn check_errors() {
        let text = "## This isn't a H1 header

### Another header";
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text: text.to_string(),
        };
        let rule = MD002::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(path, Sourcepos::from((1, 1, 1, 25)))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors() {
        let text = "# Start with a H1 header

## Then use a H2 for subsections";
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, text, &Options::default());
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
