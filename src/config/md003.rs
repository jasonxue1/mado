use serde::Deserialize;

use crate::rule::md003::HeadingStyle;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct MD003 {
    pub style: HeadingStyle,
}

impl Default for MD003 {
    fn default() -> Self {
        Self {
            style: HeadingStyle::Consistent,
        }
    }
}
