use std::path::PathBuf;

use comrak::nodes::{AstNode, NodeValue};
use miette::Result;

use super::Matcher;
use super::Rule as _;
use super::MD001;
use super::MD002;
use super::MD003;
use super::MD004;
use super::MD005;
use super::MD006;
use super::MD007;
use super::MD014;
use super::MD018;
use super::MD019;
use super::MD022;
use super::MD023;
use super::MD024;
use super::MD025;
use super::MD026;
use super::MD027;
use super::MD028;
use super::MD029;
use crate::Violation;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct NodeMatcher {
    pred: fn(&NodeValue) -> bool,
}

impl NodeMatcher {
    #[inline]
    #[must_use]
    pub fn new(pred: fn(&NodeValue) -> bool) -> Self {
        Self { pred }
    }
}

impl<'a> Matcher<&'a AstNode<'a>> for NodeMatcher {
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

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum NodeRule {
    MD001(MD001),
    MD002(MD002),
    MD003(MD003),
    MD004(MD004),
    MD005(MD005),
    MD006(MD006),
    MD007(MD007),
    MD014(MD014),
    MD018(MD018),
    MD019(MD019),
    MD022(MD022),
    MD023(MD023),
    MD024(MD024),
    MD025(MD025),
    MD026(MD026),
    MD027(MD027),
    MD028(MD028),
    MD029(MD029),
}

impl NodeRule {
    #[inline]
    pub fn run<'a>(&mut self, ctx: &NodeContext, node: &'a AstNode<'a>) -> Result<Vec<Violation>> {
        match self {
            NodeRule::MD001(rule) => rule.run(ctx, node),
            NodeRule::MD002(rule) => rule.run(ctx, node),
            NodeRule::MD003(rule) => rule.run(ctx, node),
            NodeRule::MD004(rule) => rule.run(ctx, node),
            NodeRule::MD005(rule) => rule.run(ctx, node),
            NodeRule::MD006(rule) => rule.run(ctx, node),
            NodeRule::MD007(rule) => rule.run(ctx, node),
            NodeRule::MD014(rule) => rule.run(ctx, node),
            NodeRule::MD018(rule) => rule.run(ctx, node),
            NodeRule::MD019(rule) => rule.run(ctx, node),
            NodeRule::MD022(rule) => rule.run(ctx, node),
            NodeRule::MD023(rule) => rule.run(ctx, node),
            NodeRule::MD024(rule) => rule.run(ctx, node),
            NodeRule::MD025(rule) => rule.run(ctx, node),
            NodeRule::MD026(rule) => rule.run(ctx, node),
            NodeRule::MD027(rule) => rule.run(ctx, node),
            NodeRule::MD028(rule) => rule.run(ctx, node),
            NodeRule::MD029(rule) => rule.run(ctx, node),
        }
    }

    #[inline]
    #[must_use]
    pub fn matcher(&self) -> NodeMatcher {
        match self {
            NodeRule::MD001(rule) => rule.matcher(),
            NodeRule::MD002(rule) => rule.matcher(),
            NodeRule::MD003(rule) => rule.matcher(),
            NodeRule::MD004(rule) => rule.matcher(),
            NodeRule::MD005(rule) => rule.matcher(),
            NodeRule::MD006(rule) => rule.matcher(),
            NodeRule::MD007(rule) => rule.matcher(),
            NodeRule::MD014(rule) => rule.matcher(),
            NodeRule::MD018(rule) => rule.matcher(),
            NodeRule::MD019(rule) => rule.matcher(),
            NodeRule::MD022(rule) => rule.matcher(),
            NodeRule::MD023(rule) => rule.matcher(),
            NodeRule::MD024(rule) => rule.matcher(),
            NodeRule::MD025(rule) => rule.matcher(),
            NodeRule::MD026(rule) => rule.matcher(),
            NodeRule::MD027(rule) => rule.matcher(),
            NodeRule::MD028(rule) => rule.matcher(),
            NodeRule::MD029(rule) => rule.matcher(),
        }
    }

    #[inline]
    pub fn reset(&mut self) {
        match self {
            NodeRule::MD001(rule) => rule.reset(),
            NodeRule::MD002(rule) => rule.reset(),
            NodeRule::MD003(rule) => rule.reset(),
            NodeRule::MD004(rule) => rule.reset(),
            NodeRule::MD005(rule) => rule.reset(),
            NodeRule::MD006(rule) => rule.reset(),
            NodeRule::MD007(rule) => rule.reset(),
            NodeRule::MD014(rule) => rule.reset(),
            NodeRule::MD018(rule) => rule.reset(),
            NodeRule::MD019(rule) => rule.reset(),
            NodeRule::MD022(rule) => rule.reset(),
            NodeRule::MD023(rule) => rule.reset(),
            NodeRule::MD024(rule) => rule.reset(),
            NodeRule::MD025(rule) => rule.reset(),
            NodeRule::MD026(rule) => rule.reset(),
            NodeRule::MD027(rule) => rule.reset(),
            NodeRule::MD028(rule) => rule.reset(),
            NodeRule::MD029(rule) => rule.reset(),
        }
    }
}
