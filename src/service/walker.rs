use std::path::PathBuf;

use ignore::WalkBuilder;
use ignore::WalkParallel;
use miette::miette;
use miette::Result;

#[non_exhaustive]
pub struct WalkParallelBuilder;

impl WalkParallelBuilder {
    #[inline]
    pub fn build(patterns: &[PathBuf]) -> Result<WalkParallel> {
        let (head_pattern, tail_patterns) = patterns
            .split_first()
            .ok_or(miette!("files must be non-empty"))?;
        let mut builder = WalkBuilder::new(head_pattern);
        for pattern in tail_patterns {
            builder.add(pattern);
        }

        Ok(builder.build_parallel())
    }
}
