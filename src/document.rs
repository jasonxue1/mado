use std::fs;
use std::path::{Path, PathBuf};

use markdown::mdast::Node;
use markdown::ParseOptions;
use miette::miette;
use miette::IntoDiagnostic;
use miette::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    pub path: PathBuf,
    pub ast: Node,
    pub text: String,
}

impl Document {
    pub fn open(path: &Path) -> Result<Self> {
        let text = fs::read_to_string(path).into_diagnostic()?;
        let ast =
            markdown::to_mdast(&text, &ParseOptions::default()).map_err(|err| miette!(err))?;
        Ok(Self {
            path: path.to_path_buf(),
            ast,
            text,
        })
    }
}
