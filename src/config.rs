use serde::Deserialize;

use crate::Rule;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    pub rules: Vec<Rule>,
}
