use std::fs;
use std::path::{Path, PathBuf};

use comrak::nodes::{AstNode, NodeValue};
use comrak::{parse_document, Arena, Options};
use miette::IntoDiagnostic as _;
use miette::Result;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Document<'a> {
    pub path: PathBuf,
    pub ast: &'a AstNode<'a>,
    pub text: String,
}

impl<'a> Document<'a> {
    #[inline]
    pub fn open(arena: &'a Arena<AstNode<'a>>, path: &Path) -> Result<Self> {
        let text = fs::read_to_string(path).into_diagnostic()?;
        let mut options = Options::default();
        options.extension.front_matter_delimiter = Some("---".to_owned());
        options.extension.table = true;
        let ast = parse_document(arena, &text, &options);

        Ok(Self {
            path: path.to_path_buf(),
            ast,
            text,
        })
    }

    #[inline]
    #[must_use]
    pub fn front_matter(&self) -> Option<String> {
        if let Some(node) = self.ast.first_child() {
            if let NodeValue::FrontMatter(front_matter) = node.data.borrow().value.clone() {
                return Some(front_matter);
            }
        }

        None
    }
}
