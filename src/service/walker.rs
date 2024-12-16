use std::path::PathBuf;

use ignore::WalkBuilder;
use ignore::WalkParallel;
use miette::miette;
use miette::Result;

#[non_exhaustive]
pub struct MarkdownWalker {
    // TODO: Don't export walker
    pub walker: WalkParallel,
}

impl MarkdownWalker {
    #[inline]
    pub fn new(patterns: &[PathBuf]) -> Result<Self> {
        let (head_pattern, tail_patterns) = patterns
            .split_first()
            .ok_or(miette!("files must be non-empty"))?;
        let mut builder = WalkBuilder::new(head_pattern);
        for pattern in tail_patterns {
            builder.add(pattern);
        }

        Ok(Self {
            walker: builder.build_parallel(),
        })
    }
}
