use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use markdown::unist::Position;

use crate::rule;
use crate::Rule;

#[derive(Default)]
pub struct Linter {
    rules: Vec<Box<dyn Rule>>,
}

impl Linter {
    pub fn new() -> Self {
        Self {
            rules: vec![Box::new(rule::MD001::new()), Box::new(rule::MD022::new())],
        }
    }

    pub fn check(&self, path: &Path) -> Vec<Position> {
        if path.is_file() && path.extension() == Some(OsStr::new("md")) {
            // TODO: Don't use unwrap
            let text = &fs::read_to_string(path).unwrap();
            let doc = markdown::to_mdast(text, &markdown::ParseOptions::default()).unwrap();
            self.rules
                .iter()
                .flat_map(|rule| rule.check(&doc))
                .collect()
        } else if path.is_dir() {
            // TODO: Don't use unwrap
            let files = fs::read_dir(path).unwrap();
            files
                .flat_map(|file| self.check(&file.unwrap().path()))
                .collect()
        } else {
            // TODO
            vec![]
        }
    }
}
