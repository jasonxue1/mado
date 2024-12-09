use std::path::PathBuf;

use ignore::WalkBuilder;
use ignore::WalkParallel;
use miette::miette;
use miette::Result;

pub struct MarkdownWalker {
    pub walker: WalkParallel,
}

impl MarkdownWalker {
    pub fn new(files: &[PathBuf]) -> Result<Self> {
        let (head, tail_files) = files
            .split_first()
            .ok_or(miette!("files must be non-empty"))?;
        let mut builder = WalkBuilder::new(head);
        for file in tail_files {
            builder.add(file);
        }

        Ok(Self {
            walker: builder.build_parallel(),
        })
    }
}
