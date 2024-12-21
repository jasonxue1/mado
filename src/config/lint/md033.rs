use serde::Deserialize;

use crate::rule;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
#[allow(clippy::exhaustive_structs)]
pub struct MD033 {
    pub allowed_elements: Vec<String>,
}

impl Default for MD033 {
    #[inline]
    fn default() -> Self {
        Self {
            allowed_elements: rule::MD033::DEFAULT_ALLOWED_ELEMENTS,
        }
    }
}
