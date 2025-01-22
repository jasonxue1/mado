use serde::{Deserialize, Serialize};

use crate::rule;
use crate::rule::md035::HorizontalRuleStyle;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

impl From<&MD035> for rule::MD035 {
    #[inline]
    fn from(config: &MD035) -> Self {
        Self::new(config.style.clone())
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn deserialize_for_horizontal_rule_style_consistent() {
        let text = r#"style = "consistent""#;
        let config: MD035 = toml::from_str(text).unwrap();
        assert_eq!(config.style, HorizontalRuleStyle::Consistent);
    }

    #[test]
    fn deserialize_for_horizontal_rule_style_custom() {
        let text = r#"style = "~~~""#;
        let config: MD035 = toml::from_str(text).unwrap();
        assert_eq!(config.style, HorizontalRuleStyle::Custom("~~~".to_owned()));
    }

    #[test]
    fn from_for_rule_md035() {
        let style = HorizontalRuleStyle::Custom("~~~".to_owned());
        let config = MD035 {
            style: style.clone(),
        };
        let expected = rule::MD035::new(style);
        assert_eq!(rule::MD035::from(&config), expected);
    }
}
