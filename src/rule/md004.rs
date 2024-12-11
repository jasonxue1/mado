use comrak::nodes::{ListType, NodeList, NodeValue};
use miette::Result;

use crate::violation::Violation;
use crate::Document;

use super::Rule;

pub enum ListStyle {
    Consistent,
    Asterisk,
    Plus,
    Dash,
    // Sublist, // TODO
}

pub struct MD004 {
    style: ListStyle,
}

impl MD004 {
    #[inline]
    pub fn new(style: ListStyle) -> Self {
        Self { style }
    }
}

impl Default for MD004 {
    #[inline]
    fn default() -> Self {
        Self {
            style: ListStyle::Consistent,
        }
    }
}

impl Rule for MD004 {
    #[inline]
    fn name(&self) -> String {
        "MD004".to_string()
    }

    #[inline]
    fn description(&self) -> String {
        "Unordered list style".to_string()
    }

    #[inline]
    fn tags(&self) -> Vec<String> {
        vec!["bullet".to_string(), "ul".to_string()]
    }

    #[inline]
    fn aliases(&self) -> Vec<String> {
        vec!["ul-style".to_string()]
    }

    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        let mut violations = vec![];
        let mut maybe_first_list_style = None;

        for node in doc.ast.descendants() {
            if let NodeValue::Item(
                item @ NodeList {
                    list_type: ListType::Bullet,
                    ..
                },
            ) = node.data.borrow().value
            {
                match self.style {
                    ListStyle::Consistent => match maybe_first_list_style {
                        Some(first_list_style) => {
                            if item.bullet_char != first_list_style {
                                let position = node.data.borrow().sourcepos;
                                let violation = self.to_violation(doc.path.clone(), position);
                                violations.push(violation);
                            }
                        }
                        None => maybe_first_list_style = Some(item.bullet_char),
                    },
                    ListStyle::Asterisk if item.bullet_char != b'*' => {
                        let position = node.data.borrow().sourcepos;
                        let violation = self.to_violation(doc.path.clone(), position);
                        violations.push(violation);
                    }
                    ListStyle::Plus if item.bullet_char != b'+' => {
                        let position = node.data.borrow().sourcepos;
                        let violation = self.to_violation(doc.path.clone(), position);
                        violations.push(violation);
                    }
                    ListStyle::Dash if item.bullet_char != b'-' => {
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
    fn check_errors_for_consistent() {
        let text = "* Item 1
+ Item 2
- Item 3";
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text: text.to_string(),
        };
        let rule = MD004::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![
            rule.to_violation(path.clone(), Sourcepos::from((2, 1, 2, 8))),
            rule.to_violation(path, Sourcepos::from((3, 1, 3, 8))),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_for_consistent() {
        let text = "* Item 1
* Item 2
* Item 3";
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, text, &Options::default());
        let doc = Document {
            path,
            ast,
            text: text.to_string(),
        };
        let rule = MD004::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_for_asterisk() {
        let text = "* Item 1
* Item 2
* Item 3";
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, text, &Options::default());
        let doc = Document {
            path,
            ast,
            text: text.to_string(),
        };
        let rule = MD004::new(ListStyle::Asterisk);
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_for_plus() {
        let text = "+ Item 1
+ Item 2
+ Item 3";
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, text, &Options::default());
        let doc = Document {
            path,
            ast,
            text: text.to_string(),
        };
        let rule = MD004::new(ListStyle::Plus);
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_for_dash() {
        let text = "- Item 1
- Item 2
- Item 3";
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, text, &Options::default());
        let doc = Document {
            path,
            ast,
            text: text.to_string(),
        };
        let rule = MD004::new(ListStyle::Dash);
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}
