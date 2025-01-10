use std::path::PathBuf;

use comrak::nodes::{AstNode, Sourcepos};
use line::{LineContext, LineMatcher, LineRule};
use miette::Result;
use node::{NodeContext, NodeRule, NodeValueMatcher};

use crate::{violation::Violation, Document};

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
pub enum Rule {
    MD001(MD001),
    MD002(MD002),
    MD003(MD003),
    MD004(MD004),
    MD005(MD005),
    MD006(MD006),
    MD007(MD007),
    MD009(MD009),
    MD010(MD010),
    MD012(MD012),
    MD013(MD013),
    MD014(MD014),
    MD018(MD018),
    MD019(MD019),
    MD020(MD020),
    MD021(MD021),
    MD022(MD022),
    MD023(MD023),
    MD024(MD024),
    MD025(MD025),
    MD026(MD026),
    MD027(MD027),
    MD028(MD028),
    MD029(MD029),
    MD030(MD030),
    MD031(MD031),
    MD032(MD032),
    MD033(MD033),
    MD034(MD034),
    MD035(MD035),
    MD036(MD036),
    MD037(MD037),
    MD038(MD038),
    MD039(MD039),
    MD040(MD040),
    MD041(MD041),
    MD046(MD046),
    MD047(MD047),
}

impl Rule {
    #[inline]
    pub fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        match self {
            Rule::MD001(rule) => rule.check(doc),
            Rule::MD002(rule) => rule.check(doc),
            Rule::MD003(rule) => rule.check(doc),
            Rule::MD004(rule) => rule.check(doc),
            Rule::MD005(rule) => rule.check(doc),
            Rule::MD006(rule) => rule.check(doc),
            Rule::MD007(rule) => rule.check(doc),
            Rule::MD009(rule) => rule.check(doc),
            Rule::MD010(rule) => rule.check(doc),
            Rule::MD012(rule) => rule.check(doc),
            Rule::MD013(rule) => rule.check(doc),
            Rule::MD014(rule) => rule.check(doc),
            Rule::MD018(rule) => rule.check(doc),
            Rule::MD019(rule) => rule.check(doc),
            Rule::MD020(rule) => rule.check(doc),
            Rule::MD021(rule) => rule.check(doc),
            Rule::MD022(rule) => rule.check(doc),
            Rule::MD023(rule) => rule.check(doc),
            Rule::MD024(rule) => rule.check(doc),
            Rule::MD025(rule) => rule.check(doc),
            Rule::MD026(rule) => rule.check(doc),
            Rule::MD027(rule) => rule.check(doc),
            Rule::MD028(rule) => rule.check(doc),
            Rule::MD029(rule) => rule.check(doc),
            Rule::MD030(rule) => rule.check(doc),
            Rule::MD031(rule) => rule.check(doc),
            Rule::MD032(rule) => rule.check(doc),
            Rule::MD033(rule) => rule.check(doc),
            Rule::MD034(rule) => rule.check(doc),
            Rule::MD035(rule) => rule.check(doc),
            Rule::MD036(rule) => rule.check(doc),
            Rule::MD037(rule) => rule.check(doc),
            Rule::MD038(rule) => rule.check(doc),
            Rule::MD039(rule) => rule.check(doc),
            Rule::MD040(rule) => rule.check(doc),
            Rule::MD041(rule) => rule.check(doc),
            Rule::MD046(rule) => rule.check(doc),
            Rule::MD047(rule) => rule.check(doc),
        }
    }

    #[inline]
    pub fn run_node<'a>(
        &mut self,
        ctx: &NodeContext,
        node: &'a AstNode<'a>,
    ) -> Option<Result<Vec<Violation>>> {
        match self {
            Rule::MD001(rule) => Some(rule.run(ctx, node)),
            Rule::MD002(rule) => Some(rule.run(ctx, node)),
            Rule::MD003(rule) => Some(rule.run(ctx, node)),
            Rule::MD004(rule) => Some(rule.run(ctx, node)),
            Rule::MD005(rule) => Some(rule.run(ctx, node)),
            Rule::MD006(rule) => Some(rule.run(ctx, node)),
            Rule::MD007(rule) => Some(rule.run(ctx, node)),
            Rule::MD009(_rule) => None,
            Rule::MD010(_rule) => None,
            Rule::MD012(_rule) => None,
            Rule::MD013(_rule) => None,
            Rule::MD014(rule) => Some(rule.run(ctx, node)),
            Rule::MD018(rule) => Some(rule.run(ctx, node)),
            Rule::MD019(rule) => Some(rule.run(ctx, node)),
            Rule::MD020(_rule) => None,
            Rule::MD021(_rule) => None,
            Rule::MD022(rule) => Some(rule.run(ctx, node)),
            Rule::MD023(rule) => Some(rule.run(ctx, node)),
            Rule::MD024(rule) => Some(rule.run(ctx, node)),
            Rule::MD025(rule) => Some(rule.run(ctx, node)),
            Rule::MD026(rule) => Some(rule.run(ctx, node)),
            Rule::MD027(rule) => Some(rule.run(ctx, node)),
            Rule::MD028(rule) => Some(rule.run(ctx, node)),
            Rule::MD029(rule) => Some(rule.run(ctx, node)),
            Rule::MD030(_rule) => None,
            Rule::MD031(_rule) => None,
            Rule::MD032(_rule) => None,
            Rule::MD033(_rule) => None,
            Rule::MD034(_rule) => None,
            Rule::MD035(_rule) => None,
            Rule::MD036(_rule) => None,
            Rule::MD037(_rule) => None,
            Rule::MD038(_rule) => None,
            Rule::MD039(_rule) => None,
            Rule::MD040(_rule) => None,
            Rule::MD041(_rule) => None,
            Rule::MD046(_rule) => None,
            Rule::MD047(_rule) => None,
        }
    }

    #[inline]
    pub fn run_line(&mut self, ctx: &LineContext, line: &str) -> Option<Result<Vec<Violation>>> {
        match self {
            Rule::MD001(_rule) => None,
            Rule::MD002(_rule) => None,
            Rule::MD003(_rule) => None,
            Rule::MD004(_rule) => None,
            Rule::MD005(_rule) => None,
            Rule::MD006(_rule) => None,
            Rule::MD007(_rule) => None,
            Rule::MD009(rule) => Some(rule.run(ctx, line)),
            Rule::MD010(rule) => Some(rule.run(ctx, line)),
            Rule::MD012(_rule) => None,
            Rule::MD013(rule) => Some(rule.run(ctx, line)),
            Rule::MD014(_rule) => None,
            Rule::MD018(_rule) => None,
            Rule::MD019(_rule) => None,
            Rule::MD020(_rule) => None,
            Rule::MD021(_rule) => None,
            Rule::MD022(_rule) => None,
            Rule::MD023(_rule) => None,
            Rule::MD024(_rule) => None,
            Rule::MD025(_rule) => None,
            Rule::MD026(_rule) => None,
            Rule::MD027(_rule) => None,
            Rule::MD028(_rule) => None,
            Rule::MD029(_rule) => None,
            Rule::MD030(_rule) => None,
            Rule::MD031(_rule) => None,
            Rule::MD032(_rule) => None,
            Rule::MD033(_rule) => None,
            Rule::MD034(_rule) => None,
            Rule::MD035(_rule) => None,
            Rule::MD036(_rule) => None,
            Rule::MD037(_rule) => None,
            Rule::MD038(_rule) => None,
            Rule::MD039(_rule) => None,
            Rule::MD040(_rule) => None,
            Rule::MD041(_rule) => None,
            Rule::MD046(_rule) => None,
            Rule::MD047(_rule) => None,
        }
    }

    #[inline]
    pub fn node_matcher(&self) -> Option<NodeValueMatcher> {
        match self {
            Rule::MD001(rule) => Some(rule.matcher()),
            Rule::MD002(rule) => Some(rule.matcher()),
            Rule::MD003(rule) => Some(rule.matcher()),
            Rule::MD004(rule) => Some(rule.matcher()),
            Rule::MD005(rule) => Some(rule.matcher()),
            Rule::MD006(rule) => Some(rule.matcher()),
            Rule::MD007(rule) => Some(rule.matcher()),
            Rule::MD009(_rule) => None,
            Rule::MD010(_rule) => None,
            Rule::MD012(_rule) => None,
            Rule::MD013(_rule) => None,
            Rule::MD014(rule) => Some(rule.matcher()),
            Rule::MD018(rule) => Some(rule.matcher()),
            Rule::MD019(rule) => Some(rule.matcher()),
            Rule::MD020(_rule) => None,
            Rule::MD021(_rule) => None,
            Rule::MD022(rule) => Some(rule.matcher()),
            Rule::MD023(rule) => Some(rule.matcher()),
            Rule::MD024(rule) => Some(rule.matcher()),
            Rule::MD025(rule) => Some(rule.matcher()),
            Rule::MD026(rule) => Some(rule.matcher()),
            Rule::MD027(rule) => Some(rule.matcher()),
            Rule::MD028(rule) => Some(rule.matcher()),
            Rule::MD029(rule) => Some(rule.matcher()),
            Rule::MD030(_rule) => None,
            Rule::MD031(_rule) => None,
            Rule::MD032(_rule) => None,
            Rule::MD033(_rule) => None,
            Rule::MD034(_rule) => None,
            Rule::MD035(_rule) => None,
            Rule::MD036(_rule) => None,
            Rule::MD037(_rule) => None,
            Rule::MD038(_rule) => None,
            Rule::MD039(_rule) => None,
            Rule::MD040(_rule) => None,
            Rule::MD041(_rule) => None,
            Rule::MD046(_rule) => None,
            Rule::MD047(_rule) => None,
        }
    }

    #[inline]
    pub fn line_matcher(&self) -> Option<LineMatcher> {
        match self {
            Rule::MD001(_rule) => None,
            Rule::MD002(_rule) => None,
            Rule::MD003(_rule) => None,
            Rule::MD004(_rule) => None,
            Rule::MD005(_rule) => None,
            Rule::MD006(_rule) => None,
            Rule::MD007(_rule) => None,
            Rule::MD009(rule) => Some(rule.matcher()),
            Rule::MD010(rule) => Some(rule.matcher()),
            Rule::MD012(_rule) => None,
            Rule::MD013(rule) => Some(rule.matcher()),
            Rule::MD014(_rule) => None,
            Rule::MD018(_rule) => None,
            Rule::MD019(_rule) => None,
            Rule::MD020(_rule) => None,
            Rule::MD021(_rule) => None,
            Rule::MD022(_rule) => None,
            Rule::MD023(_rule) => None,
            Rule::MD024(_rule) => None,
            Rule::MD025(_rule) => None,
            Rule::MD026(_rule) => None,
            Rule::MD027(_rule) => None,
            Rule::MD028(_rule) => None,
            Rule::MD029(_rule) => None,
            Rule::MD030(_rule) => None,
            Rule::MD031(_rule) => None,
            Rule::MD032(_rule) => None,
            Rule::MD033(_rule) => None,
            Rule::MD034(_rule) => None,
            Rule::MD035(_rule) => None,
            Rule::MD036(_rule) => None,
            Rule::MD037(_rule) => None,
            Rule::MD038(_rule) => None,
            Rule::MD039(_rule) => None,
            Rule::MD040(_rule) => None,
            Rule::MD041(_rule) => None,
            Rule::MD046(_rule) => None,
            Rule::MD047(_rule) => None,
        }
    }

    #[inline]
    pub fn reset(&mut self) {
        match self {
            Rule::MD001(rule) => rule.reset(),
            Rule::MD002(rule) => rule.reset(),
            Rule::MD003(rule) => rule.reset(),
            Rule::MD004(rule) => rule.reset(),
            Rule::MD005(rule) => rule.reset(),
            Rule::MD006(rule) => rule.reset(),
            Rule::MD007(rule) => rule.reset(),
            Rule::MD009(rule) => rule.reset(),
            Rule::MD010(rule) => rule.reset(),
            Rule::MD012(_rule) => {}
            Rule::MD013(rule) => rule.reset(),
            Rule::MD014(rule) => rule.reset(),
            Rule::MD018(rule) => rule.reset(),
            Rule::MD019(rule) => rule.reset(),
            Rule::MD020(_rule) => {}
            Rule::MD021(_rule) => {}
            Rule::MD022(rule) => rule.reset(),
            Rule::MD023(rule) => rule.reset(),
            Rule::MD024(rule) => rule.reset(),
            Rule::MD025(rule) => rule.reset(),
            Rule::MD026(rule) => rule.reset(),
            Rule::MD027(rule) => rule.reset(),
            Rule::MD028(rule) => rule.reset(),
            Rule::MD029(rule) => rule.reset(),
            Rule::MD030(_rule) => {}
            Rule::MD031(_rule) => {}
            Rule::MD032(_rule) => {}
            Rule::MD033(_rule) => {}
            Rule::MD034(_rule) => {}
            Rule::MD035(_rule) => {}
            Rule::MD036(_rule) => {}
            Rule::MD037(_rule) => {}
            Rule::MD038(_rule) => {}
            Rule::MD039(_rule) => {}
            Rule::MD040(_rule) => {}
            Rule::MD041(_rule) => {}
            Rule::MD046(_rule) => {}
            Rule::MD047(_rule) => {}
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
