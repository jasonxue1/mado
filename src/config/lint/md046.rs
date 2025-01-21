use serde::{Deserialize, Serialize};

use crate::rule;
use crate::rule::md046::CodeBlockStyle;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

impl From<&MD046> for rule::MD046 {
    #[inline]
    fn from(config: &MD046) -> Self {
        Self::new(config.style.clone())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::rule::md046::CodeBlockStyle;

    #[test]
    fn from_for_rule_md046() {
        let style = CodeBlockStyle::Indented;
        let config = MD046 {
            style: style.clone(),
        };
        let expected = rule::MD046::new(style);
        assert_eq!(rule::MD046::from(&config), expected);
    }
}
