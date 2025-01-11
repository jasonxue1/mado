use std::path::PathBuf;

use comrak::nodes::{AstNode, Sourcepos};
use miette::Result;

use crate::{violation::Violation, Document};
use line::{LineContext, LineMatcher, LineRule as _};
use node::{NodeContext, NodeRule as _, NodeValueMatcher};

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

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum NodeRuleEnum {
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

impl NodeRuleEnum {
    #[inline]
    pub fn run<'a>(&mut self, ctx: &NodeContext, node: &'a AstNode<'a>) -> Result<Vec<Violation>> {
        match self {
            NodeRuleEnum::MD001(rule) => rule.run(ctx, node),
            NodeRuleEnum::MD002(rule) => rule.run(ctx, node),
            NodeRuleEnum::MD003(rule) => rule.run(ctx, node),
            NodeRuleEnum::MD004(rule) => rule.run(ctx, node),
            NodeRuleEnum::MD005(rule) => rule.run(ctx, node),
            NodeRuleEnum::MD006(rule) => rule.run(ctx, node),
            NodeRuleEnum::MD007(rule) => rule.run(ctx, node),
            NodeRuleEnum::MD014(rule) => rule.run(ctx, node),
            NodeRuleEnum::MD018(rule) => rule.run(ctx, node),
            NodeRuleEnum::MD019(rule) => rule.run(ctx, node),
            NodeRuleEnum::MD022(rule) => rule.run(ctx, node),
            NodeRuleEnum::MD023(rule) => rule.run(ctx, node),
            NodeRuleEnum::MD024(rule) => rule.run(ctx, node),
            NodeRuleEnum::MD025(rule) => rule.run(ctx, node),
            NodeRuleEnum::MD026(rule) => rule.run(ctx, node),
            NodeRuleEnum::MD027(rule) => rule.run(ctx, node),
            NodeRuleEnum::MD028(rule) => rule.run(ctx, node),
            NodeRuleEnum::MD029(rule) => rule.run(ctx, node),
        }
    }

    #[inline]
    pub fn matcher(&self) -> NodeValueMatcher {
        match self {
            NodeRuleEnum::MD001(rule) => rule.matcher(),
            NodeRuleEnum::MD002(rule) => rule.matcher(),
            NodeRuleEnum::MD003(rule) => rule.matcher(),
            NodeRuleEnum::MD004(rule) => rule.matcher(),
            NodeRuleEnum::MD005(rule) => rule.matcher(),
            NodeRuleEnum::MD006(rule) => rule.matcher(),
            NodeRuleEnum::MD007(rule) => rule.matcher(),
            NodeRuleEnum::MD014(rule) => rule.matcher(),
            NodeRuleEnum::MD018(rule) => rule.matcher(),
            NodeRuleEnum::MD019(rule) => rule.matcher(),
            NodeRuleEnum::MD022(rule) => rule.matcher(),
            NodeRuleEnum::MD023(rule) => rule.matcher(),
            NodeRuleEnum::MD024(rule) => rule.matcher(),
            NodeRuleEnum::MD025(rule) => rule.matcher(),
            NodeRuleEnum::MD026(rule) => rule.matcher(),
            NodeRuleEnum::MD027(rule) => rule.matcher(),
            NodeRuleEnum::MD028(rule) => rule.matcher(),
            NodeRuleEnum::MD029(rule) => rule.matcher(),
        }
    }

    #[inline]
    pub fn reset(&mut self) {
        match self {
            NodeRuleEnum::MD001(rule) => rule.reset(),
            NodeRuleEnum::MD002(rule) => rule.reset(),
            NodeRuleEnum::MD003(rule) => rule.reset(),
            NodeRuleEnum::MD004(rule) => rule.reset(),
            NodeRuleEnum::MD005(rule) => rule.reset(),
            NodeRuleEnum::MD006(rule) => rule.reset(),
            NodeRuleEnum::MD007(rule) => rule.reset(),
            NodeRuleEnum::MD014(rule) => rule.reset(),
            NodeRuleEnum::MD018(rule) => rule.reset(),
            NodeRuleEnum::MD019(rule) => rule.reset(),
            NodeRuleEnum::MD022(rule) => rule.reset(),
            NodeRuleEnum::MD023(rule) => rule.reset(),
            NodeRuleEnum::MD024(rule) => rule.reset(),
            NodeRuleEnum::MD025(rule) => rule.reset(),
            NodeRuleEnum::MD026(rule) => rule.reset(),
            NodeRuleEnum::MD027(rule) => rule.reset(),
            NodeRuleEnum::MD028(rule) => rule.reset(),
            NodeRuleEnum::MD029(rule) => rule.reset(),
        }
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum LineRuleEnum {
    MD009(MD009),
    MD010(MD010),
    MD013(MD013),
}

impl LineRuleEnum {
    #[inline]
    pub fn run(&mut self, ctx: &LineContext, line: &str) -> Result<Vec<Violation>> {
        match self {
            LineRuleEnum::MD009(rule) => rule.run(ctx, line),
            LineRuleEnum::MD010(rule) => rule.run(ctx, line),
            LineRuleEnum::MD013(rule) => rule.run(ctx, line),
        }
    }

    #[inline]
    pub fn matcher(&self) -> LineMatcher {
        match self {
            LineRuleEnum::MD009(rule) => rule.matcher(),
            LineRuleEnum::MD010(rule) => rule.matcher(),
            LineRuleEnum::MD013(rule) => rule.matcher(),
        }
    }

    #[inline]
    pub fn reset(&mut self) {
        match self {
            LineRuleEnum::MD009(rule) => rule.reset(),
            LineRuleEnum::MD010(rule) => rule.reset(),
            LineRuleEnum::MD013(rule) => rule.reset(),
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

pub trait NewRuleLike: Send {
    fn metadata(&self) -> RuleMetadata;

    fn reset(&mut self);
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuleKind {
    Node,
    Line,
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
