use std::path::PathBuf;

use ignore::DirEntry;
use ignore::Error;
use ignore::Walk;
use ignore::WalkBuilder;

pub struct MarkdownWalker {
    // TODO: Use WalkParallel
    walker: Walk,
}

impl MarkdownWalker {
    pub fn new(files: &[PathBuf]) -> Self {
        let (head, tail_files) = files.split_first().expect("files must be non-empty");
        let mut builder = WalkBuilder::new(head);
        for file in tail_files {
            builder.add(file);
        }

        Self {
            walker: builder.build(),
        }
    }
}

impl Iterator for MarkdownWalker {
    type Item = Result<DirEntry, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let maybe_entry = self.walker.next();
            match maybe_entry {
                Some(Ok(entry)) => {
                    let path = entry.path();
                    if path.is_file() && path.extension() == Some("md".as_ref()) {
                        return Some(Ok(entry));
                    }
                }
                other => return other,
            }
        }
    }
}
