use crossbeam_channel::Sender;
use ignore::{ParallelVisitor, ParallelVisitorBuilder, WalkState};

use crate::{Linter, Violation};

pub struct MarkdownLintVisitor {
    linter: Linter,
    tx: Sender<Vec<Violation>>,
}

impl MarkdownLintVisitor {
    pub fn new(linter: Linter, tx: Sender<Vec<Violation>>) -> Self {
        Self { linter, tx }
    }
}

impl ParallelVisitor for MarkdownLintVisitor {
    fn visit(
        &mut self,
        either_entry: std::result::Result<ignore::DirEntry, ignore::Error>,
    ) -> WalkState {
        let entry = either_entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.extension() == Some("md".as_ref()) {
            let violations = self.linter.check(path).unwrap();
            self.tx.send(violations).unwrap();
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
