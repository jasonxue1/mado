use std::path::PathBuf;

use super::Matcher;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct LineMatcher {
    pred: fn(&str) -> bool,
}

impl LineMatcher {
    #[inline]
    #[must_use]
    pub fn new(pred: fn(&str) -> bool) -> Self {
        Self { pred }
    }
}

impl Matcher<&str> for LineMatcher {
    #[inline]
    #[must_use]
    fn is_match(&self, line: &str) -> bool {
        (self.pred)(line)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::exhaustive_structs)]
pub struct LineContext {
    pub path: PathBuf,
    pub lineno: usize,
}
