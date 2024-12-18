use serde::Deserialize;

use crate::rule::md004::ListStyle;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct MD004 {
    pub style: ListStyle,
}

impl Default for MD004 {
    fn default() -> Self {
        Self {
            style: ListStyle::Consistent,
        }
    }
}
