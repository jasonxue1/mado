use serde::Deserialize;

use crate::rule;
use crate::rule::md029::OrderedListStyle;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(default)]
#[allow(clippy::exhaustive_structs)]
pub struct MD029 {
    pub style: OrderedListStyle,
}

impl Default for MD029 {
    #[inline]
    fn default() -> Self {
        Self {
            style: rule::MD029::DEFAULT_STYLE,
        }
    }
}

impl From<&MD029> for rule::MD029 {
    #[inline]
    fn from(config: &MD029) -> rule::MD029 {
        rule::MD029::new(config.style.clone())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn from_for_rule_md029() {
        let style = OrderedListStyle::Ordered;
        let config = MD029 {
            style: style.clone(),
        };
        let expected = rule::MD029::new(style);
        assert_eq!(rule::MD029::from(&config), expected);
    }
}
