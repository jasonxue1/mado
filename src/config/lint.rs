use serde::Deserialize;

use crate::{output::Format, Rule};

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
mod md036;

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
pub use md036::MD036;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Lint {
    pub output_format: Format,
    pub rules: Vec<Rule>,
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
    pub md036: MD036,
}

impl Default for Lint {
    fn default() -> Self {
        Self {
            output_format: Format::Concise,
            rules: vec![
                Rule::MD001,
                Rule::MD002,
                Rule::MD003,
                Rule::MD004,
                Rule::MD005,
                Rule::MD006,
                Rule::MD007,
                Rule::MD009,
                Rule::MD010,
                Rule::MD012,
                Rule::MD013,
                Rule::MD014,
                Rule::MD018,
                Rule::MD019,
                Rule::MD022,
                Rule::MD023,
                Rule::MD024,
                Rule::MD025,
                Rule::MD026,
                Rule::MD027,
                Rule::MD028,
                Rule::MD029,
                Rule::MD030,
                Rule::MD031,
                Rule::MD032,
                Rule::MD033,
                Rule::MD034,
                Rule::MD036,
                Rule::MD037,
                Rule::MD038,
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
            md036: MD036::default(),
        }
    }
}
