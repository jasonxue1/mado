use comrak::nodes::NodeValue;
use miette::Result;

use crate::{violation::Violation, Document};

use super::Rule;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct MD014;

impl MD014 {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Rule for MD014 {
    #[inline]
    fn name(&self) -> String {
        "MD014".to_owned()
    }

    #[inline]
    fn description(&self) -> String {
        "Dollar signs used before commands without showing output".to_owned()
    }

    #[inline]
    fn tags(&self) -> Vec<String> {
        vec!["code".to_owned()]
    }

    #[inline]
    fn aliases(&self) -> Vec<String> {
        vec!["commands-show-output".to_owned()]
    }

    #[inline]
    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        let mut violations = vec![];

        for node in doc.ast.children() {
            if let NodeValue::CodeBlock(code) = node.data.borrow().value.clone() {
                let mut lines = code.literal.lines();
                if lines.all(|line| line.starts_with("$ ")) {
                    let position = node.data.borrow().sourcepos;
                    let violation = self.to_violation(doc.path.clone(), position);
                    violations.push(violation);
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
        let text = "```
$ ls
$ cat foo
$ less bar
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
        let rule = MD014::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(path, Sourcepos::from((1, 1, 5, 3)))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_no_dollars() {
        let text = "```
ls
cat foo
less bar
```"
        .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD014::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_with_showing_outputs() {
        let text = "```
$ ls
foo bar
$ cat foo
Hello world
$ cat bar
baz
```"
        .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD014::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}
