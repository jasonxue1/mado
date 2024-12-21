use serde::Deserialize;

use crate::rule;
use crate::rule::md003::HeadingStyle;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
#[allow(clippy::exhaustive_structs)]
pub struct MD003 {
    pub style: HeadingStyle,
}

impl Default for MD003 {
    #[inline]
    fn default() -> Self {
        Self {
            style: rule::MD003::DEFAULT_HEADING_STYLE,
        }
    }
}
