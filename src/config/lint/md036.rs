use serde::Deserialize;

use crate::rule;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
#[allow(clippy::exhaustive_structs)]
pub struct MD036 {
    pub punctuation: String,
}

impl Default for MD036 {
    #[inline]
    fn default() -> Self {
        Self {
            punctuation: rule::MD036::DEFAULT_PUNCTUATION.to_owned(),
        }
    }
}
