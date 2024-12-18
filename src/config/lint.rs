use serde::Deserialize;

use crate::{output::Format, Rule};

mod md002;
mod md003;
mod md004;
mod md007;
mod md013;

pub use md002::MD002;
pub use md003::MD003;
pub use md004::MD004;
pub use md007::MD007;
pub use md013::MD013;

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
            ],
            md002: MD002::default(),
            md003: MD003::default(),
            md004: MD004::default(),
            md007: MD007::default(),
            md013: MD013::default(),
        }
    }
}
