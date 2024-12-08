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

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn iterator_next() {
        // TODO: Use stub or temporary files
        let walker = MarkdownWalker::new(&[Path::new(".").to_path_buf()]);
        let actual: Vec<String> = walker
            .into_iter()
            .filter_map(|either_entry| match either_entry {
                Ok(entry) => entry.path().to_str().map(|s| s.to_string()),
                Err(_) => None,
            })
            .collect();
        let expected = vec!["./README.md"];
        assert_eq!(actual, expected);
    }
}
