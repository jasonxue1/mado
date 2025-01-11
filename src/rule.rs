use std::path::PathBuf;

use comrak::nodes::{AstNode, Sourcepos};
use miette::Result;

use crate::{violation::Violation, Document};
use line::{LineContext, LineMatcher};
use node::{NodeContext, NodeValueMatcher};

mod helper;
pub mod line;
mod md001;
mod md002;
pub mod md003;
pub mod md004;
mod md005;
mod md006;
mod md007;
mod md009;
mod md010;
mod md012;
mod md013;
mod md014;
mod md018;
mod md019;
mod md020;
mod md021;
mod md022;
mod md023;
mod md024;
mod md025;
mod md026;
mod md027;
mod md028;
pub mod md029;
mod md030;
mod md031;
mod md032;
mod md033;
mod md034;
pub mod md035;
mod md036;
mod md037;
mod md038;
mod md039;
mod md040;
mod md041;
pub mod md046;
mod md047;
pub mod node;

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
    pub fn matcher(&self) -> NodeValueMatcher {
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

#[derive(Debug, Clone)]
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

pub trait RuleLike: Send {
    fn name(&self) -> &'static str;

    fn description(&self) -> &'static str;

    fn tags(&self) -> Vec<&'static str>;

    fn aliases(&self) -> Vec<&'static str>;

    fn check(&self, doc: &Document) -> Result<Vec<Violation>>;

    #[inline]
    fn to_violation(&self, path: PathBuf, position: Sourcepos) -> Violation {
        Violation::new(
            path,
            self.name().to_owned(),
            self.aliases()[0].to_owned(),
            self.description().to_owned(),
            position,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct RuleMetadata {
    pub name: &'static str,
    pub description: &'static str,
    pub tags: Vec<&'static str>,
    pub aliases: Vec<&'static str>,
}

pub trait Matcher<A> {
    #[must_use]
    fn is_match(&self, value: A) -> bool;
}

pub trait Rule<Ctx, A, M: Matcher<A>>: Send {
    #[must_use]
    fn metadata(&self) -> RuleMetadata;

    #[must_use]
    fn matcher(&self) -> M;

    fn run(&mut self, ctx: Ctx, value: A) -> Result<Vec<Violation>>;

    fn reset(&mut self) {}
}

pub use md001::MD001;
pub use md002::MD002;
pub use md003::MD003;
pub use md004::MD004;
pub use md005::MD005;
pub use md006::MD006;
pub use md007::MD007;
pub use md009::MD009;
pub use md010::MD010;
pub use md012::MD012;
pub use md013::MD013;
pub use md014::MD014;
pub use md018::MD018;
pub use md019::MD019;
pub use md020::MD020;
pub use md021::MD021;
pub use md022::MD022;
pub use md023::MD023;
pub use md024::MD024;
pub use md025::MD025;
pub use md026::MD026;
pub use md027::MD027;
pub use md028::MD028;
pub use md029::MD029;
pub use md030::MD030;
pub use md031::MD031;
pub use md032::MD032;
pub use md033::MD033;
pub use md034::MD034;
pub use md035::MD035;
pub use md036::MD036;
pub use md037::MD037;
pub use md038::MD038;
pub use md039::MD039;
pub use md040::MD040;
pub use md041::MD041;
pub use md046::MD046;
pub use md047::MD047;
