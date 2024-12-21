use serde::Deserialize;

use crate::rule;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
#[allow(clippy::exhaustive_structs)]
pub struct MD025 {
    pub level: u8,
}

impl Default for MD025 {
    #[inline]
    fn default() -> Self {
        Self {
            level: rule::MD025::DEFAULT_LEVEL,
        }
    }
}
