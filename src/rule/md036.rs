use comrak::nodes::NodeValue;
use miette::Result;

use crate::{violation::Violation, Document};

use super::RuleLike;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct MD036 {
    punctuation: String,
}

impl MD036 {
    #[inline]
    #[must_use]
    pub fn new(punctuation: String) -> Self {
        Self { punctuation }
    }
}

impl Default for MD036 {
    #[inline]
    fn default() -> Self {
        Self {
            punctuation: ".,;:!?".to_owned(),
        }
    }
}

impl RuleLike for MD036 {
    #[inline]
    fn name(&self) -> String {
        "MD036".to_owned()
    }

    #[inline]
    fn description(&self) -> String {
        "Emphasis used instead of a header".to_owned()
    }

    #[inline]
    fn tags(&self) -> Vec<String> {
        vec!["headers".to_owned(), "emphasis".to_owned()]
    }

    #[inline]
    fn aliases(&self) -> Vec<String> {
        vec!["no-emphasis-as-header".to_owned()]
    }

    #[inline]
    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        let mut violations = vec![];

        for node in doc.ast.descendants() {
            if let NodeValue::Emph | NodeValue::Strong = &node.data.borrow().value {
                for child_node in node.children() {
                    if let NodeValue::Text(text) = &child_node.data.borrow().value {
                        if let Some(last_char) = text.chars().last() {
                            let position = node.data.borrow().sourcepos;
                            if !self.punctuation.contains(last_char) && position.start.column == 1 {
                                let violation = self.to_violation(doc.path.clone(), position);
                                violations.push(violation);
                            }
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
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn check_errors() {
        let text = "**My document**

Lorem ipsum dolor sit amet...

_Another section_

Consectetur adipiscing elit, sed do eiusmod."
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text,
        };
        let rule = MD036::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![
            rule.to_violation(path.clone(), Sourcepos::from((1, 1, 1, 15))),
            rule.to_violation(path, Sourcepos::from((5, 1, 5, 17))),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors() {
        let text = "# My document

Lorem ipsum dolor sit amet...

## Another section

Consectetur adipiscing elit, sed do eiusmod."
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD036::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_with_punctuation() {
        let text = "**My document.**

Lorem ipsum dolor sit amet...

_Another section?_

Consectetur adipiscing elit, sed do eiusmod."
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD036::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    // TODO: Handle this case
    //     #[test]
    //     fn check_no_errors_with_text() {
    //         let text = "foo **My document**
    //
    // _Another section_ bar"
    //             .to_owned();
    //         let path = Path::new("test.md").to_path_buf();
    //         let arena = Arena::new();
    //         let ast = parse_document(&arena, &text, &Options::default());
    //         let doc = Document { path, ast, text };
    //         let rule = MD036::default();
    //         let actual = rule.check(&doc).unwrap();
    //         let expected = vec![];
    //         assert_eq!(actual, expected);
    //     }

    #[test]
    fn check_no_errors_with_indent() {
        let text = " **My document**".to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD036::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_with_code() {
        let text = "**`My document`**".to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD036::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}
