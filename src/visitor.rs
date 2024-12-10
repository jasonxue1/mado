use crossbeam_channel::Sender;
use ignore::{ParallelVisitor, ParallelVisitorBuilder, WalkState};

use crate::{Linter, Violation};

pub struct MarkdownLintVisitor {
    linter: Linter,
    maybe_tx: Option<Sender<Vec<Violation>>>,
}

impl MarkdownLintVisitor {
    pub fn new(linter: Linter, maybe_tx: Option<Sender<Vec<Violation>>>) -> Self {
        Self { linter, maybe_tx }
    }
}

impl ParallelVisitor for MarkdownLintVisitor {
    fn visit(
        &mut self,
        either_entry: std::result::Result<ignore::DirEntry, ignore::Error>,
    ) -> WalkState {
        match &self.maybe_tx {
            Some(tx) => {
                let entry = either_entry.unwrap();
                let path = entry.path();
                if path.is_file() && path.extension() == Some("md".as_ref()) {
                    let violations = self.linter.check(path).unwrap();
                    tx.send(violations).unwrap();
                }

                WalkState::Continue
            }
            None => {
                println!("Failed to get tx. Quitting...");
                WalkState::Quit
            }
        }
    }
}

impl Drop for MarkdownLintVisitor {
    fn drop(&mut self) {
        self.maybe_tx = None;
    }
}

pub struct MarkdownLintVisitorFactory {
    maybe_tx: Option<Sender<Vec<Violation>>>,
}

impl MarkdownLintVisitorFactory {
    pub fn new(maybe_tx: Option<Sender<Vec<Violation>>>) -> Self {
        Self { maybe_tx }
    }
}

impl<'s> ParallelVisitorBuilder<'s> for MarkdownLintVisitorFactory {
    fn build(&mut self) -> Box<dyn ParallelVisitor + 's> {
        let linter = Linter::new();
        Box::new(MarkdownLintVisitor::new(linter, self.maybe_tx.clone()))
    }
}

impl Drop for MarkdownLintVisitorFactory {
    fn drop(&mut self) {
        self.maybe_tx = None;
    }
}
