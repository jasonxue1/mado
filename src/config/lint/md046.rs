use serde::Deserialize;

use crate::rule::md046::CodeBlockStyle;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct MD046 {
    pub style: CodeBlockStyle,
}

impl Default for MD046 {
    fn default() -> Self {
        Self {
            style: CodeBlockStyle::Fenced,
        }
    }
}
