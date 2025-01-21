use serde::{Deserialize, Serialize};

use crate::rule;
use crate::rule::md004::ListStyle;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

impl From<&MD004> for rule::MD004 {
    #[inline]
    fn from(config: &MD004) -> Self {
        Self::new(config.style.clone())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn from_for_rule_md004() {
        let style = ListStyle::Asterisk;
        let config = MD004 {
            style: style.clone(),
        };
        let expected = rule::MD004::new(style);
        assert_eq!(rule::MD004::from(&config), expected);
    }
}
