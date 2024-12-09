use std::path::PathBuf;
use std::process;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use ignore::WalkState;
use miette::miette;
use miette::IntoDiagnostic;
use miette::Result;

use crate::Linter;
use crate::MarkdownWalker;
use crate::Violation;

pub struct Checker {
    walker: MarkdownWalker,
}

impl Checker {
    pub fn new(files: &[PathBuf]) -> Result<Self> {
        let walker = MarkdownWalker::new(files)?;

        Ok(Self { walker })
    }

    fn collect_violations(self) -> Result<Vec<Violation>> {
        let mutex_violations: Arc<Mutex<Vec<Violation>>> = Arc::new(Mutex::new(vec![]));
        let (tx, rx) = crossbeam_channel::bounded::<Vec<Violation>>(100);

        let local_mutex_violations = mutex_violations.clone();
        let thread = thread::spawn(move || {
            for violations in rx {
                let mut acquired_violations = local_mutex_violations
                    .lock()
                    .expect("lock must be acquired");
                acquired_violations.extend(violations);
            }
        });

        self.walker.walker.run(|| {
            let linter = Linter::new();
            let tx = tx.clone();
            Box::new(move |either_entry| {
                // TODO: Handler errors
                let entry = either_entry.unwrap();
                let path = entry.path();
                if path.is_file() && path.extension() == Some("md".as_ref()) {
                    let violations = linter.check(path).unwrap();
                    tx.send(violations).unwrap();
                }

                WalkState::Continue
            })
        });

        // Wait for the completion
        drop(tx);
        thread.join().unwrap();

        // Take ownership of violations
        let lock = Arc::into_inner(mutex_violations).ok_or(miette!("Failed to unwrap Arc"))?;
        lock.into_inner().into_diagnostic()
    }

    pub fn check(self) -> Result<()> {
        let violations = self.collect_violations()?;
        if violations.is_empty() {
            println!("All checks passed!");
            return Ok(());
        }

        let num_violations = violations.len();
        for violation in violations {
            println!("{violation}");
        }

        if num_violations == 1 {
            println!("\nFound 1 error.");
        } else {
            println!("\nFound {} errors.", num_violations);
        }

        process::exit(1);
    }
}
