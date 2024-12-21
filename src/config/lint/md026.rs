use serde::Deserialize;

use crate::rule;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
#[allow(clippy::exhaustive_structs)]
pub struct MD026 {
    pub punctuation: String,
}

impl Default for MD026 {
    #[inline]
    fn default() -> Self {
        Self {
            punctuation: rule::MD026::DEFAULT_PUNCTUATION.to_owned(),
        }
    }
}
