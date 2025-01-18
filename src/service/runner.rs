extern crate alloc;

use alloc::sync::Arc;
use std::sync::{mpsc, Mutex};
use std::thread;

use ignore::WalkParallel;
use miette::miette;
use miette::{IntoDiagnostic as _, Result};

use super::visitor::MarkdownLintVisitorFactory;
use crate::config::Config;
use crate::Violation;

pub struct ParallelLintRunner {
    walker: WalkParallel,
    config: Config,
    capacity: usize,
}

impl ParallelLintRunner {
    #[inline]
    #[must_use]
    pub const fn new(walker: WalkParallel, config: Config, capacity: usize) -> Self {
        Self {
            walker,
            config,
            capacity,
        }
    }

    #[inline]
    // TODO: Don't use expect
    #[expect(clippy::expect_used)]
    #[expect(clippy::unwrap_in_result)]
    pub fn run(self) -> Result<Vec<Violation>> {
        let mutex_violations: Arc<Mutex<Vec<Violation>>> = Arc::new(Mutex::new(vec![]));
        let (tx, rx) = mpsc::sync_channel::<Vec<Violation>>(self.capacity);

        let local_mutex_violations = Arc::clone(&mutex_violations);
        let thread = thread::spawn(move || {
            for violations in rx {
                let mut acquired_violations = local_mutex_violations
                    .lock()
                    .expect("lock must be acquired");
                acquired_violations.extend(violations);
            }
        });

        let mut builder = MarkdownLintVisitorFactory::new(self.config, tx);
        self.walker.visit(&mut builder);

        // Wait for the completion
        drop(builder);
        thread
            .join()
            .map_err(|err| miette!("Failed to join thread. {:?}", err))?;

        // Take ownership of violations
        let lock =
            Arc::into_inner(mutex_violations).ok_or_else(|| miette!("Failed to unwrap Arc"))?;
        lock.into_inner().into_diagnostic()
    }
}
