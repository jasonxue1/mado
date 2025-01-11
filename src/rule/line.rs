use std::path::PathBuf;

use miette::Result;

use crate::Violation;

use super::Rule as _;
use super::{Matcher, MD009, MD010, MD013};

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

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum LineRule {
    MD009(MD009),
    MD010(MD010),
    MD013(MD013),
}

impl LineRule {
    #[inline]
    pub fn run(&mut self, ctx: &LineContext, line: &str) -> Result<Vec<Violation>> {
        match self {
            LineRule::MD009(rule) => rule.run(ctx, line),
            LineRule::MD010(rule) => rule.run(ctx, line),
            LineRule::MD013(rule) => rule.run(ctx, line),
        }
    }

    #[inline]
    #[must_use]
    pub fn matcher(&self) -> LineMatcher {
        match self {
            LineRule::MD009(rule) => rule.matcher(),
            LineRule::MD010(rule) => rule.matcher(),
            LineRule::MD013(rule) => rule.matcher(),
        }
    }

    #[inline]
    pub fn reset(&mut self) {
        match self {
            LineRule::MD009(rule) => rule.reset(),
            LineRule::MD010(rule) => rule.reset(),
            LineRule::MD013(rule) => rule.reset(),
        }
    }
}
