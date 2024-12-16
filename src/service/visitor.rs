use core::result::Result;

use comrak::Arena;
use crossbeam_channel::Sender;
use ignore::{ParallelVisitor, ParallelVisitorBuilder, WalkState};

use super::Linter;
use crate::{Document, Violation};

pub struct MarkdownLintVisitor {
    linter: Linter,
    tx: Sender<Vec<Violation>>,
}

impl MarkdownLintVisitor {
    #[inline]
    #[must_use]
    pub fn new(linter: Linter, tx: Sender<Vec<Violation>>) -> Self {
        Self { linter, tx }
    }
}

impl ParallelVisitor for MarkdownLintVisitor {
    // TODO: Don't use unwrap
    #![allow(clippy::unwrap_used)]
    fn visit(&mut self, either_entry: Result<ignore::DirEntry, ignore::Error>) -> WalkState {
        match either_entry {
            Ok(entry) => {
                // TODO: Handle errors
                let path = entry.path();
                if path.is_file() && path.extension() == Some("md".as_ref()) {
                    let arena = Arena::new();
                    let either_doc = Document::open(&arena, path);
                    match either_doc {
                        Ok(doc) => {
                            let violations = self.linter.check(&doc).unwrap();
                            self.tx.send(violations).unwrap();
                        }
                        Err(err) => println!("{err}"),
                    }
                }
            }
            Err(err) => println!("{err}"),
        }

        WalkState::Continue
    }
}

pub struct MarkdownLintVisitorFactory {
    tx: Sender<Vec<Violation>>,
}

impl MarkdownLintVisitorFactory {
    pub fn new(tx: Sender<Vec<Violation>>) -> Self {
        Self { tx }
    }
}

impl<'s> ParallelVisitorBuilder<'s> for MarkdownLintVisitorFactory {
    fn build(&mut self) -> Box<dyn ParallelVisitor + 's> {
        let linter = Linter::new();
        Box::new(MarkdownLintVisitor::new(linter, self.tx.clone()))
    }
}
