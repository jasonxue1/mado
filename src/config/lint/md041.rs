use serde::Deserialize;

use crate::rule;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
#[allow(clippy::exhaustive_structs)]
pub struct MD041 {
    pub level: u8,
}

impl Default for MD041 {
    #[inline]
    fn default() -> Self {
        Self {
            level: rule::MD041::DEFAULT_LEVEL,
        }
    }
}
