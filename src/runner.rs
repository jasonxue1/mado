use std::sync::{Arc, Mutex};
use std::thread;

use ignore::WalkParallel;
use miette::miette;
use miette::{IntoDiagnostic, Result};

use crate::visitor::MarkdownLintVisitorFactory;
use crate::Violation;

pub struct ParallelLintRunner {
    walker: WalkParallel,
    capacity: usize,
}

impl ParallelLintRunner {
    pub fn new(walker: WalkParallel, capacity: usize) -> Self {
        Self { walker, capacity }
    }

    pub fn run(self) -> Result<Vec<Violation>> {
        let mutex_violations: Arc<Mutex<Vec<Violation>>> = Arc::new(Mutex::new(vec![]));
        let (tx, rx) = crossbeam_channel::bounded::<Vec<Violation>>(self.capacity);

        let local_mutex_violations = mutex_violations.clone();
        let thread = thread::spawn(move || {
            for violations in rx {
                let mut acquired_violations = local_mutex_violations
                    .lock()
                    .expect("lock must be acquired");
                acquired_violations.extend(violations);
            }
        });

        let mut builder = MarkdownLintVisitorFactory::new(tx);
        self.walker.visit(&mut builder);

        // Wait for the completion
        drop(builder);
        thread
            .join()
            .map_err(|err| miette!("Failed to join thread. {:?}", err))?;

        // Take ownership of violations
        let lock = Arc::into_inner(mutex_violations).ok_or(miette!("Failed to unwrap Arc"))?;
        lock.into_inner().into_diagnostic()
    }
}
