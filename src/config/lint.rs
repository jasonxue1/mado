use serde::Deserialize;

use crate::output::Format;

mod md002;
mod md003;
mod md004;
mod md007;
mod md013;
mod md025;
mod md026;
mod md029;
mod md030;
mod md033;
mod md035;
mod md036;
mod md041;
mod md046;

pub use md002::MD002;
pub use md003::MD003;
pub use md004::MD004;
pub use md007::MD007;
pub use md013::MD013;
pub use md025::MD025;
pub use md026::MD026;
pub use md029::MD029;
pub use md030::MD030;
pub use md033::MD033;
pub use md035::MD035;
pub use md036::MD036;
pub use md041::MD041;
pub use md046::MD046;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
#[allow(clippy::exhaustive_structs)]
pub struct Lint {
    pub output_format: Format,
    pub rules: Vec<RuleSet>,
    pub md002: MD002,
    pub md003: MD003,
    pub md004: MD004,
    pub md007: MD007,
    pub md013: MD013,
    pub md025: MD025,
    pub md026: MD026,
    pub md029: MD029,
    pub md030: MD030,
    pub md033: MD033,
    pub md035: MD035,
    pub md036: MD036,
    pub md041: MD041,
    pub md046: MD046,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[non_exhaustive]
pub enum RuleSet {
    MD001,
    MD002,
    MD003,
    MD004,
    MD005,
    MD006,
    MD007,
    MD009,
    MD010,
    MD012,
    MD013,
    MD014,
    MD018,
    MD019,
    MD020,
    MD021,
    MD022,
    MD023,
    MD024,
    MD025,
    MD026,
    MD027,
    MD028,
    MD029,
    MD030,
    MD031,
    MD032,
    MD033,
    MD034,
    MD035,
    MD036,
    MD037,
    MD038,
    MD039,
    MD040,
    MD041,
    MD046,
    MD047,
}

impl Default for Lint {
    #[inline]
    fn default() -> Self {
        Self {
            output_format: Format::Concise,
            rules: vec![
                RuleSet::MD001,
                RuleSet::MD002,
                RuleSet::MD003,
                RuleSet::MD004,
                RuleSet::MD005,
                RuleSet::MD006,
                RuleSet::MD007,
                RuleSet::MD009,
                RuleSet::MD010,
                RuleSet::MD012,
                RuleSet::MD013,
                RuleSet::MD014,
                RuleSet::MD018,
                RuleSet::MD019,
                RuleSet::MD020,
                RuleSet::MD021,
                RuleSet::MD022,
                RuleSet::MD023,
                RuleSet::MD024,
                RuleSet::MD025,
                RuleSet::MD026,
                RuleSet::MD027,
                RuleSet::MD028,
                RuleSet::MD029,
                RuleSet::MD030,
                RuleSet::MD031,
                RuleSet::MD032,
                RuleSet::MD033,
                RuleSet::MD034,
                RuleSet::MD035,
                RuleSet::MD036,
                RuleSet::MD037,
                RuleSet::MD038,
                RuleSet::MD039,
                RuleSet::MD040,
                RuleSet::MD041,
                RuleSet::MD046,
                RuleSet::MD047,
            ],
            md002: MD002::default(),
            md003: MD003::default(),
            md004: MD004::default(),
            md007: MD007::default(),
            md013: MD013::default(),
            md025: MD025::default(),
            md026: MD026::default(),
            md029: MD029::default(),
            md030: MD030::default(),
            md033: MD033::default(),
            md035: MD035::default(),
            md036: MD036::default(),
            md041: MD041::default(),
            md046: MD046::default(),
        }
    }
}
