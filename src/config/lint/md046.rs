use serde::Deserialize;

use crate::rule;
use crate::rule::md046::CodeBlockStyle;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
#[allow(clippy::exhaustive_structs)]
pub struct MD046 {
    pub style: CodeBlockStyle,
}

impl Default for MD046 {
    #[inline]
    fn default() -> Self {
        Self {
            style: rule::MD046::DEFAULT_STYLE,
        }
    }
}
