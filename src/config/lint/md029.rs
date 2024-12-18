use serde::Deserialize;

use crate::rule::md029::OrderedListStyle;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct MD029 {
    pub style: OrderedListStyle,
}

impl Default for MD029 {
    fn default() -> Self {
        Self {
            style: OrderedListStyle::One,
        }
    }
}
