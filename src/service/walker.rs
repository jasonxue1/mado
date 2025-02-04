use std::path::PathBuf;

use ignore::WalkBuilder;
use ignore::WalkParallel;
use miette::miette;
use miette::Result;

#[non_exhaustive]
pub struct WalkParallelBuilder;

impl WalkParallelBuilder {
    #[inline]
    pub fn build(
        patterns: &[PathBuf],
        respect_ignore: bool,
        respect_gitignore: bool,
    ) -> Result<WalkParallel> {
        let (head_pattern, tail_patterns) = patterns
            .split_first()
            .ok_or_else(|| miette!("files must be non-empty"))?;
        let mut builder = WalkBuilder::new(head_pattern);
        for pattern in tail_patterns {
            builder.add(pattern);
        }

        builder.ignore(respect_ignore);
        builder.git_ignore(respect_gitignore);

        Ok(builder.build_parallel())
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use alloc::sync::Arc;
    use std::{path::Path, sync::Mutex};

    use ignore::{DirEntry, WalkState};
    use pretty_assertions::assert_eq;

    use super::WalkParallelBuilder;

    #[test]
    fn build() {
        let paths = vec![
            Path::new("action").to_path_buf(),
            Path::new("mado.toml").to_path_buf(),
            Path::new("README.md").to_path_buf(),
        ];
        let builder = WalkParallelBuilder::build(&paths, true, true).unwrap();
        let shared_paths = Arc::new(Mutex::new(vec![]));
        let walker = |either_entry: Result<DirEntry, _>| {
            if let Ok(entry) = either_entry {
                shared_paths.lock().unwrap().push(entry.into_path());
            }

            WalkState::Continue
        };
        builder.run(|| Box::new(walker));
        let mut actual = shared_paths.lock().unwrap().clone();
        actual.sort();
        let expected = vec![
            Path::new("README.md").to_path_buf(),
            Path::new("action").to_path_buf(),
            Path::new("action/entrypoint.sh").to_path_buf(),
            Path::new("mado.toml").to_path_buf(),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn build_empty_patterns() {
        let result = WalkParallelBuilder::build(&[], true, true);
        assert!(result.is_err());
    }
}
