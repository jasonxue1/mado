use serde::Deserialize;

use crate::rule;
use crate::rule::md035::HorizontalRuleStyle;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
#[allow(clippy::exhaustive_structs)]
pub struct MD035 {
    pub style: HorizontalRuleStyle,
}

impl Default for MD035 {
    #[inline]
    fn default() -> Self {
        Self {
            style: rule::MD035::DEFAULT_STYLE,
        }
    }
}
