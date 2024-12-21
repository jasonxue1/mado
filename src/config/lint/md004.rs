use serde::Deserialize;

use crate::rule;
use crate::rule::md004::ListStyle;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
#[allow(clippy::exhaustive_structs)]
pub struct MD004 {
    pub style: ListStyle,
}

impl Default for MD004 {
    #[inline]
    fn default() -> Self {
        Self {
            style: rule::MD004::DEFAULT_LIST_STYLE,
        }
    }
}
