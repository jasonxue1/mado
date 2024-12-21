use serde::Deserialize;

use crate::rule;
use crate::rule::md029::OrderedListStyle;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
#[allow(clippy::exhaustive_structs)]
pub struct MD029 {
    pub style: OrderedListStyle,
}

impl Default for MD029 {
    #[inline]
    fn default() -> Self {
        Self {
            style: rule::MD029::DEFAULT_STYLE,
        }
    }
}
