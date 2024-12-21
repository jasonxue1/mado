use serde::Deserialize;

use crate::rule;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
#[allow(clippy::exhaustive_structs)]
pub struct MD007 {
    pub indent: usize,
}

impl Default for MD007 {
    #[inline]
    fn default() -> Self {
        Self {
            indent: rule::MD007::DEFAULT_INDENT,
        }
    }
}
