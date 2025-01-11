use std::path::PathBuf;

use comrak::nodes::{AstNode, NodeValue};

use super::Matcher;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct NodeValueMatcher {
    pred: fn(&NodeValue) -> bool,
}

impl NodeValueMatcher {
    #[inline]
    #[must_use]
    pub fn new(pred: fn(&NodeValue) -> bool) -> Self {
        Self { pred }
    }
}

impl<'a> Matcher<&'a AstNode<'a>> for NodeValueMatcher {
    #[inline]
    #[must_use]
    fn is_match(&self, node: &'a AstNode<'a>) -> bool {
        (self.pred)(&node.data.borrow().value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::exhaustive_structs)]
pub struct NodeContext {
    pub path: PathBuf,
    pub level: usize,
    pub list_level: Option<usize>,
}
