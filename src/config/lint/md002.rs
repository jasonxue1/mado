use serde::Deserialize;

use crate::rule;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
#[allow(clippy::exhaustive_structs)]
pub struct MD002 {
    pub level: u8,
}

impl Default for MD002 {
    #[inline]
    fn default() -> Self {
        Self {
            level: rule::MD002::DEFAULT_LEVEL,
        }
    }
}
